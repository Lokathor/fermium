//! Operations for SDL's rectangle type.

pub use crate::{error::*, pixels::*, rwops::*, stdinc::*};

/// The structure that defines a point (integer)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
#[allow(missing_docs)]
pub struct SDL_Point {
  pub x: c_int,
  pub y: c_int,
}

/// The structure that defines a point (floating)
#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
#[repr(C)]
#[allow(missing_docs)]
pub struct SDL_FPoint {
  pub x: f32,
  pub y: f32,
}

/// A rectangle, with the origin at the upper left (integer).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
#[allow(missing_docs)]
pub struct SDL_Rect {
  pub x: c_int,
  pub y: c_int,
  pub w: c_int,
  pub h: c_int,
}

/// A rectangle, with the origin at the upper left (floating).
#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
#[repr(C)]
#[allow(missing_docs)]
pub struct SDL_FRect {
  pub x: f32,
  pub y: f32,
  pub w: f32,
  pub h: f32,
}

/// Returns `true` if a point resides inside a rectangle.
#[inline(always)]
pub const fn SDL_PointInRect(p: SDL_Point, r: SDL_Rect) -> bool {
  (p.x >= r.x) && (p.x < (r.x + r.w)) && (p.y >= r.y) && (p.y < (r.y + r.h))
}

/// Returns `true` if the rectangle has no area.
#[inline(always)]
pub const fn SDL_RectEmpty(r: SDL_Rect) -> bool {
  (r.w <= 0) || (r.h <= 0)
}

extern "C" {
  /// Determine whether two rectangles intersect.
  ///
  /// **Return:** `SDL_TRUE` if there is an intersection, `SDL_FALSE` otherwise.
  pub fn SDL_HasIntersection(
    A: *const SDL_Rect, B: *const SDL_Rect,
  ) -> SDL_bool;

  /// Calculate the intersection of two rectangles.
  ///
  /// **Return:** `SDL_TRUE` if there is an intersection, `SDL_FALSE` otherwise.
  pub fn SDL_IntersectRect(
    A: *const SDL_Rect, B: *const SDL_Rect, result: *mut SDL_Rect,
  ) -> SDL_bool;

  /// Calculate the union of two rectangles.
  pub fn SDL_UnionRect(
    A: *const SDL_Rect, B: *const SDL_Rect, result: *mut SDL_Rect,
  );

  /// Calculate a minimal rectangle enclosing a set of points.
  ///
  /// **Return:** `SDL_TRUE` if any points were within the clipping rect
  pub fn SDL_EnclosePoints(
    points: *const SDL_Point, count: c_int, clip: *const SDL_Rect,
    result: *mut SDL_Rect,
  ) -> SDL_bool;

  /// Calculate the intersection of a rectangle and line segment.
  ///
  /// **Return:** `SDL_TRUE` if there is an intersection, `SDL_FALSE` otherwise.
  pub fn SDL_IntersectRectAndLine(
    rect: *const SDL_Rect, X1: *mut c_int, Y1: *mut c_int, X2: *mut c_int,
    Y2: *mut c_int,
  ) -> SDL_bool;
}
