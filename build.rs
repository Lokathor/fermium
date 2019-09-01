#![allow(unused_imports)]
#![allow(non_snake_case)]

use std::{
  env,
  path::{Path, PathBuf},
  process::Command,
};

fn main() {
  let use_pregenerated_files = cfg!(feature = "use_pregenerated_files");
  let use_bindgen_bin = cfg!(feature = "use_bindgen_bin");
  println!("use_pregenerated_files: {}", use_pregenerated_files);
  println!("use_bindgen_bin: {}", use_bindgen_bin);
  let use_request_count = use_pregenerated_files as usize + use_bindgen_bin as usize;
  if use_request_count != 1 {
    panic!(
      "You must select exactly one style of bindings usage, you selected {}",
      use_request_count
    );
  }

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

  #[cfg(feature = "use_bindgen_bin")]
  run_bindgen_bin();
}

#[cfg(feature = "use_bindgen_bin")]
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

/*

fn declare_linking() {
  if cfg!(windows) {
    // WHAT TO LINK
    if cfg!(feature = "dynamic_link") {
      println!("cargo:rustc-link-lib=SDL2");
    } else {
      println!("cargo:rustc-link-lib=static=SDL2");
      println!("cargo:rustc-link-lib=shell32");
      println!("cargo:rustc-link-lib=user32");
      println!("cargo:rustc-link-lib=gdi32");
      println!("cargo:rustc-link-lib=winmm");
      println!("cargo:rustc-link-lib=imm32");
      println!("cargo:rustc-link-lib=ole32");
      println!("cargo:rustc-link-lib=oleaut32");
      println!("cargo:rustc-link-lib=version");
      println!("cargo:rustc-link-lib=uuid");
      println!("cargo:rustc-link-lib=dinput8");
      println!("cargo:rustc-link-lib=dxguid");
      println!("cargo:rustc-link-lib=setupapi");
    }
    // WHERE TO LOOK
    let manifest_dir =
      PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("Could not read CARGO_MANIFEST_DIR."));
    let subdirectory = if cfg!(target_env = "msvc") {
      if cfg!(feature = "dynamic_link") {
        if cfg!(target_pointer_width = "64") {
          "lib\\msvc-x64-dynamic"
        } else {
          panic!("We only support 64-bit builds, file a PR.");
        }
      } else {
        if cfg!(target_pointer_width = "64") {
          "lib\\msvc-x64-static"
        } else {
          panic!("We only support 64-bit builds, file a PR.");
        }
      }
    } else {
      panic!("This crate doesn't support the GNU toolchain on windows, file a PR I guess.");
    };
    println!(
      "cargo:rustc-link-search=native={}",
      manifest_dir.join(subdirectory).display()
    );
  } else {
    // Note(Lokathor): `pkg-config` will often enough not find SDL2 at all for
    // whatever reason. Instead, SDL2 happens to provide its own tool to help
    // you get the correct linker args, so we'll call that and do whatever it
    // says to do.
    let sd2_config_output = Command::new("sdl2-config")
      .arg(if cfg!(feature = "dynamic_link") {
        "--libs"
      } else {
        "--static-libs"
      })
      .output()
      .expect("Couldn't run `sdl2-config`.");
    // The output is space separated, of course
    for term in String::from_utf8_lossy(&sd2_config_output.stdout).split_whitespace() {
      if term.starts_with("-L") {
        println!("cargo:rustc-link-search=native={}", &term[2..]);
      } else if term.starts_with("-lSDL2") {
        if cfg!(feature = "dynamic_link") {
          println!("cargo:rustc-link-lib=SDL2")
        } else {
          println!("cargo:rustc-link-lib=static=SDL2")
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
      }
      // Note(Lokathor): There's other terms that might be generated by
      // `sdl2-config`, but they're not useful to us so we ignore them.
    }
  }
}



// compile a shared or static lib depending on the feature
#[cfg(feature = "bundled")]
fn compile_sdl2(sdl2_build_path: &Path, target_os: &str) -> PathBuf {
    let mut cfg = cmake::Config::new(sdl2_build_path);
    cfg.profile("release");

    if target_os == "windows-gnu" {
        cfg.define("VIDEO_OPENGLES", "OFF");
    }

    if cfg!(feature = "static-link") {
        cfg.define("SDL_SHARED", "OFF");
        cfg.define("SDL_STATIC", "ON");
    } else {
        cfg.define("SDL_SHARED", "ON");
        cfg.define("SDL_STATIC", "OFF");
    }

    cfg.build()
}

*/
