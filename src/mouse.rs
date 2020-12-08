//! Mouse control and interaction.

pub use crate::{error::*, stdinc::*, video::*};

/// An opaque cursor value.
///
/// The exact layout is implementation dependent.
#[repr(transparent)]
pub struct SDL_Cursor(c_void);

/// Used with [`SDL_CreateSystemCursor`]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_SystemCursor(pub i32);

/// Arrow
pub const SDL_SYSTEM_CURSOR_ARROW: SDL_SystemCursor = SDL_SystemCursor(0);
/// I-beam
pub const SDL_SYSTEM_CURSOR_IBEAM: SDL_SystemCursor = SDL_SystemCursor(1);
/// Wait
pub const SDL_SYSTEM_CURSOR_WAIT: SDL_SystemCursor = SDL_SystemCursor(2);
/// Crosshair
pub const SDL_SYSTEM_CURSOR_CROSSHAIR: SDL_SystemCursor = SDL_SystemCursor(3);
/// Small wait cursor (or Wait if not available)
pub const SDL_SYSTEM_CURSOR_WAITARROW: SDL_SystemCursor = SDL_SystemCursor(4);
/// Double arrow pointing northwest and southeast
pub const SDL_SYSTEM_CURSOR_SIZENWSE: SDL_SystemCursor = SDL_SystemCursor(5);
/// Double arrow pointing northeast and southwest
pub const SDL_SYSTEM_CURSOR_SIZENESW: SDL_SystemCursor = SDL_SystemCursor(6);
/// Double arrow pointing west and east
pub const SDL_SYSTEM_CURSOR_SIZEWE: SDL_SystemCursor = SDL_SystemCursor(7);
/// Double arrow pointing north and south
pub const SDL_SYSTEM_CURSOR_SIZENS: SDL_SystemCursor = SDL_SystemCursor(8);
/// Four pointed arrow pointing north, south, east, and west
pub const SDL_SYSTEM_CURSOR_SIZEALL: SDL_SystemCursor = SDL_SystemCursor(9);
/// Slashed circle or crossbones
pub const SDL_SYSTEM_CURSOR_NO: SDL_SystemCursor = SDL_SystemCursor(10);
/// Hand
pub const SDL_SYSTEM_CURSOR_HAND: SDL_SystemCursor = SDL_SystemCursor(11);

/// Used for array size purposes and such.
pub const SDL_NUM_SYSTEM_CURSORS: usize = 12;

/// Scroll direction types for the Scroll event
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_MouseWheelDirection(pub i32);

/// The scroll direction is normal
pub const SDL_MOUSEWHEEL_NORMAL: SDL_MouseWheelDirection =
  SDL_MouseWheelDirection(0);
/// The scroll direction is flipped / natural
pub const SDL_MOUSEWHEEL_FLIPPED: SDL_MouseWheelDirection =
  SDL_MouseWheelDirection(1);

extern "C" {
  /// Get the window which currently has mouse focus.
  pub fn SDL_GetMouseFocus() -> *mut SDL_Window;

  /// Retrieve the current state of the mouse.
  ///
  /// The current button state is returned as a button bitmask, which can be
  /// tested using the SDL_BUTTON(X) macros. `x` and `y` are set to the mouse
  /// cursor position relative to the focus window for the currently selected
  /// mouse.
  ///
  /// You can safely pass NULL for either `x` or `y`.
  pub fn SDL_GetMouseState(x: *mut c_int, y: *mut c_int) -> u32;

  /// Get the current state of the mouse, in relation to the desktop.
  ///
  /// This works just like [`SDL_GetMouseState`], but the coordinates will be
  /// reported relative to the top-left of the desktop. This can be useful if
  /// you need to track the mouse outside of a specific window and
  /// [`SDL_CaptureMouse`] doesn't fit your needs. For example, it could be
  /// useful if you need to track the mouse while dragging a window, where
  /// coordinates relative to a window might not be in sync at all times.
  ///
  /// **Note:** [`SDL_GetMouseState`] returns the mouse position as SDL
  /// understands it from the last pump of the event queue. This function,
  /// however, queries the OS for the current mouse position, and as such, might
  /// be a slightly less efficient function. Unless you know what you're doing
  /// and have a good reason to use this function, you probably want
  /// [`SDL_GetMouseState`] instead.
  ///
  /// * `x` Returns the current X coord, relative to the desktop. Can be NULL.
  /// * `y` Returns the current Y coord, relative to the desktop. Can be NULL.
  ///
  /// **Return:** The current button state as a bitmask, which can be tested
  /// using the SDL_BUTTON(X) macros.
  pub fn SDL_GetGlobalMouseState(x: *mut c_int, y: *mut c_int) -> u32;

  /// Retrieve the relative state of the mouse.
  ///
  /// The current button state is returned as a button bitmask, which can be
  /// tested using the SDL_BUTTON(X) macros, and x and y are set to the mouse
  /// deltas since the last call to `SDL_GetRelativeMouseState`.
  pub fn SDL_GetRelativeMouseState(x: *mut c_int, y: *mut c_int) -> u32;

  /// Moves the mouse to the given position within the window.
  ///
  /// * `window` The window to move the mouse into, or NULL for the current
  ///   mouse focus
  /// * `x` The x coordinate within the window
  /// * `y` The y coordinate within the window
  ///
  /// **Note:** This function generates a mouse motion event
  pub fn SDL_WarpMouseInWindow(window: *mut SDL_Window, x: c_int, y: c_int);

  /// Moves the mouse to the given position in global screen space.
  ///
  /// * `x` The x coordinate
  /// * `y` The y coordinate
  ///
  /// **Returns:** 0 on success, -1 on error (usually: unsupported by a
  /// platform).
  ///
  /// **Note:** This function generates a mouse motion event
  pub fn SDL_WarpMouseGlobal(x: c_int, y: c_int) -> c_int;

  /// Set relative mouse mode.
  ///
  /// * `enabled` Whether or not to enable relative mode
  ///
  /// **Returns:** 0 on success, or -1 if relative mode is not supported.
  ///
  /// While the mouse is in relative mode, the cursor is hidden, and the driver
  /// will try to report continuous motion in the current window. Only relative
  /// motion events will be delivered, the mouse position will not change.
  ///
  /// In other words, this is what you'd use for an "FPS" style interface.
  ///
  /// **Note:** This function will flush any pending mouse motion.
  ///
  /// See Also: [`SDL_GetRelativeMouseMode`]
  pub fn SDL_SetRelativeMouseMode(enabled: SDL_bool) -> c_int;

  /// Capture the mouse, to track input outside an SDL window.
  ///
  /// * `enabled` Whether or not to enable capturing
  ///
  /// Capturing enables your app to obtain mouse events globally, instead of
  /// just within your window. Not all video targets support this function. When
  /// capturing is enabled, the current window will get all mouse events, but
  /// unlike relative mode, no change is made to the cursor and it is not
  /// restrained to your window.
  ///
  /// This function may also deny mouse input to other windows--both those in
  /// your application and others on the system--so you should use this function
  /// sparingly, and in small bursts. For example, you might want to track the
  /// mouse while the user is dragging something, until the user releases a
  /// mouse button. It is not recommended that you capture the mouse for long
  /// periods of time, such as the entire time your app is running.
  ///
  /// While captured, mouse events still report coordinates relative to the
  /// current (foreground) window, but those coordinates may be outside the
  /// bounds of the window (including negative values). Capturing is only
  /// allowed for the foreground window. If the window loses focus while
  /// capturing, the capture will be disabled automatically.
  ///
  /// While capturing is enabled, the current window will have the
  /// `SDL_WINDOW_MOUSE_CAPTURE` flag set.
  ///
  /// **Return:** 0 on success, or -1 if not supported.
  pub fn SDL_CaptureMouse(enabled: SDL_bool) -> c_int;

  /// Query whether relative mouse mode is enabled.
  ///
  /// See Also: [`SDL_SetRelativeMouseMode`]
  pub fn SDL_GetRelativeMouseMode() -> SDL_bool;

  /// Create a cursor, using the specified bitmap data and mask (in MSB format).
  ///
  /// The cursor width must be a multiple of 8 bits.
  ///
  /// The cursor is created in black and white according to the following:
  ///
  /// | data | mask | resulting pixel on screen |
  /// |:-:|:-:|:-|
  /// |  0   |  1   | White |
  /// |  1   |  1   | Black |
  /// |  0   |  0   | Transparent |
  /// |  1   |  0   | Inverted color if possible, black if not. |
  ///
  /// See Also: [`SDL_FreeCursor`]
  pub fn SDL_CreateCursor(
    data: *const Uint8, mask: *const Uint8, w: c_int, h: c_int, hot_x: c_int,
    hot_y: c_int,
  ) -> *mut SDL_Cursor;

  /// Create a color cursor.
  ///
  /// See Also: [`SDL_FreeCursor`]
  pub fn SDL_CreateColorCursor(
    surface: *mut SDL_Surface, hot_x: c_int, hot_y: c_int,
  ) -> *mut SDL_Cursor;

  /// Create a system cursor.
  ///
  /// See Also: [`SDL_FreeCursor`]
  pub fn SDL_CreateSystemCursor(id: SDL_SystemCursor) -> *mut SDL_Cursor;

  /// Set the active cursor.
  pub fn SDL_SetCursor(cursor: *mut SDL_Cursor);

  /// Return the active cursor.
  pub fn SDL_GetCursor() -> *mut SDL_Cursor;

  /// Return the default cursor.
  pub fn SDL_GetDefaultCursor() -> *mut SDL_Cursor;

  /// Frees a cursor created with [`SDL_CreateCursor`] or similar functions.
  ///
  /// See Also: [`SDL_CreateCursor`], [`SDL_CreateColorCursor`],
  /// [`SDL_CreateSystemCursor`]
  pub fn SDL_FreeCursor(cursor: *mut SDL_Cursor);

  /// Toggle whether or not the cursor is shown.
  ///
  /// * `toggle` 1 to show the cursor, 0 to hide it, -1 to query the current
  ///   state.
  ///
  /// **Returns:** 1 if the cursor is shown, or 0 if the cursor is hidden.
  pub fn SDL_ShowCursor(toggle: c_int) -> c_int;
}

/// Used as a mask when testing buttons in `buttonstate`.
///
/// * Button 1:  Left mouse button
/// * Button 2:  Middle mouse button
/// * Button 3:  Right mouse button
pub const fn SDL_BUTTON(x: u32) -> u32 {
  1 << (x - 1)
}

/// Individual value for the left button.
pub const SDL_BUTTON_LEFT: u32 = 1;
/// Individual value for the middle button (mouse wheel).
pub const SDL_BUTTON_MIDDLE: u32 = 2;
/// Individual value for the right button.
pub const SDL_BUTTON_RIGHT: u32 = 3;
/// Individual value for the extra button 1.
pub const SDL_BUTTON_X1: u32 = 4;
/// Individual value for the extra button 2.
pub const SDL_BUTTON_X2: u32 = 5;

/// Mask for the left button.
pub const SDL_BUTTON_LMASK: u32 = SDL_BUTTON(SDL_BUTTON_LEFT);
/// Mask for the middle button (mouse wheel).
pub const SDL_BUTTON_MMASK: u32 = SDL_BUTTON(SDL_BUTTON_MIDDLE);
/// Mask for the right button.
pub const SDL_BUTTON_RMASK: u32 = SDL_BUTTON(SDL_BUTTON_RIGHT);
/// Mask for the extra button 1.
pub const SDL_BUTTON_X1MASK: u32 = SDL_BUTTON(SDL_BUTTON_X1);
/// Mask for the extra button 2.
pub const SDL_BUTTON_X2MASK: u32 = SDL_BUTTON(SDL_BUTTON_X2);
