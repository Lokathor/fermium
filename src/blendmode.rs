/// A newtype'd blend operation value.
///
/// See `SDL_BLENDMODE_*` for examples.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_BlendMode(pub u32);

/// no blending
/// ```txt
/// dstRGBA = srcRGBA
/// ```
pub const SDL_BLENDMODE_NONE: SDL_BlendMode = SDL_BlendMode(0x00000000);
/// alpha blending
/// ```txt
/// dstRGB = (srcRGB * srcA) + (dstRGB * (1-srcA))
/// dstA = srcA + (dstA * (1-srcA))
/// ```
pub const SDL_BLENDMODE_BLEND: SDL_BlendMode = SDL_BlendMode(0x00000001);
/// additive blending
/// ```txt
/// dstRGB = (srcRGB * srcA) + dstRGB
/// dstA = dstA
/// ```
pub const SDL_BLENDMODE_ADD: SDL_BlendMode = SDL_BlendMode(0x00000002);
/// color modulate
/// ```txt
/// dstRGB = srcRGB * dstRGB
/// dstA = dstA
/// ```
pub const SDL_BLENDMODE_MOD: SDL_BlendMode = SDL_BlendMode(0x00000004);
/// color multiply
/// ```txt
/// dstRGB = (srcRGB * dstRGB) + (dstRGB * (1-srcA))
/// dstA = (srcA * dstA) + (dstA * (1-srcA))
/// ```
pub const SDL_BLENDMODE_MUL: SDL_BlendMode = SDL_BlendMode(0x00000008);
/// Sentinel for an invalid blend mode.
pub const SDL_BLENDMODE_INVALID: SDL_BlendMode = SDL_BlendMode(0x7FFFFFFF);

/// A newtype'd blend operation value.
///
/// See `SDL_BLENDOPERATION_*` for examples.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_BlendOperation(pub u32);

/// Op: dst + src: supported by all renderers
pub const SDL_BLENDOPERATION_ADD: SDL_BlendOperation = SDL_BlendOperation(0x1);
/// Op: dst - src : supported by D3D9, D3D11, OpenGL, OpenGLES
pub const SDL_BLENDOPERATION_SUBTRACT: SDL_BlendOperation =
  SDL_BlendOperation(0x2);
/// Op: src - dst : supported by D3D9, D3D11, OpenGL, OpenGLES
pub const SDL_BLENDOPERATION_REV_SUBTRACT: SDL_BlendOperation =
  SDL_BlendOperation(0x3);
/// Op: min(dst, src) : supported by D3D11
pub const SDL_BLENDOPERATION_MINIMUM: SDL_BlendOperation =
  SDL_BlendOperation(0x4);
/// Op: max(dst, src) : supported by D3D11
pub const SDL_BLENDOPERATION_MAXIMUM: SDL_BlendOperation =
  SDL_BlendOperation(0x5);

/// A newtype'd blend factor value.
///
/// See the `SDL_BLENDFACTOR_*` constants for values of this type.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_BlendFactor(pub u32);

/// RGBA: 0, 0, 0, 0
pub const SDL_BLENDFACTOR_ZERO: SDL_BlendFactor = SDL_BlendFactor(0x1);
/// RGBA: 1, 1, 1, 1
pub const SDL_BLENDFACTOR_ONE: SDL_BlendFactor = SDL_BlendFactor(0x2);
/// RGBA: srcR, srcG, srcB, srcA
pub const SDL_BLENDFACTOR_SRC_COLOR: SDL_BlendFactor = SDL_BlendFactor(0x3);
/// RGBA: 1-srcR, 1-srcG, 1-srcB, 1-srcA
pub const SDL_BLENDFACTOR_ONE_MINUS_SRC_COLOR: SDL_BlendFactor =
  SDL_BlendFactor(0x4);
/// RGBA: srcA, srcA, srcA, srcA
pub const SDL_BLENDFACTOR_SRC_ALPHA: SDL_BlendFactor = SDL_BlendFactor(0x5);
/// RGBA: 1-srcA, 1-srcA, 1-srcA, 1-srcA
pub const SDL_BLENDFACTOR_ONE_MINUS_SRC_ALPHA: SDL_BlendFactor =
  SDL_BlendFactor(0x6);
/// RGBA: dstR, dstG, dstB, dstA
pub const SDL_BLENDFACTOR_DST_COLOR: SDL_BlendFactor = SDL_BlendFactor(0x7);
/// RGBA: 1-dstR, 1-dstG, 1-dstB, 1-dstA
pub const SDL_BLENDFACTOR_ONE_MINUS_DST_COLOR: SDL_BlendFactor =
  SDL_BlendFactor(0x8);
/// RGBA: dstA, dstA, dstA, dstA
pub const SDL_BLENDFACTOR_DST_ALPHA: SDL_BlendFactor = SDL_BlendFactor(0x9);
/// RGBA: 1-dstA, 1-dstA, 1-dstA, 1-dstA
pub const SDL_BLENDFACTOR_ONE_MINUS_DST_ALPHA: SDL_BlendFactor =
  SDL_BlendFactor(0xA);

extern "C" {
  /// Create a custom blend mode, which may or may not be supported by a given
  /// renderer
  ///
  /// * `srcColorFactor`: source color factor
  /// * `dstColorFactor`: destination color factor
  /// * `colorOperation`: color operation
  /// * `srcAlphaFactor`: source alpha factor
  /// * `dstAlphaFactor`: destination alpha factor
  /// * `alphaOperation`: alpha operation
  ///
  /// The result of the blend mode operation will be:
  /// ```txt
  /// dstRGB = dstRGB * dstColorFactor colorOperation srcRGB * srcColorFactor
  /// ```
  /// and
  /// ```txt
  /// dstA = dstA * dstAlphaFactor alphaOperation srcA * srcAlphaFactor
  /// ```
  pub fn SDL_ComposeCustomBlendMode(
    srcColorFactor: SDL_BlendFactor, dstColorFactor: SDL_BlendFactor,
    colorOperation: SDL_BlendOperation, srcAlphaFactor: SDL_BlendFactor,
    dstAlphaFactor: SDL_BlendFactor, alphaOperation: SDL_BlendOperation,
  ) -> SDL_BlendMode;
}
