//! Module for SDL joystick event handling.
//!
//! The term `device_index` identifies currently plugged in joystick devices
//! between 0 and SDL_NumJoysticks(), with the exact joystick behind a
//! device_index changing as joysticks are plugged and unplugged.
//!
//! The term `instance_id` is the current instantiation of a joystick device in
//! the system, if the joystick is removed and then re-inserted then it will get
//! a new instance_id, instance_id's are monotonically increasing identifiers of
//! a joystick plugged in.
//!
//! The term `JoystickGUID` is a stable 128-bit identifier for a joystick device
//! that does not change over time, it identifies class of the device (a X360
//! wired controller for example). This identifier is platform dependent.
//!
//! In order to use these functions, [`SDL_Init`] must have been called
//! with the [`SDL_INIT_JOYSTICK`] flag. This causes SDL to scan the system
//! for joysticks, and load appropriate drivers.
//!
//! If you would like to receive joystick updates while the application
//! is in the background, you should set the
//! [`SDL_HINT_JOYSTICK_ALLOW_BACKGROUND_EVENTS`] hint before calling
//! `SDL_Init`.
//!
//! See Also: [`gamecontroller`](crate::gamecontroller)

pub use crate::{c_void, error::*, stdinc::*};

/// The `SDL_Joystick` type is an opaque structure.
#[derive(Debug)]
#[repr(transparent)]
pub struct SDL_Joystick(c_void);

/// A structure that encodes the stable unique id for a joystick device.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
#[allow(missing_docs)]
pub struct SDL_JoystickGUID {
  pub data: [u8; 16],
}

/// This is a unique ID for a joystick for the time it is connected to the
/// system.
///
/// It is never reused for the lifetime of the application. If the joystick is
/// disconnected and reconnected, it will get a new ID.
///
/// The ID value starts at 0 and increments from there.
///
/// The value -1 is an invalid ID.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_JoystickID(pub i32);

/// The general physical category of a joystick device.
///
/// See `SDL_JOYSTICK_TYPE_*`
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_JoystickType(pub i32);

#[allow(missing_docs)]
pub const SDL_JOYSTICK_TYPE_UNKNOWN: SDL_JoystickType = SDL_JoystickType(0);
#[allow(missing_docs)]
pub const SDL_JOYSTICK_TYPE_GAMECONTROLLER: SDL_JoystickType =
  SDL_JoystickType(1);
#[allow(missing_docs)]
pub const SDL_JOYSTICK_TYPE_WHEEL: SDL_JoystickType = SDL_JoystickType(2);
#[allow(missing_docs)]
pub const SDL_JOYSTICK_TYPE_ARCADE_STICK: SDL_JoystickType =
  SDL_JoystickType(3);
#[allow(missing_docs)]
pub const SDL_JOYSTICK_TYPE_FLIGHT_STICK: SDL_JoystickType =
  SDL_JoystickType(4);
#[allow(missing_docs)]
pub const SDL_JOYSTICK_TYPE_DANCE_PAD: SDL_JoystickType = SDL_JoystickType(5);
#[allow(missing_docs)]
pub const SDL_JOYSTICK_TYPE_GUITAR: SDL_JoystickType = SDL_JoystickType(6);
#[allow(missing_docs)]
pub const SDL_JOYSTICK_TYPE_DRUM_KIT: SDL_JoystickType = SDL_JoystickType(7);
#[allow(missing_docs)]
pub const SDL_JOYSTICK_TYPE_ARCADE_PAD: SDL_JoystickType = SDL_JoystickType(8);
#[allow(missing_docs)]
pub const SDL_JOYSTICK_TYPE_THROTTLE: SDL_JoystickType = SDL_JoystickType(9);

/// The power level of a joystick.
///
/// See `SDL_JOYSTICK_POWER_*`
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_JoystickPowerLevel(pub i32);

/// The power level is unknown.
pub const SDL_JOYSTICK_POWER_UNKNOWN: SDL_JoystickPowerLevel =
  SDL_JoystickPowerLevel(-1);
/// <= 5%
pub const SDL_JOYSTICK_POWER_EMPTY: SDL_JoystickPowerLevel =
  SDL_JoystickPowerLevel(0);
/// <= 20%
pub const SDL_JOYSTICK_POWER_LOW: SDL_JoystickPowerLevel =
  SDL_JoystickPowerLevel(1);
/// <= 70%
pub const SDL_JOYSTICK_POWER_MEDIUM: SDL_JoystickPowerLevel =
  SDL_JoystickPowerLevel(2);
/// <= 100%
pub const SDL_JOYSTICK_POWER_FULL: SDL_JoystickPowerLevel =
  SDL_JoystickPowerLevel(3);
/// This joystick is wired to the rest of the system.
pub const SDL_JOYSTICK_POWER_WIRED: SDL_JoystickPowerLevel =
  SDL_JoystickPowerLevel(4);
/// Honestly, this particular constant is probably pointless.
pub const SDL_JOYSTICK_POWER_MAX: SDL_JoystickPowerLevel =
  SDL_JoystickPowerLevel(5);

/// A joystick axis uses the `i16` range, so this is just `i16::MAX`
pub const SDL_JOYSTICK_AXIS_MAX: i16 = 32767;

/// A joystick axis uses the `i16` range, so this is just `i16::MIN`
pub const SDL_JOYSTICK_AXIS_MIN: i16 = -32768;

#[allow(missing_docs)]
pub const SDL_HAT_CENTERED: u8 = 0x00;
#[allow(missing_docs)]
pub const SDL_HAT_UP: u8 = 0x01;
#[allow(missing_docs)]
pub const SDL_HAT_RIGHT: u8 = 0x02;
#[allow(missing_docs)]
pub const SDL_HAT_DOWN: u8 = 0x04;
#[allow(missing_docs)]
pub const SDL_HAT_LEFT: u8 = 0x08;
#[allow(missing_docs)]
pub const SDL_HAT_RIGHTUP: u8 = SDL_HAT_RIGHT | SDL_HAT_UP;
#[allow(missing_docs)]
pub const SDL_HAT_RIGHTDOWN: u8 = SDL_HAT_RIGHT | SDL_HAT_DOWN;
#[allow(missing_docs)]
pub const SDL_HAT_LEFTUP: u8 = SDL_HAT_LEFT | SDL_HAT_UP;
#[allow(missing_docs)]
pub const SDL_HAT_LEFTDOWN: u8 = SDL_HAT_LEFT | SDL_HAT_DOWN;

extern "C" {
  /// Locking for multi-threaded access to the joystick API
  ///
  /// If you are using the joystick API or handling events from multiple threads
  /// you should use these locking functions to protect access to the joysticks.
  ///
  /// In particular, you are guaranteed that the joystick list won't change, so
  /// the API functions that take a joystick index will be valid, and joystick
  /// and game controller events will not be delivered.
  pub fn SDL_LockJoysticks();

  /// See [`SDL_LockJoysticks`]
  pub fn SDL_UnlockJoysticks();

  /// Count the number of joysticks attached to the system right now
  pub fn SDL_NumJoysticks() -> c_int;

  /// Get the implementation dependent name of a joystick.
  ///
  /// This can be called before any joysticks are opened. If no name can be
  /// found, this function returns NULL.
  ///
  /// This is borrowed memory, do no free it.
  pub fn SDL_JoystickNameForIndex(device_index: c_int) -> *const c_char;

  /// Get the player index of a joystick, or -1 if it's not available.
  ///
  /// This can be called before any joysticks are opened.
  pub fn SDL_JoystickGetDevicePlayerIndex(device_index: c_int) -> c_int;

  /// Return the GUID for the joystick at this index.
  ///
  /// This can be called before any joysticks are opened.
  pub fn SDL_JoystickGetDeviceGUID(device_index: c_int) -> SDL_JoystickGUID;

  /// Get the USB vendor ID of a joystick, if available.
  ///
  /// This can be called before any joysticks are opened.
  ///
  /// If the vendor ID isn't available this function returns 0.
  pub fn SDL_JoystickGetDeviceVendor(device_index: c_int) -> Uint16;

  /// Get the USB product ID of a joystick, if available.
  ///
  /// This can be called before any joysticks are opened.
  ///
  /// If the product ID isn't available this function returns 0.
  pub fn SDL_JoystickGetDeviceProduct(device_index: c_int) -> Uint16;

  /// Get the product version of a joystick, if available.
  ///
  /// This can be called before any joysticks are opened.
  ///
  /// If the product version isn't available this function returns 0.
  pub fn SDL_JoystickGetDeviceProductVersion(device_index: c_int) -> Uint16;

  /// Get the type of a joystick, if available.
  ///
  /// This can be called before any joysticks are opened.
  pub fn SDL_JoystickGetDeviceType(device_index: c_int) -> SDL_JoystickType;

  /// Get the instance ID of a joystick.
  ///
  /// This can be called before any joysticks are opened.
  ///
  /// If the index is out of range, this function will return -1.
  pub fn SDL_JoystickGetDeviceInstanceID(device_index: c_int)
    -> SDL_JoystickID;

  /// Open a joystick for use.
  ///
  /// The index passed as an argument refers to the N'th joystick on the system.
  /// This index is not the value which will identify this joystick in future
  /// joystick events. The joystick's instance id ([`SDL_JoystickID`]) will be
  /// used there instead.
  ///
  /// **Return:** A joystick identifier, or NULL if an error occurred.
  pub fn SDL_JoystickOpen(device_index: c_int) -> *mut SDL_Joystick;

  /// Return the SDL_Joystick associated with an instance id.
  pub fn SDL_JoystickFromInstanceID(
    instance_id: SDL_JoystickID,
  ) -> *mut SDL_Joystick;

  /// Return the SDL_Joystick associated with a player index.
  pub fn SDL_JoystickFromPlayerIndex(player_index: c_int) -> *mut SDL_Joystick;

  /// Return the name for this currently opened joystick.
  ///
  /// If no name can be found, this function returns NULL.
  pub fn SDL_JoystickName(joystick: *mut SDL_Joystick) -> *const c_char;

  /// Get the player index of an opened joystick, or -1 if it's not available.
  ///
  /// For XInput controllers this returns the XInput user index.
  pub fn SDL_JoystickGetPlayerIndex(joystick: *mut SDL_Joystick) -> c_int;

  /// Set the player index of an opened joystick.
  pub fn SDL_JoystickSetPlayerIndex(
    joystick: *mut SDL_Joystick, player_index: c_int,
  );

  /// Return the GUID for this opened joystick.
  pub fn SDL_JoystickGetGUID(joystick: *mut SDL_Joystick) -> SDL_JoystickGUID;

  /// Get the USB vendor ID of an opened joystick, if available.
  ///
  /// If the vendor ID isn't available this function returns 0.
  pub fn SDL_JoystickGetVendor(joystick: *mut SDL_Joystick) -> Uint16;

  /// Get the USB product ID of an opened joystick, if available.
  ///
  /// If the product ID isn't available this function returns 0.
  pub fn SDL_JoystickGetProduct(joystick: *mut SDL_Joystick) -> Uint16;

  /// Get the product version of an opened joystick, if available.
  ///
  /// If the product ID isn't available this function returns 0.
  pub fn SDL_JoystickGetProductVersion(joystick: *mut SDL_Joystick) -> Uint16;

  /// Get the type of an opened joystick.
  pub fn SDL_JoystickGetType(joystick: *mut SDL_Joystick) -> SDL_JoystickType;

  /// Return a string representation for this guid.
  ///
  /// `pszGUID` must point to at least 33 bytes. 32 for the string, plus a NULL
  /// terminator.
  pub fn SDL_JoystickGetGUIDString(
    guid: SDL_JoystickGUID, pszGUID: *mut c_char, cbGUID: c_int,
  );

  /// Convert a string into a joystick guid
  pub fn SDL_JoystickGetGUIDFromString(
    pchGUID: *const c_char,
  ) -> SDL_JoystickGUID;

  /// Returns `SDL_TRUE` if the joystick has been opened and it is currently
  /// connected, or `SDL_FALSE` if it has not.
  pub fn SDL_JoystickGetAttached(joystick: *mut SDL_Joystick) -> SDL_bool;

  /// Get the instance ID of an opened joystick or -1 if the joystick is
  /// invalid.
  pub fn SDL_JoystickInstanceID(joystick: *mut SDL_Joystick) -> SDL_JoystickID;

  /// Get the number of general axis controls on a joystick.
  pub fn SDL_JoystickNumAxes(joystick: *mut SDL_Joystick) -> c_int;

  /// Get the number of trackballs on a joystick.
  ///
  /// Joystick trackballs have only relative motion events associated with them.
  /// There's no "absolute" position to poll.
  pub fn SDL_JoystickNumBalls(joystick: *mut SDL_Joystick) -> c_int;

  /// Get the number of POV hats on a joystick.
  pub fn SDL_JoystickNumHats(joystick: *mut SDL_Joystick) -> c_int;

  /// Get the number of buttons on a joystick.
  pub fn SDL_JoystickNumButtons(joystick: *mut SDL_Joystick) -> c_int;

  /// Update the current state of the open joysticks.
  ///
  /// This is called automatically by the event loop if any joystick events are
  /// enabled.
  pub fn SDL_JoystickUpdate();

  /// Enable/disable joystick event polling.
  ///
  /// If joystick events are disabled, you must call [`SDL_JoystickUpdate`]
  /// yourself and check the state of the joystick when you want joystick
  /// information.
  ///
  /// The state can be one of [`SDL_QUERY`], [`SDL_ENABLE`], or [`SDL_IGNORE`].
  pub fn SDL_JoystickEventState(state: c_int) -> c_int;

  /// Get the current state of an axis control on a joystick.
  ///
  /// The state is a value ranging from -32768 to 32767.
  ///
  /// The axis indices start at index 0.
  pub fn SDL_JoystickGetAxis(
    joystick: *mut SDL_Joystick, axis: c_int,
  ) -> Sint16;

  /// Get the initial state of an axis control on a joystick.
  ///
  /// The state is a value ranging from -32768 to 32767.
  ///
  /// The axis indices start at index 0.
  ///
  /// **Returns:** `SDL_TRUE` if this axis has any initial value, or `SDL_FALSE`
  /// if not.
  pub fn SDL_JoystickGetAxisInitialState(
    joystick: *mut SDL_Joystick, axis: c_int, state: *mut Sint16,
  ) -> SDL_bool;

  /// Get the current state of a POV hat on a joystick.
  ///
  /// The hat indices start at index 0.
  ///
  /// **Returns:** The return value is one of the `SDL_HAT_*` constants.
  pub fn SDL_JoystickGetHat(joystick: *mut SDL_Joystick, hat: c_int) -> u8;

  /// Get the ball axis change since the last poll.
  ///
  /// **Return:** 0, or -1 if you passed it invalid parameters.
  ///
  /// The ball indices start at index 0.
  pub fn SDL_JoystickGetBall(
    joystick: *mut SDL_Joystick, ball: c_int, dx: *mut c_int, dy: *mut c_int,
  ) -> c_int;

  /// Get the current state of a button on a joystick.
  ///
  /// The button indices start at index 0.
  pub fn SDL_JoystickGetButton(
    joystick: *mut SDL_Joystick, button: c_int,
  ) -> Uint8;

  /// Trigger a rumble effect.
  ///
  /// Each call to this function cancels any previous rumble effect, and calling
  /// it with 0 intensity stops any rumbling.
  ///
  /// * `joystick` The joystick to vibrate.
  /// * `low_frequency_rumble` The intensity of the low frequency (left) rumble
  ///   motor, from 0 to 0xFFFF.
  /// * `high_frequency_rumble` The intensity of the high frequency (right)
  ///   rumble motor, from 0 to 0xFFFF.
  /// * `duration_ms` The duration of the rumble effect, in milliseconds.
  ///
  /// **Returns:** 0, or -1 if rumble isn't supported on this joystick
  pub fn SDL_JoystickRumble(
    joystick: *mut SDL_Joystick, low_frequency_rumble: Uint16,
    high_frequency_rumble: Uint16, duration_ms: Uint32,
  ) -> c_int;

  /// Close a joystick previously opened with [`SDL_JoystickOpen`].
  pub fn SDL_JoystickClose(joystick: *mut SDL_Joystick);

  /// Return the battery level of this joystick.
  pub fn SDL_JoystickCurrentPowerLevel(
    joystick: *mut SDL_Joystick,
  ) -> SDL_JoystickPowerLevel;
}
