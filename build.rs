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
    cm.define("SDL_SHARED", "ON");
    cm.define("SDL_STATIC", "ON");
    let build_output_path = cm.build();
    println!("build_output_path: {}", build_output_path.display());

    println!(
      "cargo:rustc-link-search={}",
      build_output_path.join("lib").display()
    );

    if cfg!(windows) {
      if cfg!(feature = "dynamic_link") {
        println!("cargo:rustc-link-lib=SDL2d");
      } else {
        println!("cargo:rustc-link-lib=static=SDL2-staticd");
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
    } else if cfg!(unix) {
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

      for term in sd2_config_linking_stdout.split_whitespace() {
        if term.starts_with("-L") {
          println!("cargo:rustc-link-search=native={}", &term[2..]);
        } else if term.starts_with("-lSDL2") {
          if cfg!(feature = "dynamic_link") {
            println!("cargo:rustc-link-lib=SDL2d")
          } else {
            println!("cargo:rustc-link-lib=static=SDL2d")
          };
        } else if term.starts_with("-l") {
          // normal link
          println!("cargo:rustc-link-lib={}", &term[2..]);
        } else if term.starts_with("-Wl,-framework,") {
          // macOS framework link
          println!("cargo:rustc-link-lib=framework={}", &term[15..]);
        } else if term.starts_with("-Wl,-weak_framework,") {
          // rust doesn't seem to have "weak" framework linking so we just
          // declare a normal framework link.
          println!("cargo:rustc-link-lib=framework={}", &term[20..]);
        } else if term.starts_with("-Wl,-rpath,") {
          // I don't know why this works, but it does seem to?
          println!("cargo:rustc-env=LD_LIBRARY_PATH={}", &term[11..]);
        } else if term.starts_with("-Wl,--enable-new-dtags") {
          // Do we do anything here?
        } else if term.starts_with("-Wl,--no-undefined") {
          // Do we do anything here?
        } else if term.starts_with("-pthread") {
          // Nothing special on the Rust side, I'm told.
        } else {
          panic!("Unknown term: >>{}<<", term);
        }
      }
    } else {
      panic!("Sorry, I only know how to build/link SDL2 on windows and unix.");
    }
  }
}
