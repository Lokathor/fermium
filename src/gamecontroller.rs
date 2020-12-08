//! The SDL "game controller" API lets you view any joystick as being
//! (approximately) an XBox 360 controller.
//!
//! This sounds silly at first, but is actually pretty useful in the common
//! case. It sorts out a lot of the button and axis stuff for you.
//!
//! In order to use these functions, [`SDL_Init`] must have been called
//! with the [`SDL_INIT_GAMECONTROLLER`] flag. This causes SDL to scan the
//! system for game controllers, and load appropriate drivers.
//!
//! If you would like to receive joystick updates while the application
//! is in the background, you should set the
//! [`SDL_HINT_JOYSTICK_ALLOW_BACKGROUND_EVENTS`] hint before calling
//! `SDL_Init`.
//!
//! See Also: [`joystick`](crate::joystick)

/**
 *  To count the number of game controllers in the system for the following:
 *  int nJoysticks = SDL_NumJoysticks();
 *  int nGameControllers = 0;
 *  for (int i = 0; i < nJoysticks; i++) {
 *      if (SDL_IsGameController(i)) {
 *          nGameControllers++;
 *      }
 *  }
 *
 *  Using the SDL_HINT_GAMECONTROLLERCONFIG hint or the
 * SDL_GameControllerAddMapping() you can add support for controllers SDL is
 * unaware of or cause an existing controller to have a different binding.
 * The format is:  guid,name,mappings
 *
 *  Where GUID is the string value from SDL_JoystickGetGUIDString(), name is
 * the human readable string for the device and mappings are controller
 * mappings to joystick ones.  Under Windows there is a reserved GUID of
 * "xinput" that covers any XInput devices.  The mapping format for joystick
 * is:      bX - a joystick button, index X
 *      hX.Y - hat X with value Y
 *      aX - axis X of the joystick
 *  Buttons can be used as a controller axis and vice versa.
 *
 *  This string shows an example of a valid mapping for a controller
 *  "03000000341a00003608000000000000,PS3
 * Controller,a:b1,b:b2,y:b3,x:b0,start:b9,guide:b12,back:b8,dpup:h0.1,
 * dpleft:h0.8,dpdown:h0.4,dpright:h0.2,leftshoulder:b4,rightshoulder:b5,
 * leftstick:b10,rightstick:b11,leftx:a0,lefty:a1,rightx:a2,righty:a3,
 * lefttrigger:b6,righttrigger:b7",
 *
 */
pub use crate::{error::*, joystick::*, rwops::*, stdinc::*};

/// An SDL game controller is an opaque structure.
#[repr(transparent)]
pub struct SDL_GameController(c_void);

/// A game controller's type, sorted by console.
///
/// See `SDL_CONTROLLER_TYPE_*`
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_GameControllerType(pub i32);
#[allow(missing_docs)]
pub const SDL_CONTROLLER_TYPE_UNKNOWN: SDL_GameControllerType = SDL_GameControllerType(0);
#[allow(missing_docs)]
pub const SDL_CONTROLLER_TYPE_XBOX360: SDL_GameControllerType = SDL_GameControllerType(1);
#[allow(missing_docs)]
pub const SDL_CONTROLLER_TYPE_XBOXONE: SDL_GameControllerType = SDL_GameControllerType(2);
#[allow(missing_docs)]
pub const SDL_CONTROLLER_TYPE_PS3: SDL_GameControllerType = SDL_GameControllerType(3);
#[allow(missing_docs)]
pub const SDL_CONTROLLER_TYPE_PS4: SDL_GameControllerType = SDL_GameControllerType(4);
#[allow(missing_docs)]
pub const SDL_CONTROLLER_TYPE_NINTENDO_SWITCH_PRO: SDL_GameControllerType = SDL_GameControllerType(5);

/// The type of a binding between the underlying joystick and its controller
/// abstraction.
///
/// See `SDL_CONTROLLER_BINDTYPE_*`
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_GameControllerBindType(pub i32);
#[allow(missing_docs)]
pub const SDL_CONTROLLER_BINDTYPE_NONE: SDL_GameControllerBindType = SDL_GameControllerBindType(0);
#[allow(missing_docs)]
pub const SDL_CONTROLLER_BINDTYPE_BUTTON: SDL_GameControllerBindType = SDL_GameControllerBindType(1);
#[allow(missing_docs)]
pub const SDL_CONTROLLER_BINDTYPE_AXIS: SDL_GameControllerBindType = SDL_GameControllerBindType(2);
#[allow(missing_docs)]
pub const SDL_CONTROLLER_BINDTYPE_HAT: SDL_GameControllerBindType = SDL_GameControllerBindType(3);

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
#[allow(missing_docs)]
pub struct SDL_GameControllerButtonBind_hat_data {
  pub hat: c_int,
  pub hat_mask: c_int,
}

#[derive(Clone, Copy)]
#[repr(C, align(4))] // TODO: investigate if this will be align4 without the repr
#[allow(missing_docs)]
pub union SDL_GameControllerButtonBind_value_data {
  pub button: c_int,
  pub axis: c_int,
  pub hat: SDL_GameControllerButtonBind_hat_data,
}

/// The SDL joystick layer binding for this controller button/axis mapping.
#[derive(Clone, Copy)]
#[repr(C)]
#[allow(missing_docs)]
pub struct SDL_GameControllerButtonBind {
  pub bindType: SDL_GameControllerBindType,
  pub value: SDL_GameControllerButtonBind_value_data,
}

/// The list of axes available from a controller
///
/// Thumbstick axis values range from [`SDL_JOYSTICK_AXIS_MIN`] to
/// [`SDL_JOYSTICK_AXIS_MAX`], and are centered within ~8000 of zero, though
/// advanced UI will allow users to set or auto-detect the dead zone, which
/// varies between controllers.
///
/// Trigger axis values range from 0 to [`SDL_JOYSTICK_AXIS_MAX`].
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_GameControllerAxis(pub i32);

/// The general "invalid" entry.
///
/// Used as an error result with some functions.
pub const SDL_CONTROLLER_AXIS_INVALID: SDL_GameControllerAxis = SDL_GameControllerAxis(-1);
/// Left-stick X
pub const SDL_CONTROLLER_AXIS_LEFTX: SDL_GameControllerAxis = SDL_GameControllerAxis(0);
/// Left-stick Y
pub const SDL_CONTROLLER_AXIS_LEFTY: SDL_GameControllerAxis = SDL_GameControllerAxis(1);
/// Right-stick X
pub const SDL_CONTROLLER_AXIS_RIGHTX: SDL_GameControllerAxis = SDL_GameControllerAxis(2);
/// Right-stick Y
pub const SDL_CONTROLLER_AXIS_RIGHTY: SDL_GameControllerAxis = SDL_GameControllerAxis(3);
/// Left trigger
pub const SDL_CONTROLLER_AXIS_TRIGGERLEFT: SDL_GameControllerAxis = SDL_GameControllerAxis(4);
/// Right trigger
pub const SDL_CONTROLLER_AXIS_TRIGGERRIGHT: SDL_GameControllerAxis = SDL_GameControllerAxis(5);
/// The number of valid axis possibilities.
pub const SDL_CONTROLLER_AXIS_MAX: usize = 6;

/// The list of buttons available from a controller.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_GameControllerButton(pub i32);

/// The general "invalid" entry.
///
/// Used as an error result with some functions.
pub const SDL_CONTROLLER_BUTTON_INVALID: SDL_GameControllerButton = SDL_GameControllerButton(-1);
/// Bottom face button.
pub const SDL_CONTROLLER_BUTTON_A: SDL_GameControllerButton = SDL_GameControllerButton(0);
/// Right face button.
pub const SDL_CONTROLLER_BUTTON_B: SDL_GameControllerButton = SDL_GameControllerButton(1);
/// Left face button.
pub const SDL_CONTROLLER_BUTTON_X: SDL_GameControllerButton = SDL_GameControllerButton(2);
/// Top face button.
pub const SDL_CONTROLLER_BUTTON_Y: SDL_GameControllerButton = SDL_GameControllerButton(3);
/// Also known as "select" on many controllers.
pub const SDL_CONTROLLER_BUTTON_BACK: SDL_GameControllerButton = SDL_GameControllerButton(4);
/// The logo button. Not present on many controllers.
pub const SDL_CONTROLLER_BUTTON_GUIDE: SDL_GameControllerButton = SDL_GameControllerButton(5);
/// Start button.
pub const SDL_CONTROLLER_BUTTON_START: SDL_GameControllerButton = SDL_GameControllerButton(6);
/// Pressing in the left stick.
pub const SDL_CONTROLLER_BUTTON_LEFTSTICK: SDL_GameControllerButton = SDL_GameControllerButton(7);
/// Pressing in the right stick.
pub const SDL_CONTROLLER_BUTTON_RIGHTSTICK: SDL_GameControllerButton = SDL_GameControllerButton(8);
/// Upper left shoulder button.
pub const SDL_CONTROLLER_BUTTON_LEFTSHOULDER: SDL_GameControllerButton = SDL_GameControllerButton(9);
/// Upper right shoulder button.
pub const SDL_CONTROLLER_BUTTON_RIGHTSHOULDER: SDL_GameControllerButton = SDL_GameControllerButton(10);
/// Arrow-pad up.
pub const SDL_CONTROLLER_BUTTON_DPAD_UP: SDL_GameControllerButton = SDL_GameControllerButton(11);
/// Arrow-pad down.
pub const SDL_CONTROLLER_BUTTON_DPAD_DOWN: SDL_GameControllerButton = SDL_GameControllerButton(12);
/// Arrow-pad left.
pub const SDL_CONTROLLER_BUTTON_DPAD_LEFT: SDL_GameControllerButton = SDL_GameControllerButton(13);
/// Arrow-pad right.
pub const SDL_CONTROLLER_BUTTON_DPAD_RIGHT: SDL_GameControllerButton = SDL_GameControllerButton(14);
/// The number of valid controller button possibilities.
pub const SDL_CONTROLLER_BUTTON_MAX: usize = 15;

/// Load a set of mappings from a file.
///
/// Results are filtered by the current [`SDL_GetPlatform`].
///
/// Convenience function that calls [`SDL_GameControllerAddMappingsFromRW`].
pub unsafe fn SDL_GameControllerAddMappingsFromFile(file: *const c_char) -> c_int {
  SDL_GameControllerAddMappingsFromRW(SDL_RWFromFile(file, b"rb\0".as_ptr().cast()), 1)
}

extern "C" {
  /// Load a set of mappings from a seekable SDL data stream (memory or file).
  ///
  /// This is filtered by the current [`SDL_GetPlatform`].
  ///
  /// A community sourced database of controllers is available [on github](https://raw.github.com/gabomdq/SDL_GameControllerDB/master/gamecontrollerdb.txt)
  ///
  /// If `freerw` is non-zero, the stream will be closed after being read.
  ///
  /// **Returns:** number of mappings added, -1 on error
  pub fn SDL_GameControllerAddMappingsFromRW(rw: *mut SDL_RWops, freerw: c_int) -> c_int;

  /// Add or update an existing mapping configuration
  ///
  /// **Returns:** 1 if mapping is added, 0 if updated, -1 on error
  pub fn SDL_GameControllerAddMapping(mappingString: *const c_char) -> c_int;

  /// Get the number of mappings installed
  ///
  /// **Return:** the number of mappings
  pub fn SDL_GameControllerNumMappings() -> c_int;

  /// Get the mapping at a particular index.
  ///
  /// **Returns:** the mapping string.
  ///
  /// **Must be freed with [`SDL_free`].**
  ///
  /// Returns NULL if the index is out of range.
  pub fn SDL_GameControllerMappingForIndex(mapping_index: c_int) -> *mut c_char;

  /// Get a mapping string for a GUID
  ///
  /// **Returns:** the mapping string.
  ///
  /// **Must be freed with [`SDL_free`].**
  ///
  /// Returns NULL if no mapping is available
  pub fn SDL_GameControllerMappingForGUID(guid: SDL_JoystickGUID) -> *mut c_char;

  /// Get a mapping string for an open GameController
  ///
  /// **Returns:** the mapping string.
  ///
  /// **Must be freed with [`SDL_free`].**
  ///
  /// Returns NULL if no mapping is available.
  pub fn SDL_GameControllerMapping(gamecontroller: *mut SDL_GameController) -> *mut c_char;

  /// Is the joystick on this index supported by the game controller interface?
  pub fn SDL_IsGameController(joystick_index: c_int) -> SDL_bool;

  /// Get the implementation dependent name of a game controller.
  ///
  /// This can be called before any controllers are opened.
  ///
  /// If no name can be found, this function returns NULL.
  pub fn SDL_GameControllerNameForIndex(joystick_index: c_int) -> *const c_char;

  /// Get the type of a game controller.
  ///
  /// This can be called before any controllers are opened.
  pub fn SDL_GameControllerTypeForIndex(joystick_index: c_int) -> SDL_GameControllerType;

  /// Get the mapping of a game controller.
  ///
  /// This can be called before any controllers are opened.
  ///
  /// **Returns:** the mapping string.
  ///
  /// **Must be freed with [`SDL_free`].**
  ///
  /// Returns NULL if no mapping is available.
  pub fn SDL_GameControllerMappingForDeviceIndex(joystick_index: c_int) -> *mut c_char;

  /// Open a game controller for use.
  ///
  /// The index passed as an argument refers to the N'th game controller on the
  /// system. This index is not the value which will identify this controller
  /// in future controller events. The joystick's instance id
  /// ([`SDL_JoystickID`]) will be used there instead.
  ///
  /// **Returns:** A controller identifier, or NULL if an error occurred.
  pub fn SDL_GameControllerOpen(joystick_index: c_int) -> *mut SDL_GameController;

  /// Return the [`SDL_GameController`] associated with an instance id.
  pub fn SDL_GameControllerFromInstanceID(joyid: SDL_JoystickID) -> *mut SDL_GameController;

  /// Return the [`SDL_GameController`] associated with a player index.
  pub fn SDL_GameControllerFromPlayerIndex(player_index: c_int) -> *mut SDL_GameController;

  /// Return the name for this currently opened controller.
  pub fn SDL_GameControllerName(gamecontroller: *mut SDL_GameController) -> *const c_char;

  /// Return the type of this currently opened controller.
  pub fn SDL_GameControllerGetType(gamecontroller: *mut SDL_GameController) -> SDL_GameControllerType;

  /// Get the player index of an opened game controller, or -1 if it's not
  /// available
  ///
  /// For XInput controllers this returns the XInput user index.
  pub fn SDL_GameControllerGetPlayerIndex(gamecontroller: *mut SDL_GameController) -> c_int;

  /// Set the player index of an opened game controller.
  pub fn SDL_GameControllerSetPlayerIndex(gamecontroller: *mut SDL_GameController, player_index: c_int);

  /// Get the USB vendor ID of an opened controller, if available.
  ///
  /// If the vendor ID isn't available this function returns 0.
  pub fn SDL_GameControllerGetVendor(gamecontroller: *mut SDL_GameController) -> Uint16;

  /// Get the USB product ID of an opened controller, if available.
  ///
  /// If the product ID isn't available this function returns 0.
  pub fn SDL_GameControllerGetProduct(gamecontroller: *mut SDL_GameController) -> Uint16;

  /// Get the product version of an opened controller, if available.
  ///
  /// If the product version isn't available this function returns 0.
  pub fn SDL_GameControllerGetProductVersion(gamecontroller: *mut SDL_GameController) -> Uint16;

  /// Returns `SDL_TRUE` if the controller has been opened and currently
  /// connected, or `SDL_FALSE` if it has not.
  pub fn SDL_GameControllerGetAttached(gamecontroller: *mut SDL_GameController) -> SDL_bool;

  /// Get the underlying joystick object used by a controller.
  pub fn SDL_GameControllerGetJoystick(gamecontroller: *mut SDL_GameController) -> *mut SDL_Joystick;

  /// Enable/disable controller event polling.
  ///
  /// If controller events are disabled, you must call
  /// [`SDL_GameControllerUpdate`] yourself and check the state of the
  /// controller when you want controller information.
  ///
  /// The state can be one of [`SDL_QUERY`], [`SDL_ENABLE`], or [`SDL_IGNORE`].
  pub fn SDL_GameControllerEventState(state: c_int) -> c_int;

  /// Update the current state of the open game controllers.
  ///
  /// This is called automatically by the event loop if any game controller
  /// events are enabled.
  pub fn SDL_GameControllerUpdate();

  /// Turn this string into a axis mapping.
  pub fn SDL_GameControllerGetAxisFromString(pchString: *const c_char) -> SDL_GameControllerAxis;

  /// Turn this axis enum into a string mapping.
  pub fn SDL_GameControllerGetStringForAxis(axis: SDL_GameControllerAxis) -> *const c_char;

  /// Get the SDL joystick layer binding for this controller button mapping.
  pub fn SDL_GameControllerGetBindForAxis(gamecontroller: *mut SDL_GameController, axis: SDL_GameControllerAxis) -> SDL_GameControllerButtonBind;

  /// Get the current state of an axis control on a game controller.
  ///
  /// The state is a value ranging from -32768 to 32767 (except for the
  /// triggers, which range from 0 to 32767).
  ///
  /// The axis indices start at index 0.
  pub fn SDL_GameControllerGetAxis(gamecontroller: *mut SDL_GameController, axis: SDL_GameControllerAxis) -> Sint16;

  /// Turn this string into a button mapping.
  pub fn SDL_GameControllerGetButtonFromString(pchString: *const c_char) -> SDL_GameControllerButton;

  /// Turn this button enum into a string mapping.
  pub fn SDL_GameControllerGetStringForButton(button: SDL_GameControllerButton) -> *const c_char;

  /// Get the SDL joystick layer binding for this controller button mapping.
  pub fn SDL_GameControllerGetBindForButton(gamecontroller: *mut SDL_GameController, button: SDL_GameControllerButton) -> SDL_GameControllerButtonBind;

  /// Get the current state of a button on a game controller.
  ///
  /// The button indices start at index 0.
  pub fn SDL_GameControllerGetButton(gamecontroller: *mut SDL_GameController, button: SDL_GameControllerButton) -> Uint8;

  /// Trigger a rumble effect.
  ///
  /// Each call to this function cancels any previous rumble effect, and calling
  /// it with 0 intensity stops any rumbling.
  ///
  /// * `gamecontroller` The controller to vibrate
  /// * `low_frequency_rumble` The intensity of the low frequency (left) rumble
  ///   motor, from 0 to 0xFFFF
  /// * `high_frequency_rumble` The intensity of the high frequency (right)
  ///   rumble motor, from 0 to 0xFFFF
  /// * `duration_ms` The duration of the rumble effect, in milliseconds
  ///
  /// **Return:** 0, or -1 if rumble isn't supported on this joystick
  pub fn SDL_GameControllerRumble(gamecontroller: *mut SDL_GameController, low_frequency_rumble: Uint16, high_frequency_rumble: Uint16, duration_ms: Uint32) -> c_int;

  /// Close a controller previously opened with [`SDL_GameControllerOpen`].
  pub fn SDL_GameControllerClose(gamecontroller: *mut SDL_GameController);
}
