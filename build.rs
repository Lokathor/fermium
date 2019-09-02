#![allow(unused_imports)]
#![allow(non_snake_case)]

use std::{
  env,
  path::{Path, PathBuf},
  process::Command,
};

fn main() {
  let use_bindgen_bin = cfg!(feature = "use_bindgen_bin");
  println!("use_bindgen_bin: {}", use_bindgen_bin);

  let link_dynamic = cfg!(feature = "link_dynamic");
  let link_static = cfg!(feature = "link_static");
  println!("link_dynamic: {}", link_dynamic);
  println!("link_static: {}", link_static);
  let link_count = link_dynamic as usize + link_static as usize;
  if link_count != 1 {
    panic!(
      "You must select exactly one linking type, you selected {}",
      link_count
    );
  }

  let bind_SDL2_2_0_9 = cfg!(feature = "bind_SDL2_2_0_9");
  let bind_SDL2_2_0_10 = cfg!(feature = "bind_SDL2_2_0_10");
  println!("bind_SDL2_2_0_9: {}", bind_SDL2_2_0_9);
  println!("bind_SDL2_2_0_10: {}", bind_SDL2_2_0_10);

  if cfg!(feature = "use_bindgen_bin") {
    run_bindgen_bin();
  }

  // If we're on windows with static linking, do the build. cmake returns the
  // link location to use, so we declare that right away.
  #[cfg(all(windows, feature = "link_static"))]
  println!(
    "cargo:rustc-link-search=native={}",
    win32_build_static_libs().join("lib").display()
  );

  declare_linking();
}

fn run_bindgen_bin() {
  //
  let current_dir = env::current_dir().expect("Couldn't read the current directory.");
  let wrapper_filename = current_dir.join("wrapper.h");

  //
  let out_dir = PathBuf::from(env::var("OUT_DIR").expect("Couldn't read `OUT_DIR`"));
  let bindings_filename = out_dir.join("bindings.rs");

  // build up the whole bindgen command
  let mut bindgen = Command::new("bindgen");
  // flags, TODO: investigate --generate-inline-functions
  bindgen.arg("--disable-name-namespacing");
  bindgen.arg("--impl-debug");
  bindgen.arg("--impl-partialeq");
  bindgen.arg("--no-doc-comments");
  bindgen.arg("--no-prepend-enum-name");
  bindgen.arg("--use-core");
  bindgen.arg("--with-derive-default");
  bindgen.arg("--with-derive-partialeq");
  // options
  #[cfg(not(windows))]
  bindgen.arg("--ctypes-prefix").arg("libc");
  #[cfg(windows)]
  bindgen.arg("--ctypes-prefix").arg("winapi::ctypes");
  bindgen.arg("--default-enum-style").arg("moduleconsts");
  bindgen
    .arg("--output")
    .arg(&format!("{}", bindings_filename.display()));
  bindgen.arg("--rust-target").arg("1.33");
  bindgen.arg("--rustfmt-configuration-file").arg(
    std::env::current_dir()
      .expect("couldn't get current directory!")
      .join("rustfmt.toml")
      .to_str()
      .expect("rustfmt.toml file path isn't valid utf8, stop that"),
  );
  bindgen.arg("--whitelist-function").arg("SDL_.*");
  bindgen.arg("--whitelist-type").arg("SDL_.*");
  bindgen.arg("--whitelist-var").arg("SDL_.*");
  bindgen.arg("--whitelist-var").arg("AUDIO_.*");
  bindgen.arg("--whitelist-var").arg("SDLK_.*");
  // header
  bindgen.arg(&wrapper_filename);
  // mario kart double dash
  bindgen.arg("--");
  // clang args
  bindgen.arg("--no-warnings");
  if cfg!(feature = "bind_version_2_0_10") {
    bindgen.arg("-DBINDGEN_2_0_10");
  } else if cfg!(feature = "bind_version_2_0_9") {
    bindgen.arg("-DBINDGEN_2_0_9");
  } else {
    bindgen.arg("-DBINDGEN_2_0_8");
  }

  println!("bindgen version check: {}", {
    let mut bindgen_version = Command::new("bindgen");
    bindgen_version.arg("--version");
    let version_out = bindgen_version
      .output()
      .expect("Couldn't execute bindgen version check!");
    if version_out.status.success() {
      String::from_utf8_lossy(&version_out.stdout).into_owned()
    } else {
      panic!("bindgen version check failed!");
    }
  });
  println!("executing command: {:?}", bindgen);
  let bindgen_output = bindgen
    .output()
    .expect("Couldn't run 'bindgen', perhaps you need to 'cargo install bindgen'?");
  if bindgen_output.status.success() {
    println!("command success!")
  } else {
    println!("command failure!")
  }
  for line in String::from_utf8_lossy(&bindgen_output.stdout).lines() {
    println!("OUT:{}", line);
  }
  for line in String::from_utf8_lossy(&bindgen_output.stderr).lines() {
    println!("ERR:{}", line);
  }
  if !bindgen_output.status.success() {
    panic!("The 'bindgen' command failed! (see output log for details)");
  }
}

// Note: In addition to only calling this as needed, we must also cfg it to only
// exist as needed so that we can avoid building in the `cmake` crate unless
// it's really gonna be used.
#[cfg(all(windows, feature = "link_static"))]
fn win32_build_static_libs() -> PathBuf {
  let manifest_dir =
    PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("Could not read `CARGO_MANIFEST_DIR`!"));
  let mut cfg = cmake::Config::new(manifest_dir.join("full-source").join("SDL2-v2.0.10"));
  cfg.profile("release");
  cfg.static_crt(true);

  if cfg!(target_env = "gnu") {
    cfg.define("VIDEO_OPENGLES", "OFF");
  }

  if cfg!(feature = "link_dynamic") {
    cfg.define("SDL_SHARED", "ON");
    cfg.define("SDL_STATIC", "OFF");
  } else if cfg!(feature = "link_static") {
    cfg.define("SDL_SHARED", "OFF");
    cfg.define("SDL_STATIC", "ON");
  } else {
    panic!("You should have selected a link mode!");
  }

  cfg.build()
}

fn declare_linking() {
  if cfg!(windows) {
    declare_win32_linking()
  } else {
    declare_sd2_config_linking()
  }
}

fn declare_win32_linking() {
  // What to link
  if cfg!(feature = "link_dynamic") {
    println!("cargo:rustc-link-lib=SDL2");
  } else {
    println!("cargo:rustc-link-lib=static=SDL2");
    // Note(Lokathor): this magical seeming list comes from the CMakeLists.txt,
    // if you search for "Libraries for Win32" you'll find it.
    println!("cargo:rustc-link-lib=user32");
    println!("cargo:rustc-link-lib=gdi32");
    println!("cargo:rustc-link-lib=winmm");
    println!("cargo:rustc-link-lib=imm32");
    println!("cargo:rustc-link-lib=ole32");
    println!("cargo:rustc-link-lib=oleaut32");
    println!("cargo:rustc-link-lib=version");
    println!("cargo:rustc-link-lib=uuid");
    println!("cargo:rustc-link-lib=advapi32");
    println!("cargo:rustc-link-lib=setupapi");
    println!("cargo:rustc-link-lib=shell32");
  }

  // where to look
  let manifest_dir =
    PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("Could not read `CARGO_MANIFEST_DIR`!"));

  if cfg!(feature = "link_dynamic") {
    let sub_directory: &str = if cfg!(target_env = "gnu") {
      if cfg!(target_arch = "x86") {
        "win32-devel-files\\mingw\\i686-w64-mingw32\\lib"
      } else if cfg!(target_arch = "x86_64") {
        "win32-devel-files\\mingw\\x86_64-w64-mingw32\\lib"
      } else {
        panic!("No provided library files for this CPU type.")
      }
    } else if cfg!(target_env = "msvc") {
      if cfg!(target_arch = "x86") {
        "win32-devel-files\\VC\\lib\\x86"
      } else if cfg!(target_arch = "x86_64") {
        "win32-devel-files\\VC\\lib\\x64"
      } else {
        panic!("No provided library files for this CPU type.")
      }
    } else {
      panic!("Unknown 'target_env' value");
    };
    println!(
      "cargo:rustc-link-search=native={}",
      manifest_dir.join(sub_directory).display()
    );
  } else if cfg!(feature = "link_static") {
    println!("link search should have been emitted during the cmake build.");
  } else {
    panic!("You didn't select a link mode!");
  }
}

fn declare_sd2_config_linking() {
  // Verify that sdl2-config exists and supports the linking we want. The output
  // of this should go to stderr.
  let sdl2_config_usage = Command::new("sdl2-config")
    .output()
    .expect("couldn't run `sdl2-config`, please properly install SDL2.");
  assert!(!sdl2_config_usage.status.success());
  let usage_out_string = String::from_utf8_lossy(&sdl2_config_usage.stderr);
  println!("sdl2-config: {}", usage_out_string);
  let usage_words: Vec<String> = usage_out_string
    .split_whitespace()
    .map(|s| s.to_string())
    .collect();
  assert!(
    &usage_words[0] == "Usage:",
    "Unexpected usage message, aborting!"
  );
  if cfg!(feature = "link_dynamic") {
    assert!(
      usage_words.contains(&"[--libs]".to_string()),
      "This SDL2 install is not built for dynamic linking!"
    );
  }
  if cfg!(feature = "link_static") {
    assert!(
      usage_words.contains(&"[--static-libs]".to_string()),
      "This SDL2 install is not built for dynamic linking!"
    );
  }

  // Verify that the version installed is at least as much as the user is using
  // bindings for.
  let sdl2_config_version = Command::new("sdl2-config")
    .arg("--version")
    .output()
    .expect("couldn't run `sdl2-config`, please properly install SDL2.");
  assert!(sdl2_config_version.status.success());
  let version_out_string = String::from_utf8_lossy(&sdl2_config_version.stdout);
  println!("sdl2-config --version: {}", version_out_string);
  let version_parts: Vec<u32> = version_out_string
    .split('.')
    .map(|s| {println!("{}", s); s.parse::<u32>().unwrap()})
    .collect();
  // exact matches
  assert_eq!(version_parts[0], 2);
  assert_eq!(version_parts[1], 0);
  // greater than or equal to!
  assert!(
    version_parts[2]
      >= if cfg!(feature = "bind_SDL2_2_0_10") {
        10
      } else if cfg!(feature = "bind_SDL2_2_0_9") {
        9
      } else {
        8
      }
  );

  // Call sdl2-config for real and do what it says to do.
  let link_style_arg: &str = if cfg!(feature = "link_dynamic") {
    "--libs"
  } else if cfg!(feature = "link_static") {
    "--static-libs"
  } else {
    panic!("No link mode selected!");
  };
  let sd2_config_linking = Command::new("sdl2-config")
    .arg(link_style_arg)
    .output()
    .unwrap_or_else(|_| panic!("Couldn't run `sdl2-config {}`.", link_style_arg));
  assert!(sd2_config_linking.status.success());
  for term in String::from_utf8_lossy(&sd2_config_linking.stdout).split_whitespace() {
    if term.starts_with("-L") {
      println!("cargo:rustc-link-search=native={}", &term[2..]);
    } else if term.starts_with("-lSDL2") {
      if cfg!(feature = "link_dynamic") {
        println!("cargo:rustc-link-lib=SDL2")
      } else if cfg!(feature = "link_static") {
        println!("cargo:rustc-link-lib=static=SDL2")
      } else {
        panic!("No link mode selected!");
      };
    } else if term.starts_with("-l") {
      // normal link
      println!("cargo:rustc-link-lib={}", &term[2..]);
    } else if term.starts_with("-Wl,-framework,") {
      // macOS framework link
      println!("cargo:rustc-link-lib=framework={}", &term[15..]);
    } else if term.starts_with("-Wl,-weak_framework,") {
      // rust doesn't seem to have "weak" framework linking so we just declare
      // a normal framework link.
      println!("cargo:rustc-link-lib=framework={}", &term[20..]);
    } else if term.starts_with("-Wl,-rpath,") {
      // I don't know why this works, but it does seem to?
      println!("cargo:rustc-env=LD_LIBRARY_PATH={}", &term[11..]);
    } else if term.starts_with("-Wl,--enable-new-dtags") {
      // Do we do anything here?
    } else if term.starts_with(" -Wl,--no-undefined") {
      // Do we do anything here?
    } else {
      panic!("Unknown term: {}", term);
    }
  }
}
