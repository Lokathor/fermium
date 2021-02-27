//! This is a module that re-exports every sub-module of the crate.
//!
//! If you just want all of `fermium` to be available without a fuss:
//! ```rust
//! use fermium::prelude::*;
//! ```

pub use crate::{
  audio::*, blendmode::*, clipboard::*, cpuinfo::*, error::*, events::*,
  filesystem::*, gamecontroller::*, gesture::*, hints::*, joystick::*,
  keyboard::*, keycode::*, loadso::*, messagebox::*, mouse::*, pixels::*,
  platform::*, power::*, quit::*, rect::*, renderer::*, rwops::*, scancode::*,
  sensor::*, stdinc::*, surface::*, syswm::*, timer::*, touch::*, version::*,
  video::*, vulkan::*, *,
};
