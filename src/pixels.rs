#![allow(missing_docs)]

//! Module for pixel editing.

use crate::{c_char, c_int, stdinc::*};

/// More alpha => more opaque.
pub const SDL_ALPHA_OPAQUE: u8 = 255;
/// Transparent pixels are alpha = 0.
pub const SDL_ALPHA_TRANSPARENT: u8 = 0;

/// Pixel type.
///
/// Instances of this value are called `SDL_PIXELTYPE_*`
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_PixelType(pub u32);
/// An unknown pixel type.
pub const SDL_PIXELTYPE_UNKNOWN: SDL_PixelType = SDL_PixelType(0);
pub const SDL_PIXELTYPE_INDEX1: SDL_PixelType = SDL_PixelType(1);
pub const SDL_PIXELTYPE_INDEX4: SDL_PixelType = SDL_PixelType(2);
pub const SDL_PIXELTYPE_INDEX8: SDL_PixelType = SDL_PixelType(3);
pub const SDL_PIXELTYPE_PACKED8: SDL_PixelType = SDL_PixelType(4);
pub const SDL_PIXELTYPE_PACKED16: SDL_PixelType = SDL_PixelType(5);
pub const SDL_PIXELTYPE_PACKED32: SDL_PixelType = SDL_PixelType(6);
pub const SDL_PIXELTYPE_ARRAYU8: SDL_PixelType = SDL_PixelType(7);
pub const SDL_PIXELTYPE_ARRAYU16: SDL_PixelType = SDL_PixelType(8);
pub const SDL_PIXELTYPE_ARRAYU32: SDL_PixelType = SDL_PixelType(9);
pub const SDL_PIXELTYPE_ARRAYF16: SDL_PixelType = SDL_PixelType(10);
pub const SDL_PIXELTYPE_ARRAYF32: SDL_PixelType = SDL_PixelType(11);

/// Bitmap pixel order, high bit -> low bit.
///
/// Instances of this value are called `SDL_BITMAPORDER_*`
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_BitmapOrder(pub u32);
/// No bitmap ordering.
pub const SDL_BITMAPORDER_NONE: SDL_BitmapOrder = SDL_BitmapOrder(0);
/// Bitmap ordering from high to low.
pub const SDL_BITMAPORDER_4321: SDL_BitmapOrder = SDL_BitmapOrder(1);
/// Bitmap ordering from low to high.
pub const SDL_BITMAPORDER_1234: SDL_BitmapOrder = SDL_BitmapOrder(2);

/// Packed component order, high bit -> low bit.
///
/// Instances of this value are called `SDL_PACKEDORDER_*`
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_PackedOrder(pub u32);
pub const SDL_PACKEDORDER_NONE: SDL_PackedOrder = SDL_PackedOrder(0);
pub const SDL_PACKEDORDER_XRGB: SDL_PackedOrder = SDL_PackedOrder(1);
pub const SDL_PACKEDORDER_RGBX: SDL_PackedOrder = SDL_PackedOrder(2);
pub const SDL_PACKEDORDER_ARGB: SDL_PackedOrder = SDL_PackedOrder(3);
pub const SDL_PACKEDORDER_RGBA: SDL_PackedOrder = SDL_PackedOrder(4);
pub const SDL_PACKEDORDER_XBGR: SDL_PackedOrder = SDL_PackedOrder(5);
pub const SDL_PACKEDORDER_BGRX: SDL_PackedOrder = SDL_PackedOrder(6);
pub const SDL_PACKEDORDER_ABGR: SDL_PackedOrder = SDL_PackedOrder(7);
pub const SDL_PACKEDORDER_BGRA: SDL_PackedOrder = SDL_PackedOrder(8);

/// Array component order, low byte -> high byte.
///
/// Instances of this value are called `SDL_ARRAYORDER_*`
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_ArrayOrder(pub u32);
pub const SDL_ARRAYORDER_NONE: SDL_ArrayOrder = SDL_ArrayOrder(0);
pub const SDL_ARRAYORDER_RGB: SDL_ArrayOrder = SDL_ArrayOrder(1);
pub const SDL_ARRAYORDER_RGBA: SDL_ArrayOrder = SDL_ArrayOrder(2);
pub const SDL_ARRAYORDER_ARGB: SDL_ArrayOrder = SDL_ArrayOrder(3);
pub const SDL_ARRAYORDER_BGR: SDL_ArrayOrder = SDL_ArrayOrder(4);
pub const SDL_ARRAYORDER_BGRA: SDL_ArrayOrder = SDL_ArrayOrder(5);
pub const SDL_ARRAYORDER_ABGR: SDL_ArrayOrder = SDL_ArrayOrder(6);

/// Packed component order, high bit -> low bit.
///
/// Instances of this value are called `SDL_PACKEDLAYOUT_*`
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_PackedLayout(pub u32);
pub const SDL_PACKEDLAYOUT_NONE: SDL_PackedLayout = SDL_PackedLayout(0);
pub const SDL_PACKEDLAYOUT_332: SDL_PackedLayout = SDL_PackedLayout(1);
pub const SDL_PACKEDLAYOUT_4444: SDL_PackedLayout = SDL_PackedLayout(2);
pub const SDL_PACKEDLAYOUT_1555: SDL_PackedLayout = SDL_PackedLayout(3);
pub const SDL_PACKEDLAYOUT_5551: SDL_PackedLayout = SDL_PackedLayout(4);
pub const SDL_PACKEDLAYOUT_565: SDL_PackedLayout = SDL_PackedLayout(5);
pub const SDL_PACKEDLAYOUT_8888: SDL_PackedLayout = SDL_PackedLayout(6);
pub const SDL_PACKEDLAYOUT_2101010: SDL_PackedLayout = SDL_PackedLayout(7);
pub const SDL_PACKEDLAYOUT_1010102: SDL_PackedLayout = SDL_PackedLayout(8);

/// Converts a FourCC into a pixel format enumeration value.
pub const fn SDL_DEFINE_PIXELFOURCC(
  a: u8, b: u8, c: u8, d: u8,
) -> SDL_PixelFormatEnum {
  SDL_PixelFormatEnum(SDL_FOURCC(a, b, c, d))
}

/// Converts the pixel type parameters into a pixel format enumeration value.
pub const fn SDL_DEFINE_PIXELFORMAT(
  type_: SDL_PixelType, order: u32, layout: u32, bits: u32, bytes: u32,
) -> SDL_PixelFormatEnum {
  SDL_PixelFormatEnum(
    (1 << 28)
      | ((type_.0) << 24)
      | ((order) << 20)
      | ((layout) << 16)
      | ((bits) << 8)
      | (bytes),
  )
}

/// Gets the pixel flag bits.
///
/// Generally 1, except that FourCC enumerations can have other values.
pub const fn SDL_PIXELFLAG(x: SDL_PixelFormatEnum) -> u32 {
  ((x.0) >> 28) & 0x0F
}
/// Gets the pixel type bits.
pub const fn SDL_PIXELTYPE(x: SDL_PixelFormatEnum) -> u32 {
  ((x.0) >> 24) & 0x0F
}
/// Gets the pixel order bits.
pub const fn SDL_PIXELORDER(x: SDL_PixelFormatEnum) -> u32 {
  ((x.0) >> 20) & 0x0F
}
/// Gets the pixel layout bits.
pub const fn SDL_PIXELLAYOUT(x: SDL_PixelFormatEnum) -> u32 {
  ((x.0) >> 16) & 0x0F
}
/// Gets the bits per pixel.
pub const fn SDL_BITSPERPIXEL(x: SDL_PixelFormatEnum) -> u32 {
  ((x.0) >> 8) & 0xFF
}
/// Gets the bytes per pixel.
pub const fn SDL_BYTESPERPIXEL(x: SDL_PixelFormatEnum) -> u32 {
  if SDL_ISPIXELFORMAT_FOURCC(x) {
    if ((x.0) == SDL_PIXELFORMAT_YUY2.0)
      || ((x.0) == SDL_PIXELFORMAT_UYVY.0)
      || ((x.0) == SDL_PIXELFORMAT_YVYU.0)
    {
      2
    } else {
      1
    }
  } else {
    x.0 & 0xFF
  }
}
/// Is this pixel format an indexed format?
pub const fn SDL_ISPIXELFORMAT_INDEXED(format: SDL_PixelFormatEnum) -> bool {
  !SDL_ISPIXELFORMAT_FOURCC(format)
    && ((SDL_PIXELTYPE(format) == SDL_PIXELTYPE_INDEX1.0)
      || (SDL_PIXELTYPE(format) == SDL_PIXELTYPE_INDEX4.0)
      || (SDL_PIXELTYPE(format) == SDL_PIXELTYPE_INDEX8.0))
}
/// Is this pixel format a packed format?
pub const fn SDL_ISPIXELFORMAT_PACKED(format: SDL_PixelFormatEnum) -> bool {
  !SDL_ISPIXELFORMAT_FOURCC(format)
    && ((SDL_PIXELTYPE(format) == SDL_PIXELTYPE_PACKED8.0)
      || (SDL_PIXELTYPE(format) == SDL_PIXELTYPE_PACKED16.0)
      || (SDL_PIXELTYPE(format) == SDL_PIXELTYPE_PACKED32.0))
}
/// Is this pixel format an array format?
pub const fn SDL_ISPIXELFORMAT_ARRAY(format: SDL_PixelFormatEnum) -> bool {
  !SDL_ISPIXELFORMAT_FOURCC(format)
    && ((SDL_PIXELTYPE(format) == SDL_PIXELTYPE_ARRAYU8.0)
      || (SDL_PIXELTYPE(format) == SDL_PIXELTYPE_ARRAYU16.0)
      || (SDL_PIXELTYPE(format) == SDL_PIXELTYPE_ARRAYU32.0)
      || (SDL_PIXELTYPE(format) == SDL_PIXELTYPE_ARRAYF16.0)
      || (SDL_PIXELTYPE(format) == SDL_PIXELTYPE_ARRAYF32.0))
}
/// Does the pixel format have an alpha channel?
pub const fn SDL_ISPIXELFORMAT_ALPHA(format: SDL_PixelFormatEnum) -> bool {
  (SDL_ISPIXELFORMAT_PACKED(format)
    && ((SDL_PIXELORDER(format) == SDL_PACKEDORDER_ARGB.0)
      || (SDL_PIXELORDER(format) == SDL_PACKEDORDER_RGBA.0)
      || (SDL_PIXELORDER(format) == SDL_PACKEDORDER_ABGR.0)
      || (SDL_PIXELORDER(format) == SDL_PACKEDORDER_BGRA.0)))
    || (SDL_ISPIXELFORMAT_ARRAY(format)
      && ((SDL_PIXELORDER(format) == SDL_ARRAYORDER_ARGB.0)
        || (SDL_PIXELORDER(format) == SDL_ARRAYORDER_RGBA.0)
        || (SDL_PIXELORDER(format) == SDL_ARRAYORDER_ABGR.0)
        || (SDL_PIXELORDER(format) == SDL_ARRAYORDER_BGRA.0)))
}
/// Is the pixel format a FourCC format?
pub const fn SDL_ISPIXELFORMAT_FOURCC(format: SDL_PixelFormatEnum) -> bool {
  (format.0 != 0) && (SDL_PIXELFLAG(format) != 1)
}

/// An enumerated pixel format value.
///
/// Instances of this value are called `SDL_PIXELFORMAT_*`
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_PixelFormatEnum(pub u32);
pub const SDL_PIXELFORMAT_UNKNOWN: SDL_PixelFormatEnum = SDL_PixelFormatEnum(0);
pub const SDL_PIXELFORMAT_INDEX1LSB: SDL_PixelFormatEnum =
  SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_INDEX1, SDL_BITMAPORDER_4321.0, 0, 1, 0);
pub const SDL_PIXELFORMAT_INDEX1MSB: SDL_PixelFormatEnum =
  SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_INDEX1, SDL_BITMAPORDER_1234.0, 0, 1, 0);
pub const SDL_PIXELFORMAT_INDEX4LSB: SDL_PixelFormatEnum =
  SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_INDEX4, SDL_BITMAPORDER_4321.0, 0, 4, 0);
pub const SDL_PIXELFORMAT_INDEX4MSB: SDL_PixelFormatEnum =
  SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_INDEX4, SDL_BITMAPORDER_1234.0, 0, 4, 0);
pub const SDL_PIXELFORMAT_INDEX8: SDL_PixelFormatEnum =
  SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_INDEX8, 0, 0, 8, 1);
pub const SDL_PIXELFORMAT_RGB332: SDL_PixelFormatEnum = SDL_DEFINE_PIXELFORMAT(
  SDL_PIXELTYPE_PACKED8,
  SDL_PACKEDORDER_XRGB.0,
  SDL_PACKEDLAYOUT_332.0,
  8,
  1,
);
pub const SDL_PIXELFORMAT_XRGB4444: SDL_PixelFormatEnum =
  SDL_DEFINE_PIXELFORMAT(
    SDL_PIXELTYPE_PACKED16,
    SDL_PACKEDORDER_XRGB.0,
    SDL_PACKEDLAYOUT_4444.0,
    12,
    2,
  );
pub const SDL_PIXELFORMAT_RGB444: SDL_PixelFormatEnum =
  SDL_PIXELFORMAT_XRGB4444;
pub const SDL_PIXELFORMAT_XBGR4444: SDL_PixelFormatEnum =
  SDL_DEFINE_PIXELFORMAT(
    SDL_PIXELTYPE_PACKED16,
    SDL_PACKEDORDER_XBGR.0,
    SDL_PACKEDLAYOUT_4444.0,
    12,
    2,
  );
pub const SDL_PIXELFORMAT_BGR444: SDL_PixelFormatEnum =
  SDL_PIXELFORMAT_XBGR4444;
pub const SDL_PIXELFORMAT_XRGB1555: SDL_PixelFormatEnum =
  SDL_DEFINE_PIXELFORMAT(
    SDL_PIXELTYPE_PACKED16,
    SDL_PACKEDORDER_XRGB.0,
    SDL_PACKEDLAYOUT_1555.0,
    15,
    2,
  );
pub const SDL_PIXELFORMAT_RGB555: SDL_PixelFormatEnum =
  SDL_PIXELFORMAT_XRGB1555;
pub const SDL_PIXELFORMAT_XBGR1555: SDL_PixelFormatEnum =
  SDL_DEFINE_PIXELFORMAT(
    SDL_PIXELTYPE_PACKED16,
    SDL_PACKEDORDER_XBGR.0,
    SDL_PACKEDLAYOUT_1555.0,
    15,
    2,
  );
pub const SDL_PIXELFORMAT_BGR555: SDL_PixelFormatEnum =
  SDL_PIXELFORMAT_XBGR1555;
pub const SDL_PIXELFORMAT_ARGB4444: SDL_PixelFormatEnum =
  SDL_DEFINE_PIXELFORMAT(
    SDL_PIXELTYPE_PACKED16,
    SDL_PACKEDORDER_ARGB.0,
    SDL_PACKEDLAYOUT_4444.0,
    16,
    2,
  );
pub const SDL_PIXELFORMAT_RGBA4444: SDL_PixelFormatEnum =
  SDL_DEFINE_PIXELFORMAT(
    SDL_PIXELTYPE_PACKED16,
    SDL_PACKEDORDER_RGBA.0,
    SDL_PACKEDLAYOUT_4444.0,
    16,
    2,
  );
pub const SDL_PIXELFORMAT_ABGR4444: SDL_PixelFormatEnum =
  SDL_DEFINE_PIXELFORMAT(
    SDL_PIXELTYPE_PACKED16,
    SDL_PACKEDORDER_ABGR.0,
    SDL_PACKEDLAYOUT_4444.0,
    16,
    2,
  );
pub const SDL_PIXELFORMAT_BGRA4444: SDL_PixelFormatEnum =
  SDL_DEFINE_PIXELFORMAT(
    SDL_PIXELTYPE_PACKED16,
    SDL_PACKEDORDER_BGRA.0,
    SDL_PACKEDLAYOUT_4444.0,
    16,
    2,
  );
pub const SDL_PIXELFORMAT_ARGB1555: SDL_PixelFormatEnum =
  SDL_DEFINE_PIXELFORMAT(
    SDL_PIXELTYPE_PACKED16,
    SDL_PACKEDORDER_ARGB.0,
    SDL_PACKEDLAYOUT_1555.0,
    16,
    2,
  );
pub const SDL_PIXELFORMAT_RGBA5551: SDL_PixelFormatEnum =
  SDL_DEFINE_PIXELFORMAT(
    SDL_PIXELTYPE_PACKED16,
    SDL_PACKEDORDER_RGBA.0,
    SDL_PACKEDLAYOUT_5551.0,
    16,
    2,
  );
pub const SDL_PIXELFORMAT_ABGR1555: SDL_PixelFormatEnum =
  SDL_DEFINE_PIXELFORMAT(
    SDL_PIXELTYPE_PACKED16,
    SDL_PACKEDORDER_ABGR.0,
    SDL_PACKEDLAYOUT_1555.0,
    16,
    2,
  );
pub const SDL_PIXELFORMAT_BGRA5551: SDL_PixelFormatEnum =
  SDL_DEFINE_PIXELFORMAT(
    SDL_PIXELTYPE_PACKED16,
    SDL_PACKEDORDER_BGRA.0,
    SDL_PACKEDLAYOUT_5551.0,
    16,
    2,
  );
pub const SDL_PIXELFORMAT_RGB565: SDL_PixelFormatEnum = SDL_DEFINE_PIXELFORMAT(
  SDL_PIXELTYPE_PACKED16,
  SDL_PACKEDORDER_XRGB.0,
  SDL_PACKEDLAYOUT_565.0,
  16,
  2,
);
pub const SDL_PIXELFORMAT_BGR565: SDL_PixelFormatEnum = SDL_DEFINE_PIXELFORMAT(
  SDL_PIXELTYPE_PACKED16,
  SDL_PACKEDORDER_XBGR.0,
  SDL_PACKEDLAYOUT_565.0,
  16,
  2,
);
pub const SDL_PIXELFORMAT_RGB24: SDL_PixelFormatEnum =
  SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_ARRAYU8, SDL_ARRAYORDER_RGB.0, 0, 24, 3);
pub const SDL_PIXELFORMAT_BGR24: SDL_PixelFormatEnum =
  SDL_DEFINE_PIXELFORMAT(SDL_PIXELTYPE_ARRAYU8, SDL_ARRAYORDER_BGR.0, 0, 24, 3);
pub const SDL_PIXELFORMAT_XRGB8888: SDL_PixelFormatEnum =
  SDL_DEFINE_PIXELFORMAT(
    SDL_PIXELTYPE_PACKED32,
    SDL_PACKEDORDER_XRGB.0,
    SDL_PACKEDLAYOUT_8888.0,
    24,
    4,
  );
pub const SDL_PIXELFORMAT_RGB888: SDL_PixelFormatEnum =
  SDL_PIXELFORMAT_XRGB8888;
pub const SDL_PIXELFORMAT_RGBX8888: SDL_PixelFormatEnum =
  SDL_DEFINE_PIXELFORMAT(
    SDL_PIXELTYPE_PACKED32,
    SDL_PACKEDORDER_RGBX.0,
    SDL_PACKEDLAYOUT_8888.0,
    24,
    4,
  );
pub const SDL_PIXELFORMAT_XBGR8888: SDL_PixelFormatEnum =
  SDL_DEFINE_PIXELFORMAT(
    SDL_PIXELTYPE_PACKED32,
    SDL_PACKEDORDER_XBGR.0,
    SDL_PACKEDLAYOUT_8888.0,
    24,
    4,
  );
pub const SDL_PIXELFORMAT_BGR888: SDL_PixelFormatEnum =
  SDL_PIXELFORMAT_XBGR8888;
pub const SDL_PIXELFORMAT_BGRX8888: SDL_PixelFormatEnum =
  SDL_DEFINE_PIXELFORMAT(
    SDL_PIXELTYPE_PACKED32,
    SDL_PACKEDORDER_BGRX.0,
    SDL_PACKEDLAYOUT_8888.0,
    24,
    4,
  );
pub const SDL_PIXELFORMAT_ARGB8888: SDL_PixelFormatEnum =
  SDL_DEFINE_PIXELFORMAT(
    SDL_PIXELTYPE_PACKED32,
    SDL_PACKEDORDER_ARGB.0,
    SDL_PACKEDLAYOUT_8888.0,
    32,
    4,
  );
pub const SDL_PIXELFORMAT_RGBA8888: SDL_PixelFormatEnum =
  SDL_DEFINE_PIXELFORMAT(
    SDL_PIXELTYPE_PACKED32,
    SDL_PACKEDORDER_RGBA.0,
    SDL_PACKEDLAYOUT_8888.0,
    32,
    4,
  );
pub const SDL_PIXELFORMAT_ABGR8888: SDL_PixelFormatEnum =
  SDL_DEFINE_PIXELFORMAT(
    SDL_PIXELTYPE_PACKED32,
    SDL_PACKEDORDER_ABGR.0,
    SDL_PACKEDLAYOUT_8888.0,
    32,
    4,
  );
pub const SDL_PIXELFORMAT_BGRA8888: SDL_PixelFormatEnum =
  SDL_DEFINE_PIXELFORMAT(
    SDL_PIXELTYPE_PACKED32,
    SDL_PACKEDORDER_BGRA.0,
    SDL_PACKEDLAYOUT_8888.0,
    32,
    4,
  );
pub const SDL_PIXELFORMAT_ARGB2101010: SDL_PixelFormatEnum =
  SDL_DEFINE_PIXELFORMAT(
    SDL_PIXELTYPE_PACKED32,
    SDL_PACKEDORDER_ARGB.0,
    SDL_PACKEDLAYOUT_2101010.0,
    32,
    4,
  );
pub const SDL_PIXELFORMAT_RGBA32: SDL_PixelFormatEnum =
  if cfg!(target_endian = "big") {
    SDL_PIXELFORMAT_ABGR8888
  } else {
    SDL_PIXELFORMAT_RGBA8888
  };
pub const SDL_PIXELFORMAT_ARGB32: SDL_PixelFormatEnum =
  if cfg!(target_endian = "big") {
    SDL_PIXELFORMAT_BGRA8888
  } else {
    SDL_PIXELFORMAT_ARGB8888
  };
pub const SDL_PIXELFORMAT_BGRA32: SDL_PixelFormatEnum =
  if cfg!(target_endian = "big") {
    SDL_PIXELFORMAT_ARGB8888
  } else {
    SDL_PIXELFORMAT_BGRA8888
  };
pub const SDL_PIXELFORMAT_ABGR32: SDL_PixelFormatEnum =
  if cfg!(target_endian = "big") {
    SDL_PIXELFORMAT_RGBA8888
  } else {
    SDL_PIXELFORMAT_ABGR8888
  };
/// Planar mode: Y + V + U  (3 planes)
pub const SDL_PIXELFORMAT_YV12: SDL_PixelFormatEnum =
  SDL_DEFINE_PIXELFOURCC(b'Y', b'V', b'1', b'2');
/// Planar mode: Y + U + V  (3 planes)
pub const SDL_PIXELFORMAT_IYUV: SDL_PixelFormatEnum =
  SDL_DEFINE_PIXELFOURCC(b'I', b'Y', b'U', b'V');
/// Packed mode: Y0+U0+Y1+V0 (1 plane)
pub const SDL_PIXELFORMAT_YUY2: SDL_PixelFormatEnum =
  SDL_DEFINE_PIXELFOURCC(b'Y', b'U', b'Y', b'2');
/// Packed mode: U0+Y0+V0+Y1 (1 plane)
pub const SDL_PIXELFORMAT_UYVY: SDL_PixelFormatEnum =
  SDL_DEFINE_PIXELFOURCC(b'U', b'Y', b'V', b'Y');
/// Packed mode: Y0+V0+Y1+U0 (1 plane)
pub const SDL_PIXELFORMAT_YVYU: SDL_PixelFormatEnum =
  SDL_DEFINE_PIXELFOURCC(b'Y', b'V', b'Y', b'U');
/// Planar mode: Y + U/V interleaved  (2 planes)
pub const SDL_PIXELFORMAT_NV12: SDL_PixelFormatEnum =
  SDL_DEFINE_PIXELFOURCC(b'N', b'V', b'1', b'2');
/// Planar mode: Y + V/U interleaved  (2 planes)
pub const SDL_PIXELFORMAT_NV21: SDL_PixelFormatEnum =
  SDL_DEFINE_PIXELFOURCC(b'N', b'V', b'2', b'1');
/// Android video texture format
pub const SDL_PIXELFORMAT_EXTERNAL_OES: SDL_PixelFormatEnum =
  SDL_DEFINE_PIXELFOURCC(b'O', b'E', b'S', b' ');

/// An RGBA color value (8-bits per channel).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
#[allow(missing_docs)]
pub struct SDL_Color {
  pub r: Uint8,
  pub g: Uint8,
  pub b: Uint8,
  pub a: Uint8,
}

/// Info about a palette of colors.
///
/// Generally, you shouldn't alter these fields. In fact usually you shouldn't
/// even allocate or free these on your own, SDL2 will do so for you at the
/// appropriate times.
#[derive(Debug)]
#[repr(C)]
pub struct SDL_Palette {
  pub ncolors: c_int,
  pub colors: *mut SDL_Color,
  pub version: Uint32,
  pub refcount: c_int,
}

/// An SDL Pixel Format.
///
/// Generally, you shouldn't alter these fields.
#[derive(Debug)]
#[repr(C)]
pub struct SDL_PixelFormat {
  pub format: Uint32,
  pub palette: *mut SDL_Palette,
  pub BitsPerPixel: Uint8,
  pub BytesPerPixel: Uint8,
  pub padding: [Uint8; 2],
  pub Rmask: Uint32,
  pub Gmask: Uint32,
  pub Bmask: Uint32,
  pub Amask: Uint32,
  pub Rloss: Uint8,
  pub Gloss: Uint8,
  pub Bloss: Uint8,
  pub Aloss: Uint8,
  pub Rshift: Uint8,
  pub Gshift: Uint8,
  pub Bshift: Uint8,
  pub Ashift: Uint8,
  pub refcount: c_int,
  pub next: *mut SDL_PixelFormat,
}

extern "C" {
  /// Get the human readable name of a pixel format
  pub fn SDL_GetPixelFormatName(format: Uint32) -> *const c_char;

  /// Convert one of the enumerated pixel formats to a bpp and RGBA masks.
  ///
  /// **Return:** `SDL_TRUE`, or `SDL_FALSE` if the conversion wasn't possible.
  pub fn SDL_PixelFormatEnumToMasks(
    format: Uint32, bpp: *mut c_int, Rmask: *mut Uint32, Gmask: *mut Uint32,
    Bmask: *mut Uint32, Amask: *mut Uint32,
  ) -> SDL_bool;

  /// Convert a bpp and RGBA masks to an enumerated pixel format.
  ///
  /// **Return:** The pixel format, or `SDL_PIXELFORMAT_UNKNOWN` if the
  /// conversion wasn't possible.
  pub fn SDL_MasksToPixelFormatEnum(
    bpp: c_int, Rmask: Uint32, Gmask: Uint32, Bmask: Uint32, Amask: Uint32,
  ) -> Uint32;

  /// Create an [`SDL_PixelFormat`] structure from a pixel format enum.
  pub fn SDL_AllocFormat(pixel_format: Uint32) -> *mut SDL_PixelFormat;

  /// Free an [`SDL_PixelFormat`] structure.
  pub fn SDL_FreeFormat(format: *mut SDL_PixelFormat);

  /// Create a palette structure with the specified number of color entries.
  ///
  /// The palette entries are initialized to white.
  ///
  /// **Return:** A new palette, or NULL if there wasn't enough memory.
  pub fn SDL_AllocPalette(ncolors: c_int) -> *mut SDL_Palette;

  /// Set the palette for a pixel format structure.
  pub fn SDL_SetPixelFormatPalette(
    format: *mut SDL_PixelFormat, palette: *mut SDL_Palette,
  ) -> c_int;

  /// Set a range of colors in a palette.
  ///
  /// * `palette`: The palette to modify.
  /// * `colors`: An array of colors to copy into the palette.
  /// * `firstcolor`: The index of the first palette entry to modify.
  /// * `ncolors`: The number of entries to modify.
  ///
  /// **Return:** 0 on success, or -1 if not all of the colors could be set.
  pub fn SDL_SetPaletteColors(
    palette: *mut SDL_Palette, colors: *const SDL_Color, firstcolor: c_int,
    ncolors: c_int,
  ) -> c_int;

  /// Free a palette created with [`SDL_AllocPalette`].
  pub fn SDL_FreePalette(palette: *mut SDL_Palette);

  /// Maps an RGB triple to an opaque pixel value for a given pixel format.
  pub fn SDL_MapRGB(
    format: *const SDL_PixelFormat, r: Uint8, g: Uint8, b: Uint8,
  ) -> Uint32;

  /// Maps an RGBA quadruple to a pixel value for a given pixel format.
  pub fn SDL_MapRGBA(
    format: *const SDL_PixelFormat, r: Uint8, g: Uint8, b: Uint8, a: Uint8,
  ) -> Uint32;

  /// Get the RGB components from a pixel of the specified format.
  pub fn SDL_GetRGB(
    pixel: Uint32, format: *const SDL_PixelFormat, r: *mut Uint8,
    g: *mut Uint8, b: *mut Uint8,
  );

  /// Get the RGBA components from a pixel of the specified format.
  pub fn SDL_GetRGBA(
    pixel: Uint32, format: *const SDL_PixelFormat, r: *mut Uint8,
    g: *mut Uint8, b: *mut Uint8, a: *mut Uint8,
  );

  /// Calculate a 256 entry gamma ramp for a gamma value.
  pub fn SDL_CalculateGammaRamp(gamma: f32, ramp: *mut Uint16);
}
