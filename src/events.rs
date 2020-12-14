//! Module for event handling.

use crate::{
  c_char, c_float, c_int, c_long, c_uint, c_void, gesture::*, joystick::*,
  keyboard::*, mouse::*, stdinc::*, syswm::*, touch::*, version::*, video::*,
};

/// Button is released.
///
/// Used with both keyboard and mouse.
pub const SDL_RELEASED: u8 = 0;

/// Button is pressed.
///
/// Used with both keyboard and mouse.
pub const SDL_PRESSED: u8 = 1;

/// The types of events that can be delivered.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_EventType(pub i32);

/// Unused
pub const SDL_FIRSTEVENT: SDL_EventType = SDL_EventType(0);

/// User-requested quit
pub const SDL_QUIT: SDL_EventType = SDL_EventType(0x100);

/// The application is being terminated by the OS.
///
/// * Called on iOS in `applicationWillTerminate()`
/// * Called on Android in `onDestroy()`
pub const SDL_APP_TERMINATING: SDL_EventType = SDL_EventType(0x100 + 1);

/// The application is low on memory, free memory if possible.
///
/// * Called on iOS in `applicationDidReceiveMemoryWarning()`
/// * Called on Android in `onLowMemory()`
pub const SDL_APP_LOWMEMORY: SDL_EventType = SDL_EventType(0x100 + 2);

/// The application is about to enter the background.
///
/// * Called on iOS in `applicationWillResignActive()`
/// * Called on Android in `onPause()`
pub const SDL_APP_WILLENTERBACKGROUND: SDL_EventType = SDL_EventType(0x100 + 3);

/// The application did enter the background and may not get CPU for some time.
///
/// * Called on iOS in `applicationDidEnterBackground()`
/// * Called on Android in `onPause()`
pub const SDL_APP_DIDENTERBACKGROUND: SDL_EventType = SDL_EventType(0x100 + 4);

/// The application is about to enter the foreground.
///
/// * Called on iOS in `applicationWillEnterForeground()`
/// * Called on Android in `onResume()`
pub const SDL_APP_WILLENTERFOREGROUND: SDL_EventType = SDL_EventType(0x100 + 5);

/// The application is now interactive.
///
/// * Called on iOS in `applicationDidBecomeActive()`
/// * Called on Android in `onResume()`
pub const SDL_APP_DIDENTERFOREGROUND: SDL_EventType = SDL_EventType(0x100 + 6);

/// Display state change.
pub const SDL_DISPLAYEVENT: SDL_EventType = SDL_EventType(0x150);

/// Window state change.
pub const SDL_WINDOWEVENT: SDL_EventType = SDL_EventType(0x200);

/// System specific event.
pub const SDL_SYSWMEVENT: SDL_EventType = SDL_EventType(0x200 + 1);

/// Key pressed.
pub const SDL_KEYDOWN: SDL_EventType = SDL_EventType(0x300);

/// Key released.
pub const SDL_KEYUP: SDL_EventType = SDL_EventType(0x300 + 1);

/// Keyboard text editing (composition).
pub const SDL_TEXTEDITING: SDL_EventType = SDL_EventType(0x300 + 2);

/// Keyboard text input.
pub const SDL_TEXTINPUT: SDL_EventType = SDL_EventType(0x300 + 3);

/// Keymap changed due to a system event such as an input language or keyboard
/// layout change.
pub const SDL_KEYMAPCHANGED: SDL_EventType = SDL_EventType(0x300 + 4);

/// Mouse moved.
pub const SDL_MOUSEMOTION: SDL_EventType = SDL_EventType(0x400);

/// Mouse button pressed.
pub const SDL_MOUSEBUTTONDOWN: SDL_EventType = SDL_EventType(0x400 + 1);

/// Mouse button released.
pub const SDL_MOUSEBUTTONUP: SDL_EventType = SDL_EventType(0x400 + 2);

/// Mouse wheel motion.
pub const SDL_MOUSEWHEEL: SDL_EventType = SDL_EventType(0x400 + 3);

/// Joystick axis motion.
pub const SDL_JOYAXISMOTION: SDL_EventType = SDL_EventType(0x600);

/// Joystick trackball motion.
pub const SDL_JOYBALLMOTION: SDL_EventType = SDL_EventType(0x600 + 1);

/// Joystick hat position change.
pub const SDL_JOYHATMOTION: SDL_EventType = SDL_EventType(0x600 + 2);

/// Joystick button pressed.
pub const SDL_JOYBUTTONDOWN: SDL_EventType = SDL_EventType(0x600 + 3);

/// Joystick button released.
pub const SDL_JOYBUTTONUP: SDL_EventType = SDL_EventType(0x600 + 4);

/// A new joystick has been inserted into the system.
pub const SDL_JOYDEVICEADDED: SDL_EventType = SDL_EventType(0x600 + 5);

/// An opened joystick has been removed.
pub const SDL_JOYDEVICEREMOVED: SDL_EventType = SDL_EventType(0x600 + 6);

/// Game controller axis motion.
pub const SDL_CONTROLLERAXISMOTION: SDL_EventType = SDL_EventType(0x650);

/// Game controller button pressed.
pub const SDL_CONTROLLERBUTTONDOWN: SDL_EventType = SDL_EventType(0x650 + 1);

/// Game controller button released.
pub const SDL_CONTROLLERBUTTONUP: SDL_EventType = SDL_EventType(0x650 + 2);

/// A new Game controller has been inserted into the system.
pub const SDL_CONTROLLERDEVICEADDED: SDL_EventType = SDL_EventType(0x650 + 3);

/// An opened Game controller has been removed.
pub const SDL_CONTROLLERDEVICEREMOVED: SDL_EventType = SDL_EventType(0x650 + 4);

/// The controller mapping was updated.
pub const SDL_CONTROLLERDEVICEREMAPPED: SDL_EventType =
  SDL_EventType(0x650 + 5);

#[allow(missing_docs)]
pub const SDL_FINGERDOWN: SDL_EventType = SDL_EventType(0x700);

#[allow(missing_docs)]
pub const SDL_FINGERUP: SDL_EventType = SDL_EventType(0x700 + 1);

#[allow(missing_docs)]
pub const SDL_FINGERMOTION: SDL_EventType = SDL_EventType(0x700 + 2);

#[allow(missing_docs)]
pub const SDL_DOLLARGESTURE: SDL_EventType = SDL_EventType(0x800);

#[allow(missing_docs)]
pub const SDL_DOLLARRECORD: SDL_EventType = SDL_EventType(0x800 + 1);

#[allow(missing_docs)]
pub const SDL_MULTIGESTURE: SDL_EventType = SDL_EventType(0x800 + 2);

/// The clipboard changed.
pub const SDL_CLIPBOARDUPDATE: SDL_EventType = SDL_EventType(0x900);

/// The system requests a file open.
pub const SDL_DROPFILE: SDL_EventType = SDL_EventType(0x1000);

/// Text/plain drag-and-drop event.
pub const SDL_DROPTEXT: SDL_EventType = SDL_EventType(0x1000 + 1);

/// A new set of drops is beginning (NULL filename).
pub const SDL_DROPBEGIN: SDL_EventType = SDL_EventType(0x1000 + 2);

/// Current set of drops is now complete (NULL filename).
pub const SDL_DROPCOMPLETE: SDL_EventType = SDL_EventType(0x1000 + 3);

/// A new audio device is available.
pub const SDL_AUDIODEVICEADDED: SDL_EventType = SDL_EventType(0x1100);

/// An audio device has been removed.
pub const SDL_AUDIODEVICEREMOVED: SDL_EventType = SDL_EventType(0x1100 + 1);

/// A sensor was updated.
pub const SDL_SENSORUPDATE: SDL_EventType = SDL_EventType(0x1200);

/// The render targets have been reset and their contents need to be updated.
pub const SDL_RENDER_TARGETS_RESET: SDL_EventType = SDL_EventType(0x2000);

/// The device has been reset and all textures need to be recreated.
pub const SDL_RENDER_DEVICE_RESET: SDL_EventType = SDL_EventType(0x2000 + 1);

/// Events [`SDL_USEREVENT`] through [`SDL_LASTEVENT`] are for your use, and
/// should be allocated with [`SDL_RegisterEvents`].
pub const SDL_USEREVENT: SDL_EventType = SDL_EventType(0x8000);

/// This last event is only for bounding internal arrays.
pub const SDL_LASTEVENT: SDL_EventType = SDL_EventType(0xFFFF);

/// Fields shared by every event.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
#[allow(missing_docs)]
pub struct SDL_CommonEvent {
  pub type_: SDL_EventType,
  /// In milliseconds, populated using [`SDL_GetTicks`]
  pub timestamp: Uint32,
}

/// Display state change event data (event.display.*)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
#[allow(missing_docs)]
pub struct SDL_DisplayEvent {
  /// Should always be [`SDL_DISPLAYEVENT`]
  pub type_: SDL_EventType,
  /// In milliseconds, populated using [`SDL_GetTicks`]
  pub timestamp: Uint32,
  /// The associated display index
  pub display: Uint32,
  pub event: SDL_DisplayEventID,
  pub padding1: Uint8,
  pub padding2: Uint8,
  pub padding3: Uint8,
  /// event dependent data
  pub data1: Sint32,
}

/// Window state change event data (event.window.*)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
#[allow(missing_docs)]
pub struct SDL_WindowEvent {
  /// Should always be [`SDL_WINDOWEVENT`]
  pub type_: SDL_EventType,
  /// In milliseconds, populated using [`SDL_GetTicks`]
  pub timestamp: Uint32,
  /// The associated window
  pub windowID: Uint32,
  pub event: SDL_WindowEventID,
  pub padding1: Uint8,
  pub padding2: Uint8,
  pub padding3: Uint8,
  /// event dependent data
  pub data1: Sint32,
  /// event dependent data
  pub data2: Sint32,
}

/// Keyboard button event structure (event.key.*)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
#[allow(missing_docs)]
pub struct SDL_KeyboardEvent {
  /// Should always be [`SDL_KEYDOWN`] or [`SDL_KEYUP`].
  pub type_: SDL_EventType,
  pub timestamp: Uint32,
  /// The window with keyboard focus, if any.
  pub windowID: Uint32,
  /// [`SDL_PRESSED`] or [`SDL_RELEASED`].
  pub state: Uint8,
  /// Non-zero if this is a key repeat.
  pub repeat: Uint8,
  pub padding2: Uint8,
  pub padding3: Uint8,
  /// The key that was pressed or released.
  pub keysym: SDL_Keysym,
}

/// Size of the [`SDL_TextEditingEvent`] array.
pub const SDL_TEXTEDITINGEVENT_TEXT_SIZE: usize = 32;

/// Keyboard text editing event structure (event.edit.*)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
#[allow(missing_docs)]
pub struct SDL_TextEditingEvent {
  /// Should always be [`SDL_TEXTEDITING`].
  pub type_: SDL_EventType,
  /// In milliseconds, populated using [`SDL_GetTicks`].
  pub timestamp: Uint32,
  /// The window with keyboard focus, if any.
  pub windowID: Uint32,
  /// The editing text.
  pub text: [c_char; SDL_TEXTEDITINGEVENT_TEXT_SIZE],
  /// The start cursor of selected editing text.
  pub start: Sint32,
  /// The length of selected editing text.
  pub length: Sint32,
}

/// Size of the [`SDL_TextInputEvent`] array.
pub const SDL_TEXTINPUTEVENT_TEXT_SIZE: usize = 32;

/// Keyboard text input event structure (event.text.*)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
#[allow(missing_docs)]
pub struct SDL_TextInputEvent {
  /// Should always be [`SDL_TEXTINPUT`].
  pub type_: SDL_EventType,
  /// In milliseconds, populated using [`SDL_GetTicks`].
  pub timestamp: Uint32,
  /// The window with keyboard focus, if any.
  pub windowID: Uint32,
  /// The input text.
  pub text: [c_char; SDL_TEXTINPUTEVENT_TEXT_SIZE],
}

/// Mouse motion event structure (event.motion.*)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
#[allow(missing_docs)]
pub struct SDL_MouseMotionEvent {
  /// Should always be [`SDL_MOUSEMOTION`].
  pub type_: SDL_EventType,
  /// In milliseconds, populated using [`SDL_GetTicks`].
  pub timestamp: Uint32,
  /// The window with mouse focus, if any
  pub windowID: Uint32,
  /// The mouse instance id, or [`SDL_TOUCH_MOUSEID`]
  pub which: Uint32,
  /// The current button state
  pub state: Uint32,
  /// X coordinate, relative to window
  pub x: Sint32,
  /// Y coordinate, relative to window
  pub y: Sint32,
  /// The relative motion in the X direction
  pub xrel: Sint32,
  /// The relative motion in the Y direction
  pub yrel: Sint32,
}

/// Mouse button event structure (event.button.*)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
#[allow(missing_docs)]
pub struct SDL_MouseButtonEvent {
  /// Should always be [`SDL_MOUSEBUTTONDOWN`] or [`SDL_MOUSEBUTTONUP`]
  pub type_: SDL_EventType,
  /// In milliseconds, populated using [`SDL_GetTicks`].
  pub timestamp: Uint32,
  /// The window with mouse focus, if any.
  pub windowID: Uint32,
  /// The mouse instance id, or [`SDL_TOUCH_MOUSEID`].
  pub which: Uint32,
  /// The mouse button index
  pub button: Uint8,
  /// [`SDL_PRESSED`] or [`SDL_RELEASED`].
  pub state: Uint8,
  /// 1 for single-click, 2 for double-click, etc.
  pub clicks: Uint8,
  pub padding1: Uint8,
  /// X coordinate, relative to window
  pub x: Sint32,
  /// Y coordinate, relative to window
  pub y: Sint32,
}

/// Mouse wheel event structure (event.wheel.*)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
#[allow(missing_docs)]
pub struct SDL_MouseWheelEvent {
  /// Should always be [`SDL_MOUSEWHEEL`]
  pub type_: SDL_EventType,
  /// In milliseconds, populated using [`SDL_GetTicks`].
  pub timestamp: Uint32,
  /// The window with mouse focus, if any.
  pub windowID: Uint32,
  /// The mouse instance id, or [`SDL_TOUCH_MOUSEID`].
  pub which: Uint32,
  /// The amount scrolled horizontally, positive to the right and negative to
  /// the left
  pub x: Sint32,
  /// The amount scrolled vertically, positive away from the user and negative
  /// toward the user
  pub y: Sint32,
  /// [`SDL_MOUSEWHEEL_NORMAL`] or [`SDL_MOUSEWHEEL_FLIPPED`].
  ///
  /// When `SDL_MOUSEWHEEL_FLIPPED`, the values in X and Y will be opposite.
  /// Multiply by -1 to change them back.
  pub direction: SDL_MouseWheelDirection,
}

/// Joystick axis motion event structure (event.jaxis.*)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
#[allow(missing_docs)]
pub struct SDL_JoyAxisEvent {
  /// Should always be [`SDL_JOYAXISMOTION`]
  pub type_: SDL_EventType,
  /// In milliseconds, populated using [`SDL_GetTicks`].
  pub timestamp: Uint32,
  /// The joystick instance id
  pub which: SDL_JoystickID,
  /// The joystick axis index
  pub axis: Uint8,
  pub padding1: Uint8,
  pub padding2: Uint8,
  pub padding3: Uint8,
  /// The axis value (range: -32768 to 32767)
  pub value: Sint16,
  pub padding4: Uint16,
}

/// Joystick trackball motion event structure (event.jball.*)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
#[allow(missing_docs)]
pub struct SDL_JoyBallEvent {
  /// Should always be [`SDL_JOYBALLMOTION`]
  pub type_: SDL_EventType,
  /// In milliseconds, populated using [`SDL_GetTicks`].
  pub timestamp: Uint32,
  /// The joystick instance id
  pub which: SDL_JoystickID,
  /// The joystick trackball index
  pub ball: Uint8,
  pub padding1: Uint8,
  pub padding2: Uint8,
  pub padding3: Uint8,
  /// The relative motion in the X direction
  pub xrel: Sint16,
  /// The relative motion in the Y direction
  pub yrel: Sint16,
}

/// Joystick hat position change event structure (event.jhat.*)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
#[allow(missing_docs)]
pub struct SDL_JoyHatEvent {
  /// Should always be [`SDL_JOYHATMOTION`]
  pub type_: SDL_EventType,
  /// In milliseconds, populated using [`SDL_GetTicks`].
  pub timestamp: Uint32,
  /// The joystick instance id.
  pub which: SDL_JoystickID,
  /// The joystick hat index.
  pub hat: Uint8,
  /// The hat position value.
  pub value: Uint8,
  pub padding1: Uint8,
  pub padding2: Uint8,
}

/// Joystick button event structure (event.jbutton.*)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
#[allow(missing_docs)]
pub struct SDL_JoyButtonEvent {
  /// Should always be [`SDL_JOYBUTTONDOWN`] or [`SDL_JOYBUTTONUP`]
  pub type_: SDL_EventType,
  /// In milliseconds, populated using [`SDL_GetTicks`].
  pub timestamp: Uint32,
  /// The joystick instance id
  pub which: SDL_JoystickID,
  /// The joystick button index
  pub button: Uint8,
  /// [`SDL_PRESSED`] or [`SDL_RELEASED`].
  pub state: Uint8,
  pub padding1: Uint8,
  pub padding2: Uint8,
}

/// Joystick device event structure (event.jdevice.*)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
#[allow(missing_docs)]
pub struct SDL_JoyDeviceEvent {
  /// Should always be [`SDL_JOYDEVICEADDED`] or [`SDL_JOYDEVICEREMOVED`]
  pub type_: SDL_EventType,
  /// In milliseconds, populated using [`SDL_GetTicks`].
  pub timestamp: Uint32,
  /// The joystick device index for the `ADDED` event, or instance id for the
  /// `REMOVED` event.
  pub which: Sint32,
}

/// Game controller axis motion event structure (event.caxis.*)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
#[allow(missing_docs)]
pub struct SDL_ControllerAxisEvent {
  /// Should always be [`SDL_CONTROLLERAXISMOTION`]
  pub type_: SDL_EventType,
  /// In milliseconds, populated using [`SDL_GetTicks`].
  pub timestamp: Uint32,
  /// The joystick instance id
  pub which: SDL_JoystickID,
  /// The controller axis (SDL_GameControllerAxis)
  pub axis: Uint8,
  pub padding1: Uint8,
  pub padding2: Uint8,
  pub padding3: Uint8,
  /// The axis value (range: -32768 to 32767)
  pub value: Sint16,
  pub padding4: Uint16,
}

/// Game controller button event structure (event.cbutton.*)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
#[allow(missing_docs)]
pub struct SDL_ControllerButtonEvent {
  /// Should always be [`SDL_CONTROLLERBUTTONDOWN`] or
  /// [`SDL_CONTROLLERBUTTONUP`]
  pub type_: SDL_EventType,
  /// In milliseconds, populated using [`SDL_GetTicks`].
  pub timestamp: Uint32,
  /// The joystick instance id
  pub which: SDL_JoystickID,
  /// The controller button (SDL_GameControllerButton)
  pub button: Uint8,
  /// [`SDL_PRESSED`] or [`SDL_RELEASED`].
  pub state: Uint8,
  pub padding1: Uint8,
  pub padding2: Uint8,
}

/// Controller device event structure (event.cdevice.*)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
#[allow(missing_docs)]
pub struct SDL_ControllerDeviceEvent {
  /// Should always be [`SDL_CONTROLLERDEVICEADDED`],
  /// [`SDL_CONTROLLERDEVICEREMOVED`], or [`SDL_CONTROLLERDEVICEREMAPPED`]
  pub type_: SDL_EventType,
  /// In milliseconds, populated using [`SDL_GetTicks`].
  pub timestamp: Uint32,
  /// The joystick device index for the `ADDED` event, instance id for the
  /// `REMOVED` or `REMAPPED` event.
  pub which: Sint32,
}

/// Audio device event structure (event.adevice.*)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
#[allow(missing_docs)]
pub struct SDL_AudioDeviceEvent {
  /// Should always be [`SDL_AUDIODEVICEADDED`] or
  /// [`SDL_AUDIODEVICEREMOVED`]
  pub type_: SDL_EventType,
  /// In milliseconds, populated using [`SDL_GetTicks`].
  pub timestamp: Uint32,
  /// The audio device index for the `ADDED` event (valid until next
  /// [`SDL_GetNumAudioDevices`] call), or [`SDL_AudioDeviceID`] for the
  /// `REMOVED` event.
  pub which: Uint32,
  /// zero if an output device, non-zero if a capture device.
  pub iscapture: Uint8,
  pub padding1: Uint8,
  pub padding2: Uint8,
  pub padding3: Uint8,
}

/// Touch finger event structure (event.tfinger.*)
#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
#[repr(C)]
#[allow(missing_docs)]
pub struct SDL_TouchFingerEvent {
  /// Should always be [`SDL_FINGERMOTION`],
  /// [`SDL_FINGERDOWN`], or [`SDL_FINGERUP`]
  pub type_: SDL_EventType,
  /// In milliseconds, populated using [`SDL_GetTicks`].
  pub timestamp: Uint32,
  /// The touch device id
  pub touchId: SDL_TouchID,
  pub fingerId: SDL_FingerID,
  /// Normalized in the range 0...1
  pub x: c_float,
  /// Normalized in the range 0...1
  pub y: c_float,
  /// Normalized in the range -1...1
  pub dx: c_float,
  /// Normalized in the range -1...1
  pub dy: c_float,
  /// Normalized in the range 0...1
  pub pressure: c_float,
  /// The window underneath the finger, if any
  pub windowID: Uint32,
}

/// Multiple Finger Gesture Event (event.mgesture.*)
#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
#[repr(C)]
#[allow(missing_docs)]
pub struct SDL_MultiGestureEvent {
  /// Should always be [`SDL_MULTIGESTURE`]
  pub type_: SDL_EventType,
  /// In milliseconds, populated using [`SDL_GetTicks`].
  pub timestamp: Uint32,
  /// The touch device id
  pub touchId: SDL_TouchID,
  pub dTheta: c_float,
  pub dDist: c_float,
  pub x: c_float,
  pub y: c_float,
  pub numFingers: Uint16,
  pub padding: Uint16,
}

/// Dollar Gesture Event (event.dgesture.*)
#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
#[repr(C)]
#[allow(missing_docs)]
pub struct SDL_DollarGestureEvent {
  /// Should always be [`SDL_DOLLARGESTURE`] or [`SDL_DOLLARRECORD`]
  pub type_: SDL_EventType,
  /// In milliseconds, populated using [`SDL_GetTicks`].
  pub timestamp: Uint32,
  /// The touch device id
  pub touchId: SDL_TouchID,
  pub gestureId: SDL_GestureID,
  pub numFingers: Uint32,
  pub error: c_float,
  /// Normalized center of gesture
  pub x: c_float,
  /// Normalized center of gesture
  pub y: c_float,
}

/// An event used to request a file open by the system (event.drop.*)
///
/// This event is enabled by default, you can disable it with
/// [`SDL_EventState`].
///
/// **Note:** If this event is enabled, you **must free the filename in the
/// event**.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
#[allow(missing_docs)]
pub struct SDL_DropEvent {
  /// Should always be [`SDL_DROPBEGIN`], [`SDL_DROPFILE`], [`SDL_DROPTEXT`],
  /// or [`SDL_DROPCOMPLETE`]
  pub type_: SDL_EventType,
  /// In milliseconds, populated using [`SDL_GetTicks`].
  pub timestamp: Uint32,
  /// The file name, which should be freed with [`SDL_free`] of non-null.
  ///
  /// This is NULL on begin/complete.
  pub file: *const c_char,
  /// The window that was dropped on, if any
  pub windowID: Uint32,
}
impl Default for SDL_DropEvent {
  #[inline]
  #[must_use]
  fn default() -> Self {
    unsafe { core::mem::zeroed() }
  }
}

/// Sensor event structure (event.sensor.*)
#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
#[repr(C)]
#[allow(missing_docs)]
pub struct SDL_SensorEvent {
  /// Should always be [`SDL_SENSORUPDATE`]
  pub type_: SDL_EventType,
  /// In milliseconds, populated using [`SDL_GetTicks`].
  pub timestamp: Uint32,
  /// The instance ID of the sensor
  pub which: Sint32,
  /// Up to 6 values from the sensor - additional values can be queried using
  /// [`SDL_SensorGetData`].
  pub data: [c_float; 6],
}

/// The "quit requested" event
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
#[allow(missing_docs)]
pub struct SDL_QuitEvent {
  /// Should always be [`SDL_QUIT`]
  pub type_: SDL_EventType,
  /// In milliseconds, populated using [`SDL_GetTicks`].
  pub timestamp: Uint32,
}

/// OS Specific event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
#[allow(missing_docs)]
pub struct SDL_OSEvent {
  pub type_: SDL_EventType,
  /// In milliseconds, populated using [`SDL_GetTicks`].
  pub timestamp: Uint32,
}

/// A user-defined event type (event.user.*)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
#[allow(missing_docs)]
pub struct SDL_UserEvent {
  /// Should be in the exclusive range [`SDL_USEREVENT`] .. [`SDL_LASTEVENT`]
  pub type_: SDL_EventType,
  /// In milliseconds, populated using [`SDL_GetTicks`].
  pub timestamp: Uint32,
  /// The associated window if any
  pub windowID: Uint32,
  /// User defined event code
  pub code: Sint32,
  /// User defined data pointer
  pub data1: *mut c_void,
  /// User defined data pointer
  pub data2: *mut c_void,
}
impl Default for SDL_UserEvent {
  #[inline]
  #[must_use]
  fn default() -> Self {
    unsafe { core::mem::zeroed() }
  }
}

/// This is the data your [Window Procedure](https://docs.microsoft.com/en-us/windows/win32/winmsg/using-window-procedures) would get.
#[derive(Clone, Copy)]
#[repr(C)]
#[allow(missing_docs)]
pub struct SDL_SysWMmsg_windows {
  /// The window for the message (`HWND`)
  pub hwnd: *mut c_void,
  /// The type of message (`UINT`)
  pub msg: c_uint,
  /// WORD message parameter (`WPARAM`)
  pub wParam: usize,
  /// LONG message parameter (`LPARAM`)
  pub lParam: isize,
}

#[derive(Clone, Copy)]
#[repr(C)]
#[allow(missing_docs)]
pub union SDL_SysWMmsg_union {
  /// The inputs to a [Window Procedure](https://docs.microsoft.com/en-us/windows/win32/winmsg/using-window-procedures).
  pub win: SDL_SysWMmsg_windows,
  /// Transmute this to an `XEvent`.
  pub x11_event: [c_long; 24],
  /// This is a `DFBEvent`
  pub dfb: c_uint,
  /* The window systems without any supported events are skipped here */
}

/// Message info from the system's window manager.
#[derive(Clone, Copy)]
#[repr(C)]
#[allow(missing_docs)]
pub struct SDL_SysWMmsg {
  pub version: SDL_version,
  pub subsystem: SDL_SYSWM_TYPE,
  pub msg: SDL_SysWMmsg_union,
}

/// A video driver dependent system event (event.syswm.*)
///
/// The [`SDL_SYSWMEVENT`] event type is disabled by default, you can enable it
/// with [`SDL_EventState`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
#[allow(missing_docs)]
pub struct SDL_SysWMEvent {
  /// Should always be [`SDL_SYSWMEVENT`]
  pub type_: SDL_EventType,
  /// In milliseconds, populated using [`SDL_GetTicks`].
  pub timestamp: Uint32,
  /// driver dependent data.
  pub msg: *mut SDL_SysWMmsg,
}
impl Default for SDL_SysWMEvent {
  #[inline]
  #[must_use]
  fn default() -> Self {
    unsafe { core::mem::zeroed() }
  }
}

/// General event structure
#[derive(Clone, Copy)]
#[repr(C)]
#[allow(missing_docs)]
pub union SDL_Event {
  pub type_: SDL_EventType,
  pub common: SDL_CommonEvent,
  pub display: SDL_DisplayEvent,
  pub window: SDL_WindowEvent,
  pub key: SDL_KeyboardEvent,
  pub edit: SDL_TextEditingEvent,
  pub text: SDL_TextInputEvent,
  pub motion: SDL_MouseMotionEvent,
  pub button: SDL_MouseButtonEvent,
  pub wheel: SDL_MouseWheelEvent,
  pub jaxis: SDL_JoyAxisEvent,
  pub jball: SDL_JoyBallEvent,
  pub jhat: SDL_JoyHatEvent,
  pub jbutton: SDL_JoyButtonEvent,
  pub jdevice: SDL_JoyDeviceEvent,
  pub caxis: SDL_ControllerAxisEvent,
  pub cbutton: SDL_ControllerButtonEvent,
  pub cdevice: SDL_ControllerDeviceEvent,
  pub adevice: SDL_AudioDeviceEvent,
  pub sensor: SDL_SensorEvent,
  pub quit: SDL_QuitEvent,
  pub user: SDL_UserEvent,
  pub syswm: SDL_SysWMEvent,
  pub tfinger: SDL_TouchFingerEvent,
  pub mgesture: SDL_MultiGestureEvent,
  pub dgesture: SDL_DollarGestureEvent,
  pub drop: SDL_DropEvent,
  pub padding: [Uint8; 56],
  pub force_align: [u64; 7],
}

/// Used with [`SDL_PeepEvents`]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_eventaction(pub i32);

#[allow(missing_docs)]
pub const SDL_ADDEVENT: SDL_eventaction = SDL_eventaction(0);
#[allow(missing_docs)]
pub const SDL_PEEKEVENT: SDL_eventaction = SDL_eventaction(1);
#[allow(missing_docs)]
pub const SDL_GETEVENT: SDL_eventaction = SDL_eventaction(2);

/// Event filter operation.
pub type SDL_EventFilter = Option<
  unsafe extern "C" fn(userdata: *mut c_void, event: *mut SDL_Event) -> c_int,
>;

extern "C" {
  /// Pumps the event loop, gathering events from the input devices.
  ///
  /// This function updates the event queue and internal input device state.
  ///
  /// This should only be run in the thread that sets the video mode.
  pub fn SDL_PumpEvents();

  /// Checks the event queue for messages and optionally returns them.
  ///
  /// If `action` is [`SDL_ADDEVENT`], up to `numevents` events will be added to
  /// the back of the event queue.
  ///
  /// If `action` is [`SDL_PEEKEVENT`], up to `numevents` events at the front of
  /// the event queue, within the specified minimum and maximum type, will be
  /// returned and will not be removed from the queue.
  ///
  /// If `action` is [`SDL_GETEVENT`], up to `numevents` events at the front of
  /// the event queue, within the specified minimum and maximum type, will be
  /// returned and will be removed from the queue.
  ///
  /// **Returns:** The number of events actually stored, or -1 if there was an
  /// error.
  ///
  /// This function is thread-safe.
  pub fn SDL_PeepEvents(
    events: *mut SDL_Event, numevents: c_int, action: SDL_eventaction,
    minType: SDL_EventType, maxType: SDL_EventType,
  ) -> c_int;

  /// Checks to see if certain event types are in the event queue.
  pub fn SDL_HasEvent(type_: SDL_EventType) -> SDL_bool;

  /// Checks to see if a range of event types are in the event queue.
  pub fn SDL_HasEvents(
    minType: SDL_EventType, maxType: SDL_EventType,
  ) -> SDL_bool;

  /// This function clears events of a particular type from the event queue.
  ///
  /// This function only affects currently queued events. If you want to make
  /// sure that all pending OS events are flushed, you can call
  /// [`SDL_PumpEvents`] on the main thread immediately before the flush call.
  pub fn SDL_FlushEvent(type_: SDL_EventType);

  /// This function clears a range of event types from the event queue.
  ///
  /// This function only affects currently queued events. If you want to make
  /// sure that all pending OS events are flushed, you can call
  /// [`SDL_PumpEvents`] on the main thread immediately before the flush call.
  pub fn SDL_FlushEvents(minType: SDL_EventType, maxType: SDL_EventType);

  /// Polls for currently pending events.
  ///
  /// **Returns:** 1 if there are any pending events, or 0 if there are none
  /// available.
  ///
  /// * `event` If not NULL, the next event is removed from the queue and stored
  ///   in that area.
  pub fn SDL_PollEvent(event: *mut SDL_Event) -> c_int;

  /// Waits indefinitely for the next available event.
  ///
  /// **Returns:** 1, or 0 if there was an error while waiting for events.
  ///
  /// * `event` If not NULL, the next event is removed from the queue and stored
  ///   in that area.
  pub fn SDL_WaitEvent(event: *mut SDL_Event) -> c_int;

  /// Waits until the specified timeout (in milliseconds) for the next available
  /// event.
  ///
  /// **Returns:** 1, or 0 if there was an error while waiting for events.
  ///
  /// * `event` If not NULL, the next event is removed from the queue and stored
  ///   in that area.
  /// * `timeout` The timeout (in milliseconds) to wait for next event.
  pub fn SDL_WaitEventTimeout(event: *mut SDL_Event, timeout: c_int) -> c_int;

  /// Add an event to the event queue.
  ///
  /// **Returns:** 1 on success, 0 if the event was filtered, or -1 if the event
  /// queue was full or there was some other error.
  pub fn SDL_PushEvent(event: *mut SDL_Event) -> c_int;

  /// Sets up a filter to process all events before they change internal state
  /// and are posted to the internal event queue.
  ///
  /// If the filter returns 1, then the event will be added to the internal
  /// queue. If it returns 0, then the event will be dropped from the queue,
  /// but the internal state will still be updated. This allows selective
  /// filtering of dynamically arriving events.
  ///
  /// **Be very careful of what you do in the event filter function, as
  /// it may run in a different thread!**
  ///
  /// There is one caveat when dealing with the [`SDL_QuitEvent`] event type:
  /// * The event filter is only called when the window manager desires to close
  ///   the application window. If the event filter returns 1, then the window
  ///   will be closed, otherwise the window will remain open if possible.
  /// * If the quit event is generated by an interrupt signal, it will bypass
  ///   the internal queue and be delivered to the application at the next event
  ///   poll.
  pub fn SDL_SetEventFilter(filter: SDL_EventFilter, userdata: *mut c_void);

  /// Return the current event filter - can be used to "chain" filters.
  ///
  /// If there is no event filter set, this function returns `SDL_FALSE`.
  pub fn SDL_GetEventFilter(
    filter: *mut SDL_EventFilter, userdata: *mut *mut c_void,
  ) -> SDL_bool;

  /// Add a function which is called when an event is added to the queue.
  pub fn SDL_AddEventWatch(filter: SDL_EventFilter, userdata: *mut c_void);

  /// Remove an event watch function added with [`SDL_AddEventWatch`]
  pub fn SDL_DelEventWatch(filter: SDL_EventFilter, userdata: *mut c_void);

  /// Run the filter function on the current event queue, removing any
  /// events for which the filter returns 0.
  pub fn SDL_FilterEvents(filter: SDL_EventFilter, userdata: *mut c_void);

  /// This function allows you to set the state of processing certain events.
  ///
  /// * If `state` is set to [`SDL_IGNORE`], that event will be automatically
  ///   dropped from the event queue and will not be filtered.
  /// * If `state` is set to [`SDL_ENABLE`], that event will be processed
  ///   normally.
  /// * If `state` is set to [`SDL_QUERY`], [`SDL_EventState`] will return the
  ///   current processing state of the specified event.
  pub fn SDL_EventState(type_: SDL_EventType, state: c_int) -> Uint8;

  /// This function allocates a set of user-defined events, and returns
  /// the beginning event number for that set of events.
  ///
  /// If there aren't enough user-defined events left, this function
  /// returns `u32::MAX`
  pub fn SDL_RegisterEvents(numevents: c_int) -> Uint32;
}

/// Used with [`SDL_EventState`]
pub const SDL_QUERY: i32 = -1;
/// Used with [`SDL_EventState`]
pub const SDL_IGNORE: i32 = 0;
/// Alias for [`SDL_IGNORE`]
pub const SDL_DISABLE: i32 = 0;
/// Used with [`SDL_EventState`]
pub const SDL_ENABLE: i32 = 1;

/// As [`SDL_EventState`], but provides the "query" part for you.
#[inline]
#[must_use]
pub unsafe fn SDL_GetEventState(type_: SDL_EventType) -> Uint8 {
  SDL_EventState(type_, SDL_QUERY)
}
