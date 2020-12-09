//! Allows the creation of message boxes, for short messages to the user.

pub use crate::video::*;

/// [`SDL_MessageBox`] flags.
///
/// If supported, will display warning icon, etc.
///
/// See the `SDL_MESSAGEBOX_*` constants.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_MessageBoxFlags(pub u32);
impl_bit_ops_for_tuple_newtype!(SDL_MessageBoxFlags);

/// error dialog
pub const SDL_MESSAGEBOX_ERROR: SDL_MessageBoxFlags =
  SDL_MessageBoxFlags(0x00000010);
/// warning dialog
pub const SDL_MESSAGEBOX_WARNING: SDL_MessageBoxFlags =
  SDL_MessageBoxFlags(0x00000020);
/// informational dialog
pub const SDL_MESSAGEBOX_INFORMATION: SDL_MessageBoxFlags =
  SDL_MessageBoxFlags(0x00000040);
/// buttons placed left to right
pub const SDL_MESSAGEBOX_BUTTONS_LEFT_TO_RIGHT: SDL_MessageBoxFlags =
  SDL_MessageBoxFlags(0x00000080);
/// buttons placed right to left
pub const SDL_MESSAGEBOX_BUTTONS_RIGHT_TO_LEFT: SDL_MessageBoxFlags =
  SDL_MessageBoxFlags(0x00000100);

/// [`SDL_MessageBoxButtonData`] flags.
///
/// If supported, will display warning icon, etc.
///
/// See the `SDL_MESSAGEBOX_*` constants.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_MessageBoxButtonFlags(pub u32);
impl_bit_ops_for_tuple_newtype!(SDL_MessageBoxButtonFlags);

/// Marks the default button when return is hit.
pub const SDL_MESSAGEBOX_BUTTON_RETURNKEY_DEFAULT: SDL_MessageBoxButtonFlags =
  SDL_MessageBoxButtonFlags(0x00000001);
/// Marks the default button when escape is hit.
pub const SDL_MESSAGEBOX_BUTTON_ESCAPEKEY_DEFAULT: SDL_MessageBoxButtonFlags =
  SDL_MessageBoxButtonFlags(0x00000002);

/// Individual button data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
#[allow(missing_docs)]
pub struct SDL_MessageBoxButtonData {
  pub flags: SDL_MessageBoxButtonFlags,
  /// User defined button id (the value returned via [`SDL_ShowMessageBox`])
  pub buttonid: c_int,
  /// The UTF-8 button text
  pub text: *const c_char,
}
impl Default for SDL_MessageBoxButtonData {
  fn default() -> Self {
    unsafe { core::mem::zeroed() }
  }
}

/// RGB value used in a message box color scheme
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
#[allow(missing_docs)]
pub struct SDL_MessageBoxColor {
  pub r: Uint8,
  pub g: Uint8,
  pub b: Uint8,
}

/// See the `SDL_MESSAGEBOX_COLOR_*` constants.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_MessageBoxColorType(pub u32);

#[allow(missing_docs)]
pub const SDL_MESSAGEBOX_COLOR_BACKGROUND: SDL_MessageBoxColorType =
  SDL_MessageBoxColorType(0);
#[allow(missing_docs)]
pub const SDL_MESSAGEBOX_COLOR_TEXT: SDL_MessageBoxColorType =
  SDL_MessageBoxColorType(1);
#[allow(missing_docs)]
pub const SDL_MESSAGEBOX_COLOR_BUTTON_BORDER: SDL_MessageBoxColorType =
  SDL_MessageBoxColorType(2);
#[allow(missing_docs)]
pub const SDL_MESSAGEBOX_COLOR_BUTTON_BACKGROUND: SDL_MessageBoxColorType =
  SDL_MessageBoxColorType(3);
#[allow(missing_docs)]
pub const SDL_MESSAGEBOX_COLOR_BUTTON_SELECTED: SDL_MessageBoxColorType =
  SDL_MessageBoxColorType(4);
#[allow(missing_docs)]
pub const SDL_MESSAGEBOX_COLOR_MAX: usize = 5;

/// A set of colors to use for message box dialogs
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
#[allow(missing_docs)]
pub struct SDL_MessageBoxColorScheme {
  pub colors: [SDL_MessageBoxColor; SDL_MESSAGEBOX_COLOR_MAX],
}

/// MessageBox structure containing title, text, window, etc.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
#[allow(missing_docs)]
pub struct SDL_MessageBoxData {
  pub flags: SDL_MessageBoxFlags,
  /// Parent window, can be NULL
  pub window: *mut SDL_Window,
  /// UTF-8 title
  pub title: *const c_char,
  /// UTF-8 message text
  pub message: *const c_char,
  pub numbuttons: c_int,
  pub buttons: *const SDL_MessageBoxButtonData,
  /// This can be NULL to use system settings
  pub colorScheme: *const SDL_MessageBoxColorScheme,
}
impl Default for SDL_MessageBoxData {
  fn default() -> Self {
    unsafe { core::mem::zeroed() }
  }
}

extern "C" {
  /// Create a modal message box.
  ///
  /// * `messageboxdata` The [SDL_MessageBoxData`] structure with title, text,
  ///   etc.
  /// * `buttonid` The pointer to which user id of hit button should be copied.
  ///
  /// **Returns:** -1 on error, otherwise 0 and buttonid contains user id of
  /// button hit or -1 if dialog was closed.
  ///
  /// **Note:** This function should be called on the thread that created the
  /// parent window, or on the main thread if the messagebox has no parent.  It
  /// will block execution of that thread until the user clicks a button or
  /// closes the messagebox.
  pub fn SDL_ShowMessageBox(
    messageboxdata: *const SDL_MessageBoxData, buttonid: *mut c_int,
  ) -> c_int;

  /// Create a simple modal message box.
  ///
  /// This function may be called at any time, even before [`SDL_Init`]. This
  /// makes it useful for reporting errors like a failure to create a renderer
  /// or OpenGL context.
  ///
  /// Note that if [`SDL_Init`] would fail because there isn't any available
  /// video target, this function is likely to fail for the same reasons. If
  /// this is a concern, check the return value from this function and fall back
  /// to writing to `stderr` if you can.
  ///
  /// * `flags`    should be one of [`SDL_MESSAGEBOX_ERROR`],
  ///   [`SDL_MESSAGEBOX_WARNING`], or [`SDL_MESSAGEBOX_INFORMATION`].
  /// * `title`    UTF-8 title text.
  /// * `message`  UTF-8 message text.
  /// * `window`   The parent window, or NULL for no parent.
  ///
  /// **Returns:** 0 on success, -1 on error.
  ///
  /// See Also: [`SDL_ShowMessageBox`]
  pub fn SDL_ShowSimpleMessageBox(
    flags: SDL_MessageBoxFlags, title: *const c_char, message: *const c_char,
    window: *mut SDL_Window,
  ) -> c_int;
}
