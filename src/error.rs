//! SDL uses a thread-local buffer to store the current error string.

use crate::{c_char, c_int};

extern "C" {
  /// Get the last error message that was set for the current thread.
  ///
  /// SDL API functions may set error messages and then succeed, so you should
  /// only use the error value if a function fails.
  ///
  /// * `errstr` A buffer to fill with the last error message that was set for
  ///   the current thread.
  /// * `maxlen` The size of the buffer pointed to by the `errstr` parameter.
  ///
  /// **Returns:** `errstr`
  pub fn SDL_GetErrorMsg(errstr: *mut c_char, maxlen: c_int) -> *mut c_char;

  /// Clear the error message for the current thread.
  pub fn SDL_ClearError();
}
