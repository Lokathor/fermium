#if defined(__APPLE__)
#define MAC_OS_X_VERSION_MIN_REQUIRED 1060
#endif /* __APPLE__ */

/*
Note(Lokathor): As a sanity check against the fact that I usually generate the
linux bindings on a headless linux server, we will try to forcibly turn on the
X11 and wayland driver stuff.
*/
#if defined(__unix__)

#ifndef SDL_VIDEO_DRIVER_X11
#define SDL_VIDEO_DRIVER_X11
#endif /* SDL_VIDEO_DRIVER_X11 */

#ifndef SDL_VIDEO_DRIVER_WAYLAND
#define SDL_VIDEO_DRIVER_WAYLAND
#endif /* SDL_VIDEO_DRIVER_WAYLAND */

#endif /* __unix__ */

#include "include/SDL.h"

#include "include/SDL_syswm.h"
