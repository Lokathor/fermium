#[cfg(windows)]
fn main() {
  use std::{env::current_dir, fs::File, io::prelude::*};

  #[cfg(target_pointer_width = "32")]
  const DLL_BYTES: &[u8] = include_bytes!("../../win32-devel-files/VC/lib/x86/SDL2.dll");
  #[cfg(target_pointer_width = "64")]
  const DLL_BYTES: &[u8] = include_bytes!("../../win32-devel-files/VC/lib/x64/SDL2.dll");

  let out_path = current_dir()
    .expect("Failed to read the current directory!")
    .join("SDL2.dll");
  File::create(&out_path)
    .expect("Couldn't open 'SDL2.dll' for writing!")
    .write_all(DLL_BYTES)
    .expect("Failed to write the bytes of the DLL!");
  println!(
    "Wrote out the {}-bit SDL2 dynamic lib to 'SDL2.dll', you're ready to go.",
    if cfg!(target_pointer_width = "32") {
      32
    } else {
      64
    }
  );
}

#[cfg(not(windows))]
fn main() {
  println!("You're not on windows, you don't need the SDL2 DLL from me.");
  println!("Just install SDL2 (v2.0.8 or later) via your package manager.");
}
