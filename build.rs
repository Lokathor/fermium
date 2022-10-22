#![allow(clippy::if_same_then_else)]
#![allow(clippy::len_zero)]
#![allow(clippy::redundant_pattern_matching)]

use std::env;

fn main() {
  // Note(Lokathor): I was told by thomcc@github to put this in because it makes
  // the build faster if this crate is used within a workspace.
  //
  // The build script only uses the bundled C source to build the lib, so unless
  // we actually replace the C source, we don't need to re-run the build script.
  // This won't affect our ability to develop the Rust level bindings.
  println!("cargo:rerun-if-changed=build.rs");

  if cfg!(feature = "cargo_check") {
    println!("Run with `cargo_check` enabled, skipping the build.");
    return;
  }

  #[cfg(feature = "static_bundled_build")]
  {
    do_static_bundled();
  }
  #[cfg(not(feature = "static_bundled_build"))]
  {
    do_dynamic_system();
  }
}

#[cfg(feature = "static_bundled_build")]
fn do_static_bundled() {
  let out_dir = env::var("OUT_DIR").unwrap();
  println!("out_dir:{}", out_dir);

  let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
  println!("cargo_manifest_dir:{}", cargo_manifest_dir);

  let target = env::var("TARGET").expect("Could not read `TARGET`!");
  println!("target:{}", target);

  let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
  println!("target_os:{}", target_os);

  let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
  println!("target_arch:{}", target_arch);

  let target_vendor = env::var("CARGO_CFG_TARGET_VENDOR").unwrap();
  println!("target_vendor:{}", target_vendor);

  let mut cm = cmake::Config::new(
    std::path::Path::new(&cargo_manifest_dir).join("SDL2-2.24.1"),
  );
  cm.profile("Release");
  cm.static_crt(true);
  cm.target(&target);
  cm.define("SDL_SHARED", "ON");
  cm.define("SDL_STATIC", "ON");
  cm.define("HIDAPI", "ON");
  // We need to set extra CMake options when building for Apple platforms.
  if target_vendor == "apple" {
    // CMake can handle the x86_64/aarch64 duality of Apple platforms, but
    // needs to be told which architecture(s) we want. Since rust doesn't
    // support fat binaries, we'll only set the one architecture
    // requested. See: https://github.com/rust-lang/cargo/issues/8875
    match target_arch.as_str() {
      "aarch64" => {
        cm.define("CMAKE_OSX_ARCHITECTURES", "arm64");
      }
      "x86_64" => {
        cm.define("CMAKE_OSX_ARCHITECTURES", "x86_64");
      }
      arch => {
        println!("Unrecognized architecture for Apple platform \"{}\", not setting CMAKE_OSX_ARCHITECTURES", arch);
      }
    }
  }

  let build_output_path = cm.build();
  println!("build_output_path: {}", build_output_path.display());

  println!(
    "cargo:rustc-link-search={}",
    build_output_path.join("lib").display()
  );

  if target.contains("windows") {
    println!("cargo:rustc-link-lib=static=SDL2-static");
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
  } else {
    let sdl2_cfg_cmd = format!(
      "{}",
      build_output_path.join("bin").join("sdl2-config").display()
    );
    // Call sdl2-config and do what it says to do.
    let link_style_arg: &str =
      if cfg!(feature = "dynamic_link") { "--libs" } else { "--static-libs" };
    let sd2_config_linking = std::process::Command::new(sdl2_cfg_cmd)
      .arg(link_style_arg)
      .output()
      .unwrap_or_else(|_| {
        panic!("Couldn't run `sdl2-config {}`.", link_style_arg)
      });
    assert!(sd2_config_linking.status.success());

    let sd2_config_linking_stdout: String =
      String::from_utf8_lossy(&sd2_config_linking.stdout).into_owned();
    println!("sd2_config_linking_stdout: {}", sd2_config_linking_stdout);
    assert!(sd2_config_linking_stdout.len() > 0);

    process_sdl2_config_output(sd2_config_linking_stdout);
    println!("cargo:rustc-link-lib=static=SDL2");
  }
}

#[cfg(not(feature = "static_bundled_build"))]
fn do_dynamic_system() {
  println!("Performing dynamically-linked build against the system SDL2.");

  let out_dir = env::var("OUT_DIR").unwrap();
  println!("out_dir:{}", out_dir);

  let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
  println!("cargo_manifest_dir:{}", cargo_manifest_dir);

  let target = env::var("TARGET").expect("Could not read `TARGET`!");
  println!("target:{}", target);

  let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
  println!("target_os:{}", target_os);

  let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
  println!("target_arch:{}", target_arch);

  let target_vendor = env::var("CARGO_CFG_TARGET_VENDOR").unwrap();
  println!("target_vendor:{}", target_vendor);

  if target_os.contains("windows") {
    if target == "x86_64-pc-windows-msvc" {
      let devel_path =
        std::path::Path::new(&cargo_manifest_dir).join("SDL2-2.24.1-devel");

      // Copy the DLL file into the executable out directory. When distributing
      // your executable, you might need to provide the SDL2 DLL as well, unless
      // it's already installed via other means (eg: steam provided it, or
      // something).
      let dll_from = devel_path.join("SDL2.dll");
      println!("dll_from:{}", dll_from.display());
      let dll_to = std::path::Path::new(&out_dir).join("SDL2.dll");
      println!("dll_to:{}", dll_to.display());
      std::fs::copy(dll_from, dll_to).unwrap();

      println!("cargo:rustc-link-search={}", devel_path.display());
      println!("cargo:rustc-link-lib=SDL2");
    } else {
      panic!("On windows, dynamic_system builds are only supported for `x86_64-pc-windows-msvc`. Consider the `static_bundled_build` feature instead.");
    }
  } else {
    let sd2_config_linking = std::process::Command::new("sdl2-config")
      .arg("--libs")
      .output()
      .unwrap_or_else(|_| {
        panic!("Couldn't run `sdl2-config --libs`, is SDL2 properly installed?")
      });
    assert!(sd2_config_linking.status.success());

    let sd2_config_linking_stdout: String =
      String::from_utf8_lossy(&sd2_config_linking.stdout).into_owned();
    println!("sd2_config_linking_stdout: {}", sd2_config_linking_stdout);
    assert!(sd2_config_linking_stdout.len() > 0);

    process_sdl2_config_output(sd2_config_linking_stdout);
    println!("cargo:rustc-link-lib=SDL2");
  }
}

/// THE CALLER HAS TO HANDLE THE LINK TO SDL2 ITSELF (static or dynamic).
fn process_sdl2_config_output(sd2_config_linking_stdout: String) {
  for term in sd2_config_linking_stdout.split_whitespace() {
    if let Some(link_search_path) = term.strip_prefix("-L") {
      println!("cargo:rustc-link-search=native={}", link_search_path);
    } else if term.starts_with("-lSDL2") {
      // the caller handles this
    } else if let Some(link_target) = term.strip_prefix("-l") {
      // normal link
      if link_target.ends_with(".framework") {
        // macOS framework link, weirdly through -l
        let name_framework = link_target.rsplit('/').next().unwrap();
        let name = name_framework.split('.').next().unwrap();
        println!("cargo:rustc-link-lib=framework={name}");
      } else {
        println!("cargo:rustc-link-lib={link_target}");
      }
    } else if let Some(framework_link) = term.strip_prefix("-Wl,-framework,") {
      // macOS framework link
      println!("cargo:rustc-link-lib=framework={framework_link}");
    } else if let Some(framework_link) =
      term.strip_prefix("-Wl,-weak_framework,")
    {
      // rust doesn't seem to have "weak" framework linking so we just
      // declare a normal framework link.
      println!("cargo:rustc-link-lib=framework={framework_link}");
    } else if let Some(rpath) = term.strip_prefix("-Wl,-rpath,") {
      // I don't know why this works, but it does seem to?
      println!("cargo:rustc-env=DYLD_LIBRARY_PATH={rpath}");
    } else if let Some(_) = term.strip_prefix("-Wl,--enable-new-dtags") {
      // Do we do anything here?
    } else if let Some(_) = term.strip_prefix("-Wl,--no-undefined") {
      // Do we do anything here?
    } else if let Some(_) = term.strip_prefix("-pthread") {
      // Nothing special on the Rust side, I'm told.
    } else if let Some(_) = term.strip_prefix("-Wl,-current_version,") {
      // I don't think rust passes along a current version number?
    } else if let Some(_) = term.strip_prefix("-Wl,-compatibility_version,") {
      // I don't think rust passes along a compatibility version number?
    } else if let Some(_) = term.strip_prefix("-Wl,-undefined,error") {
      // This asks the linker to error on undefined symbols(?), which it
      // already will do.
    } else {
      //panic!("Unknown term output by `sdl2-config`: >>{}<<", term);
    }
  }
}
