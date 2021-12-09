# Changelog

## 20016.1.0

* Added `raw-window-handle` (0.4) support via feature. This makes a properly
  filled out `SDL_SysWMinfo` value able to turn itself into the correct
  `RawWindowHandle` enum.

## 20016.0.0

* Updates the SDL2 version to 2.0.16. This adds the following:
  * SDL_FlashWindow
  * SDL_GetAudioDeviceSpec
  * SDL_SetWindowAlwaysOnTop
  * SDL_SetWindowKeyboardGrab
  * SDL_SetWindowMouseGrab
  * SDL_GetWindowKeyboardGrab
  * SDL_GetWindowMouseGrab
  * SDL_UpdateNVTexture
  * SDL_GameControllerSendEffect
  * Hint(Linux): SDL_HINT_AUDIO_INCLUDE_MONITORS
  * Hint(Linux): SDL_HINT_AUDIO_DEVICE_STREAM_ROLE

As before, functions in `SDL_system.h` aren't included.
(I bring this up only because 2.0.16 did add some new system stuff.)

## 20014.4.2

* Correct the value of `SDL_PIXELFORMAT_RGBA32`, `SDL_PIXELFORMAT_ARGB32`, `SDL_PIXELFORMAT_BGRA32`, and `SDL_PIXELFORMAT_ABGR32`.

## 20014.4.1

* Correct the value of `SDLK_ESCAPE`

## 20014.4.0

* mark more fields as `pub` that accidentally were not `pub`: `SDL_Keysym` and `SDL_SensorEvent`.

## 20014.3.0

* made the `fermium` binary which will write out the x86_64 dll in the current directory.
* marked the inner fields of `VkInstance` and `VkSurfaceKHR` as `pub` to improve inter-operation with other vulkan libraries.
* Added `SDL_GetWindowFromID`

## 20014.2.0

* Added the `SDL_GLContext::is_null` method.
* Added the C types into the export list of the `prelude`.
* Docs for `SDL_Init` now list the return code meaning.
* Fixed up all the intra-doc links so they should point to the right stuff.

## 20014.1.0

Added a prelude module to make importing things easy.

## 20014.0.0

Supports SDL2 version 2.0.14
