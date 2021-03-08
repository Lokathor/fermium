#![no_std]
#![allow(bad_style)]
#![warn(missing_docs)]
#![allow(clippy::missing_safety_doc)]

//! Bindings to the SDL2 C library.
//!
//! This crate bundles a copy of the SDL2 source code and builds it
//! automatically via `build.rs`. No special setup is required on your part.
//!
//! Building SDL2 takes about a minute on an average development machine, so the
//! "first time" compilation of this crate can seem quite slow.
//!
//! # Crate Features
//! * `cargo_check`: This causes the crate to *skip* building SDL2 entirely.
//!   This allows the `cargo check` command (and similar commands that don't
//!   build an executable, such as `cargo doc`) to execute much faster.

pub use chlorine::{
  c_char, c_double, c_float, c_int, c_long, c_longlong, c_schar, c_short,
  c_uchar, c_uint, c_ulong, c_ulonglong, c_ushort, c_void,
};

macro_rules! impl_bit_ops_for_tuple_newtype {
  ($t:ty) => {
    impl core::ops::BitAnd for $t {
      type Output = Self;
      #[inline]
      #[must_use]
      fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
      }
    }
    impl core::ops::BitAndAssign for $t {
      #[inline]
      #[must_use]
      fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0
      }
    }
    impl core::ops::BitOr for $t {
      type Output = Self;
      #[inline]
      #[must_use]
      fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
      }
    }
    impl core::ops::BitOrAssign for $t {
      #[inline]
      #[must_use]
      fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0
      }
    }
    impl core::ops::BitXor for $t {
      type Output = Self;
      #[inline]
      #[must_use]
      fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
      }
    }
    impl core::ops::BitXorAssign for $t {
      #[inline]
      #[must_use]
      fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0
      }
    }
    impl core::ops::Not for $t {
      type Output = Self;
      #[inline]
      #[must_use]
      fn not(self) -> Self::Output {
        Self(!self.0)
      }
    }
  };
}

// Note(Lokathor): Declarations are organized into modules according to SDL's
// public header organization. A file like `include/SDL_foo.h` becomes a module
// named `foo`. Also there is a `prelude` module which lets you grab all exports
// in a single use statement.

pub mod prelude;

// TODO: haptic (joystick force feedback system).
// TODO: shape (allows shaped windows).
// TODO: mutex (portable, no_std mutex would be handy).
// TODO: locale (locale info)
// TODO: misc (lets you open a browser to a URL)
pub mod audio;
pub mod blendmode;
pub mod clipboard;
pub mod cpuinfo;
pub mod error;
pub mod events;
pub mod filesystem;
pub mod gamecontroller;
pub mod gesture;
pub mod hints;
pub mod joystick;
pub mod keyboard;
pub mod keycode;
pub mod loadso;
pub mod messagebox;
pub mod mouse;
pub mod mutex;
pub mod pixels;
pub mod platform;
pub mod power;
pub mod quit;
pub mod rect;
pub mod renderer;
pub mod rwops;
pub mod scancode;
pub mod sensor;
pub mod stdinc;
pub mod surface;
pub mod syswm;
pub mod timer;
pub mod touch;
pub mod version;
pub mod video;
pub mod vulkan;

/// SDL2's initialization flags.
///
/// These are the flags which may be passed to [`SDL_Init`]. You should specify
/// the subsystems which you will be using in your application.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_InitFlags(pub u32);
impl_bit_ops_for_tuple_newtype!(SDL_InitFlags);
#[allow(missing_docs)]
pub const SDL_INIT_TIMER: SDL_InitFlags = SDL_InitFlags(0x00000001);
#[allow(missing_docs)]
pub const SDL_INIT_AUDIO: SDL_InitFlags = SDL_InitFlags(0x00000010);
/// `SDL_INIT_VIDEO` implies [`SDL_INIT_EVENTS`]
pub const SDL_INIT_VIDEO: SDL_InitFlags = SDL_InitFlags(0x00000020);
/// `SDL_INIT_JOYSTICK` implies [`SDL_INIT_EVENTS`]
pub const SDL_INIT_JOYSTICK: SDL_InitFlags = SDL_InitFlags(0x00000200);
#[allow(missing_docs)]
pub const SDL_INIT_HAPTIC: SDL_InitFlags = SDL_InitFlags(0x00001000);
/// `SDL_INIT_GAMECONTROLLER` implies [`SDL_INIT_JOYSTICK`]
pub const SDL_INIT_GAMECONTROLLER: SDL_InitFlags = SDL_InitFlags(0x00002000);
#[allow(missing_docs)]
pub const SDL_INIT_EVENTS: SDL_InitFlags = SDL_InitFlags(0x00004000);
#[allow(missing_docs)]
pub const SDL_INIT_SENSOR: SDL_InitFlags = SDL_InitFlags(0x00008000);
/// compatibility; this flag is ignored.
pub const SDL_INIT_NOPARACHUTE: SDL_InitFlags = SDL_InitFlags(0x00100000);
#[allow(missing_docs)]
pub const SDL_INIT_EVERYTHING: SDL_InitFlags = SDL_InitFlags(
  SDL_INIT_TIMER.0
    | SDL_INIT_AUDIO.0
    | SDL_INIT_VIDEO.0
    | SDL_INIT_EVENTS.0
    | SDL_INIT_JOYSTICK.0
    | SDL_INIT_HAPTIC.0
    | SDL_INIT_GAMECONTROLLER.0
    | SDL_INIT_SENSOR.0,
);

extern "C" {
  /// This function initializes the subsystems specified by `flags`.
  ///
  /// **Returns:** 0 on success or a negative error code on failure
  pub fn SDL_Init(flags: SDL_InitFlags) -> c_int;

  /// This function initializes specific SDL subsystems
  ///
  /// Subsystem initialization is ref-counted, you must call
  /// [`SDL_QuitSubSystem`] for each [`SDL_InitSubSystem`] to correctly shutdown
  /// a subsystem manually (or call [`SDL_Quit`] to force shutdown). If a
  /// subsystem is already loaded then this call will increase the ref-count and
  /// return.
  pub fn SDL_InitSubSystem(flags: SDL_InitFlags) -> c_int;

  /// This function cleans up specific SDL subsystems.
  pub fn SDL_QuitSubSystem(flags: SDL_InitFlags) -> c_int;

  /// This function returns a mask of the specified subsystems which have
  /// previously been initialized.
  ///
  /// If `flags` is 0, it returns a mask of all initialized subsystems.
  pub fn SDL_WasInit(flags: SDL_InitFlags) -> c_int;

  /// This function cleans up all initialized subsystems.
  ///
  /// You should call it upon all exit conditions.
  pub fn SDL_Quit();
}
