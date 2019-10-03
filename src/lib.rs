#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::redundant_static_lifetimes)]
#![allow(clippy::cognitive_complexity)]

//! The `fermium` crate is raw bindings to the SDL2 C API.
//!
//! Compared to the common alternative,
//! [sdl2-sys](https://crates.io/crates/sdl2-sys), this is has consts instead of
//! enums, it is slightly more complete, and it works _much_ better on windows
//! MSVC (no special setup at all).
//!
//! Depending on how you configure the crate, you can target a minimum SDL2
//! version of 2.0.8 or later. You can compile against a later version than your
//! minimum bind version, but not an earlier one, so for best compatability you
//! should target the earliest SDL2 API version you can. The current version of
//! SDL2 in Ubuntu 18 is 2.0.8, and Debian Stable has 2.0.9, so we don't at this
//! time bother to support SDL2 versions before 2.0.8.
//!
//! `bindgen` is used to generate the bindings from the official SDL2 include
//! files. At the moment we include the following SDL2 headers:
//!
//! * `SDL2.h`
//! * `SDL_syswm.h`
//! * `SDL_vulkan.h`
//!
//! However, `SDL_syswm.h` in particular pulls in a bunch of extra code and it
//! overwhelms the generated bindings. To avoid this, we only keep the following
//! whitelist of items:
//!
//! * `SDL_` (functions, types, and vars)
//! * `SDLK_` (vars)
//! * `AUDIO_` (vars)
//! * Any other items that the above depend on.
//!
//! It is thought that this will expose all needed functionality, but if you
//! think something should be added to the whitelist please [submit an
//! issue](https://github.com/Lokathor/fermium/issues).
//!
//! ## Docs
//!
//! Bindgen doesn't understand how to convert doxygen style docs into rustdoc
//! style docs. What it does generate makes rustdoc think there a bunch of
//! random code block all over that it should run as test cases. Sadly, rustdoc
//! has no way to turn this off, so I have to tell bindgen to just emit no docs
//! at all.
//!
//! Instead you should check out the [SDL2 Wiki](https://wiki.libsdl.org/)

use cfg_if::cfg_if;

// re-export the variable-length C types from their source crate to ease the end
// user experience.
cfg_if! {
  if #[cfg(windows)] {
    pub use winapi::ctypes::{
      c_char, c_int, c_long, c_longlong, c_short, c_uint, c_ulong, c_ulonglong, c_ushort,
    };
  } else {
    pub use libc::{
      c_char, c_int, c_long, c_longlong, c_short, c_uint, c_ulong, c_ulonglong, c_ushort,
    };
  }
}

// bring in the correct bindings file. The bindgen binary will place them into
// OUT_DIR, but for the pre-generated bindings they'll be within the source
// directory along side the lib.rs file.
cfg_if! {
  if #[cfg(feature = "use_bindgen_bin")] {
    include!(concat!(
      env!("OUT_DIR"),
      "/SDL2-v2.0.",
      env!("BIND_PATCH_LEVEL"),
      "-",
      env!("TARGET"),
      ".rs"
    ));
  } else {
    include!(concat!(
      "SDL2-v2.0.",
      env!("BIND_PATCH_LEVEL"),
      "-",
      env!("TARGET"),
      ".rs"
    ));
  }
}

// Note(Lokathor): Bindgen doesn't parse all things properly on its own, and it
// doesn't parse CPP macros at all, so we must define some more stuff here.

/// See remarks of [`SDL_PixelFormatEnum`](https://wiki.libsdl.org/SDL_PixelFormatEnum)
pub type PixelType = _bindgen_ty_1;

/// See remarks of [`SDL_PixelFormatEnum`](https://wiki.libsdl.org/SDL_PixelFormatEnum#Remarks)
pub type BitmapOrder = _bindgen_ty_2;

/// See remarks of [`SDL_PixelFormatEnum`](https://wiki.libsdl.org/SDL_PixelFormatEnum#Remarks)
pub type PackedOrder = _bindgen_ty_3;

/// See remarks of [`SDL_PixelFormatEnum`](https://wiki.libsdl.org/SDL_PixelFormatEnum#Remarks)
pub type ArrayOrder = _bindgen_ty_4;

/// See remarks of [`SDL_PixelFormatEnum`](https://wiki.libsdl.org/SDL_PixelFormatEnum#Remarks)
pub type PackedLayout = _bindgen_ty_5;

// For whatever reason, bindgen processes this type properly in the 2.0.10
// headers and later, so after ty_6 there's a discepency between what the
// unknown types mean.
cfg_if! {
  if #[cfg(not(feature = "bind_SDL2_2_0_10"))] {
    /// See [`SDL_PixelFormatEnum`](https://wiki.libsdl.org/SDL_PixelFormatEnum)
    pub type SDL_PixelFormatEnum = _bindgen_ty_6;
  }
}

cfg_if! {
  if #[cfg(not(feature = "bind_SDL2_2_0_10"))] {
    /// See [`SDL_Keycode`](https://wiki.libsdl.org/SDL_Keycode)
    pub type SDLK = _bindgen_ty_7;
  } else {
    /// See [`SDL_Keycode`](https://wiki.libsdl.org/SDL_Keycode)
    pub type SDLK = _bindgen_ty_6;
  }
}

cfg_if! {
  if #[cfg(not(feature = "bind_SDL2_2_0_10"))] {
    /// See [`SDL_LOG_CATEGORY`](https://wiki.libsdl.org/SDL_LOG_CATEGORY)
    pub type LogCategory = _bindgen_ty_8;
  } else {
    /// See [`SDL_LOG_CATEGORY`](https://wiki.libsdl.org/SDL_LOG_CATEGORY)
    pub type LogCategory = _bindgen_ty_7;
  }
}

/// `SDL_touch.h`: Used as the device ID for mouse events simulated with touch
/// input
pub const SDL_TOUCH_MOUSEID: u32 = -1i32 as u32;

// only present in 2.0.10 and later
cfg_if! {
  if #[cfg(feature = "bind_SDL2_2_0_10")] {
    /// `SDL_touch.h`: Used as the SDL_TouchID for touch events simulated with
    /// mouse input
    pub const SDL_MOUSE_TOUCHID: u64 = -1i64 as u64;
  }
}

/// `SDL_surface.h`: Evaluates to true if the surface needs to be locked before
/// access.
#[inline(always)]
pub unsafe fn SDL_MUSTLOCK(surface: *const SDL_Surface) -> bool {
  (*surface).flags & SDL_RLEACCEL != 0
}

/// `SDL_pixels.h`: "internal" macro to check if a value is a pixel format value.
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
pub const fn SDL_PIXELORDER(format: SDL_PixelFormatEnum) -> SDL_PixelFormatEnum {
  (format >> 20) & 0x0F
}

/// `SDL_pixels.h`: Channel width layout of this format.
#[inline(always)]
pub const fn SDL_PIXELLAYOUT(format: SDL_PixelFormatEnum) -> SDL_PixelFormatEnum {
  (format >> 16) & 0x0F
}

/// `SDL_pixels.h`: Bits per pixel.
#[inline(always)]
pub const fn SDL_BITSPERPIXEL(format: SDL_PixelFormatEnum) -> SDL_PixelFormatEnum {
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
