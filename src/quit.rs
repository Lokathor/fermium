//! Quit stuff.
//!
//! An [`SDL_QUIT`] event is generated when the user tries to close the
//! application window. If it is ignored or filtered out, the window will remain
//! open. If it is not ignored or filtered, it is queued normally and the window
//! is allowed to close.  When the window is closed, screen updates will
//! complete, but have no effect.
//!
//! [`SDL_Init`] installs signal handlers for `SIGINT` (keyboard interrupt) and
//! `SIGTERM` (system termination request), if handlers do not already exist,
//! that generate [`SDL_QUIT`] events as well.  There is no way to determine the
//! cause of an [`SDL_QUIT`] event, but setting a signal handler in your
//! application will override the default generation of quit events for that
//! signal.

use crate::events::*;

// makes rustdoc link properly!
#[allow(unused)]
use crate::*;

/// Checks if a quit event is currently in the queue.
#[inline]
#[must_use]
pub unsafe fn SDL_QuitRequested() -> bool {
  SDL_PumpEvents();
  SDL_PeepEvents(core::ptr::null_mut(), 0, SDL_PEEKEVENT, SDL_QUIT, SDL_QUIT)
    > 0
}
