//! Lets you get info about the runtime SDL version.

pub use crate::{c_char, stdinc::*};

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

#[allow(missing_docs)]
pub const SDL_MAJOR_VERSION: u8 = 2;
#[allow(missing_docs)]
pub const SDL_MINOR_VERSION: u8 = 0;
#[allow(missing_docs)]
pub const SDL_PATCHLEVEL: u8 = 12;

extern "C" {
  /// Get the version of SDL that is linked against your program.
  ///
  /// It is possible that the runtime SDL version will be different than the
  /// version you compiled against.
  ///
  /// This function may be called safely at any time, even before [`SDL_Init`].
  pub fn SDL_GetVersion(ver: *mut SDL_version);

  /// Get the code revision of SDL that is linked against your program.
  ///
  /// Returns an arbitrary string (a hash value) uniquely identifying the exact
  /// revision of the SDL library in use. This is only useful in comparing
  /// against other revisions. It is NOT an incrementing number.
  pub fn SDL_GetRevision() -> *const c_char;
}
