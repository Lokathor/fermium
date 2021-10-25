//! Interactions with the keyboard.

use crate::{
  c_char, c_int, keycode::*, rect::*, scancode::*, stdinc::*, video::*,
};

/// SDL uses this for key events.
///
/// Key events are for raw key events. For translated textual input you want the
/// "text input" events instead.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct SDL_Keysym {
  /// SDL physical key code
  pub scancode: SDL_Scancode,
  /// SDL virtual key code
  pub sym: SDL_Keycode,
  /// current key modifiers
  pub mod_: Uint16,
  /// unused padding field
  #[allow(unused)]
  pub unused: Uint32,
}

extern "C" {
  /// Get the window which currently has keyboard focus.
  pub fn SDL_GetKeyboardFocus() -> *mut SDL_Window;

  /// Get a snapshot of the current state of the keyboard.
  ///
  /// * `numkeys` if non-NULL, receives the length of the returned array.
  ///
  /// **Returns:** An array of key states. Indexes into this array are obtained
  /// by using [`SDL_Scancode`] values.
  pub fn SDL_GetKeyboardState(numkeys: *mut c_int) -> *const u8;

  /// Get the current key modifier state for the keyboard.
  pub fn SDL_GetModState() -> SDL_Keymod;

  /// Set the current key modifier state for the keyboard.
  ///
  /// This does not change the keyboard state, only the key modifier flags.
  pub fn SDL_SetModState(modstate: SDL_Keymod);

  /// Get the key code corresponding to the given scancode according to the
  /// current keyboard layout.
  ///
  /// See [`SDL_Keycode`] for details.
  ///
  /// See Also: [`SDL_GetKeyName`]
  pub fn SDL_GetKeyFromScancode(scancode: SDL_Scancode) -> SDL_Keycode;

  /// Get the scancode corresponding to the given key code according to the
  /// current keyboard layout.
  ///
  /// See [`SDL_Scancode`]
  ///
  /// See Also: [`SDL_GetScancodeName`]
  pub fn SDL_GetScancodeFromKey(key: SDL_Keycode) -> SDL_Scancode;

  /// Get a human-readable name for a scancode.
  ///
  /// **Returns** A pointer to the name for the scancode. If the scancode
  /// doesn't have a name, this function returns an empty string ("").
  pub fn SDL_GetScancodeName(scancode: SDL_Scancode) -> *const c_char;

  /// Get a scancode from a human-readable name.
  ///
  /// **Return:** the scancode, or `SDL_SCANCODE_UNKNOWN` if the name wasn't
  /// recognized.
  pub fn SDL_GetScancodeFromName(name: *const c_char) -> SDL_Scancode;

  /// Get a human-readable name for a key.
  ///
  /// **Return:** A pointer to a UTF-8 string that stays valid at least until
  /// the next call to this function. If you need it around any longer, you must
  /// copy it. If the key doesn't have a name, this function returns an empty
  /// string ("").
  pub fn SDL_GetKeyName(key: SDL_Keycode) -> *const c_char;

  /// Get a key code from a human-readable name.
  ///
  /// **Returns:** the key code, or `SDLK_UNKNOWN` if the name wasn't
  /// recognized.
  pub fn SDL_GetKeyFromName(name: *const c_char) -> SDL_Keycode;

  /// Start accepting Unicode text input events.
  ///
  /// This function will show the on-screen keyboard if supported.
  ///
  /// See Also: [`SDL_StopTextInput`], [`SDL_SetTextInputRect`],
  /// [`SDL_HasScreenKeyboardSupport`]
  pub fn SDL_StartTextInput();

  /// Return whether or not Unicode text input events are enabled.
  ///
  /// See Also: [`SDL_StartTextInput`], [`SDL_StopTextInput`]
  pub fn SDL_IsTextInputActive() -> SDL_bool;

  /// Stop receiving any text input events.
  ///
  /// This function will hide the on-screen keyboard if supported.
  ///
  /// See Also: [`SDL_StartTextInput`], [`SDL_HasScreenKeyboardSupport`]
  pub fn SDL_StopTextInput();

  /// Set the rectangle used to type Unicode text inputs.
  ///
  /// This is used as a hint for IME and on-screen keyboard placement.
  ///
  /// See Also: [`SDL_StartTextInput`]
  pub fn SDL_SetTextInputRect(rect: *mut SDL_Rect);

  /// Returns whether the platform has some screen keyboard support.
  ///
  /// **Returns:** `SDL_TRUE` if some keyboard support is available, else
  /// `SDL_FALSE`.
  ///
  /// **Note:** Not all screen keyboard functions are supported on all
  /// platforms.
  ///
  /// See Also: [`SDL_IsScreenKeyboardShown`]
  pub fn SDL_HasScreenKeyboardSupport() -> SDL_bool;

  /// Returns whether the screen keyboard is shown for given window.
  ///
  /// * `window` The window for which screen keyboard should be queried.
  ///
  /// **Returns:** `SDL_TRUE` if screen keyboard is shown, else `SDL_FALSE`.
  ///
  /// See Also: [`SDL_HasScreenKeyboardSupport`]
  pub fn SDL_IsScreenKeyboardShown(window: *mut SDL_Window) -> SDL_bool;
}
