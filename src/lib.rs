#![no_std]
#![allow(bad_style)]
#![warn(missing_docs)]

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
// named `foo`, and `SDL.h` itself is `lib.rs`. As with SDL, all the
// declarations are exported as a single flat namespace at the top level.

pub mod platform;
pub use platform::*;

pub mod stdinc;
pub use stdinc::*;

pub mod error;
pub use error::*;

pub mod rwops;
pub use rwops::*;

pub mod audio;
pub use audio::*;

pub mod blendmode;
pub use blendmode::*;

pub mod clipboard;
pub use clipboard::*;

pub mod cpuinfo;
pub use cpuinfo::*;

pub mod pixels;
pub use pixels::*;

pub mod rect;
pub use rect::*;

pub mod surface;
pub use surface::*;

pub mod video;
pub use video::*;

pub mod scancode;
pub use scancode::*;

pub mod keycode;
pub use keycode::*;

pub mod keyboard;
pub use keyboard::*;

pub mod mouse;
pub use mouse::*;

pub mod joystick;
pub use joystick::*;

pub mod gamecontroller;
pub use gamecontroller::*;

pub mod touch;
pub use touch::*;

pub mod gesture;
pub use gesture::*;

pub mod events;
pub use events::*;

//pub mod quit;
//pub use quit::*;

// filesystem

// haptic

// hints

// loadso

// messagebox

// power

// renderer

// sensor

// shape

// syswm

// timer

// version

// vulkan
