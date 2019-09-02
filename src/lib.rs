#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::redundant_static_lifetimes)]
#![allow(clippy::cognitive_complexity)]

//! The `fermium` crate is bindings to SDL2.
//!
//! Depending on how you configure the crate, you can target a minimum SDL2
//! version of 2.0.8 or later. You can compile against a later version than your
//! minimum bind version, but not an earlier one, so for best compatability you
//! should target the earliest SDL2 API version you can. The current version of
//! SDL2 in Ubuntu 18 is 2.0.8, and Debian Stable has 2.0.9, so we don't at this
//! time bother to support SDL2 versions before 2.0.8.
//!
//! `bindgen` is used to generate the bindings from the official SDL2 include
//! files. At the moment we include the following SDL2 headers:
//!
//! * `SDL2.h`
//! * `SDL_syswm.h`
//! * `SDL_vulkan.h`
//!
//! However, `SDL_syswm.h` in particular pulls in a bunch of extra code and it
//! overwhelms the generated bindings. To avoid this, we only keep the following
//! whitelist of items:
//!
//! * `SDL_` (functions, types, and vars)
//! * `SDLK_` (vars)
//! * `AUDIO_` (vars)
//! * Any other items that the above depend on.
//!
//! It is thought that this will expose all needed functionality, but if you
//! think something should be added to the whitelist please [submit an
//! issue](https://github.com/Lokathor/fermium/issues).
//!
//! ## Docs
//!
//! Bindgen doesn't understand how to convert doxygen style docs into rustdoc
//! style docs, so instead you should look up the docs on the [SDL2
//! Wiki](https://wiki.libsdl.org/)

#[cfg(not(windows))]
pub use libc::{
  c_char, c_int, c_long, c_longlong, c_short, c_uint, c_ulong, c_ulonglong, c_ushort,
};
#[cfg(windows)]
pub use winapi::ctypes::{
  c_char, c_int, c_long, c_longlong, c_short, c_uint, c_ulong, c_ulonglong, c_ushort,
};

use cfg_if::cfg_if;

cfg_if! {
  if #[cfg(feature = "use_bindgen_bin")] {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
  } else if #[cfg(all(target_arch="x86_64", target_os="windows", target_env="msvc"))] {
    cfg_if! {
      if #[cfg(feature = "bind_SDL2_2_0_10")] {
        include!("SDL2-v2.0.10/x86_64-pc-windows-msvc.rs");
      } else if #[cfg(feature = "bind_SDL2_2_0_9")] {
        include!("SDL2-v2.0.9/x86_64-pc-windows-msvc.rs");
      } else {
        include!("SDL2-v2.0.8/x86_64-pc-windows-msvc.rs");
      }
    }
  } else if #[cfg(all(target_arch="x86", target_os="windows", target_env="msvc"))] {
    cfg_if! {
      if #[cfg(feature = "bind_SDL2_2_0_10")] {
        include!("SDL2-v2.0.10/i686-pc-windows-msvc.rs");
      } else if #[cfg(feature = "bind_SDL2_2_0_9")] {
        include!("SDL2-v2.0.9/i686-pc-windows-msvc.rs");
      } else {
        include!("SDL2-v2.0.8/i686-pc-windows-msvc.rs");
      }
    }
  } else if #[cfg(all(target_arch="x86_64", target_os="macos", target_env=""))] {
    cfg_if! {
      if #[cfg(feature = "bind_SDL2_2_0_10")] {
        include!("SDL2-v2.0.10/x86_64-apple-darwin.rs");
      } else if #[cfg(feature = "bind_SDL2_2_0_9")] {
        include!("SDL2-v2.0.9/x86_64-apple-darwin.rs");
      } else {
        include!("SDL2-v2.0.8/x86_64-apple-darwin.rs");
      }
    }
  } else if #[cfg(all(target_arch="x86_64", target_os="linux", target_env="gnu"))] {
    // Generated on a Debian machine with 2.0.9 installed from source
    cfg_if! {
      if #[cfg(feature = "bind_SDL2_2_0_10")] {
        compile_error!("No pre-made bindings found and you didn't run bindgen!");
      } else if #[cfg(feature = "bind_SDL2_2_0_9")] {
        include!("SDL2-v2.0.9/x86_64-unknown-linux-gnu.rs");
      } else {
        include!("SDL2-v2.0.8/x86_64-unknown-linux-gnu.rs");
      }
    }
  } else if #[cfg(all(target_arch="arm", target_os="linux", target_env="gnu"))] {
    // Generated on an rpi4 with 2.0.9 from the package manager
    cfg_if! {
      if #[cfg(feature = "bind_SDL2_2_0_10")] {
        compile_error!("No pre-made bindings found and you didn't run bindgen!");
      } else if #[cfg(feature = "bind_SDL2_2_0_9")] {
        include!("SDL2-v2.0.9/armv7-unknown-linux-gnueabihf.rs");
      } else {
        include!("SDL2-v2.0.8/armv7-unknown-linux-gnueabihf.rs");
      }
    }
  } else {
    compile_error!("No pre-made bindings found and you didn't run bindgen!");
  }
}

// Note(Lokathor): Bindgen doesn't parse all things properly on its own, so we
// define a few more things here "by hand".

/// `SDL_touch.h`: Used as the device ID for mouse events simulated with touch input
pub const SDL_TOUCH_MOUSEID: u32 = -1i32 as u32;

/// `SDL_touch.h`: Used as the SDL_TouchID for touch events simulated with mouse input
pub const SDL_MOUSE_TOUCHID: u64 = -1i64 as u64;

#[inline(always)]
pub unsafe fn SDL_MUSTLOCK(surface: *const SDL_Surface) -> bool {
  (*surface).flags & SDL_RLEACCEL != 0
}
