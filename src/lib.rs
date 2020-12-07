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

mod platform;
pub use platform::*;

mod stdinc;
pub use stdinc::*;

mod error;
pub use error::*;

mod rwops;
pub use rwops::*;

// TODO: the code was first written from the 2.0.14 headers, so we should
// recheck each module to ensure that we're using only the 2.0.12 stuff.

mod audio;
pub use audio::*;

mod blendmode;
pub use blendmode::*;

mod clipboard;
pub use clipboard::*;

mod cpuinfo;
pub use cpuinfo::*;

mod pixels;
pub use pixels::*;

mod rect;
pub use rect::*;

mod surface;
pub use surface::*;

mod video;
pub use video::*;

mod scancode;
pub use scancode::*;

mod keycode;
pub use keycode::*;

mod keyboard;
pub use keyboard::*;

// mouse

// joystick

// gamecontroller

// quit

// touch

// gesture

// events

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
