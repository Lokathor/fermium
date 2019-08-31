#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

//! The `fermium` crate is bindings to SDL2.
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
