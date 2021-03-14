//! Portable mutexes, semaphores, and condition variables.
use crate::{c_int, c_void};

/// The SDL mutex structure. A portable mutex.
///
/// Rust code should treat this as opaque, and use it as `*mut SDL_mutex`
#[repr(transparent)]
pub struct SDL_mutex(c_void);

extern "C" {
  /// Create a mutex, initialized unlocked.
  pub fn SDL_CreateMutex() -> *mut SDL_mutex;

  /// Lock the mutex. Returns 0 or -1 on error.
  ///
  /// Warning: It's likely UB to lock a mutex while your thread already holds
  /// the mutex (this is system specific).
  pub fn SDL_LockMutex(mutex: *mut SDL_mutex) -> c_int;

  /// Try to lock the mutex. Returns 0, [`SDL_MUTEX_TIMEDOUT`], or -1 on error
  pub fn SDL_TryLockMutex(mutex: *mut SDL_mutex) -> c_int;

  /// Unlock the mutex. Returns 0, or -1 on error.
  ///
  /// # Warning
  /// It is an error to unlock a mutex that has not been locked by
  /// the current thread, and doing so results in undefined behavior.
  pub fn SDL_UnlockMutex(mutex: *mut SDL_mutex) -> c_int;

  /// Destroy a mutex.
  pub fn SDL_DestroyMutex(mutex: *mut SDL_mutex);
}

/// An alias for [`SDL_LockMutex`], for some reason.
pub use SDL_LockMutex as SDL_mutexP;
/// An alias for [`SDL_UnlockMutex`], for some reason.
pub use SDL_UnlockMutex as SDL_mutexV;

/// The SDL semaphore structure.
///
/// Rust code should treat this as opaque, and use it as `*mut SDL_sem`.
#[repr(transparent)]
pub struct SDL_sem(c_void);

// In the C code it's `struct SDL_semaphore` but the type alias that lets you
// use it without `struct` is `SDL_sem`. The headers use `SDL_sem`, so we will
// too.

/// Alias for [`SDL_sem`].
pub type SDL_semaphore = SDL_sem;

extern "C" {
  /// Create a semaphore, initialized with value, returns null on failure.
  pub fn SDL_CreateSemaphore(initial_value: u32) -> *mut SDL_sem;

  /// Destroy a semaphore.
  pub fn SDL_DestroySemaphore(sem: *mut SDL_sem);

  /// This function suspends the calling thread until the semaphore pointed
  /// to by `sem` has a positive count. It then atomically decreases the
  /// semaphore count.
  pub fn SDL_SemWait(sem: *mut SDL_sem) -> c_int;

  /// Non-blocking variant of [`SDL_SemWait`].
  ///
  /// return 0 if the wait succeeds, [`SDL_MUTEX_TIMEDOUT`] if the wait would
  /// block, and -1 on error.
  pub fn SDL_SemTryWait(sem: *mut SDL_sem) -> c_int;

  /// Variant of [`SDL_SemWait`] with a timeout in milliseconds.
  ///
  /// Returns 0 if the wait succeeds, [`SDL_MUTEX_TIMEDOUT`] if the wait does
  /// not succeed in the allotted time, and -1 on error.
  ///
  /// # Warning
  /// On some platforms this function is implemented by looping with a
  /// delay of 1 ms, and so should be avoided if possible.
  pub fn SDL_SemWaitTimeout(sem: *mut SDL_sem, ms: c_int) -> c_int;

  /// Atomically increases the semaphore's count (not blocking).
  /// Returns 0, or -1 on error.
  pub fn SDL_SemPost(sem: *mut SDL_sem) -> c_int;

  /// Returns the current count of the semaphore.
  pub fn SDL_SemValue(sem: *mut SDL_sem) -> u32;
}

/// The SDL condition variable structure.
///
/// Rust code should treat this as opaque, and use it as `*mut SDL_cond`.
#[repr(transparent)]
pub struct SDL_cond(c_void);

extern "C" {
  /// Create a condition variable.
  pub fn SDL_CreateCond() -> *mut SDL_cond;

  /// Destroy a condition variable.
  pub fn SDL_DestroyCond(cond: *mut SDL_cond);
  /// Restart one of the threads that are waiting on the condition variable.
  ///
  /// Returns 0 or -1 on error.
  pub fn SDL_CondSignal(cond: *mut SDL_cond) -> c_int;

  /// Restart all threads that are waiting on the condition variable.
  ///
  /// Returns 0 or -1 on error.
  pub fn SDL_CondBroadcast(cond: *mut SDL_cond) -> c_int;

  /// Wait on the condition variable, unlocking the provided mutex.
  ///
  /// Returns 0 when it is signaled, or -1 on error.
  ///
  /// # Warning
  /// The mutex must be locked before entering this function!
  ///
  /// The mutex is re-locked once the condition variable is signaled.
  pub fn SDL_CondWait(cond: *mut SDL_cond, mutex: *mut SDL_mutex) -> c_int;

  /// Waits for at most `ms` milliseconds, and returns 0 if the condition
  /// variable is signaled, `SDL_MUTEX_TIMEDOUT` if the condition is not
  /// signaled in the allotted time, and -1 on error.
  ///
  /// # Warning
  /// On some platforms this function is implemented by looping with a
  /// delay of 1 ms, and so should be avoided if possible.
  pub fn SDL_CondWaitTimeout(
    cond: *mut SDL_cond, mutex: *mut SDL_mutex, ms: u32,
  ) -> c_int;
}

/// Synchronization functions which can time out return this value
/// if they time out.
pub const SDL_MUTEX_TIMEDOUT: c_int = 1;

/// This is the timeout value which corresponds to never time out.
pub const SDL_MUTEX_MAXWAIT: u32 = !0u32;
