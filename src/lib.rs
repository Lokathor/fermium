#![no_std]
#![allow(bad_style)]

pub use chlorine::*;

pick! {
  if #[cfg(feature = "use_bindgen_bin")] {
    include!(concat!(
      env!("OUT_DIR"),
      "/SDL2-2.0.12-",
      env!("TARGET"),
      ".rs"
    ));
  } else {
    include!(concat!(
      "SDL2-2.0.12-",
      env!("TARGET"),
      ".rs"
    ));
  }
}
