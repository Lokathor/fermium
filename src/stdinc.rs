#![allow(dead_code)]

//! The "standard include" which almost every other include tends to use.

use crate::{c_int, c_void};

// Note(Lokathor): We want to be able to use these in the definitions for easier
// interaction with the SDL headers and bindgen, but they're dumb to expose to
// the user. If we make them only available within the crate, then outside users
// are just told the normal rust type instead.
pub(crate) type Uint8 = u8;
pub(crate) type Sint8 = i8;
pub(crate) type Uint16 = u16;
pub(crate) type Sint16 = i16;
pub(crate) type Uint32 = u32;
pub(crate) type Sint32 = i32;
pub(crate) type Uint64 = u64;
pub(crate) type Sint64 = i64;

/// Define a four character code as a `u32`.
///
/// Inputs are generally ASCII values.
pub const fn SDL_FOURCC(a: u8, b: u8, c: u8, d: u8) -> u32 {
  (a as u32) << 0 | (b as u32) << 8 | (c as u32) << 16 | (d as u32) << 24
}

/// SDL's `bool` approximation.
///
/// Because C has no real `bool` type, we get some named constants.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_bool(u32);
/// `false` value
pub const SDL_FALSE: SDL_bool = SDL_bool(0);
/// `true` value
pub const SDL_TRUE: SDL_bool = SDL_bool(1);

extern "C" {
  /// Works like [`malloc`](https://man7.org/linux/man-pages/man3/malloc.3.html)
  pub fn SDL_malloc(size: usize) -> *mut c_void;

  /// Works like [`calloc`](https://man7.org/linux/man-pages/man3/calloc.3p.html)
  pub fn SDL_calloc(nmemb: usize, size: usize) -> *mut c_void;

  /// Works like [`realloc`](https://man7.org/linux/man-pages/man3/realloc.3p.html)
  pub fn SDL_realloc(mem: *mut c_void, size: usize) -> *mut c_void;

  /// Works like [`free`](https://man7.org/linux/man-pages/man3/free.3p.html)
  pub fn SDL_free(mem: *mut c_void);

  /// Get the current SDL memory functions.
  pub fn SDL_GetMemoryFunctions(
    malloc_func: *mut SDL_malloc_func, calloc_func: *mut SDL_calloc_func,
    realloc_func: *mut SDL_realloc_func, free_func: *mut SDL_free_func,
  );

  /// Set new SDL memory functions.
  ///
  /// **Caution:** before using this, be sure to call [`SDL_GetNumAllocations`]
  /// and check that it's 0.
  pub fn SDL_SetMemoryFunctions(
    malloc_func: SDL_malloc_func, calloc_func: SDL_calloc_func,
    realloc_func: SDL_realloc_func, free_func: SDL_free_func,
  ) -> c_int;

  /// The number of outstanding allocations.
  pub fn SDL_GetNumAllocations() -> c_int;
}

/// Works like [`malloc`](https://man7.org/linux/man-pages/man3/malloc.3.html)
pub type SDL_malloc_func =
  Option<unsafe extern "C" fn(size: usize) -> *mut c_void>;

/// Works like [`calloc`](https://man7.org/linux/man-pages/man3/calloc.3p.html)
pub type SDL_calloc_func =
  Option<unsafe extern "C" fn(nmemb: usize, size: usize) -> *mut c_void>;

/// Works like [`realloc`](https://man7.org/linux/man-pages/man3/realloc.3p.html)
pub type SDL_realloc_func =
  Option<unsafe extern "C" fn(mem: *mut c_void, size: usize) -> *mut c_void>;

/// Works like [`free`](https://man7.org/linux/man-pages/man3/free.3p.html)
pub type SDL_free_func = Option<unsafe extern "C" fn(mem: *mut c_void)>;
