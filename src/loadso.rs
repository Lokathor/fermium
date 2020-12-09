//! Module for loading "shared objects" (aka dynamic libraries / DLLs).

pub use crate::{c_char, c_void};

extern "C" {
  /// This function dynamically loads a shared object and returns a pointer to
  /// the object handle (or NULL if there was an error).
  ///
  /// The `sofile` parameter is a system dependent name of the object file.
  pub fn SDL_LoadObject(sofile: *const c_char) -> *mut c_void;

  /// Given an object handle, this function looks up the address of the named
  /// function in the shared object and returns it.
  ///
  /// This address is no longer valid after calling [`SDL_UnloadObject`].
  pub fn SDL_LoadFunction(
    handle: *mut c_void, name: *const c_char,
  ) -> *mut c_void;

  /// Unload a shared object from memory.
  pub fn SDL_UnloadObject(handle: *mut c_void);
}
