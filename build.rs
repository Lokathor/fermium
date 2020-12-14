use std::env;

fn main() {
  if cfg!(not(feature = "cargo_check")) {
    let manifest_dir = std::path::PathBuf::from(
      env::var("CARGO_MANIFEST_DIR")
        .expect("Could not read `CARGO_MANIFEST_DIR`!"),
    );

    let mut cm = cmake::Config::new(manifest_dir.join("SDL2-2.0.12"));
    cm.static_crt(true);
    cm.target(&env::var("TARGET").expect("Couldn't read `TARGET`"));
    if cfg!(feature = "dynamic_link") {
      cm.define("SDL_SHARED", "ON");
      cm.define("SDL_STATIC", "OFF");
    } else {
      cm.define("SDL_SHARED", "OFF");
      cm.define("SDL_STATIC", "ON");
    }
    let build_output_path = cm.build();

    println!(
      "cargo:rustc-link-search={}",
      build_output_path.join("lib").display()
    );

    if cfg!(feature = "dynamic_link") {
      println!("cargo:rustc-link-lib=SDL2");
    } else {
      println!("cargo:rustc-link-lib=static=SDL2d");
      if cfg!(windows) {
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
    }
  }
}
