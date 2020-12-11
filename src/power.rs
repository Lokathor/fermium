//! Lets you get power info about the system.

use crate::c_int;

/// The basic state for the system's power supply.
///
/// See the `SDL_POWERSTATE_*` constants.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_PowerState(pub u32);

/// Cannot determine power status.
pub const SDL_POWERSTATE_UNKNOWN: SDL_PowerState = SDL_PowerState(0);
/// Not plugged in, running on the battery.
pub const SDL_POWERSTATE_ON_BATTERY: SDL_PowerState = SDL_PowerState(1);
/// Plugged in, no battery available.
pub const SDL_POWERSTATE_NO_BATTERY: SDL_PowerState = SDL_PowerState(2);
/// Plugged in, charging battery.
pub const SDL_POWERSTATE_CHARGING: SDL_PowerState = SDL_PowerState(3);
/// Plugged in, battery charged.
pub const SDL_POWERSTATE_CHARGED: SDL_PowerState = SDL_PowerState(4);

extern "C" {
  /// Get the current power supply details.
  ///
  /// * `secs` Seconds of battery life left. You can pass a NULL here if you
  ///   don't care. Will return -1 if we can't determine a value, or if we're
  ///   not running on a battery.
  ///
  /// * `pct` Percentage of battery life left, between 0 and 100. You can pass a
  ///   NULL here if you don't care. Will return -1 if we can't determine a
  ///   value, or if we're not running on a battery.
  ///
  /// **Returns:** The state of the battery (if any).
  pub fn SDL_GetPowerInfo(secs: *mut c_int, pct: *mut c_int) -> SDL_PowerState;
}
