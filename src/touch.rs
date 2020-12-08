//! Module for touch interface interactions.

pub use crate::{c_float, stdinc::*};

/// Used with touch events.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_TouchID(pub Sint64);

/// Used to identify a finger object.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_FingerID(pub Sint64);

/// The types of a touch device.
///
/// See `SDL_TOUCH_DEVICE_*` constants.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_TouchDeviceType(pub i32);

/// The "invalid" touch type. Used for error values and such.
pub const SDL_TOUCH_DEVICE_INVALID: SDL_TouchDeviceType = SDL_TouchDeviceType(-1);
/// Touch screen with window-relative coordinates
pub const SDL_TOUCH_DEVICE_DIRECT: SDL_TouchDeviceType = SDL_TouchDeviceType(0);
/// Trackpad with absolute device coordinates
pub const SDL_TOUCH_DEVICE_INDIRECT_ABSOLUTE: SDL_TouchDeviceType = SDL_TouchDeviceType(1);
/// Trackpad with screen cursor-relative coordinates
pub const SDL_TOUCH_DEVICE_INDIRECT_RELATIVE: SDL_TouchDeviceType = SDL_TouchDeviceType(2);

/// Info for a finger object.
#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
#[repr(C)]
#[allow(missing_docs)]
pub struct SDL_Finger {
  pub id: SDL_FingerID,
  pub x: c_float,
  pub y: c_float,
  pub pressure: c_float,
}

/// Used as the device ID for *mouse events* simulated with touch input.
pub const SDL_TOUCH_MOUSEID: u32 = -1_i32 as u32; // TODO: give this the correct type?

/// Used as the `SDL_TouchID` for *touch events* simulated with mouse input.
pub const SDL_MOUSE_TOUCHID: SDL_TouchID = SDL_TouchID(-1);

extern "C" {
  /// Get the number of registered touch devices.
  pub fn SDL_GetNumTouchDevices();

  /// Get the touch ID with the given index, or 0 if the index is invalid.
  pub fn SDL_GetTouchDevice(index: c_int) -> SDL_TouchID;

  /// Get the type of the given touch device.
  pub fn SDL_GetTouchDeviceType(touchID: SDL_TouchID) -> SDL_TouchDeviceType;

  /// Get the number of active fingers for a given touch device.
  pub fn SDL_GetNumTouchFingers(touchID: SDL_TouchID) -> c_int;

  /// Get the finger object of the given touch, with the given index.
  pub fn SDL_GetTouchFinger(touchID: SDL_TouchID, index: c_int) -> *mut SDL_Finger;
}
