use crate::c_char;

extern "C" {
  /// Gets the current error string, stored in a thread local buffer.
  pub fn SDL_GetError() -> *const c_char;

  /// Clears the thread-local current error buffer.
  pub fn SDL_ClearError();
}
