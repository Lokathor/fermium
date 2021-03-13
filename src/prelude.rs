//! This is a module that re-exports every sub-module of the crate.
//!
//! If you just want all of `fermium` to be available without a fuss:
//! ```rust
//! use fermium::prelude::*;
//! ```

pub use crate::{
  audio::*, blendmode::*, c_char, c_double, c_float, c_int, c_long, c_longlong,
  c_schar, c_short, c_uchar, c_uint, c_ulong, c_ulonglong, c_ushort, c_void,
  clipboard::*, cpuinfo::*, error::*, events::*, filesystem::*,
  gamecontroller::*, gesture::*, haptic::*, hints::*, joystick::*, keyboard::*,
  keycode::*, loadso::*, messagebox::*, mouse::*, mutex::*, pixels::*,
  platform::*, power::*, quit::*, rect::*, renderer::*, rwops::*, scancode::*,
  sensor::*, stdinc::*, surface::*, syswm::*, timer::*, touch::*, version::*,
  video::*, vulkan::*, *,
};
