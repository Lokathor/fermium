#![no_std]
#![allow(bad_style)]
#![warn(missing_docs)]
#![allow(clippy::missing_safety_doc)]

//! Bindings to the SDL2 C library.

pub use chlorine::*;

macro_rules! impl_bit_ops_for_tuple_newtype {
  ($t:ty) => {
    impl core::ops::BitAnd for $t {
      type Output = Self;
      #[inline]
      fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
      }
    }
    impl core::ops::BitAndAssign for $t {
      fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0
      }
    }
    impl core::ops::BitOr for $t {
      type Output = Self;
      #[inline]
      fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
      }
    }
    impl core::ops::BitOrAssign for $t {
      fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0
      }
    }
    impl core::ops::BitXor for $t {
      type Output = Self;
      #[inline]
      fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
      }
    }
    impl core::ops::BitXorAssign for $t {
      fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0
      }
    }
    impl core::ops::Not for $t {
      type Output = Self;
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
