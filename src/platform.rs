//! Platform based functions.

use crate::c_char;

extern "C" {
  /// Gets the name of the platform.
  pub fn SDL_GetPlatform() -> *const c_char;
}
