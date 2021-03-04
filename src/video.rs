//! Module for interacting with the video subsystem.

use crate::{c_char, c_int, c_void, rect::*, stdinc::*, surface::*};

// makes rustdoc link properly!
#[allow(unused)]
use crate::error::*;
#[allow(unused)]
use crate::vulkan::*;

/// The structure that defines a display mode
#[derive(Debug)]
#[repr(C)]
pub struct SDL_DisplayMode {
  /// pixel format
  pub format: Uint32,
  /// width, in screen coordinates
  pub w: c_int,
  /// height, in screen coordinates
  pub h: c_int,
  /// refresh rate (or zero for unspecified)
  pub refresh_rate: c_int,
  /// driver-specific data, initialize to 0
  pub driverdata: *mut c_void,
}

/// The type used to identify a window (newtype'd `c_void`).
#[repr(transparent)]
pub struct SDL_Window(c_void);

/// The flags on a window
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_WindowFlags(pub u32);
impl_bit_ops_for_tuple_newtype!(SDL_WindowFlags);
#[allow(missing_docs)]
pub const SDL_WINDOW_FULLSCREEN: SDL_WindowFlags = SDL_WindowFlags(0x00000001);
#[allow(missing_docs)]
pub const SDL_WINDOW_OPENGL: SDL_WindowFlags = SDL_WindowFlags(0x00000002);
#[allow(missing_docs)]
pub const SDL_WINDOW_SHOWN: SDL_WindowFlags = SDL_WindowFlags(0x00000004);
#[allow(missing_docs)]
pub const SDL_WINDOW_HIDDEN: SDL_WindowFlags = SDL_WindowFlags(0x00000008);
#[allow(missing_docs)]
pub const SDL_WINDOW_BORDERLESS: SDL_WindowFlags = SDL_WindowFlags(0x00000010);
#[allow(missing_docs)]
pub const SDL_WINDOW_RESIZABLE: SDL_WindowFlags = SDL_WindowFlags(0x00000020);
#[allow(missing_docs)]
pub const SDL_WINDOW_MINIMIZED: SDL_WindowFlags = SDL_WindowFlags(0x00000040);
#[allow(missing_docs)]
pub const SDL_WINDOW_MAXIMIZED: SDL_WindowFlags = SDL_WindowFlags(0x00000080);
#[allow(missing_docs)]
pub const SDL_WINDOW_INPUT_GRABBED: SDL_WindowFlags =
  SDL_WindowFlags(0x00000100);
#[allow(missing_docs)]
pub const SDL_WINDOW_INPUT_FOCUS: SDL_WindowFlags = SDL_WindowFlags(0x00000200);
#[allow(missing_docs)]
pub const SDL_WINDOW_MOUSE_FOCUS: SDL_WindowFlags = SDL_WindowFlags(0x00000400);
#[allow(missing_docs)]
pub const SDL_WINDOW_FULLSCREEN_DESKTOP: SDL_WindowFlags =
  SDL_WindowFlags(SDL_WINDOW_FULLSCREEN.0 | 0x00001000);
#[allow(missing_docs)]
pub const SDL_WINDOW_FOREIGN: SDL_WindowFlags = SDL_WindowFlags(0x00000800);
#[allow(missing_docs)]
pub const SDL_WINDOW_ALLOW_HIGHDPI: SDL_WindowFlags =
  SDL_WindowFlags(0x00002000);
#[allow(missing_docs)]
pub const SDL_WINDOW_MOUSE_CAPTURE: SDL_WindowFlags =
  SDL_WindowFlags(0x00004000);
#[allow(missing_docs)]
pub const SDL_WINDOW_ALWAYS_ON_TOP: SDL_WindowFlags =
  SDL_WindowFlags(0x00008000);
#[allow(missing_docs)]
pub const SDL_WINDOW_SKIP_TASKBAR: SDL_WindowFlags =
  SDL_WindowFlags(0x00010000);
#[allow(missing_docs)]
pub const SDL_WINDOW_UTILITY: SDL_WindowFlags = SDL_WindowFlags(0x00020000);
#[allow(missing_docs)]
pub const SDL_WINDOW_TOOLTIP: SDL_WindowFlags = SDL_WindowFlags(0x00040000);
#[allow(missing_docs)]
pub const SDL_WINDOW_POPUP_MENU: SDL_WindowFlags = SDL_WindowFlags(0x00080000);
#[allow(missing_docs)]
pub const SDL_WINDOW_VULKAN: SDL_WindowFlags = SDL_WindowFlags(0x10000000);
#[allow(missing_docs)]
pub const SDL_WINDOW_METAL: SDL_WindowFlags = SDL_WindowFlags(0x20000000);

/// Specifies that you don't care about the window position.
pub const SDL_WINDOWPOS_UNDEFINED: i32 = 0x1FFF0000;

/// Specifies that you want the window centered.
pub const SDL_WINDOWPOS_CENTERED: i32 = 0x2FFF0000;

/// Event subtype for window events.
///
/// Technically a `u32`, altered to be `u8` to better fit with the usage within
/// the API.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_WindowEventID(pub u8);
#[allow(missing_docs)]
pub const SDL_WINDOWEVENT_NONE: SDL_WindowEventID = SDL_WindowEventID(0);
#[allow(missing_docs)]
pub const SDL_WINDOWEVENT_SHOWN: SDL_WindowEventID = SDL_WindowEventID(1);
#[allow(missing_docs)]
pub const SDL_WINDOWEVENT_HIDDEN: SDL_WindowEventID = SDL_WindowEventID(2);
#[allow(missing_docs)]
pub const SDL_WINDOWEVENT_EXPOSED: SDL_WindowEventID = SDL_WindowEventID(3);
#[allow(missing_docs)]
pub const SDL_WINDOWEVENT_MOVED: SDL_WindowEventID = SDL_WindowEventID(4);
#[allow(missing_docs)]
pub const SDL_WINDOWEVENT_RESIZED: SDL_WindowEventID = SDL_WindowEventID(5);
#[allow(missing_docs)]
pub const SDL_WINDOWEVENT_SIZE_CHANGED: SDL_WindowEventID =
  SDL_WindowEventID(6);
#[allow(missing_docs)]
pub const SDL_WINDOWEVENT_MINIMIZED: SDL_WindowEventID = SDL_WindowEventID(7);
#[allow(missing_docs)]
pub const SDL_WINDOWEVENT_MAXIMIZED: SDL_WindowEventID = SDL_WindowEventID(8);
#[allow(missing_docs)]
pub const SDL_WINDOWEVENT_RESTORED: SDL_WindowEventID = SDL_WindowEventID(9);
#[allow(missing_docs)]
pub const SDL_WINDOWEVENT_ENTER: SDL_WindowEventID = SDL_WindowEventID(10);
#[allow(missing_docs)]
pub const SDL_WINDOWEVENT_LEAVE: SDL_WindowEventID = SDL_WindowEventID(11);
#[allow(missing_docs)]
pub const SDL_WINDOWEVENT_FOCUS_GAINED: SDL_WindowEventID =
  SDL_WindowEventID(12);
#[allow(missing_docs)]
pub const SDL_WINDOWEVENT_FOCUS_LOST: SDL_WindowEventID = SDL_WindowEventID(13);
#[allow(missing_docs)]
pub const SDL_WINDOWEVENT_CLOSE: SDL_WindowEventID = SDL_WindowEventID(14);
#[allow(missing_docs)]
pub const SDL_WINDOWEVENT_TAKE_FOCUS: SDL_WindowEventID = SDL_WindowEventID(15);
#[allow(missing_docs)]
pub const SDL_WINDOWEVENT_HIT_TEST: SDL_WindowEventID = SDL_WindowEventID(16);

/// Event subtype for display events.
///
/// Technically a `u32`, altered to be `u8` to better fit with the usage within
/// the API.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_DisplayEventID(pub u8);
/// Never used
pub const SDL_DISPLAYEVENT_NONE: SDL_DisplayEventID = SDL_DisplayEventID(0);
/// Display orientation has changed to `data1`
pub const SDL_DISPLAYEVENT_ORIENTATION: SDL_DisplayEventID =
  SDL_DisplayEventID(1);
/// Display has been added to the system
pub const SDL_DISPLAYEVENT_CONNECTED: SDL_DisplayEventID =
  SDL_DisplayEventID(2);
/// Display has been removed from the system
pub const SDL_DISPLAYEVENT_DISCONNECTED: SDL_DisplayEventID =
  SDL_DisplayEventID(3);

/// Orientations a display can have.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_DisplayOrientation(pub u32);
#[allow(missing_docs)]
pub const SDL_ORIENTATION_UNKNOWN: SDL_DisplayOrientation =
  SDL_DisplayOrientation(0);
#[allow(missing_docs)]
pub const SDL_ORIENTATION_LANDSCAPE: SDL_DisplayOrientation =
  SDL_DisplayOrientation(1);
#[allow(missing_docs)]
pub const SDL_ORIENTATION_LANDSCAPE_FLIPPED: SDL_DisplayOrientation =
  SDL_DisplayOrientation(2);
#[allow(missing_docs)]
pub const SDL_ORIENTATION_PORTRAIT: SDL_DisplayOrientation =
  SDL_DisplayOrientation(3);
#[allow(missing_docs)]
pub const SDL_ORIENTATION_PORTRAIT_FLIPPED: SDL_DisplayOrientation =
  SDL_DisplayOrientation(4);

/// A handle to an OpenGL context.
#[derive(Debug, Clone, Copy)]
#[allow(unused)]
#[repr(transparent)]
pub struct SDL_GLContext(pub *mut c_void);
impl SDL_GLContext {
  /// Checks if the context pointer is null.
  pub fn is_null(self) -> bool {
    self.0.is_null()
  }
}

/// OpenGL configuration attributes
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_GLattr(pub u32);
#[allow(missing_docs)]
pub const SDL_GL_RED_SIZE: SDL_GLattr = SDL_GLattr(0);
#[allow(missing_docs)]
pub const SDL_GL_GREEN_SIZE: SDL_GLattr = SDL_GLattr(1);
#[allow(missing_docs)]
pub const SDL_GL_BLUE_SIZE: SDL_GLattr = SDL_GLattr(2);
#[allow(missing_docs)]
pub const SDL_GL_ALPHA_SIZE: SDL_GLattr = SDL_GLattr(3);
#[allow(missing_docs)]
pub const SDL_GL_BUFFER_SIZE: SDL_GLattr = SDL_GLattr(4);
#[allow(missing_docs)]
pub const SDL_GL_DOUBLEBUFFER: SDL_GLattr = SDL_GLattr(5);
#[allow(missing_docs)]
pub const SDL_GL_DEPTH_SIZE: SDL_GLattr = SDL_GLattr(6);
#[allow(missing_docs)]
pub const SDL_GL_STENCIL_SIZE: SDL_GLattr = SDL_GLattr(7);
#[allow(missing_docs)]
pub const SDL_GL_ACCUM_RED_SIZE: SDL_GLattr = SDL_GLattr(8);
#[allow(missing_docs)]
pub const SDL_GL_ACCUM_GREEN_SIZE: SDL_GLattr = SDL_GLattr(9);
#[allow(missing_docs)]
pub const SDL_GL_ACCUM_BLUE_SIZE: SDL_GLattr = SDL_GLattr(10);
#[allow(missing_docs)]
pub const SDL_GL_ACCUM_ALPHA_SIZE: SDL_GLattr = SDL_GLattr(11);
#[allow(missing_docs)]
pub const SDL_GL_STEREO: SDL_GLattr = SDL_GLattr(12);
#[allow(missing_docs)]
pub const SDL_GL_MULTISAMPLEBUFFERS: SDL_GLattr = SDL_GLattr(13);
#[allow(missing_docs)]
pub const SDL_GL_MULTISAMPLESAMPLES: SDL_GLattr = SDL_GLattr(14);
#[allow(missing_docs)]
pub const SDL_GL_ACCELERATED_VISUAL: SDL_GLattr = SDL_GLattr(15);
#[allow(missing_docs)]
pub const SDL_GL_RETAINED_BACKING: SDL_GLattr = SDL_GLattr(16);
#[allow(missing_docs)]
pub const SDL_GL_CONTEXT_MAJOR_VERSION: SDL_GLattr = SDL_GLattr(17);
#[allow(missing_docs)]
pub const SDL_GL_CONTEXT_MINOR_VERSION: SDL_GLattr = SDL_GLattr(18);
#[allow(missing_docs)]
pub const SDL_GL_CONTEXT_EGL: SDL_GLattr = SDL_GLattr(19);
#[allow(missing_docs)]
pub const SDL_GL_CONTEXT_FLAGS: SDL_GLattr = SDL_GLattr(20);
#[allow(missing_docs)]
pub const SDL_GL_CONTEXT_PROFILE_MASK: SDL_GLattr = SDL_GLattr(21);
#[allow(missing_docs)]
pub const SDL_GL_SHARE_WITH_CURRENT_CONTEXT: SDL_GLattr = SDL_GLattr(22);
#[allow(missing_docs)]
pub const SDL_GL_FRAMEBUFFER_SRGB_CAPABLE: SDL_GLattr = SDL_GLattr(23);
#[allow(missing_docs)]
pub const SDL_GL_CONTEXT_RELEASE_BEHAVIOR: SDL_GLattr = SDL_GLattr(24);
#[allow(missing_docs)]
pub const SDL_GL_CONTEXT_RESET_NOTIFICATION: SDL_GLattr = SDL_GLattr(25);
#[allow(missing_docs)]
pub const SDL_GL_CONTEXT_NO_ERROR: SDL_GLattr = SDL_GLattr(26);

/// The GL Profile: Core, Compatibility, or ES.
///
/// See the `SDL_GL_CONTEXT_PROFILE_*` constants.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_GLprofile(pub u32);
#[allow(missing_docs)]
pub const SDL_GL_CONTEXT_PROFILE_CORE: SDL_GLprofile = SDL_GLprofile(0x0001);
#[allow(missing_docs)]
pub const SDL_GL_CONTEXT_PROFILE_COMPATIBILITY: SDL_GLprofile =
  SDL_GLprofile(0x0002);
#[allow(missing_docs)]
pub const SDL_GL_CONTEXT_PROFILE_ES: SDL_GLprofile = SDL_GLprofile(0x0004);

/// SDL Context Flags.
///
/// See the `SDL_GL_CONTEXT_*` constants.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_GLcontextFlag(pub u32);
impl_bit_ops_for_tuple_newtype!(SDL_GLcontextFlag);
#[allow(missing_docs)]
pub const SDL_GL_CONTEXT_DEBUG_FLAG: SDL_GLcontextFlag =
  SDL_GLcontextFlag(0x0001);
#[allow(missing_docs)]
pub const SDL_GL_CONTEXT_FORWARD_COMPATIBLE_FLAG: SDL_GLcontextFlag =
  SDL_GLcontextFlag(0x0002);
#[allow(missing_docs)]
pub const SDL_GL_CONTEXT_ROBUST_ACCESS_FLAG: SDL_GLcontextFlag =
  SDL_GLcontextFlag(0x0004);
#[allow(missing_docs)]
pub const SDL_GL_CONTEXT_RESET_ISOLATION_FLAG: SDL_GLcontextFlag =
  SDL_GLcontextFlag(0x0008);

/// Affects GL's behavior when you release the context.
///
/// See `SDL_GL_CONTEXT_RELEASE_BEHAVIOR_*`
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_GLcontextReleaseFlag(pub u32);
#[allow(missing_docs)]
pub const SDL_GL_CONTEXT_RELEASE_BEHAVIOR_NONE: SDL_GLcontextReleaseFlag =
  SDL_GLcontextReleaseFlag(0x0000);
#[allow(missing_docs)]
pub const SDL_GL_CONTEXT_RELEASE_BEHAVIOR_FLUSH: SDL_GLcontextReleaseFlag =
  SDL_GLcontextReleaseFlag(0x0001);

/// Affects GL's behavior when the context is reset.
///
/// See `SDL_GL_CONTEXT_RESET_*`
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_GLContextResetNotification(pub u32);
#[allow(missing_docs)]
pub const SDL_GL_CONTEXT_RESET_NO_NOTIFICATION: SDL_GLContextResetNotification =
  SDL_GLContextResetNotification(0x0000);
#[allow(missing_docs)]
pub const SDL_GL_CONTEXT_RESET_LOSE_CONTEXT: SDL_GLContextResetNotification =
  SDL_GLContextResetNotification(0x0001);

/// The results of a hit test.
///
/// See `SDL_HITTEST_*`
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_HitTestResult(pub u32);
#[allow(missing_docs)]
pub const SDL_HITTEST_NORMAL: SDL_HitTestResult = SDL_HitTestResult(0);
#[allow(missing_docs)]
pub const SDL_HITTEST_DRAGGABLE: SDL_HitTestResult = SDL_HitTestResult(1);
#[allow(missing_docs)]
pub const SDL_HITTEST_RESIZE_TOPLEFT: SDL_HitTestResult = SDL_HitTestResult(2);
#[allow(missing_docs)]
pub const SDL_HITTEST_RESIZE_TOP: SDL_HitTestResult = SDL_HitTestResult(3);
#[allow(missing_docs)]
pub const SDL_HITTEST_RESIZE_TOPRIGHT: SDL_HitTestResult = SDL_HitTestResult(4);
#[allow(missing_docs)]
pub const SDL_HITTEST_RESIZE_RIGHT: SDL_HitTestResult = SDL_HitTestResult(5);
#[allow(missing_docs)]
pub const SDL_HITTEST_RESIZE_BOTTOMRIGHT: SDL_HitTestResult =
  SDL_HitTestResult(6);
#[allow(missing_docs)]
pub const SDL_HITTEST_RESIZE_BOTTOM: SDL_HitTestResult = SDL_HitTestResult(7);
#[allow(missing_docs)]
pub const SDL_HITTEST_RESIZE_BOTTOMLEFT: SDL_HitTestResult =
  SDL_HitTestResult(8);
#[allow(missing_docs)]
pub const SDL_HITTEST_RESIZE_LEFT: SDL_HitTestResult = SDL_HitTestResult(9);

/// SDL's "hit test" function pointer type.
pub type SDL_HitTest = Option<
  unsafe extern "C" fn(
    win: *mut SDL_Window,
    area: *const SDL_Point,
    data: *mut c_void,
  ) -> SDL_HitTestResult,
>;

extern "C" {
  /// The number of available video drivers.
  ///
  /// **Returns:** >=1 on success, or negative on error (call `SDL_GetError`)
  pub fn SDL_GetNumVideoDrivers() -> c_int;

  /// Lookup the name of a video driver.
  ///
  /// Do not free this pointer.
  pub fn SDL_GetVideoDriver(index: c_int) -> *const c_char;

  /// Initialize the video subsystem (and other subsystems it depends on)
  pub fn SDL_VideoInit(driver_name: *const c_char) -> c_int;

  /// Quit the video subsystem (and also other event subsystems)
  pub fn SDL_VideoQuit();

  /// The name of the current video driver.
  ///
  /// Do not free this pointer.
  pub fn SDL_GetCurrentVideoDriver() -> *const c_char;

  /// The number of displays available.
  pub fn SDL_GetNumVideoDisplays() -> c_int;

  /// Gets the name of a display.
  ///
  /// Do not free this pointer.
  pub fn SDL_GetDisplayName(displayIndex: c_int) -> *const c_char;

  /// Writes the bounds of a given display to the rect given.
  ///
  /// The primary display is at (0,0).
  ///
  /// **Returns:** 0 on success, negative on failure. Call [`SDL_GetErrorMsg`]
  /// for more.
  pub fn SDL_GetDisplayBounds(
    displayIndex: c_int, rect: *mut SDL_Rect,
  ) -> c_int;

  /// As [`SDL_GetDisplayBounds`], but system reserved area (eg: the taskbar) is
  /// removed for you.
  ///
  /// Generally, you should use this for non-fullscreen programs.
  pub fn SDL_GetDisplayUsableBounds(
    displayIndex: c_int, rect: *mut SDL_Rect,
  ) -> c_int;

  /// Get the diagonal, horizontal, and vertical DPI of a display.
  ///
  /// Any pointer may safely be null if you don't want that value.
  ///
  /// **Returns:** 0 on success, negative on failure. Call [`SDL_GetErrorMsg`]
  /// for more.
  pub fn SDL_GetDisplayDPI(
    displayIndex: c_int, ddpi: *mut f32, hdpi: *mut f32, vdpi: *mut f32,
  ) -> c_int;

  /// Gets the orientation of a display.
  pub fn SDL_GetDisplayOrientation(
    displayIndex: c_int,
  ) -> SDL_DisplayOrientation;

  /// Gets the number of available display modes.
  ///
  /// **Returns:** >=1 on success, or negative on error (call `SDL_GetError`)
  pub fn SDL_GetNumDisplayModes(displayIndex: c_int) -> c_int;

  /// Gets info about a display mode.
  ///
  /// **Returns:** 0 on success, negative on failure. Call [`SDL_GetErrorMsg`]
  /// for more.
  pub fn SDL_GetDisplayMode(
    displayIndex: c_int, modeIndex: c_int, mode: *mut SDL_DisplayMode,
  ) -> c_int;

  /// Gets info about the desktop's display mode.
  ///
  /// **Returns:** 0 on success, negative on failure. Call [`SDL_GetErrorMsg`]
  /// for more.
  pub fn SDL_GetDesktopDisplayMode(
    displayIndex: c_int, mode: *mut SDL_DisplayMode,
  ) -> c_int;

  /// Gets info about the current display mode.
  ///
  /// **Returns:** 0 on success, negative on failure. Call [`SDL_GetErrorMsg`]
  /// for more.
  pub fn SDL_GetCurrentDisplayMode(
    displayIndex: c_int, mode: *mut SDL_DisplayMode,
  ) -> c_int;

  /// Gets the closest display mode to the `mode` requested.
  ///
  /// **Returns:** Either the `closest` pointer, or null on failure. Call
  /// [`SDL_GetErrorMsg`] for more.
  pub fn SDL_GetClosestDisplayMode(
    displayIndex: c_int, mode: *const SDL_DisplayMode,
    closest: *mut SDL_DisplayMode,
  ) -> *mut SDL_DisplayMode;

  /// Gets the display index for the center of a given window.
  ///
  /// **Returns:** non-negative, or negative on failure. Call
  /// [`SDL_GetErrorMsg`] for more.
  pub fn SDL_GetWindowDisplayIndex(window: *mut SDL_Window) -> c_int;

  /// Get a window from a stored ID, or NULL if it doesn't exist.
  pub fn SDL_GetWindowFromID(id: u32) -> *mut SDL_Window;

  /// Sets the display mode to use when the given window is visible and
  /// fullscreen.
  ///
  /// **Returns:** 0 on success, negative on failure. Call [`SDL_GetErrorMsg`]
  /// for more.
  pub fn SDL_SetWindowDisplayMode(
    window: *mut SDL_Window, mode: *const SDL_DisplayMode,
  ) -> c_int;

  /// Gets the display mode used when the given window is visible and
  /// fullscreen.
  ///
  /// **Returns:** 0 on success, negative on failure. Call [`SDL_GetErrorMsg`]
  /// for more.
  pub fn SDL_GetWindowDisplayMode(
    window: *mut SDL_Window, mode: *mut SDL_DisplayMode,
  ) -> c_int;

  /// Gets the pixel format of the window.
  ///
  /// **Return:** the pixel format on success, or `SDL_PIXELFORMAT_UNKNOWN` on
  /// failure. Call [`SDL_GetErrorMsg`] for more.
  pub fn SDL_GetWindowPixelFormat(window: *mut SDL_Window) -> Uint32;

  /// Create a window with the specified position, dimensions, and flags.
  ///
  /// * `title` The title of the window, in UTF-8 encoding.
  /// * `x` The x position of the window, [`SDL_WINDOWPOS_CENTERED`], or
  ///   [`SDL_WINDOWPOS_UNDEFINED`].
  /// * `y` The y position of the window, [`SDL_WINDOWPOS_CENTERED`], or
  ///   [`SDL_WINDOWPOS_UNDEFINED`].
  /// * `w` The width of the window, in screen coordinates.
  /// * `h` The height of the window, in screen coordinates.
  /// * `flags` The flags for the window, a mask of any of the following:
  ///   [`SDL_WINDOW_FULLSCREEN`],    [`SDL_WINDOW_OPENGL`],
  ///   [`SDL_WINDOW_HIDDEN`],        [`SDL_WINDOW_BORDERLESS`],
  ///   [`SDL_WINDOW_RESIZABLE`],     [`SDL_WINDOW_MAXIMIZED`],
  ///   [`SDL_WINDOW_MINIMIZED`],     [`SDL_WINDOW_INPUT_GRABBED`],
  ///   [`SDL_WINDOW_ALLOW_HIGHDPI`], [`SDL_WINDOW_VULKAN`],
  ///   [`SDL_WINDOW_METAL`].
  ///
  /// **Returns:** The created window, or `NULL` if window creation failed.
  ///
  /// If the window is created with the `SDL_WINDOW_ALLOW_HIGHDPI` flag, its
  /// size in pixels may differ from its size in screen coordinates on
  /// platforms with high-DPI support (e.g. iOS and Mac OS X). Use
  /// SDL_GetWindowSize() to query the client area's size in screen
  /// coordinates, and SDL_GL_GetDrawableSize(), SDL_Vulkan_GetDrawableSize(),
  /// or SDL_GetRendererOutputSize() to query the drawable size in pixels.
  ///
  /// If the window is created with any of the SDL_WINDOW_OPENGL or
  /// `SDL_WINDOW_VULKAN` flags, then the corresponding LoadLibrary function
  /// ([`SDL_GL_LoadLibrary`] or [`SDL_Vulkan_LoadLibrary`]) is called and the
  /// corresponding UnloadLibrary function is called by [`SDL_DestroyWindow`]().
  ///
  /// If `SDL_WINDOW_VULKAN` is specified and there isn't a working Vulkan
  /// driver, [`SDL_CreateWindow`]() will fail because
  /// [`SDL_Vulkan_LoadLibrary`]() will fail.
  ///
  /// If `SDL_WINDOW_METAL` is specified on an OS that does not support Metal,
  /// [`SDL_CreateWindow`]() will fail.
  ///
  /// **Note:** On non-Apple devices, SDL requires you to either not link to the
  /// Vulkan loader or link to a dynamic library version. This limitation
  /// may be removed in a future version of SDL.
  ///
  /// See Also: [`SDL_DestroyWindow`], [`SDL_GL_LoadLibrary`],
  /// [`SDL_Vulkan_LoadLibrary`]
  pub fn SDL_CreateWindow(
    title: *const c_char, x: c_int, y: c_int, w: c_int, h: c_int, flags: Uint32,
  ) -> *mut SDL_Window;

  /// Creates a window from driver-dependent window creation data, typically a
  /// native window pointer.
  ///
  /// **Returns:** the window pointer, or NULL on failure. Call
  /// [`SDL_GetErrorMsg`] for more.
  pub fn SDL_CreateWindowFrom(data: *const c_void) -> *mut SDL_Window;

  /// Gets the window's ID, or 0 on failure.
  pub fn SDL_GetWindowID(window: *mut SDL_Window) -> Uint32;

  /// Gets the flags of the window.
  pub fn SDL_GetWindowFlags(window: *mut SDL_Window) -> Uint32;

  /// Sets the title of a window.
  pub fn SDL_SetWindowTitle(window: *mut SDL_Window, title: *const c_char);

  /// Gets the window's title, or `""` if there is no title.
  pub fn SDL_GetWindowTitle(window: *mut SDL_Window) -> *const c_char;

  /// Sets the window's icon.
  pub fn SDL_SetWindowIcon(window: *mut SDL_Window, icon: *mut SDL_Surface);

  /// Associates a named user data pointer to the window.
  ///
  /// The name is case sensitive.
  pub fn SDL_SetWindowData(
    window: *mut SDL_Window, name: *const c_char, userdata: *mut c_void,
  ) -> *mut c_void;

  /// Gets the named user data pointer of the window.
  ///
  /// The name is case sensitive.
  pub fn SDL_GetWindowData(
    window: *mut SDL_Window, name: *const c_char,
  ) -> *mut c_void;

  /// Sets the window's position.
  pub fn SDL_SetWindowPosition(window: *mut SDL_Window, x: c_int, y: c_int);

  /// Gets the window's position.
  ///
  /// Either pointer may safely be null.
  pub fn SDL_GetWindowPosition(
    window: *mut SDL_Window, x: *mut c_int, y: *mut c_int,
  );

  /// Sets the window's client area size (in screen coordinates).
  ///
  /// Fails if either input is 0 or less.
  pub fn SDL_SetWindowSize(window: *mut SDL_Window, w: c_int, h: c_int);

  /// Gets the window's client area size (in screen coordinates).
  ///
  /// Either pointer may safely be null.
  pub fn SDL_GetWindowSize(
    window: *mut SDL_Window, w: *mut c_int, h: *mut c_int,
  );

  /// Gets the size of the border decoration around the client area.
  ///
  /// Any pointer may safely be null.
  pub fn SDL_GetWindowBordersSize(
    window: *mut SDL_Window, top: *mut c_int, left: *mut c_int,
    bottom: *mut c_int, right: *mut c_int,
  ) -> c_int;

  /// Sets the minimum client area size of the window.
  pub fn SDL_SetWindowMinimumSize(
    window: *mut SDL_Window, min_w: c_int, min_h: c_int,
  );

  /// Gets the minimum client area size of the window.
  pub fn SDL_GetWindowMinimumSize(
    window: *mut SDL_Window, w: *mut c_int, h: *mut c_int,
  );

  /// Sets the maximum client area size of the window.
  pub fn SDL_SetWindowMaximumSize(
    window: *mut SDL_Window, max_w: c_int, max_h: c_int,
  );

  /// Gets the maximum client area size of the window.
  pub fn SDL_GetWindowMaximumSize(
    window: *mut SDL_Window, w: *mut c_int, h: *mut c_int,
  );

  /// Add/remove the border of the window.
  ///
  /// This doesn't work on a fullscreen window.
  pub fn SDL_SetWindowBordered(window: *mut SDL_Window, bordered: SDL_bool);

  /// Set if the window should allow resizing.
  pub fn SDL_SetWindowResizable(window: *mut SDL_Window, resizable: SDL_bool);

  /// Shows a window.
  pub fn SDL_ShowWindow(window: *mut SDL_Window);

  /// Hides a window.
  pub fn SDL_HideWindow(window: *mut SDL_Window);

  /// Raises a window to the front and sets it for input focus.
  pub fn SDL_RaiseWindow(window: *mut SDL_Window);

  /// Maximize a window.
  pub fn SDL_MaximizeWindow(window: *mut SDL_Window);

  /// Minimize a window.
  pub fn SDL_MinimizeWindow(window: *mut SDL_Window);

  /// Restores a minimized/maximized window to its previous size and position.
  pub fn SDL_RestoreWindow(window: *mut SDL_Window);

  /// Sets the window's desired fullscreen state.
  ///
  /// * `SDL_WINDOW_FULLSCREEN`
  /// * `SDL_WINDOW_FULLSCREEN_DESKTOP`
  /// * 0 for a "windowed" window.
  ///
  /// **Returns:** 0 on success, negative on failure. Call [`SDL_GetErrorMsg`]
  /// for more.
  pub fn SDL_SetWindowFullscreen(
    window: *mut SDL_Window, flags: Uint32,
  ) -> c_int;

  /// Gets the surface of the window.
  ///
  /// This is a "borrowed" surface, associated to the window's lifetime and
  /// invalidated whenever the window is resized. Do not free it.
  ///
  /// You cannot combine usage of this surface with any other rendering API
  /// system (SDL's, GL, Vulkan, etc). Use [`SDL_UpdateWindowSurface`] to
  /// finalize your changes for the user to see.
  ///
  /// **Returns:** the surface pointer, or NULL on failure. Call
  /// [`SDL_GetErrorMsg`] for more.
  pub fn SDL_GetWindowSurface(window: *mut SDL_Window) -> *mut SDL_Surface;

  /// Updates the window surface data for the user to see.
  ///
  /// **Returns:** 0 on success, negative on failure. Call [`SDL_GetErrorMsg`]
  /// for more.
  pub fn SDL_UpdateWindowSurface(window: *mut SDL_Window) -> c_int;

  /// As [`SDL_UpdateWindowSurfaceRects`], but limited to the list of rects
  /// given.
  pub fn SDL_UpdateWindowSurfaceRects(
    window: *mut SDL_Window, rects: *const SDL_Rect, numrects: c_int,
  ) -> c_int;

  /// Sets if the window "grabs" the mouse pointer (locking it inside the
  /// window).
  ///
  /// Only one window can have a grab at a time. If another window is already
  /// grabbing the mouse, that grab is cancelled in favor of this grab.
  pub fn SDL_SetWindowGrab(window: *mut SDL_Window, grabbed: SDL_bool);

  /// Gets if the window is grabbing the mouse.
  pub fn SDL_GetWindowGrab(window: *mut SDL_Window) -> SDL_bool;

  /// Gets which window has the mouse grab (if any).
  pub fn SDL_GetGrabbedWindow() -> *mut SDL_Window;

  /// Set the brightness (gamma) for the display that owns the window.
  ///
  /// **Returns:** 0 on success, negative on failure. Call [`SDL_GetErrorMsg`]
  /// for more.
  pub fn SDL_SetWindowBrightness(
    window: *mut SDL_Window, brightness: f32,
  ) -> c_int;

  /// Gets the brightness (gamma) of the display that owns the window.
  pub fn SDL_GetWindowBrightness(window: *mut SDL_Window) -> f32;

  /// Sets a window's opacity.
  ///
  /// **Returns:** 0 on success, negative on failure. Call [`SDL_GetErrorMsg`]
  /// for more.
  pub fn SDL_SetWindowOpacity(window: *mut SDL_Window, opacity: f32) -> c_int;

  /// Gets a window's opacity (to the pointer given).
  ///
  /// **Returns:** 0 on success, negative on failure. Call [`SDL_GetErrorMsg`]
  /// for more.
  pub fn SDL_GetWindowOpacity(
    window: *mut SDL_Window, out_opacity: *mut f32,
  ) -> c_int;

  /// Sets a window as a modal for another window (X11 only).
  pub fn SDL_SetWindowModalFor(
    modal_window: *mut SDL_Window, parent_window: *mut SDL_Window,
  ) -> c_int;

  /// Sets input focus to a given window.
  pub fn SDL_SetWindowInputFocus(window: *mut SDL_Window) -> c_int;

  /// Sets a gamma ramp for the window.
  ///
  /// Each pointer should be a pointer to a 256-element translation table, or
  /// null.
  ///
  /// **Returns:** 0 on success, negative on failure. Call [`SDL_GetErrorMsg`]
  /// for more.
  pub fn SDL_SetWindowGammaRamp(
    window: *mut SDL_Window, red: *const Uint16, green: *const Uint16,
    blue: *const Uint16,
  ) -> c_int;

  /// Gets the gamma ramp of the window.
  pub fn SDL_GetWindowGammaRamp(
    window: *mut SDL_Window, red: *mut Uint16, green: *mut Uint16,
    blue: *mut Uint16,
  ) -> c_int;

  /// Sets the hit test callback hit test user data for the window.
  pub fn SDL_SetWindowHitTest(
    window: *mut SDL_Window, callback: SDL_HitTest, callback_data: *mut c_void,
  ) -> c_int;

  /// Destroys a window.
  pub fn SDL_DestroyWindow(window: *mut SDL_Window);

  /// If the screensaver is currently enabled.
  pub fn SDL_IsScreenSaverEnabled() -> SDL_bool;

  /// Enables the screensaver.
  pub fn SDL_EnableScreenSaver();

  /// Disables the screensaver.
  pub fn SDL_DisableScreenSaver();

  /// Load a GL library using the file path given.
  ///
  /// Use this *after* initializing the video driver, *before* opening any GL
  /// using windows.
  ///
  /// You can also skip using this function and the default GL
  /// library will be loaded when a GL window is created.
  ///
  /// **Returns:** 0 on success, negative on failure. Call [`SDL_GetErrorMsg`]
  /// for more.
  pub fn SDL_GL_LoadLibrary(path: *const c_char) -> c_int;

  /// Get a GL function pointer for a given function name.
  ///
  /// * On Windows, all GL function pointers are technically context specific.
  ///   You must have a current context before you call this. In practice the
  ///   function pointers will be shared across contexts, but MSDN says that
  ///   they're allowed to be context specific.
  /// * On X11, the pointers work for any context, but you can also get non-null
  ///   pointers that aren't safe to call. If the function in question is not
  ///   part of the core profile for the GL version of the active GL context,
  ///   use [`SDL_GL_ExtensionSupported`] to check that the pointer will be
  ///   valid.
  /// * You should generally use [`SDL_GL_ExtensionSupported`] for any extension
  ///   yo use. In some cases, a pointer for an extension function will be
  ///   non-null but still invalid.
  /// * If you're transmuting this output to a function pointer yourself:
  ///   Remember that all GL functions mut use the `extern "system"` ABI.
  pub fn SDL_GL_GetProcAddress(proc_: *const c_char) -> *mut c_void;

  /// Unloads the previously loaded GL library.
  pub fn SDL_GL_UnloadLibrary();

  /// If the given extension is supported in the current context.
  pub fn SDL_GL_ExtensionSupported(extension: *const c_char) -> SDL_bool;

  /// Resets all GL attributes to their default values.
  pub fn SDL_GL_ResetAttributes();

  /// Sets a GL attribute to the given value.
  ///
  /// **Returns:** 0 on success, negative on failure. Call [`SDL_GetErrorMsg`]
  /// for more.
  pub fn SDL_GL_SetAttribute(attr: SDL_GLattr, value: c_int) -> c_int;

  /// Gets a GL attribute's value (to the pointer given).
  ///
  /// **Returns:** 0 on success, negative on failure. Call [`SDL_GetErrorMsg`]
  /// for more.
  pub fn SDL_GL_GetAttribute(attr: SDL_GLattr, value: *mut c_int) -> c_int;

  /// Creates a new context for use with the window, and also makes it current.
  pub fn SDL_GL_CreateContext(window: *mut SDL_Window) -> SDL_GLContext;

  /// Make a context current and associated with the given window.
  ///
  /// You can pass NULL to make no context be the current context.
  ///
  /// **Returns:** 0 on success, negative on failure. Call [`SDL_GetErrorMsg`]
  /// for more.
  pub fn SDL_GL_MakeCurrent(
    window: *mut SDL_Window, context: SDL_GLContext,
  ) -> c_int;

  /// Gets the window associated with the current context.
  pub fn SDL_GL_GetCurrentWindow() -> *mut SDL_Window;

  /// Gets the current context.
  pub fn SDL_GL_GetCurrentContext() -> SDL_GLContext;

  /// Gets the drawable size (in pixels) of the client area.
  ///
  /// This is for use with `glViewport`.
  ///
  /// Either pointer may safely be null.
  pub fn SDL_GL_GetDrawableSize(
    window: *mut SDL_Window, w: *mut c_int, h: *mut c_int,
  );

  /// Sets the swap interval of GL swaps.
  ///
  /// * 0 for immediate swapping.
  /// * 1 for standard vsync swapping.
  /// * -1 for adaptive vsync swapping.
  ///
  /// Some systems will not support adaptive vsync but still support standard
  /// vsync.
  ///
  /// **Returns:** 0 on success, -1 if setting the swap interval isn't
  /// supported. Call [`SDL_GetErrorMsg`] for more.
  pub fn SDL_GL_SetSwapInterval(interval: c_int) -> c_int;

  /// Gets the swap interval setting.
  ///
  /// * 0 for immediate swapping.
  /// * 1 for standard vsync swapping.
  /// * -1 for adaptive vsync swapping.
  ///
  /// You can also call [`SDL_GetErrorMsg`] for more.
  pub fn SDL_GL_GetSwapInterval() -> c_int;

  /// Swaps the front buffer and back buffer of a GL using window.
  pub fn SDL_GL_SwapWindow(window: *mut SDL_Window);

  /// Deletes a GL context.
  ///
  /// If the context is current, it will be made un-current first.
  pub fn SDL_GL_DeleteContext(context: SDL_GLContext);
}
