pub use crate::{pixels::*, rect::*, stdinc::*, surface::*};

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

/// The type used to identify a window
#[allow(unused)]
#[repr(transparent)]
pub struct SDL_Window(c_void);

/// The flags on a window
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_WindowFlags(pub u32);
// TODO: bit-ops for SDL_WindowFlags
pub const SDL_WINDOW_FULLSCREEN: SDL_WindowFlags = SDL_WindowFlags(0x00000001);
pub const SDL_WINDOW_OPENGL: SDL_WindowFlags = SDL_WindowFlags(0x00000002);
pub const SDL_WINDOW_SHOWN: SDL_WindowFlags = SDL_WindowFlags(0x00000004);
pub const SDL_WINDOW_HIDDEN: SDL_WindowFlags = SDL_WindowFlags(0x00000008);
pub const SDL_WINDOW_BORDERLESS: SDL_WindowFlags = SDL_WindowFlags(0x00000010);
pub const SDL_WINDOW_RESIZABLE: SDL_WindowFlags = SDL_WindowFlags(0x00000020);
pub const SDL_WINDOW_MINIMIZED: SDL_WindowFlags = SDL_WindowFlags(0x00000040);
pub const SDL_WINDOW_MAXIMIZED: SDL_WindowFlags = SDL_WindowFlags(0x00000080);
pub const SDL_WINDOW_INPUT_GRABBED: SDL_WindowFlags =
  SDL_WindowFlags(0x00000100);
pub const SDL_WINDOW_INPUT_FOCUS: SDL_WindowFlags = SDL_WindowFlags(0x00000200);
pub const SDL_WINDOW_MOUSE_FOCUS: SDL_WindowFlags = SDL_WindowFlags(0x00000400);
pub const SDL_WINDOW_FULLSCREEN_DESKTOP: SDL_WindowFlags =
  SDL_WindowFlags(SDL_WINDOW_FULLSCREEN.0 | 0x00001000);
pub const SDL_WINDOW_FOREIGN: SDL_WindowFlags = SDL_WindowFlags(0x00000800);
pub const SDL_WINDOW_ALLOW_HIGHDPI: SDL_WindowFlags =
  SDL_WindowFlags(0x00002000);
pub const SDL_WINDOW_MOUSE_CAPTURE: SDL_WindowFlags =
  SDL_WindowFlags(0x00004000);
pub const SDL_WINDOW_ALWAYS_ON_TOP: SDL_WindowFlags =
  SDL_WindowFlags(0x00008000);
pub const SDL_WINDOW_SKIP_TASKBAR: SDL_WindowFlags =
  SDL_WindowFlags(0x00010000);
pub const SDL_WINDOW_UTILITY: SDL_WindowFlags = SDL_WindowFlags(0x00020000);
pub const SDL_WINDOW_TOOLTIP: SDL_WindowFlags = SDL_WindowFlags(0x00040000);
pub const SDL_WINDOW_POPUP_MENU: SDL_WindowFlags = SDL_WindowFlags(0x00080000);
pub const SDL_WINDOW_VULKAN: SDL_WindowFlags = SDL_WindowFlags(0x10000000);
pub const SDL_WINDOW_METAL: SDL_WindowFlags = SDL_WindowFlags(0x20000000);

pub const SDL_WINDOWPOS_UNDEFINED: i32 = 0x1FFF0000;

pub const SDL_WINDOWPOS_CENTERED: i32 = 0x2FFF0000;

/// Event subtype for window events
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_WindowEventID(pub u32);
pub const SDL_WINDOWEVENT_NONE: SDL_WindowEventID = SDL_WindowEventID(0);
pub const SDL_WINDOWEVENT_SHOWN: SDL_WindowEventID = SDL_WindowEventID(1);
pub const SDL_WINDOWEVENT_HIDDEN: SDL_WindowEventID = SDL_WindowEventID(2);
pub const SDL_WINDOWEVENT_EXPOSED: SDL_WindowEventID = SDL_WindowEventID(3);
pub const SDL_WINDOWEVENT_MOVED: SDL_WindowEventID = SDL_WindowEventID(4);
pub const SDL_WINDOWEVENT_RESIZED: SDL_WindowEventID = SDL_WindowEventID(5);
pub const SDL_WINDOWEVENT_SIZE_CHANGED: SDL_WindowEventID =
  SDL_WindowEventID(6);
pub const SDL_WINDOWEVENT_MINIMIZED: SDL_WindowEventID = SDL_WindowEventID(7);
pub const SDL_WINDOWEVENT_MAXIMIZED: SDL_WindowEventID = SDL_WindowEventID(8);
pub const SDL_WINDOWEVENT_RESTORED: SDL_WindowEventID = SDL_WindowEventID(9);
pub const SDL_WINDOWEVENT_ENTER: SDL_WindowEventID = SDL_WindowEventID(10);
pub const SDL_WINDOWEVENT_LEAVE: SDL_WindowEventID = SDL_WindowEventID(11);
pub const SDL_WINDOWEVENT_FOCUS_GAINED: SDL_WindowEventID =
  SDL_WindowEventID(12);
pub const SDL_WINDOWEVENT_FOCUS_LOST: SDL_WindowEventID = SDL_WindowEventID(13);
pub const SDL_WINDOWEVENT_CLOSE: SDL_WindowEventID = SDL_WindowEventID(14);
pub const SDL_WINDOWEVENT_TAKE_FOCUS: SDL_WindowEventID = SDL_WindowEventID(15);
pub const SDL_WINDOWEVENT_HIT_TEST: SDL_WindowEventID = SDL_WindowEventID(16);

/// Event subtype for display events
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_DisplayEventID(pub u32);
/// Never used
pub const SDL_DISPLAYEVENT_NONE: SDL_DisplayEventID = SDL_DisplayEventID(0);
/// Display orientation has changed to data1
pub const SDL_DISPLAYEVENT_ORIENTATION: SDL_DisplayEventID =
  SDL_DisplayEventID(1);

/// Event subtype for window events
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_DisplayOrientation(pub u32);
pub const SDL_ORIENTATION_UNKNOWN: SDL_DisplayOrientation =
  SDL_DisplayOrientation(0);
pub const SDL_ORIENTATION_LANDSCAPE: SDL_DisplayOrientation =
  SDL_DisplayOrientation(1);
pub const SDL_ORIENTATION_LANDSCAPE_FLIPPED: SDL_DisplayOrientation =
  SDL_DisplayOrientation(2);
pub const SDL_ORIENTATION_PORTRAIT: SDL_DisplayOrientation =
  SDL_DisplayOrientation(3);
pub const SDL_ORIENTATION_PORTRAIT_FLIPPED: SDL_DisplayOrientation =
  SDL_DisplayOrientation(4);

/// An opaque handle to an OpenGL context.
#[derive(Debug, Clone, Copy)]
#[allow(unused)]
#[repr(transparent)]
pub struct SDL_GLContext(*mut c_void);

/// OpenGL configuration attributes
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_GLattr(pub u32);
pub const SDL_GL_RED_SIZE: SDL_GLattr = SDL_GLattr(0);
pub const SDL_GL_GREEN_SIZE: SDL_GLattr = SDL_GLattr(1);
pub const SDL_GL_BLUE_SIZE: SDL_GLattr = SDL_GLattr(2);
pub const SDL_GL_ALPHA_SIZE: SDL_GLattr = SDL_GLattr(3);
pub const SDL_GL_BUFFER_SIZE: SDL_GLattr = SDL_GLattr(4);
pub const SDL_GL_DOUBLEBUFFER: SDL_GLattr = SDL_GLattr(5);
pub const SDL_GL_DEPTH_SIZE: SDL_GLattr = SDL_GLattr(6);
pub const SDL_GL_STENCIL_SIZE: SDL_GLattr = SDL_GLattr(7);
pub const SDL_GL_ACCUM_RED_SIZE: SDL_GLattr = SDL_GLattr(8);
pub const SDL_GL_ACCUM_GREEN_SIZE: SDL_GLattr = SDL_GLattr(9);
pub const SDL_GL_ACCUM_BLUE_SIZE: SDL_GLattr = SDL_GLattr(10);
pub const SDL_GL_ACCUM_ALPHA_SIZE: SDL_GLattr = SDL_GLattr(11);
pub const SDL_GL_STEREO: SDL_GLattr = SDL_GLattr(12);
pub const SDL_GL_MULTISAMPLEBUFFERS: SDL_GLattr = SDL_GLattr(13);
pub const SDL_GL_MULTISAMPLESAMPLES: SDL_GLattr = SDL_GLattr(14);
pub const SDL_GL_ACCELERATED_VISUAL: SDL_GLattr = SDL_GLattr(15);
pub const SDL_GL_RETAINED_BACKING: SDL_GLattr = SDL_GLattr(16);
pub const SDL_GL_CONTEXT_MAJOR_VERSION: SDL_GLattr = SDL_GLattr(17);
pub const SDL_GL_CONTEXT_MINOR_VERSION: SDL_GLattr = SDL_GLattr(18);
pub const SDL_GL_CONTEXT_EGL: SDL_GLattr = SDL_GLattr(19);
pub const SDL_GL_CONTEXT_FLAGS: SDL_GLattr = SDL_GLattr(20);
pub const SDL_GL_CONTEXT_PROFILE_MASK: SDL_GLattr = SDL_GLattr(21);
pub const SDL_GL_SHARE_WITH_CURRENT_CONTEXT: SDL_GLattr = SDL_GLattr(22);
pub const SDL_GL_FRAMEBUFFER_SRGB_CAPABLE: SDL_GLattr = SDL_GLattr(23);
pub const SDL_GL_CONTEXT_RELEASE_BEHAVIOR: SDL_GLattr = SDL_GLattr(24);
pub const SDL_GL_CONTEXT_RESET_NOTIFICATION: SDL_GLattr = SDL_GLattr(25);
pub const SDL_GL_CONTEXT_NO_ERROR: SDL_GLattr = SDL_GLattr(26);

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_GLprofile(pub u32);
pub const SDL_GL_CONTEXT_PROFILE_CORE: SDL_GLprofile = SDL_GLprofile(0x0001);
pub const SDL_GL_CONTEXT_PROFILE_COMPATIBILITY: SDL_GLprofile =
  SDL_GLprofile(0x0002);
pub const SDL_GL_CONTEXT_PROFILE_ES: SDL_GLprofile = SDL_GLprofile(0x0004);

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_GLcontextFlag(pub u32);
pub const SDL_GL_CONTEXT_DEBUG_FLAG: SDL_GLcontextFlag =
  SDL_GLcontextFlag(0x0001);
pub const SDL_GL_CONTEXT_FORWARD_COMPATIBLE_FLAG: SDL_GLcontextFlag =
  SDL_GLcontextFlag(0x0002);
pub const SDL_GL_CONTEXT_ROBUST_ACCESS_FLAG: SDL_GLcontextFlag =
  SDL_GLcontextFlag(0x0004);
pub const SDL_GL_CONTEXT_RESET_ISOLATION_FLAG: SDL_GLcontextFlag =
  SDL_GLcontextFlag(0x0008);

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_GLcontextReleaseFlag(pub u32);
pub const SDL_GL_CONTEXT_RELEASE_BEHAVIOR_NONE: SDL_GLcontextReleaseFlag =
  SDL_GLcontextReleaseFlag(0x0000);
pub const SDL_GL_CONTEXT_RELEASE_BEHAVIOR_FLUSH: SDL_GLcontextReleaseFlag =
  SDL_GLcontextReleaseFlag(0x0001);

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_GLContextResetNotification(pub u32);
pub const SDL_GL_CONTEXT_RESET_NO_NOTIFICATION: SDL_GLContextResetNotification =
  SDL_GLContextResetNotification(0x0000);
pub const SDL_GL_CONTEXT_RESET_LOSE_CONTEXT: SDL_GLContextResetNotification =
  SDL_GLContextResetNotification(0x0001);

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_HitTestResult(pub u32);
pub const SDL_HITTEST_NORMAL: SDL_HitTestResult = SDL_HitTestResult(0);
pub const SDL_HITTEST_DRAGGABLE: SDL_HitTestResult = SDL_HitTestResult(1);
pub const SDL_HITTEST_RESIZE_TOPLEFT: SDL_HitTestResult = SDL_HitTestResult(2);
pub const SDL_HITTEST_RESIZE_TOP: SDL_HitTestResult = SDL_HitTestResult(3);
pub const SDL_HITTEST_RESIZE_TOPRIGHT: SDL_HitTestResult = SDL_HitTestResult(4);
pub const SDL_HITTEST_RESIZE_RIGHT: SDL_HitTestResult = SDL_HitTestResult(5);
pub const SDL_HITTEST_RESIZE_BOTTOMRIGHT: SDL_HitTestResult =
  SDL_HitTestResult(6);
pub const SDL_HITTEST_RESIZE_BOTTOM: SDL_HitTestResult = SDL_HitTestResult(7);
pub const SDL_HITTEST_RESIZE_BOTTOMLEFT: SDL_HitTestResult =
  SDL_HitTestResult(8);
pub const SDL_HITTEST_RESIZE_LEFT: SDL_HitTestResult = SDL_HitTestResult(9);

pub type SDL_HitTest = Option<
  unsafe extern "C" fn(
    win: *mut SDL_Window,
    area: *const SDL_Point,
    data: *mut c_void,
  ) -> SDL_HitTestResult,
>;

extern "C" {
  pub fn SDL_GetNumVideoDrivers() -> c_int;
  pub fn SDL_GetVideoDriver(index: c_int) -> *const c_char;
  pub fn SDL_VideoInit(driver_name: *const c_char) -> c_int;
  pub fn SDL_VideoQuit();
  pub fn SDL_GetCurrentVideoDriver() -> *const c_char;
  pub fn SDL_GetNumVideoDisplays() -> c_int;
  pub fn SDL_GetDisplayName(displayIndex: c_int) -> *const c_char;
  pub fn SDL_GetDisplayBounds(
    displayIndex: c_int, rect: *mut SDL_Rect,
  ) -> c_int;
  pub fn SDL_GetDisplayUsableBounds(
    displayIndex: c_int, rect: *mut SDL_Rect,
  ) -> c_int;
  pub fn SDL_GetDisplayDPI(
    displayIndex: c_int, ddpi: *mut f32, hdpi: *mut f32, vdpi: *mut f32,
  ) -> c_int;
  pub fn SDL_GetDisplayOrientation(
    displayIndex: c_int,
  ) -> SDL_DisplayOrientation;
  pub fn SDL_GetNumDisplayModes(displayIndex: c_int) -> c_int;
  pub fn SDL_GetDisplayMode(
    displayIndex: c_int, modeIndex: c_int, mode: *mut SDL_DisplayMode,
  ) -> c_int;
  pub fn SDL_GetDesktopDisplayMode(
    displayIndex: c_int, mode: *mut SDL_DisplayMode,
  ) -> c_int;
  pub fn SDL_GetCurrentDisplayMode(
    displayIndex: c_int, mode: *mut SDL_DisplayMode,
  ) -> c_int;
  pub fn SDL_GetClosestDisplayMode(
    displayIndex: c_int, mode: *const SDL_DisplayMode,
    closest: *mut SDL_DisplayMode,
  ) -> *mut SDL_DisplayMode;
  pub fn SDL_GetWindowDisplayIndex(window: *mut SDL_Window) -> c_int;
  pub fn SDL_SetWindowDisplayMode(
    window: *mut SDL_Window, mode: *const SDL_DisplayMode,
  ) -> c_int;
  pub fn SDL_GetWindowDisplayMode(
    window: *mut SDL_Window, mode: *mut SDL_DisplayMode,
  ) -> c_int;
  pub fn SDL_GetWindowPixelFormat(window: *mut SDL_Window) -> Uint32;
  pub fn SDL_CreateWindow(
    title: *const c_char, x: c_int, y: c_int, w: c_int, h: c_int, flags: Uint32,
  ) -> *mut SDL_Window;
  pub fn SDL_CreateWindowFrom(data: *const c_void) -> *mut SDL_Window;
  pub fn SDL_GetWindowID(window: *mut SDL_Window) -> Uint32;
  pub fn SDL_GetWindowFlags(window: *mut SDL_Window) -> Uint32;
  pub fn SDL_SetWindowTitle(window: *mut SDL_Window, title: *const c_char);
  pub fn SDL_GetWindowTitle(window: *mut SDL_Window) -> *const c_char;
  pub fn SDL_SetWindowIcon(window: *mut SDL_Window, icon: *mut SDL_Surface);
  pub fn SDL_SetWindowData(
    window: *mut SDL_Window, name: *const c_char, userdata: *mut c_void,
  ) -> *mut c_void;
  pub fn SDL_GetWindowData(
    window: *mut SDL_Window, name: *const c_char,
  ) -> *mut c_void;
  pub fn SDL_SetWindowPosition(window: *mut SDL_Window, x: c_int, y: c_int);
  pub fn SDL_GetWindowPosition(
    window: *mut SDL_Window, x: *mut c_int, y: *mut c_int,
  );
  pub fn SDL_SetWindowSize(window: *mut SDL_Window, w: c_int, h: c_int);
  pub fn SDL_GetWindowSize(
    window: *mut SDL_Window, w: *mut c_int, h: *mut c_int,
  );
  pub fn SDL_GetWindowBordersSize(
    window: *mut SDL_Window, top: *mut c_int, left: *mut c_int,
    bottom: *mut c_int, right: *mut c_int,
  ) -> c_int;
  pub fn SDL_SetWindowMinimumSize(
    window: *mut SDL_Window, min_w: c_int, min_h: c_int,
  );
  pub fn SDL_GetWindowMinimumSize(
    window: *mut SDL_Window, w: *mut c_int, h: *mut c_int,
  );
  pub fn SDL_SetWindowMaximumSize(
    window: *mut SDL_Window, max_w: c_int, max_h: c_int,
  );
  pub fn SDL_GetWindowMaximumSize(
    window: *mut SDL_Window, w: *mut c_int, h: *mut c_int,
  );
  pub fn SDL_SetWindowBordered(window: *mut SDL_Window, bordered: SDL_bool);
  pub fn SDL_SetWindowResizable(window: *mut SDL_Window, resizable: SDL_bool);
  pub fn SDL_ShowWindow(window: *mut SDL_Window);
  pub fn SDL_HideWindow(window: *mut SDL_Window);
  pub fn SDL_RaiseWindow(window: *mut SDL_Window);
  pub fn SDL_MaximizeWindow(window: *mut SDL_Window);
  pub fn SDL_MinimizeWindow(window: *mut SDL_Window);
  pub fn SDL_RestoreWindow(window: *mut SDL_Window);
  pub fn SDL_SetWindowFullscreen(
    window: *mut SDL_Window, flags: Uint32,
  ) -> c_int;
  pub fn SDL_GetWindowSurface(window: *mut SDL_Window) -> *mut SDL_Surface;
  pub fn SDL_UpdateWindowSurface(window: *mut SDL_Window) -> c_int;
  pub fn SDL_UpdateWindowSurfaceRects(
    window: *mut SDL_Window, rects: *const SDL_Rect, numrects: c_int,
  ) -> c_int;
  pub fn SDL_SetWindowGrab(window: *mut SDL_Window, grabbed: SDL_bool);
  pub fn SDL_GetWindowGrab(window: *mut SDL_Window) -> SDL_bool;
  pub fn SDL_GetGrabbedWindow() -> *mut SDL_Window;
  pub fn SDL_SetWindowBrightness(
    window: *mut SDL_Window, brightness: f32,
  ) -> c_int;
  pub fn SDL_GetWindowBrightness(window: *mut SDL_Window) -> f32;
  pub fn SDL_SetWindowOpacity(window: *mut SDL_Window, opacity: f32) -> c_int;
  pub fn SDL_GetWindowOpacity(
    window: *mut SDL_Window, out_opacity: *mut f32,
  ) -> c_int;
  pub fn SDL_SetWindowModalFor(
    modal_window: *mut SDL_Window, parent_window: *mut SDL_Window,
  ) -> c_int;
  pub fn SDL_SetWindowInputFocus(window: *mut SDL_Window) -> c_int;
  pub fn SDL_SetWindowGammaRamp(
    window: *mut SDL_Window, red: *const Uint16, green: *const Uint16,
    blue: *const Uint16,
  ) -> c_int;
  pub fn SDL_GetWindowGammaRamp(
    window: *mut SDL_Window, red: *mut Uint16, green: *mut Uint16,
    blue: *mut Uint16,
  ) -> c_int;
  pub fn SDL_SetWindowHitTest(
    window: *mut SDL_Window, callback: SDL_HitTest, callback_data: *mut c_void,
  ) -> c_int;
  pub fn SDL_DestroyWindow(window: *mut SDL_Window);
  pub fn SDL_IsScreenSaverEnabled() -> SDL_bool;
  pub fn SDL_EnableScreenSaver();
  pub fn SDL_DisableScreenSaver();
  pub fn SDL_GL_LoadLibrary(path: *const c_char) -> c_int;
  pub fn SDL_GL_GetProcAddress(proc_: *const c_char) -> *mut c_void;
  pub fn SDL_GL_UnloadLibrary();
  pub fn SDL_GL_ExtensionSupported(extension: *const c_char) -> SDL_bool;
  pub fn SDL_GL_ResetAttributes();
  pub fn SDL_GL_SetAttribute(attr: SDL_GLattr, value: c_int) -> c_int;
  pub fn SDL_GL_GetAttribute(attr: SDL_GLattr, value: *mut c_int) -> c_int;
  pub fn SDL_GL_CreateContext(window: *mut SDL_Window) -> SDL_GLContext;
  pub fn SDL_GL_MakeCurrent(
    window: *mut SDL_Window, context: SDL_GLContext,
  ) -> c_int;
  pub fn SDL_GL_GetCurrentWindow() -> *mut SDL_Window;
  pub fn SDL_GL_GetCurrentContext() -> SDL_GLContext;
  pub fn SDL_GL_GetDrawableSize(
    window: *mut SDL_Window, w: *mut c_int, h: *mut c_int,
  );
  pub fn SDL_GL_SetSwapInterval(interval: c_int) -> c_int;
  pub fn SDL_GL_GetSwapInterval() -> c_int;
  pub fn SDL_GL_SwapWindow(window: *mut SDL_Window);
  pub fn SDL_GL_DeleteContext(context: SDL_GLContext);
}
