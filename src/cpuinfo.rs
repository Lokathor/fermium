//! Get some useful info about the current CPU.

pub use crate::{c_int, stdinc::*};

extern "C" {
  /// This function returns the number of CPU cores available.
  pub fn SDL_GetCPUCount() -> c_int;

  /// This function returns the L1 cache line size of the CPU.
  pub fn SDL_GetCPUCacheLineSize() -> c_int;

  /// This function returns the amount of RAM configured in the system, in MB.
  pub fn SDL_GetSystemRAM() -> c_int;
}
