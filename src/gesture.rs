//! Module for touch gestures.

pub use crate::{rwops::*, touch::*};

/// Used with gesture events.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_GestureID(pub Sint64);

extern "C" {
  /// Begin Recording a gesture on the specified touch, or all touches (-1).
  pub fn SDL_RecordGesture(touchId: SDL_TouchID) -> c_int;

  /// Save all currently loaded Dollar Gesture templates.
  pub fn SDL_SaveAllDollarTemplates(dst: *mut SDL_RWops) -> c_int;

  /// Save a currently loaded Dollar Gesture template.
  pub fn SDL_SaveDollarTemplate(
    gestureId: SDL_GestureID, dst: *mut SDL_RWops,
  ) -> c_int;

  /// Load Dollar Gesture templates from a file.
  pub fn SDL_LoadDollarTemplates(
    touchId: SDL_TouchID, src: *mut SDL_RWops,
  ) -> c_int;
}
