//! The SDL haptic subsystem allows you to control haptic (force feedback)
//! devices.
use crate::prelude::*;

/// The opaque haptic structure used to identify an SDL haptic device.
#[repr(transparent)]
pub struct SDL_Haptic(c_void);
/// Haptic effect type.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_HapticEffectType(pub u16);

impl SDL_HapticEffectType {
  /// Convert a [`SDL_HapticEffectType`] into a [`SDL_HapticFeatures`]
  /// indicating support for the effect type.
  #[inline]
  pub const fn feature(self) -> SDL_HapticFeatures {
    SDL_HapticFeatures(self.0 as u32)
  }
}
/// Constant haptic effect.
///
/// Used with [`SDL_HapticConstant`].
pub const SDL_HAPTIC_CONSTANT: SDL_HapticEffectType =
  SDL_HapticEffectType(1 << 0);
/// Sine wave haptic effect.
///
/// Periodic haptic effect that simulates sine waves.
///
/// Used with [`SDL_HapticPeriodic`].
pub const SDL_HAPTIC_SINE: SDL_HapticEffectType = SDL_HapticEffectType(1 << 1);
/// Left/Right haptic effect.
///
/// Haptic effect for direct control over high/low frequency motors.
///
/// Used with [`SDL_HapticLeftRight`].
pub const SDL_HAPTIC_LEFTRIGHT: SDL_HapticEffectType =
  SDL_HapticEffectType(1 << 2);
/// Triangle wave haptic effect.
///
/// Periodic haptic effect that simulates triangular waves.
///
/// Used with [`SDL_HapticPeriodic`].
pub const SDL_HAPTIC_TRIANGLE: SDL_HapticEffectType =
  SDL_HapticEffectType(1 << 3);
/// "Sawtoothup" wave haptic effect.
///
/// Periodic haptic effect that simulates saw tooth up waves.
///
/// Used with [`SDL_HapticPeriodic`].
pub const SDL_HAPTIC_SAWTOOTHUP: SDL_HapticEffectType =
  SDL_HapticEffectType(1 << 4);
/// "Sawtoothdown" wave haptic effect.
///
/// Periodic haptic effect that simulates saw tooth down waves.
///
/// Used with [`SDL_HapticPeriodic`].
pub const SDL_HAPTIC_SAWTOOTHDOWN: SDL_HapticEffectType =
  SDL_HapticEffectType(1 << 5);
/// Ramp haptic effect.
///
/// Used with [`SDL_HapticRamp`].
pub const SDL_HAPTIC_RAMP: SDL_HapticEffectType = SDL_HapticEffectType(1 << 6);

/// Spring effect - uses axes position.
///
/// Condition haptic effect that simulates a spring.  Effect is based on the
/// axes position.
///
/// Used with [`SDL_HapticCondition`].
pub const SDL_HAPTIC_SPRING: SDL_HapticEffectType =
  SDL_HapticEffectType(1 << 7);
/// Damper effect - uses axes velocity.
///
/// Condition haptic effect that simulates dampening.  Effect is based on the
/// axes velocity.
///
/// Used with [`SDL_HapticCondition`].
pub const SDL_HAPTIC_DAMPER: SDL_HapticEffectType =
  SDL_HapticEffectType(1 << 8);
/// Inertia effect - uses axes acceleration.
///
/// Condition haptic effect that simulates inertia.  Effect is based on the axes
/// acceleration.
///
/// Used with [`SDL_HapticCondition`].
pub const SDL_HAPTIC_INERTIA: SDL_HapticEffectType =
  SDL_HapticEffectType(1 << 9);

/// Friction effect - uses axes movement.
///
/// Condition haptic effect that simulates friction.  Effect is based on the
/// axes movement.
///
/// Used with [`SDL_HapticCondition`].
pub const SDL_HAPTIC_FRICTION: SDL_HapticEffectType =
  SDL_HapticEffectType(1 << 10);

/// User defined custom haptic effect.
///
/// Used with [`SDL_HapticCustom`].
pub const SDL_HAPTIC_CUSTOM: SDL_HapticEffectType =
  SDL_HapticEffectType(1 << 11);

/// Represents a bitmask of features which are potentially supported by a haptic
/// device. Used with [`SDL_HapticQuery`].
///
/// Each of the [`SDL_HapticEffectType`] values can be converted into a
/// [`SDL_HapticFeatures`] that indicates if the device supports use of that
/// effect type using the [`SDL_HapticEffectType::features`] method.
///
/// Tests can be performed in various ways, for example:
/// ```text
/// let want = SDL_HAPTIC_CONSTANT.feature() | SDL_HAPTIC_PAUSE;
/// let have = SDL_HapticQuery(haptic);
/// if (have & want) == want {
///     // Device supports the constant haptic effect, and pausing.
/// }
/// ```
///
/// # Rationale
///
/// This type doesn't exist in the C SDL2 headers. Instead, the effect types are
/// `#define`d constants, which are used either as a feature bitset, or a effect
/// type constant. This doesn't really work for Rust, especially since these two
/// uses have different sizes. As a result, we provide separate types for
/// `SDL_HapticFeatures` (used with [`SDL_HapticQuery`]), and
/// [`SDL_HapticEffectType`] (used with most of the other haptics APIs).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_HapticFeatures(pub u32);
impl_bit_ops_for_tuple_newtype!(SDL_HapticFeatures);

/// Haptic device supports global gain.
///
/// Used to test the result of [`SDL_HapticQuery`] to determine if the device
/// supports [`SDL_HapticSetGain`], etc.
pub const SDL_HAPTIC_GAIN: SDL_HapticFeatures = SDL_HapticFeatures(1 << 12);

/// Haptic device supports setting autocenter.
///
/// Used to test the result of [`SDL_HapticQuery`] to determine if the device
/// supports [`SDL_HapticSetAutocenter`], etc.
pub const SDL_HAPTIC_AUTOCENTER: SDL_HapticFeatures =
  SDL_HapticFeatures(1 << 13);
/// Haptic device supports querying effect status.
///
/// Used to test the result of [`SDL_HapticQuery`] to determine if the device
/// supports [`SDL_HapticGetEffectStatus`], etc.
pub const SDL_HAPTIC_STATUS: SDL_HapticFeatures = SDL_HapticFeatures(1 << 14);

/// Haptic device supports being paused.
///
/// Used to test the result of [`SDL_HapticQuery`] to determine if the device
/// supports [`SDL_HapticPause`], [`SDL_HapticUnpause`], etc.
pub const SDL_HAPTIC_PAUSE: SDL_HapticFeatures = SDL_HapticFeatures(1 << 15);

/// Used to play a device an infinite number of times.
///
/// See [`SDL_HapticRunEffect`].
pub const SDL_HAPTIC_INFINITY: u32 = 4294967295u32;

/// A type used with [`SDL_HapticDirection`]. One of
/// - [`SDL_HAPTIC_POLAR`]
/// - [`SDL_HAPTIC_CARTESIAN`]
/// - [`SDL_HAPTIC_SPHERICAL`]
/// - [`SDL_HAPTIC_STEERING_AXIS`]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_HapticDirectionType(pub u8);

/// Uses polar coordinates for the direction in [`SDL_HapticDirection`].
pub const SDL_HAPTIC_POLAR: SDL_HapticDirectionType =
  SDL_HapticDirectionType(0);
/// Uses cartesian coordinates for the direction in [`SDL_HapticDirection`].
pub const SDL_HAPTIC_CARTESIAN: SDL_HapticDirectionType =
  SDL_HapticDirectionType(1);
/// Uses spherical coordinates for the direction in [`SDL_HapticDirection`].
pub const SDL_HAPTIC_SPHERICAL: SDL_HapticDirectionType =
  SDL_HapticDirectionType(2);
/// Use this value to play an effect on the steering wheel axis. This provides
/// better compatibility across platforms and devices as SDL will guess the
/// correct axis.
pub const SDL_HAPTIC_STEERING_AXIS: SDL_HapticDirectionType =
  SDL_HapticDirectionType(3);

/// Structure that represents a haptic direction.
///
/// This is the direction where the force comes from, instead of the direction
/// in which the force is exerted.
///
/// Cardinal directions of the haptic device are relative to the positioning of
/// the device.  North is considered to be away from the user.
///
/// See <https://wiki.libsdl.org/SDL_HapticDirection> for detailed diagrams
/// explaining this type.
///
/// # Interpretation of `dir`
///
/// ## [`SDL_HAPTIC_POLAR`]
/// If type is [`SDL_HAPTIC_POLAR`], direction is encoded by hundredths of a
/// degree starting north and turning clockwise. [`SDL_HAPTIC_POLAR`] only uses
/// the first `dir` parameter.  The cardinal directions would be:
/// - North: 0 (0 degrees)
/// - East: 9000 (90 degrees)
/// - South: 18000 (180 degrees)
/// - West: 27000 (270 degrees)
///
/// ## [`SDL_HAPTIC_CARTESIAN`]
/// If type is [`SDL_HAPTIC_CARTESIAN`], direction is encoded by three positions
/// (X axis, Y axis and Z axis (with 3 axes)).  [`SDL_HAPTIC_CARTESIAN`] uses
/// the first three `dir` parameters.  The cardinal directions would be:
/// - North:  0,-1, 0
/// - East:   1, 0, 0
/// - South:  0, 1, 0
/// - West:  -1, 0, 0
///
/// The Z axis represents the height of the effect if supported, otherwise it's
/// unused.  In cartesian encoding (1, 2) would be the same as (2, 4), you can
/// use any multiple you want, only the direction matters.
///
/// ## [`SDL_HAPTIC_SPHERICAL`]
/// If type is [`SDL_HAPTIC_SPHERICAL`], direction is encoded by two rotations.
/// The first two `dir` parameters are used.  The `dir` parameters are as
/// follows (all values are in hundredths of degrees):
/// - Degrees from (1, 0) rotated towards (0, 1).
/// - Degrees towards (0, 0, 1) (device needs at least 3 axes).
///
/// ## [`SDL_HAPTIC_STEERING_AXIS`]
/// This is new in SDL 2.0.14, and is not yet well documented, but as far as I
/// can tell, all of the fields of `dir` are ignored when this is in use, and it
/// computes the direction entirely based on the steering wheel.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_HapticDirection {
  /// The type of encoding.
  pub type_: SDL_HapticDirectionType,
  /// The encoded direction.
  pub dir: [Sint32; 3],
}

/// A structure containing a template for a Constant effect.
///
/// This struct is exclusively for the [`SDL_HAPTIC_CONSTANT`] effect.
///
/// A constant effect applies a constant force in the specified direction
/// to the joystick.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SDL_HapticConstant {
  /// Must be [`SDL_HAPTIC_CONSTANT`].
  pub type_: SDL_HapticEffectType,
  /// Direction of the effect
  pub direction: SDL_HapticDirection,

  /* Replay */
  /// Duration of the effect
  pub length: Uint32,
  /// Delay before starting the effect.
  pub delay: Uint16,

  /* Trigger */
  /// Button that triggers the effect.
  pub button: Uint16,
  /// How soon it can be triggered again after button.
  pub interval: Uint16,

  /* Constant */
  /// Strength of the constant effect.
  pub level: Sint16,

  /* Envelope */
  /// Duration of the attack.
  pub attack_length: Uint16,
  /// Level at the start of the attack.
  pub attack_level: Uint16,
  /// Duration of the fade.
  pub fade_length: Uint16,
  /// Level at the end of the fade.
  pub fade_level: Uint16,
}

/// A structure containing a template for a Periodic effect.
///
/// The struct handles the following effects:
/// - [`SDL_HAPTIC_SINE`]
/// - [`SDL_HAPTIC_TRIANGLE`]
/// - [`SDL_HAPTIC_SAWTOOTHUP`]
/// - [`SDL_HAPTIC_SAWTOOTHDOWN`]
///
/// A periodic effect consists in a wave-shaped effect that repeats itself over
/// time.  The type determines the shape of the wave and the parameters
/// determine the dimensions of the wave.
///
/// Phase is given by hundredth of a degree meaning that giving the phase a
/// value of 9000 will displace it 25% of its period.  Here are sample values:
///  - 0: No phase displacement.
///  - 9000: Displaced 25% of its period.
///  - 18000: Displaced 50% of its period.
///  - 27000: Displaced 75% of its period.
///  - 36000: Displaced 100% of its period, same as 0, but 0 is preferred.
///
/// # Caveats
/// The SDL2 official documentation indicates you can use `SDL_HAPTIC_LEFTRIGHT`
/// for this structure, however from reading over the source, it is clear that
/// this is stale information from whien this constant referred to
/// `SDL_HAPTIC_SQUARE`, which no longer exists.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SDL_HapticPeriodic {
  /// Type of effect, must be one of:
  /// - [`SDL_HAPTIC_SINE`]
  /// - [`SDL_HAPTIC_TRIANGLE`]
  /// - [`SDL_HAPTIC_SAWTOOTHUP`]
  /// - [`SDL_HAPTIC_SAWTOOTHDOWN`]
  pub type_: SDL_HapticEffectType,
  /// Direction of the effect
  pub direction: SDL_HapticDirection,

  /* Replay */
  /// Duration of the effect
  pub length: Uint32,
  /// Delay before starting the effect.
  pub delay: Uint16,

  /* Trigger */
  /// Button that triggers the effect.
  pub button: Uint16,
  /// How soon it can be triggered again after button.
  pub interval: Uint16,

  /* Periodic */
  /// Period of the wave.
  pub period: Uint16,
  /// Peak value; if negative, equivalent to 180 degrees extra phase shift.
  pub magnitude: Sint16,
  /// Mean value of the wave.
  pub offset: Sint16,
  /// Positive phase shift given by hundredth of a degree.
  pub phase: Uint16,

  /* Envelope */
  /// Duration of the attack.
  pub attack_length: Uint16,
  /// Level at the start of the attack.
  pub attack_level: Uint16,
  /// Duration of the fade.
  pub fade_length: Uint16,
  /// Level at the end of the fade.
  pub fade_level: Uint16,
}

/// A structure containing a template for a Condition effect.
///
/// The struct handles the following effects:
/// - [`SDL_HAPTIC_SPRING`]: Effect based on axes position.
/// - [`SDL_HAPTIC_DAMPER`]: Effect based on axes velocity.
/// - [`SDL_HAPTIC_INERTIA`]: Effect based on axes acceleration.
/// - [`SDL_HAPTIC_FRICTION`]: Effect based on axes movement.
///
/// Direction is handled by condition internals instead of a direction member.
/// The condition effect specific members have three parameters.  The first
/// refers to the X axis, the second refers to the Y axis and the third refers
/// to the Z axis.  The right terms refer to the positive side of the axis and
/// the left terms refer to the negative side of the axis.  Please refer to the
/// [`SDL_HapticDirection`] [diagram][direction-diagram] for which side is
/// positive and which is negative.
///
/// [direction-diagram]: https://wiki.libsdl.org/SDL_HapticDirection#Remarks
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SDL_HapticCondition {
  /// Type of effect, must be one of:
  /// - [`SDL_HAPTIC_SPRING`]
  /// - [`SDL_HAPTIC_DAMPER`]
  /// - [`SDL_HAPTIC_INERTIA`]
  /// - [`SDL_HAPTIC_FRICTION`]
  pub type_: SDL_HapticEffectType,
  /// Direction of the effect
  pub direction: SDL_HapticDirection,

  /* Replay */
  /// Duration of the effect
  pub length: Uint32,
  /// Delay before starting the effect.
  pub delay: Uint16,

  /* Trigger */
  /// Button that triggers the effect.
  pub button: Uint16,
  /// How soon it can be triggered again after button.
  pub interval: Uint16,

  /* Condition */
  /// Level when joystick is to the positive side; max 0xFFFF.
  pub right_sat: [Uint16; 3],
  /// Level when joystick is to the negative side; max 0xFFFF.
  pub left_sat: [Uint16; 3],
  /// How fast to increase the force towards the positive side.
  pub right_coeff: [Sint16; 3],
  /// How fast to increase the force towards the negative side.
  pub left_coeff: [Sint16; 3],
  /// Size of the dead zone; max 0xFFFF: whole axis-range when 0-centered.
  pub deadband: [Uint16; 3],
  /// Position of the dead zone.
  pub center: [Sint16; 3],
}

/// A structure containing a template for a Ramp effect.
///
/// This struct is exclusively for the [`SDL_HAPTIC_RAMP`] effect.
///
/// The ramp effect starts at start strength and ends at end strength.
/// It augments in linear fashion.  If you use attack and fade with a ramp
/// the effects get added to the ramp effect making the effect become
/// quadratic instead of linear.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SDL_HapticRamp {
  /// Must be [`SDL_HAPTIC_RAMP`].
  pub type_: SDL_HapticEffectType,
  /// Direction of the effect.
  pub direction: SDL_HapticDirection,

  /* Replay */
  /// Duration of the effect
  pub length: Uint32,
  /// Delay before starting the effect.
  pub delay: Uint16,

  /* Trigger */
  /// Button that triggers the effect.
  pub button: Uint16,
  /// How soon it can be triggered again after button.
  pub interval: Uint16,

  /* Ramp */
  /// Beginning strength level.
  pub start: Sint16,
  /// Ending strength level.
  pub end: Sint16,

  /* Envelope */
  /// Duration of the attack.
  pub attack_length: Uint16,
  /// Level at the start of the attack.
  pub attack_level: Uint16,
  /// Duration of the fade.
  pub fade_length: Uint16,
  /// Level at the end of the fade.
  pub fade_level: Uint16,
}

/// A structure containing a template for a Left/Right effect.
///
/// This struct is exclusively for the [`SDL_HAPTIC_LEFTRIGHT`] effect.
///
/// The Left/Right effect is used to explicitly control the large and small
/// motors, commonly found in modern game controllers. The small (right) motor
/// is high frequency, and the large (left) motor is low frequency.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SDL_HapticLeftRight {
  /// Type of effect, must be [`SDL_HAPTIC_LEFTRIGHT`].
  pub type_: SDL_HapticEffectType,
  /* Replay */
  /// Duration of the effect in milliseconds.
  pub length: Uint32,

  /* Rumble */
  /// Control of the large controller motor.
  pub large_magnitude: Uint16,
  /// Control of the small controller motor.
  pub small_magnitude: Uint16,
}

/// A structure containing a template for the [`SDL_HAPTIC_CUSTOM`] effect.
///
/// This struct is exclusively for the [`SDL_HAPTIC_CUSTOM`] effect.
///
/// A custom force feedback effect is much like a periodic effect, where the
/// application can define its exact shape.  You will have to allocate the data
/// yourself.  Data should consist of channels * samples Uint16 samples.
///
/// If channels is one, the effect is rotated using the defined direction.
/// Otherwise it uses the samples in data for the different axes.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SDL_HapticCustom {
  /// Type of effect, must be [`SDL_HAPTIC_CUSTOM`].
  pub type_: SDL_HapticEffectType,
  /// Direction of the effect
  pub direction: SDL_HapticDirection,

  /* Replay */
  /// Duration of the effect
  pub length: Uint32,
  /// Delay before starting the effect.
  pub delay: Uint16,

  /* Trigger */
  /// Button that triggers the effect.
  pub button: Uint16,
  /// How soon it can be triggered again after button.
  pub interval: Uint16,

  /* Custom */
  /// Axes to use, minimum of one.
  pub channels: Uint8,
  /// Sample periods.
  pub period: Uint16,
  /// Amount of samples.
  pub samples: Uint16,
  /// Should contain channels*samples items.
  pub data: *mut Uint16,

  /* Envelope */
  /// Duration of the attack.
  pub attack_length: Uint16,
  /// Level at the start of the attack.
  pub attack_level: Uint16,
  /// Duration of the fade.
  pub fade_length: Uint16,
  /// Level at the end of the fade.
  pub fade_level: Uint16,
}

/// The generic template for any haptic effect.
///
/// All values max at 32767 (0x7FFF).  Signed values also can be negative. Time
/// values unless specified otherwise are in milliseconds.
///
/// You can also pass [`SDL_HAPTIC_INFINITY`] to length instead of a 0-32767
/// value.  Neither delay, interval, attack_length nor fade_length support
/// [`SDL_HAPTIC_INFINITY`].  Fade will also not be used since effect never
/// ends.
///
/// Additionally, the ::SDL_HAPTIC_RAMP effect does not support a duration of
/// [`SDL_HAPTIC_INFINITY`].
///
/// Button triggers may not be supported on all devices, it is advised to not
/// use them if possible.  Buttons start at index 1 instead of index 0 like the
/// joystick.
///
/// If both attack_length and fade_level are 0, the envelope is not used,
/// otherwise both values are used.
#[repr(C)]
#[derive(Clone, Copy)]
pub union SDL_HapticEffect {
  /// Effect type. Always present
  pub type_: SDL_HapticEffectType,
  /// Constant effect. Used only when the type is [`SDL_HAPTIC_CONSTANT`].
  pub constant: SDL_HapticConstant,
  /// Periodic effect. Used when the type is one of the periodic types:
  /// - [`SDL_HAPTIC_SINE`]
  /// - [`SDL_HAPTIC_TRIANGLE`]
  /// - [`SDL_HAPTIC_SAWTOOTHDOWN`]
  /// - [`SDL_HAPTIC_SAWTOOTHUP`]
  pub periodic: SDL_HapticPeriodic,
  /// Condition effect. Used when the type is one of the condition types:
  /// - [`SDL_HAPTIC_SPRING`]
  /// - [`SDL_HAPTIC_DAMPER`]
  /// - [`SDL_HAPTIC_INERTIA`]
  /// - [`SDL_HAPTIC_FRICTION`]
  pub condition: SDL_HapticCondition,
  /// Ramp effect. Used only when the type is [`SDL_HAPTIC_RAMP`].
  pub ramp: SDL_HapticRamp,
  /// Left/Right effect. Used only when the type is [`SDL_HAPTIC_LEFTRIGHT`].
  pub leftright: SDL_HapticLeftRight,
  /// Custom effect. Used only when the type is [`SDL_HAPTIC_CUSTOM`].
  pub custom: SDL_HapticCustom,
}

/// Wrapper around an integer effect ID.
///
/// This is the type returned by uploading a [`SDL_HapticEffect`] onto a device
/// using [`SDL_HapticNewEffect`].
///
/// Note: The value -1 is an invalid ID!
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_HapticEffectID(pub c_int);

impl SDL_HapticEffectID {
  /// Convenience constant for the invalid haptic effect ID.
  ///
  /// Provided for convenience.
  pub const INVALID: Self = Self(-1);
  /// Returns true if this effect is valid.
  ///
  /// Provided for convenience.
  #[inline]
  pub const fn is_valid(self) -> bool {
    self.0 != Self::INVALID.0
  }
  /// Returns `None` if `self` is invalid, and `Some(self)` otherwise.
  ///
  /// Provided for convenience.
  #[inline]
  pub const fn valid(self) -> Option<Self> {
    if self.is_valid() {
      Some(self)
    } else {
      None
    }
  }
}

extern "C" {
  /// Count the number of haptic devices attached to the system.
  pub fn SDL_NumHaptics() -> c_int;
  /// Get the implementation dependent name of a haptic device.
  ///
  /// This can be called before any joysticks are opened. If no name can be
  /// found, this function returns null.
  pub fn SDL_HapticName(device_index: c_int) -> *const c_char;
  /// Opens a haptic device for use.
  ///
  /// The index passed as an argument refers to the N'th haptic device on this
  /// system.
  ///
  /// When opening a haptic device, its gain will be set to maximum and
  /// autocenter will be disabled.  To modify these values use
  /// [`SDL_HapticSetGain`] and [`SDL_HapticSetAutocenter`].
  pub fn SDL_HapticOpen(device_index: c_int) -> *mut SDL_Haptic;

  /// Checks if the haptic device at index has been opened.
  ///
  /// Returns 1 if it has been opened, and 0 if it has not.
  pub fn SDL_HapticOpened(device_index: c_int) -> c_int;

  /// Gets the index of a haptic device.
  ///
  /// Returns the index of the haptic device or -1 on error.
  pub fn SDL_HapticIndex(haptic: *mut SDL_Haptic) -> c_int;

  /// Gets whether or not the current mouse has haptic capabilities
  ///
  /// Returns [`SDL_TRUE`] if the mouse is haptic, [`SDL_FALSE`] if it is not.
  pub fn SDL_MouseIsHaptic() -> c_int;

  /// Tries to open a haptic device from the current mouse.
  ///
  /// Returns the haptic device, or NULL on error.
  pub fn SDL_HapticOpenFromMouse() -> *mut SDL_Haptic;

  /// Tries to open a haptic device from the current mouse.
  ///
  /// Returns 1 if the joystick is haptic, 0 if it isn't or -1 if an error
  /// occurred.
  pub fn SDL_JoystickIsHaptic(js: *mut SDL_Joystick) -> c_int;

  /// Opens a haptic device for use from a joystick device.
  ///
  /// You must still close the haptic device separately.  It will not be closed
  /// with the joystick.
  ///
  /// When opening from a joystick you should first close the haptic device
  /// before closing the joystick device.  If not, on some implementations the
  /// haptic device will also get unallocated and you'll be unable to use
  /// force feedback on that device.
  ///
  /// Returns the haptic device, or NULL on error.
  pub fn SDL_HapticOpenFromJoystick(js: *mut SDL_Joystick) -> *mut SDL_Haptic;

  /// Closes a haptic device previously opened with SDL_HapticOpen().
  pub fn SDL_HapticClose(h: *mut SDL_Haptic);

  /// Returns the number of effects a haptic device can store.
  ///
  /// On some platforms this isn't fully supported, and therefore is an
  /// approximation.  Always check to see if your created effect was actually
  /// created and do not rely solely on SDL_HapticNumEffects().
  ///
  /// Returns the number of effects the haptic device can store or -1 on error.
  pub fn SDL_HapticNumEffects(haptic: *mut SDL_Haptic) -> c_int;
  /// Returns the number of effects a haptic device can play at the same time.
  ///
  /// This is not supported on all platforms, but will always return a value.
  /// Added here for the sake of completeness.
  ///
  /// Returns The number of effects the haptic device can play at the same time
  /// or -1 on error.
  pub fn SDL_HapticNumEffectsPlaying(haptic: *mut SDL_Haptic) -> c_int;

  /// Gets the haptic device's supported features in bitwise manner
  ///
  /// See also [`SDL_HapticFeatures`]
  pub fn SDL_HapticQuery(haptic: *mut SDL_Haptic) -> SDL_HapticFeatures;

  /// Gets the number of haptic axes the device has
  pub fn SDL_HapticNumAxes(haptic: *mut SDL_Haptic) -> c_int;

  /// Checks to see if effect is supported by haptic.
  ///
  /// Returns 1 if effect is supported, 0 if it isn't or -1 on error.
  pub fn SDL_HapticEffectSupported(
    haptic: *mut SDL_Haptic, effect: *mut SDL_HapticEffect,
  ) -> c_int;

  /// Creates a new haptic effect on the device.
  ///
  /// Returns the identifier of the effect on success or -1 on error
  pub fn SDL_HapticNewEffect(
    haptic: *mut SDL_Haptic, effect: *mut SDL_HapticEffect,
  ) -> SDL_HapticEffectID;
  /// Updates the properties of an effect.
  ///
  /// Can be used dynamically, although behavior when dynamically changing
  /// direction may be strange.  Specifically the effect may reupload itself
  /// and start playing from the start.  You cannot change the type either when
  /// running [`SDL_HapticUpdateEffect`].
  ///
  /// Returns 0 on success or -1 on error.
  pub fn SDL_HapticUpdateEffect(
    haptic: *mut SDL_Haptic, effect: SDL_HapticEffectID,
    data: *mut SDL_HapticEffect,
  ) -> c_int;
  /// Runs the haptic effect on its associated haptic device.
  ///
  /// If `iterations` are [`SDL_HAPTIC_INFINITY`], it'll run the effect over and
  /// over repeating the envelope (attack and fade) every time.  If you only
  /// want the effect to last forever, set [`SDL_HAPTIC_INFINITY`] in the
  /// effect's length parameter.
  ///
  /// Returns 0 on success or -1 on error.
  pub fn SDL_HapticRunEffect(
    haptic: *mut SDL_Haptic, effect: SDL_HapticEffectID, iterations: Uint32,
  ) -> c_int;
  /// Stops the haptic effect on its associated haptic device.
  pub fn SDL_HapticStopEffect(
    haptic: *mut SDL_Haptic, effect: SDL_HapticEffectID,
  ) -> c_int;
  /// Destroys a haptic effect on the device.
  ///
  /// This will stop the effect if it's running.  Effects are automatically
  /// destroyed when the device is closed.
  pub fn SDL_HapticDestroyEffect(
    haptic: *mut SDL_Haptic, effect: SDL_HapticEffectID,
  );
  /// Gets the status of the current effect on the haptic device.
  ///
  /// Device must support the [`SDL_HAPTIC_STATUS`] feature
  ///
  /// Returns 0 if it isn't playing, 1 if it is playing or -1 on error.
  pub fn SDL_HapticGetEffectStatus(
    haptic: *mut SDL_Haptic, effect: SDL_HapticEffectID,
  ) -> c_int;
  /// Sets the global gain of the device.
  ///
  /// `gain` should be a value beween 0 and 100.
  ///
  /// Device must support the [`SDL_HAPTIC_GAIN`] feature.
  ///
  /// The user may specify the maximum gain by setting the environment variable
  /// `"SDL_HAPTIC_GAIN_MAX"` which should be between 0 and 100.  All calls to
  /// `SDL_HapticSetGain` will scale linearly using SDL_HAPTIC_GAIN_MAX as the
  /// maximum.
  ///
  /// Returns 0 on success or -1 on error.
  pub fn SDL_HapticSetGain(haptic: *mut SDL_Haptic, gain: c_int) -> c_int;
  /// Sets the global autocenter of the device.
  ///
  /// `autocenter` should be between 0 and 100.  Setting it to 0 will disable
  /// autocentering.
  ///
  /// Device must support the [`SDL_HAPTIC_AUTOCENTER`] feature.
  ///
  /// The user may specify the maximum gain by setting the environment variable
  /// `"SDL_HAPTIC_GAIN_MAX"` which should be between 0 and 100.  All calls to
  /// `SDL_HapticSetGain` will scale linearly using SDL_HAPTIC_GAIN_MAX as the
  /// maximum.
  ///
  /// Returns 0 on success or -1 on error.
  pub fn SDL_HapticSetAutocenter(
    haptic: *mut SDL_Haptic, autocenter: c_int,
  ) -> c_int;
  /// Pauses a haptic device.
  ///
  /// Device must support the [`SDL_HAPTIC_PAUSE`] feature.
  ///
  /// Call [`SDL_HapticUnpause`] to resume playback.
  ///
  /// Do not modify the effects nor add new ones while the device is paused.
  /// That can cause all sorts of weird errors.
  ///
  /// Returns 0 on success or -1 on error.
  pub fn SDL_HapticPause(haptic: *mut SDL_Haptic) -> c_int;
  /// Unpauses a haptic device after pausing with [`SDL_HapticPause`].
  ///
  /// Returns 0 on success or -1 on error.
  pub fn SDL_HapticUnpause(haptic: *mut SDL_Haptic) -> c_int;
  /// Stops all the currently playing effects on a haptic device.
  ///
  /// Returns 0 on success or -1 on error.
  pub fn SDL_HapticStopAll(haptic: *mut SDL_Haptic) -> c_int;

  /// Checks to see if rumble is supported on a haptic device.
  ///
  /// Returns 1 if effect is supported, 0 if it isn't or -1 on error.
  pub fn SDL_HapticRumbleSupported(haptic: *mut SDL_Haptic) -> c_int;
  /// Initializes the haptic device for simple rumble playback.
  ///
  /// Returns 0 on success or -1 on error
  pub fn SDL_HapticRumbleInit(haptic: *mut SDL_Haptic) -> c_int;
  /// Runs simple rumble on a haptic device
  ///
  /// - `strength` should be between 0.0 and 1.0.
  /// - `length` is in milliseconds (or [`SDL_HAPTIC_INFINITY`]).
  ///
  /// Returns 0 on success, or -1 on error
  pub fn SDL_HapticRumblePlay(
    haptic: *mut SDL_Haptic, strength: f32, length: Uint32,
  ) -> c_int;
  /// Stops the simple rumble on a haptic device.
  ///
  /// Returns 0 on success or -1 on error
  pub fn SDL_HapticRumbleStop(haptic: *mut SDL_Haptic) -> c_int;
}
