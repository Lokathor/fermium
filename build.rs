#![cfg_attr(
  not(any(feature = "use_bindgen_bin", feature = "use_bindgen_lib")),
  allow(unused_imports)
)]

use std::{
  env,
  path::{Path, PathBuf},
};

fn main() {
  #[cfg(all(feature = "use_bindgen_bin", feature = "use_bindgen_lib"))]
  {
    compile_error!("If you want to build fresh bindings please enable `use_bindgen_bin` or `use_bindgen_lib`, but NOT both.");
  }
  #[cfg(feature = "use_bindgen_bin")]
  {
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("Couldn't read `OUT_DIR` value."));
    generate_bindings_file_via_cli(&out_dir);
  }
  #[cfg(feature = "use_bindgen_lib")]
  {
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("Couldn't read `OUT_DIR` value."));
    generate_bindings_file_via_lib(&out_dir);
  }
  declare_linking();
}

#[cfg(feature = "use_bindgen_bin")]
fn generate_bindings_file_via_cli(out_dir: &Path) {
  // where from
  let current_dir = std::env::current_dir().expect("Couldn't read the current dir.");
  let wrapper_filename = current_dir.join("wrapper.h");
  // where to
  let bindings_filename = out_dir.join("bindings.rs");
  let mut bindings_command = std::process::Command::new("bindgen");
  // flags
  bindings_command.arg("--impl-debug");
  bindings_command.arg("--impl-partialeq");
  bindings_command.arg("--use-core");
  bindings_command.arg("--with-derive-default");
  bindings_command.arg("--with-derive-partialeq");
  // options
  bindings_command.arg("--ctypes-prefix").arg("libc");
  bindings_command
    .arg("--default-enum-style")
    .arg("moduleconsts");
  bindings_command.arg("--output").arg(&format!("{}",bindings_filename.display()));
  bindings_command.arg("--rust-target").arg("1.33");
  bindings_command.arg("--rustfmt-configuration-file").arg(
    std::env::current_dir()
      .expect("couldn't get current directory!")
      .join("rustfmt.toml")
      .to_str()
      .expect("rustfmt.toml file path isn't valid utf8, stop that"),
  );
  // OS specific type handling because of `SDL_syswm.h`
  if cfg!(windows) {
    bindings_command.arg("--opaque-type").arg("_IMAGE.*");
    bindings_command.arg("--opaque-type").arg("tagMONITORINFOEXA");
  }
  // header
  bindings_command.arg(&wrapper_filename);

  println!("EXECUTING:{:?}", bindings_command);
  let bindings_command_output = bindings_command
    .output()
    .expect("Couldn't run the 'bindgen' command, perhaps you need to 'cargo install bindgen'?");
  if bindings_command_output.status.success() {
    println!("SUCCESS!")
  } else {
    println!("FAILURE!")
  }
  for line in String::from_utf8_lossy(&bindings_command_output.stdout).lines() {
    println!("OUT:{}", line);
  }
  for line in String::from_utf8_lossy(&bindings_command_output.stderr).lines() {
    println!("ERR:{}", line);
  }
  if !bindings_command_output.status.success() {
    panic!("The 'bindgen' process FAILED! (see output log for details)");
  }
}

#[cfg(feature = "use_bindgen_lib")]
fn generate_bindings_file_via_lib(out_dir: &Path) {
  let bindings_filename = out_dir.join("bindings.rs");
  #[allow(unused_mut)]
  let mut bindings = bindgen::builder()
    .header("wrapper.h")
    .use_core()
    .ctypes_prefix("libc")
    .default_enum_style(bindgen::EnumVariation::ModuleConsts)
    .impl_debug(true)
    .derive_default(true)
    .derive_partialeq(true)
    .time_phases(true) // Note(Lokathor): just for fun!
    .rustfmt_bindings(true)
    .rustfmt_configuration_file(Some(PathBuf::from("rustfmt.toml")));
  if cfg!(windows) {
    bindings = bindings
      .opaque_type("_IMAGE.*")
      .opaque_type("tagMONITORINFOEXA");
  }
  bindings
    .generate()
    .expect("Couldn't generate the bindings.");
  bindings
    .write_to_file(&bindings_filename)
    .expect("Couldn't write the bindings file.");
}

fn declare_linking() {
  // WHAT TO LINK
  if cfg!(feature = "dynamic_link") {
    println!("cargo:rustc-link-lib=SDL2");
  } else {
    println!("cargo:rustc-link-lib=static=SDL2");
    if cfg!(windows) {
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
    if cfg!(target_os = "macos") {
      println!("cargo:rustc-link-lib=iconv");
      println!("cargo:rustc-link-lib=framework=CoreAudio");
      println!("cargo:rustc-link-lib=framework=AudioToolbox");
      println!("cargo:rustc-link-lib=framework=ForceFeedback");
      println!("cargo:rustc-link-lib=framework=CoreVideo");
      println!("cargo:rustc-link-lib=framework=Cocoa");
      println!("cargo:rustc-link-lib=framework=Carbon");
      println!("cargo:rustc-link-lib=framework=IOKit");
      println!("cargo:rustc-link-lib=framework=QuartzCore");
      println!("cargo:rustc-link-lib=framework=Metal");
    }
  }

  // WHERE TO LOOK

  // If the user points us to a specific directory, follow their advice.
  println!("cargo:rerun-if-env-changed=FERMIUM_SDL2_DIR");
  if let Ok(path) = env::var("FERMIUM_SDL2_DIR") {
    println!(
      "cargo:rustc-link-search={}",
      path
    );
    return;
  }
  #[cfg(windows)]
  {
    let manifest_dir =
      PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("Could not read CARGO_MANIFEST_DIR."));
    let subdirectory = if cfg!(target_env="msvc") {
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
  }
  #[cfg(not(windows))]
  {
    if pkg_config::Config::new()
        .statik(!cfg!(feature = "dynamic_link"))
        .probe("SDL2")
        .is_ok() {
      // pkg-config will have printed the various info
      return;
    } else {
      eprintln!("Could not configure link search directories via pkg-config...");
    }
    // Fall back to LD_LIBRARY_PATH, as a last resort.
    if let Ok(ld_library_path) = env::var("LD_LIBRARY_PATH") {
      for dir in ld_library_path.split(":") {
        println!("cargo:rustc-link-search=native={}", dir);
      }
    } else {
      eprintln!("Couldn't read LD_LIBRARY_PATH, but will attempt to build anyway...");
    }
  }
}
