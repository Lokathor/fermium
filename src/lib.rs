#![no_std]
#![allow(bad_style)]

//! Bindings to SDL2.
//!
//! This version exposes bindings for `SDL2-2.0.12`, and if you avoid calling
//! any of the newer functions you can also use older versions of SDL2.
//!
//! The default docs are for the x86_64 Windows MSVC version, but this crate
//! also works just fine on Mac and Linux. Things are about 95% the same from
//! target to target, but there are some places where the OS differences come
//! through.
//!
//! [The SDL2 Wiki](https://wiki.libsdl.org/) has information on function usage.
//! What's here is just the function signatures and structs.

use core::fmt::Debug;

pub use chlorine::*;

pick! {
  if #[cfg(feature = "use_bindgen_bin")] {
    include!(concat!(
      env!("OUT_DIR"),
      "/SDL2-2.0.12-",
      env!("TARGET"),
      ".rs"
    ));
  } else {
    include!(concat!(
      "SDL2-2.0.12-",
      env!("TARGET"),
      ".rs"
    ));
  }
}

// Note(Lokathor): Sometimes this is defined already within the bindings, but
// other types bindgen just uses `usize` directly. This alias lets us always
// define things in terms of `size_t`.
#[cfg(any(target_os = "macos", target_os = "linux"))]
pub type size_t = usize;

/// `SDL_touch.h`: Used as the device ID for mouse events simulated with touch
/// input
pub const SDL_TOUCH_MOUSEID: u32 = -1i32 as u32;

/// `SDL_touch.h`: Used as the SDL_TouchID for touch events simulated with mouse
/// input
///
/// * 2.0.10 or later
pub const SDL_MOUSE_TOUCHID: i64 = -1i64;

/// `SDL_surface.h`: Evaluates to true if the surface needs to be locked before
/// access.
///
/// ## Safety
///
/// This must be a valid `SDL_Surface` pointer.
#[inline(always)]
pub unsafe fn SDL_MUSTLOCK(surface: *const SDL_Surface) -> bool {
  (*surface).flags & SDL_RLEACCEL != 0
}

/// `SDL_pixels.h`: "internal" macro to check if a value is a pixel format
/// value.
#[inline(always)]
pub const fn SDL_PIXELFLAG(format: SDL_PixelFormatEnum) -> SDL_PixelFormatEnum {
  (format >> 28) & 0x0F
}

/// `SDL_pixels.h`: Pixel type of this format.
#[inline(always)]
pub const fn SDL_PIXELTYPE(format: SDL_PixelFormatEnum) -> SDL_PixelFormatEnum {
  (format >> 24) & 0x0F
}

/// `SDL_pixels.h`: Component ordering of this format.
#[inline(always)]
pub const fn SDL_PIXELORDER(
  format: SDL_PixelFormatEnum,
) -> SDL_PixelFormatEnum {
  (format >> 20) & 0x0F
}

/// `SDL_pixels.h`: Channel width layout of this format.
#[inline(always)]
pub const fn SDL_PIXELLAYOUT(
  format: SDL_PixelFormatEnum,
) -> SDL_PixelFormatEnum {
  (format >> 16) & 0x0F
}

/// `SDL_pixels.h`: Bits per pixel.
#[inline(always)]
pub const fn SDL_BITSPERPIXEL(
  format: SDL_PixelFormatEnum,
) -> SDL_PixelFormatEnum {
  (format >> 8) & 0xFF
}

/// `SDL_pixels.h`: Bytes per pixel.
#[inline(always)]
pub fn SDL_BYTESPERPIXEL(format: SDL_PixelFormatEnum) -> SDL_PixelFormatEnum {
  if SDL_ISPIXELFORMAT_FOURCC(format) {
    if format == SDL_PIXELFORMAT_YUY2
      || format == SDL_PIXELFORMAT_UYVY
      || format == SDL_PIXELFORMAT_YVYU
    {
      2
    } else {
      1
    }
  } else {
    format & 0xFF
  }
}

/// `SDL_pixels.h`: Is this an indexed format?
#[inline(always)]
pub fn SDL_ISPIXELFORMAT_INDEXED(format: SDL_PixelFormatEnum) -> bool {
  !SDL_ISPIXELFORMAT_FOURCC(format)
    && (SDL_PIXELTYPE(format) == SDL_PIXELTYPE_INDEX1
      || SDL_PIXELTYPE(format) == SDL_PIXELTYPE_INDEX4
      || SDL_PIXELTYPE(format) == SDL_PIXELTYPE_INDEX8)
}

/// `SDL_pixels.h`: Is this a packed format?
#[inline(always)]
pub fn SDL_ISPIXELFORMAT_PACKED(format: SDL_PixelFormatEnum) -> bool {
  !SDL_ISPIXELFORMAT_FOURCC(format)
    && (SDL_PIXELTYPE(format) == SDL_PIXELTYPE_PACKED8
      || SDL_PIXELTYPE(format) == SDL_PIXELTYPE_PACKED16
      || SDL_PIXELTYPE(format) == SDL_PIXELTYPE_PACKED32)
}

/// `SDL_pixels.h`: Is this an array format?
#[inline(always)]
pub fn SDL_ISPIXELFORMAT_ARRAY(format: SDL_PixelFormatEnum) -> bool {
  !SDL_ISPIXELFORMAT_FOURCC(format)
    && (SDL_PIXELTYPE(format) == SDL_PIXELTYPE_ARRAYU8
      || SDL_PIXELTYPE(format) == SDL_PIXELTYPE_ARRAYU16
      || SDL_PIXELTYPE(format) == SDL_PIXELTYPE_ARRAYU32
      || SDL_PIXELTYPE(format) == SDL_PIXELTYPE_ARRAYF16
      || SDL_PIXELTYPE(format) == SDL_PIXELTYPE_ARRAYF32)
}

/// `SDL_pixels.h`: Does this format have an alpha channel?
#[inline(always)]
pub fn SDL_ISPIXELFORMAT_ALPHA(format: SDL_PixelFormatEnum) -> bool {
  (SDL_ISPIXELFORMAT_PACKED(format)
    && (SDL_PIXELORDER(format) == SDL_PACKEDORDER_ARGB
      || SDL_PIXELORDER(format) == SDL_PACKEDORDER_RGBA
      || SDL_PIXELORDER(format) == SDL_PACKEDORDER_ABGR
      || SDL_PIXELORDER(format) == SDL_PACKEDORDER_BGRA))
    || (SDL_ISPIXELFORMAT_ARRAY(format)
      && (SDL_PIXELORDER(format) == SDL_ARRAYORDER_ARGB
        || SDL_PIXELORDER(format) == SDL_ARRAYORDER_RGBA
        || SDL_PIXELORDER(format) == SDL_ARRAYORDER_ABGR
        || SDL_PIXELORDER(format) == SDL_ARRAYORDER_BGRA))
}

/// `SDL_pixels.h`: Is this a FourCC format?
#[inline(always)]
pub fn SDL_ISPIXELFORMAT_FOURCC(format: SDL_PixelFormatEnum) -> bool {
  (format != 0) && (SDL_PIXELFLAG(format) != 1)
}
