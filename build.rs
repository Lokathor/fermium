#![cfg_attr(
  not(any(feature = "use_bindgen_bin", feature = "use_bindgen_lib")),
  allow(unused_imports)
)]

use std::{
  env,
  path::{Path, PathBuf},
  process::Command,
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
  let mut bindings_command = Command::new("bindgen");
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
  bindings_command
    .arg("--output")
    .arg(&format!("{}", bindings_filename.display()));
  bindings_command.arg("--rust-target").arg("1.33");
  bindings_command.arg("--rustfmt-configuration-file").arg(
    std::env::current_dir()
      .expect("couldn't get current directory!")
      .join("rustfmt.toml")
      .to_str()
      .expect("rustfmt.toml file path isn't valid utf8, stop that"),
  );
  bindings_command.arg("--whitelist-function").arg("SDL_.*");
  bindings_command.arg("--whitelist-type").arg("SDL_.*");
  bindings_command.arg("--whitelist-var").arg("SDL_.*");
  bindings_command.arg("--whitelist-var").arg("AUDIO_.*");
  bindings_command.arg("--whitelist-var").arg("SDLK_.*");
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
  let mut builder = bindgen::builder()
    .header("wrapper.h")
    .use_core()
    .ctypes_prefix("libc")
    .default_enum_style(bindgen::EnumVariation::ModuleConsts)
    .impl_debug(true)
    .derive_default(true)
    .derive_partialeq(true)
    .time_phases(true) // Note(Lokathor): just for fun!
    .rustfmt_bindings(true)
    .rustfmt_configuration_file(Some(PathBuf::from("rustfmt.toml")))
    .whitelist_function("SDL_.*")
    .whitelist_type("SDL_.*")
    .whitelist_var("SDL_.*")
    .whitelist_var("AUDIO_.*")
    .whitelist_var("SDLK_.*");
  let bindings = builder.generate().expect("Couldn't generate the bindings.");
  bindings
    .write_to_file(&bindings_filename)
    .expect("Couldn't write the bindings file.");
}

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
