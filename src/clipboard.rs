//! Interact with the system clipboard.

use crate::{c_char, c_int, stdinc::*};

extern "C" {
  /// Put UTF-8 text into the clipboard
  pub fn SDL_SetClipboardText(text: *const c_char) -> c_int;

  /// Get UTF-8 text from the clipboard, which must be freed with [`SDL_free`]
  pub fn SDL_GetClipboardText() -> *mut c_char;

  /// Returns a flag indicating whether the clipboard exists and contains a text
  /// string that is non-empty.
  pub fn SDL_HasClipboardText() -> SDL_bool;
}
