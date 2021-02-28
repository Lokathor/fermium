//! SDL's portable 2D rendering functions.
//!
//! This API supports the following features:
//! * single pixel points
//! * single pixel lines
//! * filled rectangles
//! * texture images
//!
//! The primitives may be drawn in opaque, blended, or additive modes.
//!
//! The texture images may be drawn in opaque, blended, or additive modes. They
//! can have an additional color tint or alpha modulation applied to them, and
//! may also be stretched with linear interpolation.
//!
//! This API is designed to accelerate simple 2D operations. You may want more
//! functionality such as polygons and particle effects and in that case you
//! should use SDL's OpenGL/Direct3D support or one of the many good 3D engines.
//!
//! These functions must be called from the main thread. See this bug for
//! details: <http://bugzilla.libsdl.org/show_bug.cgi?id=1995>

use crate::{
  blendmode::*, c_char, c_int, c_void, rect::*, stdinc::*, surface::*, video::*,
};

// makes rustdoc link properly!
#[allow(unused)]
use crate::pixels::*;

/// Flags used when creating a rendering context
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_RendererFlags(pub u32);
impl_bit_ops_for_tuple_newtype!(SDL_RendererFlags);
/// The renderer is a software fallback
pub const SDL_RENDERER_SOFTWARE: SDL_RendererFlags =
  SDL_RendererFlags(0x00000001);
/// The renderer uses hardware acceleration
pub const SDL_RENDERER_ACCELERATED: SDL_RendererFlags =
  SDL_RendererFlags(0x00000002);
/// Present is synchronized with the refresh rate
pub const SDL_RENDERER_PRESENTVSYNC: SDL_RendererFlags =
  SDL_RendererFlags(0x00000004);
/// The renderer supports rendering to texture
pub const SDL_RENDERER_TARGETTEXTURE: SDL_RendererFlags =
  SDL_RendererFlags(0x00000008);

/// Information on the capabilities of a render driver or context.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
#[allow(missing_docs)]
pub struct SDL_RendererInfo {
  /// The name of the renderer (borrowed)
  pub name: *const c_char,
  /// Supported [`SDL_RendererFlags`]
  pub flags: Uint32,
  /// The number of available texture formats
  pub num_texture_formats: Uint32,
  /// The available texture formats
  pub texture_formats: [Uint32; 16],
  /// The maximum texture width
  pub max_texture_width: c_int,
  /// The maximum texture height
  pub max_texture_height: c_int,
}
impl Default for SDL_RendererInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    unsafe { core::mem::zeroed() }
  }
}

/// The scaling mode for a texture.
///
/// See `SDL_ScaleMode*` constants.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_ScaleMode(pub i32);
/// nearest pixel sampling
pub const SDL_ScaleModeNearest: SDL_ScaleMode = SDL_ScaleMode(0);
/// linear filtering
pub const SDL_ScaleModeLinear: SDL_ScaleMode = SDL_ScaleMode(1);
/// anisotropic filtering
pub const SDL_ScaleModeBest: SDL_ScaleMode = SDL_ScaleMode(2);

/// The access pattern allowed for a texture.
///
/// See `SDL_TEXTUREACCESS_*` constants.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_TextureAccess(pub i32);
/// Changes rarely, not lockable.
pub const SDL_TEXTUREACCESS_STATIC: SDL_TextureAccess = SDL_TextureAccess(0);
/// Changes frequently, lockable.
pub const SDL_TEXTUREACCESS_STREAMING: SDL_TextureAccess = SDL_TextureAccess(1);
/// Texture can be used as a render target.
pub const SDL_TEXTUREACCESS_TARGET: SDL_TextureAccess = SDL_TextureAccess(2);

/// The texture channel modulation used in [`SDL_RenderCopy`]
///
/// See `SDL_TEXTUREMODULATE_*` constants.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_TextureModulate(pub i32);
/// No modulation.
pub const SDL_TEXTUREMODULATE_NONE: SDL_TextureModulate =
  SDL_TextureModulate(0);
/// `srcC = srcC * color`
pub const SDL_TEXTUREMODULATE_COLOR: SDL_TextureModulate =
  SDL_TextureModulate(1);
/// `srcA = srcA * alpha`
pub const SDL_TEXTUREMODULATE_ALPHA: SDL_TextureModulate =
  SDL_TextureModulate(2);

/// Flip values for [`SDL_RenderCopyEx`]
///
/// See `SDL_FLIP_*` constants.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_RendererFlip(pub u32);
impl_bit_ops_for_tuple_newtype!(SDL_RendererFlip);
/// Do not flip
pub const SDL_FLIP_NONE: SDL_RendererFlip = SDL_RendererFlip(0x00000000);
/// flip horizontally
pub const SDL_FLIP_HORIZONTAL: SDL_RendererFlip = SDL_RendererFlip(0x00000001);
/// flip vertically
pub const SDL_FLIP_VERTICAL: SDL_RendererFlip = SDL_RendererFlip(0x00000002);

/// An opaque structure representing rendering state.
#[derive(Debug)]
#[repr(transparent)]
pub struct SDL_Renderer(c_void);

/// An opaque, efficient, driver-specific representation of pixel data.
#[derive(Debug)]
#[repr(transparent)]
pub struct SDL_Texture(c_void);

extern "C" {
  /// Get the number of 2D rendering drivers available for the current display.
  ///
  /// A render driver is a set of code that handles rendering and texture
  /// management on a particular display. Normally there is only one, but
  /// some drivers may have several available with different capabilities.
  ///
  /// See Also: [`SDL_GetRenderDriverInfo`] and [`SDL_CreateRenderer`]
  pub fn SDL_GetNumRenderDrivers() -> c_int;

  /// Get information about a specific 2D rendering driver for the current
  /// display.
  ///
  /// * `index` The index of the driver to query information about.
  /// * `info`  A pointer to an SDL_RendererInfo struct to be filled with
  ///   information on the rendering driver.
  ///
  /// **Returns:** 0 on success, -1 if the index was out of range.
  ///
  /// See Also: [`SDL_CreateRenderer`]
  pub fn SDL_GetRenderDriverInfo(
    index: c_int, info: *mut SDL_RendererInfo,
  ) -> c_int;

  /// Create a window and default renderer
  ///
  /// * `width` The width of the window
  /// * `height` The height of the window
  /// * `window_flags` The flags used to create the window
  /// * `window` A pointer filled with the window, or NULL on error
  /// * `renderer` A pointer filled with the renderer, or NULL on error
  ///
  /// **Returns:** 0 on success, or -1 on error
  pub fn SDL_CreateWindowAndRenderer(
    width: c_int, height: c_int, window_flags: Uint32,
    window: *mut *mut SDL_Window, renderer: *mut *mut SDL_Renderer,
  ) -> c_int;

  /// Create a 2D rendering context for a window.
  ///
  /// * `window` The window where rendering is displayed.
  /// * `index` The index of the rendering driver to initialize, or -1 to
  ///   initialize the first one supporting the requested flags.
  /// * `flags` ::SDL_RendererFlags.
  ///
  /// **Returns:** A valid rendering context or NULL if there was an error.
  ///
  /// See Also: [`SDL_CreateSoftwareRenderer`], [`SDL_GetRendererInfo`],
  /// [`SDL_DestroyRenderer`]
  pub fn SDL_CreateRenderer(
    window: *mut SDL_Window, index: c_int, flags: Uint32,
  ) -> *mut SDL_Renderer;

  /// Create a 2D software rendering context for a surface.
  ///
  /// * `surface` The surface where rendering is done.
  ///
  /// **Returns:** A valid rendering context or NULL if there was an error.
  ///
  /// See Also: [`SDL_CreateRenderer`], [`SDL_DestroyRenderer`]
  pub fn SDL_CreateSoftwareRenderer(
    surface: *mut SDL_Surface,
  ) -> *mut SDL_Renderer;

  /// Get the renderer associated with a window.
  pub fn SDL_GetRenderer(window: *mut SDL_Window) -> *mut SDL_Renderer;

  /// Get information about a rendering context.
  pub fn SDL_GetRendererInfo(
    renderer: *mut SDL_Renderer, info: *mut SDL_RendererInfo,
  ) -> c_int;

  /// Get the output size in pixels of a rendering context.
  pub fn SDL_GetRendererOutputSize(
    renderer: *mut SDL_Renderer, w: *mut c_int, h: *mut c_int,
  ) -> c_int;

  /// Create a texture for a rendering context.
  ///
  /// * `renderer` The renderer.
  /// * `format` The format of the texture.
  /// * `access` One of the enumerated values in ::SDL_TextureAccess.
  /// * `w` The width of the texture in pixels.
  /// * `h` The height of the texture in pixels.
  ///
  /// **Returns:** The created texture is returned, or NULL if no rendering
  /// context was active,  the format was unsupported, or the width or height
  /// were out of range.
  ///
  /// The contents of the texture are not defined at creation.
  ///
  /// See Also: [`SDL_QueryTexture`], [`SDL_UpdateTexture`],
  /// [`SDL_DestroyTexture`]
  pub fn SDL_CreateTexture(
    renderer: *mut SDL_Renderer, format: Uint32, access: c_int, w: c_int,
    h: c_int,
  ) -> *mut SDL_Texture;

  /// Create a texture from an existing surface.
  ///
  /// * `renderer` The renderer.
  /// * `surface` The surface containing pixel data used to fill the texture.
  ///
  /// **Returns:** The created texture is returned, or NULL on error.
  ///
  /// The surface is not modified or freed by this function.
  ///
  /// See Also: [`SDL_QueryTexture`], [`SDL_DestroyTexture`]
  pub fn SDL_CreateTextureFromSurface(
    renderer: *mut SDL_Renderer, surface: *mut SDL_Surface,
  ) -> *mut SDL_Texture;

  /// Query the attributes of a texture
  ///
  /// * `texture` A texture to be queried.
  /// * `format` A pointer filled in with the raw format of the texture. The
  ///   actual format may differ, but pixel transfers will use this format.
  /// * `access` A pointer filled in with the actual access to the texture.
  /// * `w` A pointer filled in with the width of the texture in pixels.
  /// * `h` A pointer filled in with the height of the texture in pixels.
  ///
  /// **Returns:** 0 on success, or -1 if the texture is not valid.
  pub fn SDL_QueryTexture(
    texture: *mut SDL_Texture, format: *mut Uint32, access: *mut c_int,
    w: *mut c_int, h: *mut c_int,
  ) -> c_int;

  /// Set an additional color value used in render copy operations.
  ///
  /// * `texture` The texture to update.
  /// * `r` The red color value multiplied into copy operations.
  /// * `g` The green color value multiplied into copy operations.
  /// * `b` The blue color value multiplied into copy operations.
  ///
  /// **Returns:** 0 on success, or -1 if the texture is not valid or color
  /// modulation is not supported.
  ///
  /// See Also: [`SDL_GetTextureColorMod`]
  pub fn SDL_SetTextureColorMod(
    texture: *mut SDL_Texture, r: Uint8, g: Uint8, b: Uint8,
  ) -> c_int;

  ///  Get the additional color value used in render copy operations.
  ///
  /// * `texture` The texture to query.
  /// * `r` A pointer filled in with the current red color value.
  /// * `g` A pointer filled in with the current green color value.
  /// * `b` A pointer filled in with the current blue color value.
  ///
  /// **Returns:** 0 on success, or -1 if the texture is not valid.
  ///
  /// See Also: [`SDL_SetTextureColorMod`]
  pub fn SDL_GetTextureColorMod(
    texture: *mut SDL_Texture, r: *mut Uint8, g: *mut Uint8, b: *mut Uint8,
  ) -> c_int;

  /// Set an additional alpha value used in render copy operations.
  ///
  /// * `texture` The texture to update.
  /// * `alpha` The alpha value multiplied into copy operations.
  ///
  /// **Returns:** 0 on success, or -1 if the texture is not valid or alpha
  /// modulation is not supported.
  ///
  /// See Also: [`SDL_GetTextureAlphaMod`]
  pub fn SDL_SetTextureAlphaMod(
    texture: *mut SDL_Texture, alpha: Uint8,
  ) -> c_int;

  /// Get the additional alpha value used in render copy operations.
  ///
  /// * `texture` The texture to query.
  /// * `alpha` A pointer filled in with the current alpha value.
  ///
  /// **Returns:** 0 on success, or -1 if the texture is not valid.
  ///
  /// See Also: [`SDL_SetTextureAlphaMod`]
  pub fn SDL_GetTextureAlphaMod(
    texture: *mut SDL_Texture, alpha: *mut Uint8,
  ) -> c_int;

  /// Set the blend mode used for texture copy operations.
  ///
  /// * `texture` The texture to update.
  /// * `blendMode` [`SDL_BlendMode`] to use for texture blending.
  ///
  /// **Returns:** 0 on success, or -1 if the texture is not valid or the blend
  /// mode is not supported.
  ///
  /// If the blend mode is not supported, the closest supported mode is chosen.
  ///
  /// See Also: [`SDL_GetTextureBlendMode`]
  pub fn SDL_SetTextureBlendMode(
    texture: *mut SDL_Texture, blendMode: SDL_BlendMode,
  ) -> c_int;

  /// Get the blend mode used for texture copy operations.
  ///
  /// * `texture` The texture to query.
  /// * `blendMode` A pointer filled in with the current blend mode.
  ///
  /// **Returns:** 0 on success, or -1 if the texture is not valid.
  ///
  /// See Also: [`SDL_SetTextureBlendMode`]
  pub fn SDL_GetTextureBlendMode(
    texture: *mut SDL_Texture, blendMode: *mut SDL_BlendMode,
  ) -> c_int;

  /// Set the scale mode used for texture scale operations.
  ///
  /// * `texture` The texture to update.
  /// * `scaleMode` [`SDL_ScaleMode`] to use for texture scaling.
  ///
  /// **Returns:** 0 on success, or -1 if the texture is not valid.
  ///
  /// If the scale mode is not supported, the closest supported mode is chosen.
  ///
  /// See Also: [`SDL_GetTextureScaleMode`]
  pub fn SDL_SetTextureScaleMode(
    texture: *mut SDL_Texture, scaleMode: SDL_ScaleMode,
  ) -> c_int;

  /// Get the scale mode used for texture scale operations.
  ///
  /// * `texture` The texture to query.
  /// * `scaleMode` A pointer filled in with the current scale mode.
  ///
  /// **Returns:** 0 on success, or -1 if the texture is not valid.
  ///
  /// See Also: [`SDL_SetTextureScaleMode`]
  pub fn SDL_GetTextureScaleMode(
    texture: *mut SDL_Texture, scaleMode: *mut SDL_ScaleMode,
  ) -> c_int;

  /// Update the given texture rectangle with new pixel data.
  ///
  /// * `texture` The texture to update
  /// * `rect` A pointer to the rectangle of pixels to update, or NULL to update
  ///   the entire texture.
  /// * `pixels` The raw pixel data in the format of the texture.
  /// * `pitch` The number of bytes in a row of pixel data, including padding
  ///   between lines.
  ///
  /// The pixel data must be in the format of the texture. The pixel format can
  /// be queried with [`SDL_QueryTexture`].
  ///
  /// **Returns:** 0 on success, or -1 if the texture is not valid.
  ///
  /// This is a fairly slow function.
  pub fn SDL_UpdateTexture(
    texture: *mut SDL_Texture, rect: *const SDL_Rect, pixels: *const c_void,
    pitch: c_int,
  ) -> c_int;

  /// Update a rectangle within a planar YV12 or IYUV texture with new pixel
  /// data.
  ///
  /// * `texture` The texture to update
  /// * `rect` A pointer to the rectangle of pixels to update, or NULL to update
  ///   the entire texture.
  /// * `Yplane` The raw pixel data for the Y plane.
  /// * `Ypitch` The number of bytes between rows of pixel data for the Y plane.
  /// * `Uplane` The raw pixel data for the U plane.
  /// * `Upitch` The number of bytes between rows of pixel data for the U plane.
  /// * `Vplane` The raw pixel data for the V plane.
  /// * `Vpitch` The number of bytes between rows of pixel data for the V plane.
  ///
  /// **Returns:** 0 on success, or -1 if the texture is not valid.
  ///
  /// You can use [`SDL_UpdateTexture`] as long as your pixel data is
  /// a contiguous block of Y and U/V planes in the proper order, but
  /// this function is available if your pixel data is not contiguous.
  pub fn SDL_UpdateYUVTexture(
    texture: *mut SDL_Texture, rect: *const SDL_Rect, Yplane: *const Uint8,
    Ypitch: c_int, Uplane: *const Uint8, Upitch: c_int, Vplane: *const Uint8,
    Vpitch: c_int,
  ) -> c_int;

  /// Lock a portion of the texture for write-only pixel access.
  ///
  /// * `texture` The texture to lock for access, which was created with
  ///   [`SDL_TEXTUREACCESS_STREAMING`].
  /// * `rect` A pointer to the rectangle to lock for access. If the rect is
  ///   `NULL`, the entire texture will be locked.
  /// * `pixels` This is filled in with a pointer to the locked pixels,
  ///   appropriately offset by the locked area.
  /// * `pitch` This is filled in with the pitch of the locked pixels.
  ///
  /// **Returns:** 0 on success, or -1 if the texture is not valid or was not
  /// created with [`SDL_TEXTUREACCESS_STREAMING`].
  ///
  /// See Also: [`SDL_UnlockTexture`]
  pub fn SDL_LockTexture(
    texture: *mut SDL_Texture, rect: *const SDL_Rect, pixels: *mut *mut c_void,
    pitch: *mut c_int,
  ) -> c_int;

  /// Lock a portion of the texture for write-only pixel access. Expose it as a
  /// SDL surface.
  ///
  /// * `texture` The texture to lock for access, which was created with
  ///   [`SDL_TEXTUREACCESS_STREAMING`].
  /// * `rect` A pointer to the rectangle to lock for access. If the rect is
  ///   `NULL`, the entire texture will be locked.
  /// * `surface` This is filled in with a SDL surface representing the locked
  ///   area Surface is freed internally after calling [`SDL_UnlockTexture`] or
  ///   [`SDL_DestroyTexture`].
  ///
  /// **Returns:** 0 on success, or -1 if the texture is not valid or was not
  /// created with [`SDL_TEXTUREACCESS_STREAMING`].
  ///
  /// See Also: [`SDL_UnlockTexture`]
  pub fn SDL_LockTextureToSurface(
    texture: *mut SDL_Texture, rect: *const SDL_Rect,
    surface: *mut *mut SDL_Surface,
  ) -> c_int;

  /// Unlock a texture, uploading the changes to video memory, if needed.
  ///
  /// If [`SDL_LockTextureToSurface`] was called for locking, the SDL surface is
  /// freed.
  ///
  /// See Also: [`SDL_LockTexture`], [`SDL_LockTextureToSurface`]
  pub fn SDL_UnlockTexture(texture: *mut SDL_Texture);

  /// Determines whether a window supports the use of render targets
  ///
  /// * `renderer` The renderer that will be checked
  ///
  /// **Returns:** SDL_TRUE if supported, SDL_FALSE if not.
  pub fn SDL_RenderTargetSupported(renderer: *mut SDL_Renderer) -> SDL_bool;

  /// Set a texture as the current rendering target.
  ///
  /// * `renderer` The renderer.
  /// * `texture` The targeted texture, which must be created with the
  ///   SDL_TEXTUREACCESS_TARGET flag, or NULL for the default render target
  ///
  /// **Returns:** 0 on success, or -1 on error
  ///
  /// See Also: [`SDL_GetRenderTarget`]
  pub fn SDL_SetRenderTarget(
    renderer: *mut SDL_Renderer, texture: *mut SDL_Texture,
  ) -> c_int;

  /// Get the current render target or `NULL` for the default render target.
  ///
  /// **Returns:** The current render target
  ///
  /// See Also: [`SDL_SetRenderTarget`]
  pub fn SDL_GetRenderTarget(renderer: *mut SDL_Renderer) -> *mut SDL_Texture;

  /// Set device independent resolution for rendering
  ///
  /// * `renderer` The renderer for which resolution should be set.
  /// * `w` The width of the logical resolution
  /// * `h` The height of the logical resolution
  ///
  /// This function uses the viewport and scaling functionality to allow a fixed
  /// logical resolution for rendering, regardless of the actual output
  /// resolution. If the actual output resolution doesn't have the same aspect
  /// ratio the output rendering will be centered within the output display.
  ///
  /// If the output display is a window, mouse events in the window will be
  /// filtered and scaled so they seem to arrive within the logical
  /// resolution.
  ///
  /// If this function results in scaling or subpixel drawing by the
  /// rendering backend, it will be handled using the appropriate
  /// quality hints.
  ///
  /// See Also: [`SDL_RenderGetLogicalSize`], [`SDL_RenderSetScale`],
  /// [`SDL_RenderSetViewport`]
  pub fn SDL_RenderSetLogicalSize(
    renderer: *mut SDL_Renderer, w: c_int, h: c_int,
  ) -> c_int;

  /// Get device independent resolution for rendering
  ///
  /// * `renderer` The renderer from which resolution should be queried.
  /// * `w` A pointer filled with the width of the logical resolution
  /// * `h` A pointer filled with the height of the logical resolution
  ///
  /// See Also: [`SDL_RenderSetLogicalSize`]
  pub fn SDL_RenderGetLogicalSize(
    renderer: *mut SDL_Renderer, w: *mut c_int, h: *mut c_int,
  );

  /// Set whether to force integer scales for resolution-independent rendering
  ///
  /// * `renderer` The renderer for which integer scaling should be set.
  /// * `enable` Enable or disable integer scaling
  ///
  /// This function restricts the logical viewport to integer values - that is,
  /// when a resolution is between two multiples of a logical size, the
  /// viewport size is rounded down to the lower multiple.
  ///
  /// See Also: [`SDL_RenderSetLogicalSize`]
  pub fn SDL_RenderSetIntegerScale(
    renderer: *mut SDL_Renderer, enable: SDL_bool,
  ) -> c_int;

  /// Get whether integer scales are forced for resolution-independent rendering
  ///
  /// * `renderer` The renderer from which integer scaling should be queried.
  ///
  /// See Also: [`SDL_RenderSetIntegerScale`]
  pub fn SDL_RenderGetIntegerScale(renderer: *mut SDL_Renderer) -> SDL_bool;

  /// Set the drawing area for rendering on the current target.
  ///
  /// * `renderer` The renderer for which the drawing area should be set.
  /// * `rect` The rectangle representing the drawing area, or NULL to set the
  ///   viewport to the entire target.
  ///
  /// The x,y of the viewport rect represents the origin for rendering.
  ///
  /// **Returns:** 0 on success, or -1 on error
  ///
  /// If the window associated with the renderer is resized, the viewport is
  /// automatically reset.
  ///
  /// See Also: [`SDL_RenderGetViewport`], [`SDL_RenderSetLogicalSize`]
  pub fn SDL_RenderSetViewport(
    renderer: *mut SDL_Renderer, rect: *const SDL_Rect,
  ) -> c_int;

  /// Get the drawing area for the current target.
  ///
  /// See Also: [`SDL_RenderSetViewport`]
  pub fn SDL_RenderGetViewport(
    renderer: *mut SDL_Renderer, rect: *mut SDL_Rect,
  ) -> c_void;

  /// Set the clip rectangle for the current target.
  ///
  /// * `renderer` The renderer for which clip rectangle should be set.
  /// * `rect` A pointer to the rectangle to set as the clip rectangle, relative
  ///   to the viewport, or NULL to disable clipping.
  ///
  /// **Returns:** 0 on success, or -1 on error
  ///
  /// See Also: [`SDL_RenderGetClipRect`]
  pub fn SDL_RenderSetClipRect(
    renderer: *mut SDL_Renderer, rect: *const SDL_Rect,
  ) -> c_int;

  /// Get the clip rectangle for the current target.
  ///
  /// * `renderer` The renderer from which clip rectangle should be queried.
  /// * `rect` A pointer filled in with the current clip rectangle, or an empty
  ///   rectangle if clipping is disabled.
  ///
  /// See Also: [`SDL_RenderSetClipRect`]
  pub fn SDL_RenderGetClipRect(
    renderer: *mut SDL_Renderer, rect: *mut SDL_Rect,
  ) -> c_void;

  /// Get whether clipping is enabled on the given renderer.
  ///
  /// * `renderer` The renderer from which clip state should be queried.
  ///
  /// See Also: [`SDL_RenderGetClipRect`]
  pub fn SDL_RenderIsClipEnabled(renderer: *mut SDL_Renderer) -> SDL_bool;

  /// Set the drawing scale for rendering on the current target.
  ///
  /// * `renderer` The renderer for which the drawing scale should be set.
  /// * `scaleX` The horizontal scaling factor
  /// * `scaleY` The vertical scaling factor
  ///
  /// The drawing coordinates are scaled by the x/y scaling factors
  /// before they are used by the renderer.  This allows resolution
  /// independent drawing with a single coordinate system.
  ///
  /// If this results in scaling or subpixel drawing by the
  /// rendering backend, it will be handled using the appropriate
  /// quality hints.  For best results use integer scaling factors.
  ///
  /// See Also: [`SDL_RenderGetScale`], [`SDL_RenderSetLogicalSize`]
  pub fn SDL_RenderSetScale(
    renderer: *mut SDL_Renderer, scaleX: f32, scaleY: f32,
  ) -> c_int;

  /// Get the drawing scale for the current target.
  ///
  /// * `renderer` The renderer from which drawing scale should be queried.
  /// * `scaleX` A pointer filled in with the horizontal scaling factor
  /// * `scaleY` A pointer filled in with the vertical scaling factor
  ///
  /// See Also: [`SDL_RenderSetScale`]
  pub fn SDL_RenderGetScale(
    renderer: *mut SDL_Renderer, scaleX: *mut f32, scaleY: *mut f32,
  );

  /// Set the color used for drawing operations (Rect, Line and Clear).
  ///
  /// * `renderer` The renderer for which drawing color should be set.
  /// * `r` The red value used to draw on the rendering target.
  /// * `g` The green value used to draw on the rendering target.
  /// * `b` The blue value used to draw on the rendering target.
  /// * `a` The alpha value used to draw on the rendering target, usually
  ///   `SDL_ALPHA_OPAQUE` (255).
  ///
  /// **Returns:** 0 on success, or -1 on error
  pub fn SDL_SetRenderDrawColor(
    renderer: *mut SDL_Renderer, r: Uint8, g: Uint8, b: Uint8, a: Uint8,
  ) -> c_int;

  /// Get the color used for drawing operations (Rect, Line and Clear).
  ///
  /// * `renderer` The renderer from which drawing color should be queried.
  /// * `r` A pointer to the red value used to draw on the rendering target.
  /// * `g` A pointer to the green value used to draw on the rendering target.
  /// * `b` A pointer to the blue value used to draw on the rendering target.
  /// * `a` A pointer to the alpha value used to draw on the rendering target,
  ///   usually [`SDL_ALPHA_OPAQUE`] (255).
  ///
  /// **Returns:** 0 on success, or -1 on error
  pub fn SDL_GetRenderDrawColor(
    renderer: *mut SDL_Renderer, r: *mut Uint8, g: *mut Uint8, b: *mut Uint8,
    a: *mut Uint8,
  ) -> c_int;

  /// Set the blend mode used for drawing operations (Fill and Line).
  ///
  /// * `renderer` The renderer for which blend mode should be set.
  /// * `blendMode` [`SDL_BlendMode`] to use for blending.
  ///
  /// **Returns:** 0 on success, or -1 on error
  ///
  /// If the blend mode is not supported, the closest supported mode is
  /// chosen.
  ///
  /// See Also: [`SDL_GetRenderDrawBlendMode`]
  pub fn SDL_SetRenderDrawBlendMode(
    renderer: *mut SDL_Renderer, blendMode: SDL_BlendMode,
  ) -> c_int;

  /// Get the blend mode used for drawing operations.
  ///
  /// * `renderer` The renderer from which blend mode should be queried.
  /// * `blendMode` A pointer filled in with the current blend mode.
  ///
  /// **Returns:** 0 on success, or -1 on error
  ///
  /// See Also: [`SDL_SetRenderDrawBlendMode`]
  pub fn SDL_GetRenderDrawBlendMode(
    renderer: *mut SDL_Renderer, blendMode: *mut SDL_BlendMode,
  ) -> c_int;

  /// Clear the current rendering target with the drawing color
  ///
  /// This function clears the entire rendering target, ignoring the viewport
  /// and the clip rectangle.
  ///
  /// **Returns:** 0 on success, or -1 on error
  pub fn SDL_RenderClear(renderer: *mut SDL_Renderer) -> c_int;

  /// Draw a point on the current rendering target.
  ///
  /// * `renderer` The renderer which should draw a point.
  /// * `x` The x coordinate of the point.
  /// * `y` The y coordinate of the point.
  ///
  /// **Returns:** 0 on success, or -1 on error
  pub fn SDL_RenderDrawPoint(
    renderer: *mut SDL_Renderer, x: c_int, y: c_int,
  ) -> c_int;

  /// Draw multiple points on the current rendering target.
  ///
  /// * `renderer` The renderer which should draw multiple points.
  /// * `points` The points to draw
  /// * `count` The number of points to draw
  ///
  /// **Returns:** 0 on success, or -1 on error
  pub fn SDL_RenderDrawPoints(
    renderer: *mut SDL_Renderer, points: *const SDL_Point, count: c_int,
  ) -> c_int;

  /// Draw a line on the current rendering target.
  ///
  /// * `renderer` The renderer which should draw a line.
  /// * `x1` The x coordinate of the start point.
  /// * `y1` The y coordinate of the start point.
  /// * `x2` The x coordinate of the end point.
  /// * `y2` The y coordinate of the end point.
  ///
  /// **Returns:** 0 on success, or -1 on error
  pub fn SDL_RenderDrawLine(
    renderer: *mut SDL_Renderer, x1: c_int, y1: c_int, x2: c_int, y2: c_int,
  ) -> c_int;

  /// Draw a series of connected lines on the current rendering target.
  ///
  /// * `renderer` The renderer which should draw multiple lines.
  /// * `points` The points along the lines
  /// * `count` The number of points, drawing count-1 lines
  ///
  /// **Returns:** 0 on success, or -1 on error
  pub fn SDL_RenderDrawLines(
    renderer: *mut SDL_Renderer, points: *const SDL_Point, count: c_int,
  ) -> c_int;

  /// Draw a rectangle on the current rendering target.
  ///
  /// * `renderer` The renderer which should draw a rectangle.
  /// * `rect` A pointer to the destination rectangle, or NULL to outline the
  ///   entire rendering target.
  ///
  /// **Returns:** 0 on success, or -1 on error
  pub fn SDL_RenderDrawRect(
    renderer: *mut SDL_Renderer, rect: *const SDL_Rect,
  ) -> c_int;

  /// Draw some number of rectangles on the current rendering target.
  ///
  /// * `renderer` The renderer which should draw multiple rectangles.
  /// * `rects` A pointer to an array of destination rectangles.
  /// * `count` The number of rectangles.
  ///
  /// **Returns:** 0 on success, or -1 on error
  pub fn SDL_RenderDrawRects(
    renderer: *mut SDL_Renderer, rects: *const SDL_Rect, count: c_int,
  ) -> c_int;

  /// Fill a rectangle on the current rendering target with the drawing color.
  ///
  /// * `renderer` The renderer which should fill a rectangle.
  /// * `rect` A pointer to the destination rectangle, or NULL for the entire
  ///   rendering target.
  ///
  /// **Returns:** 0 on success, or -1 on error
  pub fn SDL_RenderFillRect(
    renderer: *mut SDL_Renderer, rect: *const SDL_Rect,
  ) -> c_int;

  /// Fill some number of rectangles on the current rendering target with the
  /// drawing color.
  ///
  /// * `renderer` The renderer which should fill multiple rectangles.
  /// * `rects` A pointer to an array of destination rectangles.
  /// * `count` The number of rectangles.
  ///
  /// **Returns:** 0 on success, or -1 on error
  pub fn SDL_RenderFillRects(
    renderer: *mut SDL_Renderer, rects: *const SDL_Rect, count: c_int,
  ) -> c_int;

  /// Copy a portion of the texture to the current rendering target.
  ///
  /// * `renderer` The renderer which should copy parts of a texture.
  /// * `texture` The source texture.
  /// * `srcrect` A pointer to the source rectangle, or `NULL` for the entire
  ///   texture.
  /// * `dstrect` A pointer to the destination rectangle, or `NULL` for the
  ///   entire rendering target.
  ///
  /// **Returns:** 0 on success, or -1 on error
  pub fn SDL_RenderCopy(
    renderer: *mut SDL_Renderer, texture: *mut SDL_Texture,
    srcrect: *const SDL_Rect, dstrect: *const SDL_Rect,
  ) -> c_int;

  /// Copy a portion of the source texture to the current rendering target,
  /// rotating it by angle around the given center
  ///
  /// * `renderer` The renderer which should copy parts of a texture.
  /// * `texture` The source texture.
  /// * `srcrect` A pointer to the source rectangle, or `NULL` for the entire
  ///   texture.
  /// * `dstrect` A pointer to the destination rectangle, or `NULL` for the
  ///   entire rendering target.
  /// * `angle` An angle in degrees that indicates the rotation that will be
  ///   applied to dstrect, rotating it in a clockwise direction
  /// * `center` A pointer to a point indicating the point around which dstrect
  ///   will be rotated (if `NULL`, rotation will be done around `dstrect.w/2`,
  ///   `dstrect.h/2`).
  /// * `flip` An [`SDL_RendererFlip`] value stating which flipping actions
  ///   should be performed on the texture
  ///
  /// **Returns:** 0 on success, or -1 on error
  pub fn SDL_RenderCopyEx(
    renderer: *mut SDL_Renderer, texture: *mut SDL_Texture,
    srcrect: *const SDL_Rect, dstrect: *const SDL_Rect, angle: f64,
    center: *const SDL_Point, flip: SDL_RendererFlip,
  ) -> c_int;

  /// Draw a point on the current rendering target.
  ///
  /// * `renderer` The renderer which should draw a point.
  /// * `x` The x coordinate of the point.
  /// * `y` The y coordinate of the point.
  ///
  /// **Returns:** 0 on success, or -1 on error
  pub fn SDL_RenderDrawPointF(
    renderer: *mut SDL_Renderer, x: f32, y: f32,
  ) -> c_int;

  /// Draw multiple points on the current rendering target.
  ///
  /// * `renderer` The renderer which should draw multiple points.
  /// * `points` The points to draw
  /// * `count` The number of points to draw
  ///
  /// **Returns:** 0 on success, or -1 on error
  pub fn SDL_RenderDrawPointsF(
    renderer: *mut SDL_Renderer, points: *const SDL_FPoint, count: c_int,
  ) -> c_int;

  /// Draw a line on the current rendering target.
  ///
  /// * `renderer` The renderer which should draw a line.
  /// * `x1` The x coordinate of the start point.
  /// * `y1` The y coordinate of the start point.
  /// * `x2` The x coordinate of the end point.
  /// * `y2` The y coordinate of the end point.
  ///
  /// **Returns:** 0 on success, or -1 on error
  pub fn SDL_RenderDrawLineF(
    renderer: *mut SDL_Renderer, x1: f32, y1: f32, x2: f32, y2: f32,
  ) -> c_int;

  /// Draw a series of connected lines on the current rendering target.
  ///
  /// * `renderer` The renderer which should draw multiple lines.
  /// * `points` The points along the lines
  /// * `count` The number of points, drawing count-1 lines
  ///
  /// **Returns:** 0 on success, or -1 on error
  pub fn SDL_RenderDrawLinesF(
    renderer: *mut SDL_Renderer, points: *const SDL_FPoint, count: c_int,
  ) -> c_int;

  /// Draw a rectangle on the current rendering target.
  ///
  /// * `renderer` The renderer which should draw a rectangle.
  /// * `rect` A pointer to the destination rectangle, or `NULL` to outline the
  ///   entire rendering target.
  ///
  /// **Returns:** 0 on success, or -1 on error
  pub fn SDL_RenderDrawRectF(
    renderer: *mut SDL_Renderer, rect: *const SDL_FRect,
  ) -> c_int;

  /// Draw some number of rectangles on the current rendering target.
  ///
  /// * `renderer` The renderer which should draw multiple rectangles.
  /// * `rects` A pointer to an array of destination rectangles.
  /// * `count` The number of rectangles.
  ///
  /// **Returns:** 0 on success, or -1 on error
  pub fn SDL_RenderDrawRectsF(
    renderer: *mut SDL_Renderer, rects: *const SDL_FRect, count: c_int,
  ) -> c_int;

  /// Fill a rectangle on the current rendering target with the drawing color.
  ///
  /// * `renderer` The renderer which should fill a rectangle.
  /// * `rect` A pointer to the destination rectangle, or NULL for the entire
  ///   rendering target.
  ///
  /// **Returns:** 0 on success, or -1 on error
  pub fn SDL_RenderFillRectF(
    renderer: *mut SDL_Renderer, rect: *const SDL_FRect,
  ) -> c_int;

  /// Fill some number of rectangles on the current rendering target with the
  /// drawing color.
  ///
  /// * `renderer` The renderer which should fill multiple rectangles.
  /// * `rects` A pointer to an array of destination rectangles.
  /// * `count` The number of rectangles.
  ///
  /// **Returns:** 0 on success, or -1 on error
  pub fn SDL_RenderFillRectsF(
    renderer: *mut SDL_Renderer, rects: *const SDL_FRect, count: c_int,
  ) -> c_int;

  /// Copy a portion of the texture to the current rendering target.
  ///
  /// * `renderer` The renderer which should copy parts of a texture.
  /// * `texture` The source texture.
  /// * `srcrect` A pointer to the source rectangle, or NULL for the entire
  ///   texture.
  /// * `dstrect` A pointer to the destination rectangle, or NULL for the entire
  ///   rendering target.
  ///
  /// **Returns:** 0 on success, or -1 on error
  pub fn SDL_RenderCopyF(
    renderer: *mut SDL_Renderer, texture: *mut SDL_Texture,
    srcrect: *const SDL_Rect, dstrect: *const SDL_FRect,
  ) -> c_int;

  /// Copy a portion of the source texture to the current rendering target,
  /// rotating it by angle around the given center
  ///
  /// * `renderer` The renderer which should copy parts of a texture.
  /// * `texture` The source texture.
  /// * `srcrect` A pointer to the source rectangle, or NULL for the entire
  ///   texture.
  /// * `dstrect` A pointer to the destination rectangle, or NULL for the entire
  ///   rendering target.
  /// * `angle` An angle in degrees that indicates the rotation that will be
  ///   applied to dstrect, rotating it in a clockwise direction
  /// * `center` A pointer to a point indicating the point around which dstrect
  ///   will be rotated (if NULL, rotation will be done around dstrect.w/2,
  ///   dstrect.h/2).
  /// * `flip` An SDL_RendererFlip value stating which flipping actions should
  ///   be performed on the texture
  ///
  /// **Returns:** 0 on success, or -1 on error
  pub fn SDL_RenderCopyExF(
    renderer: *mut SDL_Renderer, texture: *mut SDL_Texture,
    srcrect: *const SDL_Rect, dstrect: *const SDL_FRect, angle: f64,
    center: *const SDL_FPoint, flip: SDL_RendererFlip,
  ) -> c_int;

  /// Read pixels from the current rendering target.
  ///
  /// * `renderer` The renderer from which pixels should be read.
  /// * `rect` A pointer to the rectangle to read, or NULL for the entire render
  ///   target.
  /// * `format` The desired format of the pixel data, or 0 to use the format of
  ///   the rendering target
  /// * `pixels` A pointer to be filled in with the pixel data
  /// * `pitch` The pitch of the pixels parameter.
  ///
  /// **Returns:** 0 on success, or -1 if pixel reading is not supported.
  ///
  /// **Warning:** This is a very slow operation, and should not be used
  /// frequently.
  pub fn SDL_RenderReadPixels(
    renderer: *mut SDL_Renderer, rect: *const SDL_Rect, format: Uint32,
    pixels: *mut c_void, pitch: c_int,
  ) -> c_int;

  /// Update the screen with rendering performed.
  pub fn SDL_RenderPresent(renderer: *mut SDL_Renderer);

  /// Destroy the specified texture.
  ///
  /// See Also: [`SDL_CreateTexture`], [`SDL_CreateTextureFromSurface`]
  pub fn SDL_DestroyTexture(texture: *mut SDL_Texture);

  /// Destroy the rendering context for a window and free associated textures.
  ///
  /// See Also: [`SDL_CreateRenderer`]
  pub fn SDL_DestroyRenderer(renderer: *mut SDL_Renderer);

  /// Force the rendering context to flush any pending commands to the
  /// underlying rendering API.
  ///
  /// You do not need to (and in fact, shouldn't) call this function unless
  /// you are planning to call into OpenGL/Direct3D/Metal/whatever directly
  /// in addition to using an SDL_Renderer.
  ///
  /// This is for a very-specific case: if you are using SDL's render API,
  /// you asked for a specific renderer backend (OpenGL, Direct3D, etc),
  /// you set SDL_HINT_RENDER_BATCHING to "1", and you plan to make
  /// OpenGL/D3D/whatever calls in addition to SDL render API calls. If all of
  /// this applies, you should call SDL_RenderFlush() between calls to SDL's
  /// render API and the low-level API you're using in cooperation.
  ///
  /// In all other cases, you can ignore this function. This is only here to
  /// get maximum performance out of a specific situation. In all other cases,
  /// SDL will do the right thing, perhaps at a performance loss.
  ///
  /// This function is first available in SDL 2.0.10, and is not needed in
  /// 2.0.9 and earlier, as earlier versions did not queue rendering commands
  /// at all, instead flushing them to the OS immediately.
  pub fn SDL_RenderFlush(renderer: *mut SDL_Renderer) -> c_int;

  /// Bind the texture to the current OpenGL/ES/ES2 context for use with OpenGL
  /// instructions.
  ///
  /// * `texture` The SDL texture to bind.
  /// * `texw` A pointer to a float that will be filled with the texture width.
  /// * `texh` A pointer to a float that will be filled with the texture height.
  ///
  /// **Returns:** 0 on success, or -1 if the operation is not supported
  pub fn SDL_GL_BindTexture(
    texture: *mut SDL_Texture, texw: *mut f32, texh: *mut f32,
  ) -> c_int;

  /// Unbind a texture from the current OpenGL/ES/ES2 context.
  ///
  /// * `texture` The SDL texture to unbind
  ///
  /// **Returns:** 0 on success, or -1 if the operation is not supported
  pub fn SDL_GL_UnbindTexture(texture: *mut SDL_Texture) -> c_int;

  /// Get the `CAMetalLayer` associated with the given Metal renderer
  ///
  /// * `renderer` The renderer to query
  ///
  /// **Returns:** `CAMetalLayer*` on success, or `NULL` if the renderer isn't a
  /// Metal renderer
  ///
  /// See Also: [`SDL_RenderGetMetalCommandEncoder`]
  pub fn SDL_RenderGetMetalLayer(renderer: *mut SDL_Renderer) -> *mut c_void;

  /// Get the Metal command encoder for the current frame
  ///
  /// * `renderer` The renderer to query
  ///
  /// **Returns:** `id<MTLRenderCommandEncoder>` on success, or `NULL` if the
  /// renderer isn't a Metal renderer
  ///
  /// See Also: [`SDL_RenderGetMetalLayer`]
  pub fn SDL_RenderGetMetalCommandEncoder(
    renderer: *mut SDL_Renderer,
  ) -> *mut c_void;
}
