#if defined(__APPLE__)
#define MAC_OS_X_VERSION_MIN_REQUIRED 1060
#endif /* __APPLE__ */

#include "include/SDL.h"

// Note(Lokathor): It seems like we need to explicitly try turning on the video
// driver flags here, *probably* because my "linux dev machine" is a virtual
// server with no video system installed.
#if defined(__unix__)

#ifndef SDL_VIDEO_DRIVER_X11
#define SDL_VIDEO_DRIVER_X11
#endif /* SDL_VIDEO_DRIVER_X11 */

#ifndef SDL_VIDEO_DRIVER_WAYLAND
#define SDL_VIDEO_DRIVER_WAYLAND
#endif /* SDL_VIDEO_DRIVER_WAYLAND */

#endif /* __unix__ */

#include "include/SDL_syswm.h"
