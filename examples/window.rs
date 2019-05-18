// This line makes your release build program not have a dummy terminal attached
// to it on Win32. It has no effect on the other operating systems.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! This is an "Opening a window" demo using only the raw SDL2 calls.
//!
//! It's documented quite a bit so that hopefully a Rust programmer that doesn't
//! know about SDL2 can read it and follow along with what's going on.

use fermium::{SDL_EventType::*, SDL_WindowFlags::*, *};
use libc::{c_char, c_int};

// bindgen isn't perfect, and sometimes it turns C macro definitions into not
// quite the right Rust types that we want. In this case, it has the "mask" but
// not the plain value on its own. So we declare this under the name we expect,
// but we also must cast it because the mask is typed as `Uint32`/`u32`, but
// `SDL_CreateWindow` wants a `c_int`/`i32` for position values, and Rust
// doesn't auto-convert like C does.
const SDL_WINDOWPOS_CENTERED: c_int = SDL_WINDOWPOS_CENTERED_MASK as c_int;

unsafe fn print_error() {
  // This is a pointer to the start of a C-style string, so we process it
  // one character at a time until we see the null terminator. In this case
  // we're inefficiently printing it out, but you could also collect these
  // bytes into a String for example. https://wiki.libsdl.org/SDL_GetError
  let mut c_char_ptr: *const c_char = SDL_GetError();
  while *c_char_ptr != 0 {
    // technically this is less efficient than locking stdout once to print the
    // whole string, but whatever.
    print!("{}", *c_char_ptr as u8 as char);
    c_char_ptr = c_char_ptr.offset(1);
  }
  println!();
}

fn main() {
  // It's a bindings library, so it's almost all FFI calls. Laissez-faire Ferris
  unsafe {
    // Use the argument to specify the subsystems that you want initialized,
    // returns 0 on success or negative error code on failure.
    // https://wiki.libsdl.org/SDL_Init
    if SDL_Init(SDL_INIT_VIDEO) != 0 {
      print_error();
    } else {
      // A window is effectively !Send (because of Win32), and also it must only
      // be created from the main thread (because of Mac)
      // https://wiki.libsdl.org/SDL_CreateWindow
      let window = SDL_CreateWindow(
        "Window Demo\0".as_ptr() as *const c_char,
        SDL_WINDOWPOS_CENTERED,
        SDL_WINDOWPOS_CENTERED,
        800,
        600,
        SDL_WINDOW_SHOWN as u32,
      );
      if window.is_null() {
        print_error();
      } else {
        // Main loop of the program, I'm sure you're all familiar.
        'game_loop: loop {
          // This is a union of a tag and also a bunch of types that always have a
          // tag as the first field. In other words, we can always safely check
          // the value of the tag. https://wiki.libsdl.org/SDL_Event
          let mut event = SDL_Event::default();
          // Polls for one event out of the window's event queue. It's 1 on
          // success and 0 on failure, https://wiki.libsdl.org/SDL_PollEvent
          while SDL_PollEvent(&mut event) != 0 {
            match event.type_ as SDL_EventType::Type {
              SDL_QUIT => {
                // Every union variant has a different set of payload data of
                // course. In this case, the quit event has a timestamp, so we
                // might as well print it.
                println!("Quit after {} milliseconds.", event.quit.timestamp);
                break 'game_loop;
              }
              _ => (),
            }
          }
          // After your process your event queue you'd do a game frame, wait for
          // vsync, all that stuff.
        }
        // Destroys the window. It's even safe to use on a null pointer, you
        // just get an error message. https://wiki.libsdl.org/SDL_DestroyWindow
        SDL_DestroyWindow(window);
      }
    };
    // "Use this function to clean up all initialized subsystems. You should
    // call it upon all exit conditions." ... "It is safe to call this function
    // even in the case of errors in initialization."
    // https://wiki.libsdl.org/SDL_Quit
    SDL_Quit()
  }
}
