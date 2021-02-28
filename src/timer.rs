//! Timer related functionality.

use crate::{c_int, c_void, stdinc::*};

/// Function prototype for the timer callback function.
///
/// The callback function is passed the current timer interval and returns the
/// next timer interval. If the returned value is the same as the one passed in,
/// the periodic alarm continues, otherwise a new alarm is scheduled. If the
/// callback returns 0, the periodic alarm is cancelled.
pub type SDL_TimerCallback =
  Option<unsafe extern "C" fn(interval: Uint32, param: *mut c_void) -> Uint32>;

/// Definition of the timer ID type.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_TimerID(pub c_int);

extern "C" {
  /// Get the number of milliseconds since the SDL library initialization.
  ///
  /// **Note:** This value wraps if the program runs for more than ~49 days.
  pub fn SDL_GetTicks() -> Uint32;

  /// Get the current value of the high resolution counter.
  pub fn SDL_GetPerformanceCounter() -> Uint64;

  /// Get the count per second of the high resolution counter.
  pub fn SDL_GetPerformanceFrequency() -> Uint64;

  /// Wait a specified number of milliseconds before returning.
  ///
  /// This is essentially the same as
  /// [`std::thread::sleep_ms`](https://doc.rust-lang.org/std/thread/fn.sleep_ms.html),
  /// which incidentally is deprecated for being a slightly inferior interface
  /// compared to
  /// [`std::thread::sleep`](https://doc.rust-lang.org/std/thread/fn.sleep.html).
  pub fn SDL_Delay(ms: Uint32);

  /// Add a new timer to the pool of timers already running.
  ///
  /// **Returns:** A timer ID, or 0 when an error occurs.
  pub fn SDL_AddTimer(
    interval: Uint32, callback: SDL_TimerCallback, param: *mut c_void,
  ) -> SDL_TimerID;

  /// Remove a timer by ID.
  ///
  /// **Returns:** A boolean value indicating success or failure.
  ///
  /// **Warning:** It is not safe to remove a timer multiple times.
  pub fn SDL_RemoveTimer(id: SDL_TimerID) -> SDL_bool;
}
