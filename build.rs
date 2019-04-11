use std::{
  env,
  path::{Path, PathBuf},
};

const WRAPPER_DOT_H: &str = r##"
  #if defined(__APPLE__)
  #define MAC_OS_X_VERSION_MIN_REQUIRED 1060
  #endif

  #include "include/SDL.h"
  "##;

fn main() {
  let out_dir = PathBuf::from(env::var("OUT_DIR").expect("Couldn't read `OUT_DIR` value."));
  generate_bindings_file(&out_dir);
  declare_linking();
}

fn generate_bindings_file(out_dir: &Path) {
  let bindings_filename = out_dir.join("bindings.rs");
  let bindings = bindgen::builder()
      .header_contents("wrapper.h",WRAPPER_DOT_H)
      .use_core()
      .ctypes_prefix(if cfg!(windows) {
          "winapi::ctypes"
        } else {
          "libc"
        })
      .default_enum_style(bindgen::EnumVariation::Consts)
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
  println!("cargo:rustc-link-lib=SDL2");
  #[cfg(windows)]
  {
    if cfg!(target_pointer_width = "32") {
      println!("cargo:rustc-link-search=native=lib/x86");
    } else if cfg!(target_pointer_width = "64") {
      println!("cargo:rustc-link-search=native=lib/x64");
    } else {
      panic!("What on earth is the size of a pointer on this device!?");
    }
  }
}
