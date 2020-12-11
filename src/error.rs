//! SDL uses a thread-local buffer to store the current error string.

use crate::c_char;

extern "C" {
  /// Gets the current error string, stored in a thread local buffer.
  ///
  /// This function can be used even without having called [`SDL_Init`].
  pub fn SDL_GetError() -> *const c_char;

  /// Clears the thread-local current error buffer.
  pub fn SDL_ClearError();
}
