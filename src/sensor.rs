//! Lets SDL use various sensors (such as the accelerometer in a phone).
//!
//! [`SDL_Init`] must have been called with the [`SDL_INIT_SENSOR`] flag. This
//! causes SDL to scan the system for sensors, and load appropriate drivers.

use crate::{c_char, c_float, c_int, c_void, stdinc::*};

/// SDL's opaque sensor type.
#[repr(transparent)]
pub struct SDL_Sensor(c_void);

/// This is a unique ID for a sensor for the time it is connected to the system.
///
/// It is never reused for the lifetime of the application.
///
/// The ID value starts at 0 and increments from there. The value -1 is an
/// invalid ID.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_SensorID(pub Sint32);

/// The different sensors defined by SDL
///
/// Additional sensors may be available, using platform-dependent semantics.
///
/// Hare are the additional Android sensors:
/// https://developer.android.com/reference/android/hardware/SensorEvent.html#values
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_SensorType(pub i32);

/// Returned for an invalid sensor
pub const SDL_SENSOR_INVALID: SDL_SensorType = SDL_SensorType(-1);
/// Unknown sensor type
pub const SDL_SENSOR_UNKNOWN: SDL_SensorType = SDL_SensorType(0);
/// Accelerometer
pub const SDL_SENSOR_ACCEL: SDL_SensorType = SDL_SensorType(1);
/// Gyroscope
pub const SDL_SENSOR_GYRO: SDL_SensorType = SDL_SensorType(2);

/// Accelerometer sensor
///
/// The accelerometer returns the current acceleration in SI meters per
/// second squared. This includes gravity, so a device at rest will have
/// an acceleration of `SDL_STANDARD_GRAVITY` straight down.
///
/// * values[0]: Acceleration on the x axis
/// * values[1]: Acceleration on the y axis
/// * values[2]: Acceleration on the z axis
///
/// For phones held in portrait mode, the axes are defined as follows:
/// * -X ... +X : left ... right
/// * -Y ... +Y : bottom ... top
/// * -Z ... +Z : farther ... closer
///
/// The axis data is not changed when the phone is rotated.
///
/// See Also: [`SDL_GetDisplayOrientation`]
pub const SDL_STANDARD_GRAVITY: c_float = 9.80665;

extern "C" {
  /// Count the number of sensors attached to the system right now.
  pub fn SDL_NumSensors() -> c_int;

  /// Get the implementation dependent name of a sensor.
  ///
  /// This can be called before any sensors are opened.
  ///
  /// **Returns:** The sensor name, or NULL if `device_index` is out of range.
  pub fn SDL_SensorGetDeviceName(device_index: c_int) -> *const c_char;

  /// Get the type of a sensor.
  ///
  /// This can be called before any sensors are opened.
  ///
  /// **Returns:** The sensor type, or `SDL_SENSOR_INVALID` if device_index is
  /// out of range.
  pub fn SDL_SensorGetDeviceType(device_index: c_int) -> SDL_SensorType;

  /// Get the platform dependent type of a sensor.
  ///
  /// This can be called before any sensors are opened.
  ///
  /// **Returns:** The sensor platform dependent type, or -1 if `device_index`
  /// is out of range.
  pub fn SDL_SensorGetDeviceNonPortableType(device_index: c_int) -> c_int;

  /// Get the instance ID of a sensor.
  ///
  /// This can be called before any sensors are opened.
  ///
  /// **Returns:** The sensor instance ID, or -1 if `device_index` is out of
  /// range.
  pub fn SDL_SensorGetDeviceInstanceID(device_index: c_int) -> SDL_SensorID;

  /// Open a sensor for use.
  ///
  /// The index passed as an argument refers to the N'th sensor on the system.
  ///
  /// **Returns:** A sensor identifier, or NULL if an error occurred.
  pub fn SDL_SensorOpen(device_index: c_int) -> *mut SDL_Sensor;

  /// Return the SDL_Sensor associated with an instance id.
  pub fn SDL_SensorFromInstanceID(instance_id: SDL_SensorID)
    -> *mut SDL_Sensor;

  /// Get the implementation dependent name of a sensor.
  ///
  /// **Returns:** The sensor name, or NULL if the sensor is NULL.
  pub fn SDL_SensorGetName(sensor: *mut SDL_Sensor) -> *const c_char;

  /// Get the type of a sensor.
  ///
  /// This can be called before any sensors are opened.
  ///
  /// **Returns:** The sensor type, or SDL_SENSOR_INVALID if the sensor is NULL.
  pub fn SDL_SensorGetType(sensor: *mut SDL_Sensor) -> SDL_SensorType;

  /// Get the platform dependent type of a sensor.
  ///
  /// This can be called before any sensors are opened.
  ///
  /// **Returns:** The sensor platform dependent type, or -1 if the sensor is
  /// NULL.
  pub fn SDL_SensorGetNonPortableType(sensor: *mut SDL_Sensor) -> c_int;

  /// Get the instance ID of a sensor.
  ///
  /// This can be called before any sensors are opened.
  ///
  /// **Returns:** The sensor instance ID, or -1 if the sensor is NULL.
  pub fn SDL_SensorGetInstanceID(sensor: *mut SDL_Sensor) -> SDL_SensorID;

  /// Get the current state of an opened sensor.
  ///
  /// The number of values and interpretation of the data is sensor dependent.
  ///
  /// * `sensor` The sensor to query
  /// * `data` A pointer filled with the current sensor state
  /// * `num_values` The number of values to write to data
  ///
  /// **Returns:** 0 or -1 if an error occurred.
  pub fn SDL_SensorGetData(
    sensor: *mut SDL_Sensor, data: *mut c_float, num_values: c_int,
  ) -> c_int;

  /// Close a sensor previously opened with [`SDL_SensorOpen`]
  pub fn SDL_SensorClose(sensor: *mut SDL_Sensor);

  /// Update the current state of the open sensors.
  ///
  /// This is called automatically by the event loop if sensor events are
  /// enabled.
  ///
  /// This needs to be called from the thread that initialized the sensor
  /// subsystem.
  pub fn SDL_SensorUpdate();
}
