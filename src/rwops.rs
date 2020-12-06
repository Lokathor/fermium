pub use crate::{c_char, c_void, error::*, stdinc::*};

/// SDL's read/write abstraction.
///
/// This is necessary for interaction with some of the other SDL systems, but
/// I'm dubious as to its general value compared to using `std::fs` most of the
/// time. So, currently, there's only bindings to allocate one from a file path
/// and to free it later.
#[allow(unused)]
#[repr(transparent)]
pub struct SDL_RWops(c_void);

extern "C" {
  /// Creates a [`SDL_RWops`] from a filename and mode string.
  ///
  /// * `file` supports unicode filenames if it's a utf8-encoded name given,
  ///   even if the local OS doesn't use utf8 encoding for filenames (it's
  ///   recoded for you).
  /// * The `mode` string works like with unix `fopen`, basically.
  /// * Both strings are null-terminated, of course.
  pub fn SDL_RWFromFile(
    file: *const c_char, mode: *const c_char,
  ) -> *mut SDL_RWops;

  /// Free an [`SDL_RWops`] after use.
  ///
  /// Some functions already have a flag that they will free an `SDL_RWops`
  /// given after they use it, so this isn't always necessary.
  pub fn SDL_FreeRW(area: *mut SDL_RWops);
}
