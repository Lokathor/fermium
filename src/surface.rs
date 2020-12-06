pub use crate::{blendmode::*, pixels::*, rect::*, rwops::*, stdinc::*};

/// Surface uses preallocated memory
pub const SDL_PREALLOC: u32 = 0x00000001;
/// Surface is RLE encoded
pub const SDL_RLEACCEL: u32 = 0x00000002;
/// Surface is referenced internally
pub const SDL_DONTFREE: u32 = 0x00000004;
/// Surface uses aligned memory
pub const SDL_SIMD_ALIGNED: u32 = 0x00000008;

/// Checks if an [`SDL_Surface`] must be locked before the pixels are accessed
/// directly.
#[inline(always)]
pub unsafe fn SDL_MUSTLOCK(s: *const SDL_Surface) -> bool {
  ((*s).flags & SDL_RLEACCEL) != 0
}

#[allow(unused)]
#[repr(transparent)]
pub struct SDL_BlitMap(c_void);

#[derive(Debug)]
#[repr(C)]
pub struct SDL_Surface {
  pub flags: Uint32,
  pub format: *mut SDL_PixelFormat,
  pub w: c_int,
  pub h: c_int,
  pub pitch: c_int,
  pub pixels: *mut c_void,
  pub userdata: *mut c_void,
  pub locked: c_int,
  pub lock_data: *mut c_void,
  pub clip_rect: SDL_Rect,
  pub map: *mut SDL_BlitMap,
  pub refcount: c_int,
}

/// The formula used for converting between YUV and RGB.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_YUV_CONVERSION_MODE(pub u32);
/// Full range JPEG
pub const SDL_YUV_CONVERSION_JPEG: SDL_YUV_CONVERSION_MODE =
  SDL_YUV_CONVERSION_MODE(0);
/// BT.601 (the default)
pub const SDL_YUV_CONVERSION_BT601: SDL_YUV_CONVERSION_MODE =
  SDL_YUV_CONVERSION_MODE(1);
/// BT.709
pub const SDL_YUV_CONVERSION_BT709: SDL_YUV_CONVERSION_MODE =
  SDL_YUV_CONVERSION_MODE(2);
/// BT.601 for SD content, BT.709 for HD content
pub const SDL_YUV_CONVERSION_AUTOMATIC: SDL_YUV_CONVERSION_MODE =
  SDL_YUV_CONVERSION_MODE(3);

pub unsafe fn SDL_LoadBMP(file: *const c_char) -> *mut SDL_Surface {
  SDL_LoadBMP_RW(SDL_RWFromFile(file, b"rb\0".as_ptr().cast()), 1)
}

pub unsafe fn SDL_SaveBMP(
  surface: *mut SDL_Surface, file: *const c_char,
) -> c_int {
  SDL_SaveBMP_RW(surface, SDL_RWFromFile(file, b"wb\0".as_ptr().cast()), 1)
}

extern "C" {
  /// Allocate and free an RGB surface.
  ///
  /// If the depth is 4 or 8 bits, an empty palette is allocated for the
  /// surface. If the depth is greater than 8 bits, the pixel format is set
  /// using the flags '[RGB]mask'.
  ///
  /// If the function runs out of memory, it will return NULL.
  ///
  /// * `flags`: The `flags` are obsolete and should be set to 0.
  /// * `width`: The width in pixels of the surface to create.
  /// * `height`: The height in pixels of the surface to create.
  /// * `depth`: The depth in bits of the surface to create.
  /// * `Rmask`: The red mask of the surface to create.
  /// * `Gmask`: The green mask of the surface to create.
  /// * `Bmask`: The blue mask of the surface to create.
  /// * `Amask`: The alpha mask of the surface to create.
  pub fn SDL_CreateRGBSurface(
    flags: Uint32, width: c_int, height: c_int, depth: c_int, Rmask: Uint32,
    Gmask: Uint32, Bmask: Uint32, Amask: Uint32,
  ) -> *mut SDL_Surface;

  pub fn SDL_CreateRGBSurfaceWithFormat(
    flags: Uint32, width: c_int, height: c_int, depth: c_int, format: Uint32,
  ) -> *mut SDL_Surface;

  pub fn SDL_CreateRGBSurfaceFrom(
    pixels: *mut c_void, width: c_int, height: c_int, depth: c_int,
    pitch: c_int, Rmask: Uint32, Gmask: Uint32, Bmask: Uint32, Amask: Uint32,
  ) -> *mut SDL_Surface;

  pub fn SDL_CreateRGBSurfaceWithFormatFrom(
    pixels: *mut c_void, width: c_int, height: c_int, depth: c_int,
    pitch: c_int, format: Uint32,
  ) -> *mut SDL_Surface;

  pub fn SDL_FreeSurface(surface: *mut SDL_Surface);

  /// Set the palette used by a surface.
  ///
  /// A single palette can be shared with many surfaces.
  ///
  /// **Return:** 0, or -1 if the surface format doesn't use a palette.
  pub fn SDL_SetSurfacePalette(
    surface: *mut SDL_Surface, palette: *mut SDL_Palette,
  ) -> c_int;

  /// Sets up a surface for directly accessing the pixels.
  ///
  /// Between calls to [`SDL_LockSurface`] / [`SDL_UnlockSurface`], you can
  /// write to and read from `surface->pixels`, using the pixel format stored
  /// in `surface->format`.  Once you are done accessing the surface, you
  /// should use [`SDL_UnlockSurface`] to release it.
  ///
  /// Not all surfaces require locking.  If
  /// [`SDL_MUSTLOCK(surface)`](SDL_MUSTLOCK) evaluates to 0, then you can
  /// read and write to the surface at any time, and the pixel format of the
  /// surface will not change.
  ///
  /// No operating system or library calls should be made between lock/unlock
  /// pairs, as critical system locks may be held during this time.
  ///
  /// [`SDL_LockSurface`] returns 0, or -1 if the surface couldn't be locked.
  pub fn SDL_LockSurface(surface: *mut SDL_Surface) -> c_int;

  /// See [`SDL_LockSurface`]
  pub fn SDL_UnlockSurface(surface: *mut SDL_Surface);

  /// Load a surface from a seekable SDL data stream (memory or file).
  ///
  /// If `freesrc` is non-zero, the stream will be closed after being read.
  ///
  /// The new surface should be freed with SDL_FreeSurface().
  ///
  /// **Return:** the new surface, or NULL if there was an error.
  pub fn SDL_LoadBMP_RW(
    src: *mut SDL_RWops, freesrc: c_int,
  ) -> *mut SDL_Surface;

  /// Save a surface to a seekable SDL data stream (memory or file).
  ///
  /// Surfaces with a 24-bit, 32-bit and paletted 8-bit format get saved in the
  /// BMP directly. Other RGB formats with 8-bit or higher get converted to a
  /// 24-bit surface or, if they have an alpha mask or a colorkey, to a 32-bit
  /// surface before they are saved. YUV and paletted 1-bit and 4-bit formats
  /// are not supported.
  ///
  /// If `freedst` is non-zero, the stream will be closed after being written.
  ///
  /// **Return:** 0 if successful or -1 if there was an error.
  pub fn SDL_SaveBMP_RW(
    surface: *mut SDL_Surface, dst: *mut SDL_RWops, freedst: c_int,
  ) -> c_int;

  /// Sets the RLE acceleration hint for a surface.
  ///
  /// If RLE is enabled, colorkey and alpha blending blits are much faster,
  /// but the surface must be locked before directly accessing the pixels.
  ///
  /// **Return:** 0 on success, or -1 if the surface is not valid
  pub fn SDL_SetSurfaceRLE(surface: *mut SDL_Surface, flag: c_int) -> c_int;

  /// Sets the color key (transparent pixel) in a blit-able surface.
  ///
  /// You can pass SDL_RLEACCEL to enable RLE accelerated blits.
  ///
  /// * `surface`: The surface to update
  /// * `flag`: Non-zero to enable colorkey and 0 to disable colorkey
  /// * `key`: The transparent pixel in the native surface format
  ///
  /// **Return:** 0 on success, or -1 if the surface is not valid
  pub fn SDL_SetColorKey(
    surface: *mut SDL_Surface, flag: c_int, key: Uint32,
  ) -> c_int;

  /// Returns whether the surface has a color key
  ///
  /// **Return:** `SDL_TRUE` if the surface has a color key, or `SDL_FALSE` if
  /// the surface is NULL or has no color key
  pub fn SDL_HasColorKey(surface: *mut SDL_Surface) -> SDL_bool;

  /// Gets the color key (transparent pixel) in a blit-able surface.
  ///
  /// * `surface`: The surface to update
  /// * `key`: A pointer filled in with the transparent pixel in the native
  ///   surface format
  ///
  /// **Return:** 0 on success, or -1 if the surface is not valid or colorkey is
  /// not enabled.
  pub fn SDL_GetColorKey(surface: *mut SDL_Surface, key: *mut Uint32) -> c_int;

  /// Set an additional color value used in blit operations.
  ///
  /// * `surface`: The surface to update.
  /// * `r`: The red color value multiplied into blit operations.
  /// * `g`: The green color value multiplied into blit operations.
  /// * `b`: The blue color value multiplied into blit operations.
  ///
  /// **Return:** 0 on success, or -1 if the surface is not valid.
  pub fn SDL_SetSurfaceColorMod(
    surface: *mut SDL_Surface, r: Uint8, g: Uint8, b: Uint8,
  ) -> c_int;

  /// Get the additional color value used in blit operations.
  ///
  /// * `surface`: The surface to query.
  /// * `r`: A pointer filled in with the current red color value.
  /// * `g`: A pointer filled in with the current green color value.
  /// * `b`: A pointer filled in with the current blue color value.
  ///
  /// **Return:** 0 on success, or -1 if the surface is not valid.
  pub fn SDL_GetSurfaceColorMod(
    surface: *mut SDL_Surface, r: *mut Uint8, g: *mut Uint8, b: *mut Uint8,
  ) -> c_int;

  /// Get the additional alpha value used in blit operations.
  ///
  /// * `surface`: The surface to query.
  /// * `alpha`: A pointer filled in with the current alpha value.
  ///
  /// **Return:** 0 on success, or -1 if the surface is not valid.
  pub fn SDL_SetSurfaceAlphaMod(
    surface: *mut SDL_Surface, alpha: Uint8,
  ) -> c_int;

  /// Set the blend mode used for blit operations.
  ///
  /// * `surface`: The surface to update.
  /// * `blendMode`: [`SDL_BlendMode`] to use for blit blending.
  ///
  /// **Return:** 0 on success, or -1 if the parameters are not valid.
  pub fn SDL_SetSurfaceBlendMode(
    surface: *mut SDL_Surface, blendMode: SDL_BlendMode,
  ) -> c_int;

  /// Get the blend mode used for blit operations.
  ///
  /// * `surface`:   The surface to query.
  /// * `blendMode`: A pointer filled in with the current blend mode.
  ///
  /// **Return:** 0 on success, or -1 if the surface is not valid.
  pub fn SDL_GetSurfaceBlendMode(
    surface: *mut SDL_Surface, blendMode: *mut SDL_BlendMode,
  ) -> c_int;

  /// Sets the clipping rectangle for the destination surface in a blit.
  ///
  /// If the clip rectangle is NULL, clipping will be disabled.
  ///
  /// If the clip rectangle doesn't intersect the surface, the function will
  /// return `SDL_FALSE` and blits will be completely clipped.  Otherwise the
  /// function returns `SDL_TRUE` and blits to the surface will be clipped to
  /// the intersection of the surface area and the clipping rectangle.
  ///
  /// Note that blits are automatically clipped to the edges of the source
  /// and destination surfaces.
  pub fn SDL_SetClipRect(
    surface: *mut SDL_Surface, rect: *const SDL_Rect,
  ) -> SDL_bool;

  // Gets the clipping rectangle for the destination surface in a blit.
  //
  // `rect` must be a pointer to a valid rectangle which will be filled
  // with the correct values.
  pub fn SDL_GetClipRect(surface: *mut SDL_Surface, rect: *mut SDL_Rect);

  /// Creates a new surface identical to the existing surface
  pub fn SDL_DuplicateSurface(surface: *mut SDL_Surface) -> *mut SDL_Surface;

  /// Creates a new surface of the specified format, and then copies and maps
  /// the given surface to it so the blit of the converted surface will be as
  /// fast as possible.  If this function fails, it returns NULL.
  ///
  /// The `flags` parameter is passed to [`SDL_CreateRGBSurface`] and has those
  /// semantics.  You can also pass `SDL_RLEACCEL` in the flags parameter and
  /// SDL will try to RLE accelerate colorkey and alpha blits in the resulting
  /// surface.
  pub fn SDL_ConvertSurface(
    src: *mut SDL_Surface, fmt: *const SDL_PixelFormat, flags: Uint32,
  ) -> *mut SDL_Surface;

  /// See [`SDL_ConvertSurface`]
  pub fn SDL_ConvertSurfaceFormat(
    src: *mut SDL_Surface, pixel_format: Uint32, flags: Uint32,
  ) -> *mut SDL_Surface;

  /// Copy a block of pixels of one format to another format
  ///
  /// **Return:** 0 on success, or -1 if there was an error
  pub fn SDL_ConvertPixels(
    width: c_int, height: c_int, src_format: Uint32, src: *const c_void,
    src_pitch: c_int, dst_format: Uint32, dst: *mut c_void, dst_pitch: c_int,
  ) -> c_int;

  /// Performs a fast fill of the given rectangle with `color`.
  ///
  /// If `rect` is NULL, the whole surface will be filled with `color`.
  ///
  /// The color should be a pixel of the format used by the surface, and
  /// can be generated by the SDL_MapRGB() function.
  ///
  /// **Return:** 0 on success, or -1 on error.
  pub fn SDL_FillRect(
    dst: *mut SDL_Surface, rect: *const SDL_Rect, color: Uint32,
  ) -> c_int;

  /// See [`SDL_FillRect`]
  pub fn SDL_FillRects(
    dst: *mut SDL_Surface, rects: *const SDL_Rect, count: c_int, color: Uint32,
  ) -> c_int;

  /// Performs a fast blit from the source surface to the destination surface.
  ///
  /// This assumes that the source and destination rectangles are
  /// the same size.  If either `srcrect` or `dstrect` are NULL, the entire
  /// surface (`src` or `dst`) is copied.  The final blit rectangles are saved
  /// in `srcrect` and `dstrect` after all clipping is performed.
  ///
  /// **Return:** If the blit is successful, it returns 0, otherwise it returns
  /// -1.
  ///
  /// The blit function should **not** be called on a locked surface.
  ///
  /// The blit semantics for surfaces with and without blending and colorkey
  /// are defined as follows:
  /// ```txt
  /// RGBA->RGB:
  ///   Source surface blend mode set to SDL_BLENDMODE_BLEND:
  ///     alpha-blend (using the source alpha-channel and per-surface alpha)
  ///     SDL_SRCCOLORKEY ignored.
  ///   Source surface blend mode set to SDL_BLENDMODE_NONE:
  ///     copy RGB.
  ///     if SDL_SRCCOLORKEY set, only copy the pixels matching the
  ///     RGB values of the source color key, ignoring alpha in the
  ///     comparison.
  /// RGB->RGBA:
  ///   Source surface blend mode set to SDL_BLENDMODE_BLEND:
  ///     alpha-blend (using the source per-surface alpha)
  ///   Source surface blend mode set to SDL_BLENDMODE_NONE:
  ///     copy RGB, set destination alpha to source per-surface alpha value.
  ///   both:
  ///     if SDL_SRCCOLORKEY set, only copy the pixels matching the
  ///     source color key.
  /// RGBA->RGBA:
  ///   Source surface blend mode set to SDL_BLENDMODE_BLEND:
  ///     alpha-blend (using the source alpha-channel and per-surface alpha)
  ///     SDL_SRCCOLORKEY ignored.
  ///   Source surface blend mode set to SDL_BLENDMODE_NONE:
  ///     copy all of RGBA to the destination.
  ///     if SDL_SRCCOLORKEY set, only copy the pixels matching the
  ///     RGB values of the source color key, ignoring alpha in the
  ///     comparison.
  /// RGB->RGB:
  ///   Source surface blend mode set to SDL_BLENDMODE_BLEND:
  ///     alpha-blend (using the source per-surface alpha)
  ///   Source surface blend mode set to SDL_BLENDMODE_NONE:
  ///     copy RGB.
  ///   both:
  ///     if SDL_SRCCOLORKEY set, only copy the pixels matching the
  ///     source color key.
  /// ```
  ///
  /// You should call [`SDL_BlitSurface`] unless you know exactly how SDL
  /// blitting works internally and how to use the other blit functions.
  #[link_name = "SDL_UpperBlit"]
  pub fn SDL_BlitSurface(
    src: *mut SDL_Surface, srcrect: *const SDL_Rect, dst: *mut SDL_Surface,
    dstrect: *mut SDL_Rect,
  ) -> c_int;

  /// This is the public scaled blit function, SDL_BlitScaled(), and it performs
  /// rectangle validation and clipping before passing it to
  /// SDL_LowerBlitScaled()
  #[link_name = "SDL_UpperBlitScaled"]
  pub fn SDL_BlitScaled(
    src: *mut SDL_Surface, srcrect: *const SDL_Rect, dst: *mut SDL_Surface,
    dstrect: *mut SDL_Rect,
  ) -> c_int;

  /// Set the YUV conversion mode
  pub fn SDL_SetYUVConversionMode(mode: SDL_YUV_CONVERSION_MODE);

  /// Get the YUV conversion mode
  pub fn SDL_GetYUVConversionMode() -> SDL_YUV_CONVERSION_MODE;

  /// Get the YUV conversion mode.
  ///
  /// Returns the correct mode for the resolution when the current conversion
  /// mode is `SDL_YUV_CONVERSION_AUTOMATIC`
  pub fn SDL_GetYUVConversionModeForResolution(
    width: c_int, height: c_int,
  ) -> SDL_YUV_CONVERSION_MODE;
}
