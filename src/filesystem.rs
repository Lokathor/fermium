//! Filesystem helper functions.

pub use crate::c_char;

extern "C" {
  /// Get the path where the application resides.
  ///
  /// This is the directory where the application was run from, which is
  /// probably the installation directory, and may or may not be the process's
  /// current working directory.
  ///
  /// This returns an absolute path in UTF-8 encoding, and is guaranteed to end
  /// with a path separator ('\\' on Windows, '/' most other places).
  ///
  /// **The pointer returned by this function is owned by you.** Please call
  /// [`SDL_free`] on the pointer when you are done with it, or it will be a
  /// memory leak. This is not necessarily a fast call, though, so you should
  /// call this once near startup and save the string if you need it.
  ///
  /// Some platforms can't determine the application's path, and on other
  /// platforms, this might be meaningless. In such cases, this function will
  /// return NULL.
  ///
  /// **Returns:** String of base dir in UTF-8 encoding, or NULL on error.
  ///
  /// See Also: [`SDL_GetPrefPath`]
  pub fn SDL_GetBasePath() -> *mut c_char;

  /// Get the user-and-app-specific path where files can be written.
  ///
  /// This is meant to be where users can write personal files (preferences and
  /// save games, etc) that are specific to your application. This directory is
  /// unique per user, per application.
  ///
  /// This function will decide the appropriate location in the native
  /// filesystem, create the directory if necessary, and return a string of the
  /// absolute path to the directory in UTF-8 encoding.
  ///
  /// * On Windows, the string might look like:
  ///   "C:\\Users\\bob\\AppData\\Roaming\\My Company\\My Program Name\\"
  /// * On Linux, the string might look like: "/home/bob/.local/share/My Program
  ///   Name/"
  /// * On Mac OS X, the string might look like: "/Users/bob/Library/Application
  ///   Support/My Program Name/"
  /// * (etc.)
  ///
  /// You specify the name of your organization (if it's not a real
  /// organization, your name or an Internet domain you own might do) and the
  /// name of your application. These should be untranslated proper names.
  ///
  /// Both the org and app strings may become part of a directory name, so
  /// please follow these rules:
  ///
  /// * Try to use the same org string (including case-sensitivity) for all your
  ///   applications that use this function.
  /// * Always use a unique app string for each one, and make sure it never
  ///   changes for an app once you've decided on it.
  /// * Unicode characters are legal, as long as it's UTF-8 encoded, but...
  /// * ...only use letters, numbers, and spaces. Avoid punctuation like "Game
  ///   Name 2: Bad Guy's Revenge!", just "Game Name 2" is sufficient.
  ///
  /// This returns an absolute path in UTF-8 encoding, and is guaranteed to end
  /// with a path separator ('\\' on Windows, '/' most other places).
  ///
  /// **The pointer returned by this function is owned by you.** Please call
  /// [`SDL_free`] on the pointer when you are done with it, or it will be a
  /// memory leak. This is not necessarily a fast call, though, so you should
  /// call this once near startup and save the string if you need it.
  ///
  /// You should assume the path returned by this function is the only safe
  /// place to write files (and that [`SDL_GetBasePath`], while it might be
  /// writable, or even the parent of the returned path, is not where you should
  /// be writing things).
  ///
  /// Some platforms can't determine the pref path, and on other platforms, this
  /// might be meaningless. In such cases, this function will return NULL.
  ///
  /// * `org` The name of your organization.
  /// * `app` The name of your application.
  ///
  /// **Returns:** UTF-8 string of user dir in platform-dependent notation. NULL
  /// if there's a problem (creating directory failed, etc).
  ///
  /// See Also: [`SDL_GetBasePath`]
  pub fn SDL_GetPrefPath(org: *const c_char, app: *const c_char) -> *mut c_char;
}