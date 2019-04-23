#![cfg_attr(not(any(feature = "use_bindgen_bin", feature = "use_bindgen_lib")), allow(unused_imports))]

use std::{
  env,
  path::{Path, PathBuf},
};

#[cfg_attr(not(any(feature = "use_bindgen_bin", feature = "use_bindgen_lib")), allow(dead_code))]
const WRAPPER_DOT_H: &str = r##"
  #if defined(__APPLE__)
  #define MAC_OS_X_VERSION_MIN_REQUIRED 1060
  #endif

  #include "include/SDL.h"
"##;

fn main() {
  #[cfg(all(feature = "use_bindgen_bin", feature = "use_bindgen_lib"))]
  {
    compile_error!("Please enable `use_bindgen_bin` OR `use_bindgen_lib`, but NOT both.");
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
  let current_dir = std::env::current_dir().expect("Couldn't read the current dir.");
  let mut copy_options = fs_extra::dir::CopyOptions::new();
  copy_options.copy_inside = true;
  copy_options.skip_exist = true;
  fs_extra::copy_items(&vec![current_dir.join("include")], out_dir, &copy_options)
    .expect("Couldn't copy the 'include' directory.");
  let bindings_filename = out_dir.join("bindings.rs");
  let bindings_filename_str = bindings_filename
    .to_str()
    .expect("output file path isn't valid utf8, stop that");
  let wrapper_filename = out_dir.join("wrapper.h");
  let mut wrapper_dot_h_file =
    std::fs::File::create(&wrapper_filename).expect("Couldn't make a temporary 'wrapper.h' file.");
  use std::io::prelude::*;
  wrapper_dot_h_file
    .write_all(WRAPPER_DOT_H.as_bytes())
    .expect("Couldn't write the wrapper file content.");
  let mut bindings_command = std::process::Command::new("bindgen");
  // flags
  bindings_command.arg("--impl-debug");
  bindings_command.arg("--impl-partialeq");
  bindings_command.arg("--use-core");
  bindings_command.arg("--with-derive-default");
  bindings_command.arg("--with-derive-partialeq");
  // options
  bindings_command
    .arg("--ctypes-prefix")
    .arg("libc");
  bindings_command.arg("--default-enum-style").arg("moduleconsts");
  bindings_command.arg("--output").arg(&bindings_filename_str);
  bindings_command.arg("--rust-target").arg("1.33");
  bindings_command.arg("--rustfmt-configuration-file").arg(
    std::env::current_dir()
      .expect("couldn't get current directory!")
      .join("rustfmt.toml")
      .to_str()
      .expect("rustfmt.toml file path isn't valid utf8, stop that"),
  );
  // header
  bindings_command.arg(&wrapper_filename);

  println!("EXECUTING:{:?}", bindings_command);
  let bindings_command_output = bindings_command
    .output()
    .expect("Couldn't run the 'bindgen' command.");
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
  let bindings = bindgen::builder()
    .header_contents("wrapper.h",WRAPPER_DOT_H)
    .use_core()
    .ctypes_prefix("libc")
    .default_enum_style(bindgen::EnumVariation::ModuleConsts)
    .impl_debug(true)
    .derive_default(true)
    .derive_partialeq(true)
    .time_phases(true) // Note(Lokathor): just for fun!
    .rustfmt_bindings(true)
    .rustfmt_configuration_file(Some(PathBuf::from("rustfmt.toml")))
    .generate()
    .expect("Couldn't generate the bindings.");
  bindings
    .write_to_file(&bindings_filename)
    .expect("Couldn't write the bindings file.");
}

fn declare_linking() {
  // WHAT TO LINK
  if cfg!(feature = "static_link_sdl2_use_with_caution") {
    println!("cargo:rustc-link-lib=static=SDL2");
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
  } else {
    println!("cargo:rustc-link-lib=SDL2");
  }

  // WHERE TO LOOK
  #[cfg(windows)]
  {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("Could not read CARGO_MANIFEST_DIR."));
    let subdirectory = if cfg!(target_pointer_width = "32") {
      "lib\\x86"
    } else if cfg!(target_pointer_width = "64") {
      "lib\\x64"
    } else {
      panic!("What on earth is the size of a pointer on this device!?");
    };
    println!("cargo:rustc-link-search=native={}", manifest_dir.join(subdirectory).display());
  }
  #[cfg(not(windows))]
  {
    if let Ok(ld_library_path) = env::var("LD_LIBRARY_PATH") {
      for dir in ld_library_path.split(":") {
        println!("cargo:rustc-link-search=native={}", dir);
      }
    } else {
      eprintln!("Couldn't read LD_LIBRARY_PATH, but will attempt to build anyway...");
    }
  }
}
