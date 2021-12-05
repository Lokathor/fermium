//! SDL's custom system window manager hooks.
//!
//! This is mostly needed to make support for the
//! [`raw-window-handle`](https://docs.rs/raw-window-handle) crate possible.

use crate::{c_uint, c_ulong, c_void, stdinc::*, version::*, video::*};

/// These are the various supported windowing subsystems.
///
/// See `SDL_SYSWM_*` constants
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_SYSWM_TYPE(pub i32);

#[allow(missing_docs)]
pub const SDL_SYSWM_UNKNOWN: SDL_SYSWM_TYPE = SDL_SYSWM_TYPE(0);
#[allow(missing_docs)]
pub const SDL_SYSWM_WINDOWS: SDL_SYSWM_TYPE = SDL_SYSWM_TYPE(1);
#[allow(missing_docs)]
pub const SDL_SYSWM_X11: SDL_SYSWM_TYPE = SDL_SYSWM_TYPE(2);
#[allow(missing_docs)]
pub const SDL_SYSWM_DIRECTFB: SDL_SYSWM_TYPE = SDL_SYSWM_TYPE(3);
#[allow(missing_docs)]
pub const SDL_SYSWM_COCOA: SDL_SYSWM_TYPE = SDL_SYSWM_TYPE(4);
#[allow(missing_docs)]
pub const SDL_SYSWM_UIKIT: SDL_SYSWM_TYPE = SDL_SYSWM_TYPE(5);
#[allow(missing_docs)]
pub const SDL_SYSWM_WAYLAND: SDL_SYSWM_TYPE = SDL_SYSWM_TYPE(6);
#[allow(missing_docs)]
pub const SDL_SYSWM_MIR: SDL_SYSWM_TYPE = SDL_SYSWM_TYPE(7);
#[allow(missing_docs)]
pub const SDL_SYSWM_WINRT: SDL_SYSWM_TYPE = SDL_SYSWM_TYPE(8);
#[allow(missing_docs)]
pub const SDL_SYSWM_ANDROID: SDL_SYSWM_TYPE = SDL_SYSWM_TYPE(9);
#[allow(missing_docs)]
pub const SDL_SYSWM_VIVANTE: SDL_SYSWM_TYPE = SDL_SYSWM_TYPE(10);
#[allow(missing_docs)]
pub const SDL_SYSWM_OS2: SDL_SYSWM_TYPE = SDL_SYSWM_TYPE(11);
#[allow(missing_docs)]
pub const SDL_SYSWM_HAIKU: SDL_SYSWM_TYPE = SDL_SYSWM_TYPE(12);

#[derive(Clone, Copy)]
#[repr(C)]
#[allow(missing_docs)]
pub struct SDL_SysWMinfo_windows {
  /// The window handle (`HWND`)
  pub window: *mut c_void,
  /// The window device context (`HDC`)
  pub hdc: *mut c_void,
  /// The instance handle (`HINSTANCE`)
  pub hinstance: *mut c_void,
}

#[derive(Clone, Copy)]
#[repr(C)]
#[allow(missing_docs)]
pub struct SDL_SysWMinfo_winrt {
  /// The WinRT CoreWindow (`IInspectable*`)
  pub window: *mut c_void,
}

#[derive(Clone, Copy)]
#[repr(C)]
#[allow(missing_docs)]
pub struct SDL_SysWMinfo_x11 {
  /// The X11 display (`Display*`)
  pub display: *mut c_void,
  /// The X11 window (`Window`)
  pub window: c_ulong,
}

#[derive(Clone, Copy)]
#[repr(C)]
#[allow(missing_docs)]
pub struct SDL_SysWMinfo_directfb {
  /// The directfb main interface (`IDirectFB*`)
  pub dfb: *mut c_void,
  /// The directfb window handle (`IDirectFBWindow*`)
  pub window: *mut c_void,
  /// The directfb client surface (`IDirectFBSurface*`)
  pub surface: *mut c_void,
}

#[derive(Clone, Copy)]
#[repr(C)]
#[allow(missing_docs)]
pub struct SDL_SysWMinfo_cocoa {
  /// The Cocoa window (`NSWindow*`)
  pub window: *mut c_void,
}

#[derive(Clone, Copy)]
#[repr(C)]
#[allow(missing_docs)]
pub struct SDL_SysWMinfo_uikit {
  /// The UIKit window (`UIWindow*`)
  pub window: *mut c_void,
  /// The GL view's Framebuffer Object. It must be bound when rendering to the
  /// screen using GL. (`GLuint`)
  pub framebuffer: c_uint,
  /// The GL view's color Renderbuffer Object. It must be bound when
  /// SDL_GL_SwapWindow is called.
  pub colorbuffer: c_uint,
  /// The Framebuffer Object which holds the resolve color Renderbuffer, when
  /// MSAA is used.
  pub resolveFramebuffer: c_uint,
}

#[derive(Clone, Copy)]
#[repr(C)]
#[allow(missing_docs)]
pub struct SDL_SysWMinfo_wayland {
  /// Wayland display (`wl_display*`)
  pub display: *mut c_void,
  /// Wayland surface (`wl_surface*`)
  pub surface: *mut c_void,
  /// Wayland shell_surface (window manager handle) (`wl_shell_surface*`)
  pub shell_surface: *mut c_void,
}

#[derive(Clone, Copy)]
#[repr(C)]
#[allow(missing_docs)]
pub struct SDL_SysWMinfo_android {
  /// Android native window (`ANativeWindow*`)
  pub window: *mut c_void,
  /// Embedded GL surface (`EGLSurface`)
  pub surface: *mut c_void,
}

#[derive(Clone, Copy)]
#[repr(C)]
#[allow(missing_docs)]
pub struct SDL_SysWMinfo_vivante {
  /// (`EGLNativeDisplayType`)
  pub display: *mut c_void,
  /// (`EGLNativeWindowType`)
  pub window: *mut c_void,
}

#[derive(Clone, Copy)]
#[repr(C)]
#[allow(missing_docs)]
pub union SDL_SysWMinfo_union {
  pub win: SDL_SysWMinfo_windows,
  pub winrt: SDL_SysWMinfo_winrt,
  pub x11: SDL_SysWMinfo_x11,
  pub dfb: SDL_SysWMinfo_directfb,
  pub cocoa: SDL_SysWMinfo_cocoa,
  pub uikit: SDL_SysWMinfo_uikit,
  pub wl: SDL_SysWMinfo_wayland,
  /* MIR entry skipped because it's no longer available */
  pub android: SDL_SysWMinfo_android,
  pub vivante: SDL_SysWMinfo_vivante,
  /* os2 support skipped, file an issue if you care. */
  /// Dummy field to ensure that the union is always at least 64 bytes.
  pub dummy: [u8; 64],
}

/// The custom window manager information structure.
///
/// When this structure is returned, it holds information about which low level
/// system it is using, and will be one of SDL_SYSWM_TYPE.
#[derive(Clone, Copy)]
#[repr(C)]
#[allow(missing_docs)]
pub struct SDL_SysWMinfo {
  pub version: SDL_version,
  pub subsystem: SDL_SYSWM_TYPE,
  pub info: SDL_SysWMinfo_union,
}
impl Default for SDL_SysWMinfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    unsafe { core::mem::zeroed() }
  }
}

#[cfg(feature = "raw-window-handle")]
impl SDL_SysWMinfo {
  /// Attempts to convert the info into the correct [RawWindowHandle] value
  ///
  /// ## Safety
  /// * The `subsystem` must be correct for the `info` data, or bad things
  ///   happen.
  pub unsafe fn try_into(self) -> Option<raw_window_handle::RawWindowHandle> {
    use raw_window_handle::*;
    Some(match self.subsystem {
      SDL_SYSWM_WINDOWS => {
        let mut win32_handle = Win32Handle::empty();
        win32_handle.hinstance = self.info.win.hinstance;
        win32_handle.hwnd = self.info.win.window;
        RawWindowHandle::Win32(win32_handle)
      }
      SDL_SYSWM_X11 => {
        let mut xlib_handle = XlibHandle::empty();
        xlib_handle.display = self.info.x11.display;
        xlib_handle.window = self.info.x11.window;
        RawWindowHandle::Xlib(xlib_handle)
      }
      SDL_SYSWM_COCOA => {
        let mut appkit_handle = AppKitHandle::empty();
        appkit_handle.ns_window = self.info.cocoa.window;
        RawWindowHandle::AppKit(appkit_handle)
      }
      SDL_SYSWM_UIKIT => {
        let mut uikit_handle = UiKitHandle::empty();
        uikit_handle.ui_window = self.info.uikit.window;
        RawWindowHandle::UiKit(uikit_handle)
      }
      SDL_SYSWM_WAYLAND => {
        let mut wayland_handle = WaylandHandle::empty();
        wayland_handle.display = self.info.wl.display;
        wayland_handle.surface = self.info.wl.surface;
        RawWindowHandle::Wayland(wayland_handle)
      }
      SDL_SYSWM_WINRT => {
        let mut winrt_handle = WinRtHandle::empty();
        winrt_handle.core_window = self.info.winrt.window;
        RawWindowHandle::WinRt(winrt_handle)
      }
      SDL_SYSWM_ANDROID => {
        let mut android_handle = AndroidNdkHandle::empty();
        android_handle.a_native_window = self.info.android.window;
        RawWindowHandle::AndroidNdk(android_handle)
      }
      /* I don't think RWH supports any of these? */
      SDL_SYSWM_MIR | SDL_SYSWM_VIVANTE | SDL_SYSWM_OS2 | SDL_SYSWM_HAIKU
      | SDL_SYSWM_DIRECTFB => return None,
      _ => return /*We shouldn't be in this case! Oh well!*/ None,
    })
  }
}

extern "C" {
  /// This function allows access to driver-dependent window information.
  ///
  /// * `window` The window about which information is being requested
  /// * `info` This structure must be initialized with the SDL version, and is
  ///   then filled in with information about the given window.
  ///
  /// **Returns:** `SDL_TRUE` if the function is implemented and the version
  /// member of the `info` struct is valid, `SDL_FALSE` otherwise.
  ///
  /// You typically use this function like this:
  /// ```no_run
  /// # use fermium::prelude::*;
  /// let window = unimplemented!("make the window");
  /// let mut info = SDL_SysWMinfo::default();
  /// SDL_VERSION(&mut info.version);
  /// if SDL_TRUE == unsafe { SDL_GetWindowWMInfo(window, &mut info) } {
  ///   unimplemented!("now you have your info");
  /// }
  /// ```
  pub fn SDL_GetWindowWMInfo(
    window: *mut SDL_Window, info: *mut SDL_SysWMinfo,
  ) -> SDL_bool;
}
