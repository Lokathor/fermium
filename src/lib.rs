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

use cfg_if::cfg_if;

cfg_if! {
  if #[cfg(feature = "use_bindgen_bin")] {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
  }
}
// TODO: all the else branches

/*
// Note(Lokathor): Bindgen doesn't parse all things properly on its own, so we
// define a few more things here "by hand".

/// Used as the device ID for mouse events simulated with touch input
///
/// `SDL_touch.h`, line 53
pub const SDL_TOUCH_MOUSEID: u32 = -1i32 as u32;
*/
