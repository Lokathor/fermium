# Changelog

## 20014.4.0

* mark more fields as `pub` that accidentally were not `pub`: SDL_Keysym and SDL_SensorEvent.

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
