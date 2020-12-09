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

    cm.build();
  }
}
