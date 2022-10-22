#[cfg(windows)]
use std::io::Write;

#[cfg(windows)]
const DLL_BYTES: &[u8] = include_bytes!("../../SDL2-2.24.1-devel/SDL2.dll");

#[cfg(windows)]
fn main() -> std::io::Result<()> {
  let mut f = std::fs::File::create("SDL2.dll")?;
  f.write_all(DLL_BYTES)?;
  println!("Successfully wrote out SDL2.dll (x86_64)");
  Ok(())
}

#[cfg(not(windows))]
fn main() {
  println!("You're not on Windows, so I'm just gonna print this message.");
}
