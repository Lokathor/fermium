//! This is a module that re-exports every sub-module of the crate.
//!
//! If you just want all of `fermium` to be available without a fuss:
//! ```rust
//! use fermium::prelude::*;
//! ```

pub use audio::*;
pub use blendmode::*;
pub use clipboard::*;
pub use cpuinfo::*;
pub use error::*;
pub use events::*;
pub use filesystem::*;
pub use gamecontroller::*;
pub use gesture::*;
pub use hints::*;
pub use joystick::*;
pub use keyboard::*;
pub use keycode::*;
pub use loadso::*;
pub use messagebox::*;
pub use mouse::*;
pub use pixels::*;
pub use platform::*;
pub use power::*;
pub use quit::*;
pub use rect::*;
pub use renderer::*;
pub use rwops::*;
pub use scancode::*;
pub use sensor::*;
pub use stdinc::*;
pub use surface::*;
pub use syswm::*;
pub use timer::*;
pub use touch::*;
pub use version::*;
pub use video::*;
pub use vulkan::*;
