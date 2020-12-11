//! Lets you get info about the runtime SDL version.

use crate::{c_char, stdinc::*};

/// Information about the version of SDL in use.
///
/// Represents the library's version as three levels:
/// * major revision (increments with massive changes, additions, and
///   enhancements)
/// * minor revision (increments with backwards-compatible changes to the major
///   revision)
/// * patchlevel (increments with fixes to the minor revision).
///
/// See Also: [`SDL_VERSION`], [`SDL_GetVersion`]
///
/// **To Be Clear: The SDL library doesn't subscribe to SemVer.**
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
#[allow(missing_docs)]
pub struct SDL_version {
  /// major version
  pub major: Uint8,
  /// minor version
  pub minor: Uint8,
  /// update version
  pub patch: Uint8,
}

/// SDL Major version that this lib was compiled against.
pub const SDL_MAJOR_VERSION: u8 = 2;
/// SDL Minor version that this lib was compiled against.
pub const SDL_MINOR_VERSION: u8 = 0;
/// SDL Patch version that this lib was compiled against.
pub const SDL_PATCHLEVEL: u8 = 12;

/// Writes the version of SDL that this lib was compiled against into the
/// struct.
pub fn SDL_VERSION(x: &mut SDL_version) {
  x.major = SDL_MAJOR_VERSION;
  x.minor = SDL_MINOR_VERSION;
  x.patch = SDL_PATCHLEVEL;
}

extern "C" {
  /// Get the version of SDL that is being used at runtime.
  ///
  /// It is possible that the runtime SDL version will be higher than the
  /// version you compiled against. In this case, the version will still be ABI
  /// compatible with the version you targeted at compile time.
  ///
  /// This function may be called safely at any time, even before [`SDL_Init`].
  pub fn SDL_GetVersion(ver: *mut SDL_version);

  /// Get the "code revision" of SDL that is being used at runtime.
  ///
  /// Returns an arbitrary string (a hash value) uniquely identifying the exact
  /// code revision of the SDL library in use. This is only useful in comparing
  /// against other revisions. It is NOT an incrementing number.
  ///
  /// This is a static string, do not free it.
  pub fn SDL_GetRevision() -> *const c_char;
}
