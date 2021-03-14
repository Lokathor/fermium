#[cfg(windows)]
fn main() -> std::io::Result<()> {
  use std::io::Write;
  const DLL_BYTES: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/SDL2.dll"));
  let mut f = std::fs::File::create("SDL2.dll")?;
  f.write_all(DLL_BYTES)?;
  println!("Successfully wrote out SDL2.dll (x86_64)");
  Ok(())
}

#[cfg(not(windows))]
fn main() {
  println!("You're not on Windows, so I'm just gonna print this message.");
}
