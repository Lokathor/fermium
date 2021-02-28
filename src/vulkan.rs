//! Functions to make Vulkan and SDL work together.
//!
//! This is only a basic level of functionality that lets you "turn it on". For
//! a full Vulkan experience you'll need a vulkan bindings crate such as
//! [ash](https:docs.rs/ash), or similar.

use crate::{c_char, c_int, c_uint, c_void, stdinc::*, video::*};

// makes rustdoc link properly!
#[allow(unused)]
use crate::hints::*;

/// Vulkan instance pointer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkInstance(*mut u8);
impl Default for VkInstance {
  #[inline]
  #[must_use]
  fn default() -> Self {
    unsafe { core::mem::zeroed() }
  }
}

/// Vulkan surface handle.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkSurfaceKHR(u64);

/// Alternate type name in some docs.
pub type SDL_vulkanInstance = VkInstance;

/// Alternate type name in some docs.
pub type SDL_vulkanSurface = VkSurfaceKHR;

extern "C" {
  /// Dynamically load a Vulkan loader library.
  ///
  /// * \[in\] `path` The platform dependent Vulkan loader library name, or
  ///   `NULL`.
  ///
  /// **Returns:** `0` on success, or `-1` if the library couldn't be loaded.
  ///
  /// If `path` is `NULL`, SDL will use the value of the environment variable
  /// `SDL_VULKAN_LIBRARY`, if set, otherwise it loads the default Vulkan
  /// loader library.
  ///
  /// This should be called after initializing the video driver, but *before*
  /// creating any Vulkan windows. If no Vulkan loader library is loaded, the
  /// default library will be loaded upon creation of the first Vulkan window.
  ///
  /// It is fairly common for Vulkan applications to link with `libvulkan`
  /// instead of explicitly loading it at run time. This will work with
  /// SDL provided the application links to a dynamic library and both it
  /// and SDL use the same search path.
  ///
  /// If you specify a non-NULL `path`, an application should retrieve all
  /// of the Vulkan functions it uses from the dynamic library using
  /// [`SDL_Vulkan_GetVkGetInstanceProcAddr`] unless you can guarantee
  /// `path` points to the same vulkan loader library the application
  /// linked to.
  ///
  /// On Apple devices, if `path` is NULL, SDL will attempt to find
  /// the `vkGetInstanceProcAddr` address within all the mach-o images of
  /// the current process. This is because it is fairly common for Vulkan
  /// applications to link with `libvulkan` (and historically MoltenVK was
  /// provided as a static library). If it is not found then, on macOS, SDL
  /// will attempt to load `vulkan.framework/vulkan`, `libvulkan.1.dylib`,
  /// followed by `libvulkan.dylib`, in that order.
  /// On iOS SDL will attempt to load `libvulkan.dylib` only. Applications
  /// using a dynamic framework or .dylib must ensure it is included in its
  /// application bundle.
  ///
  /// On non-Apple devices, application linking with a static `libvulkan` is
  /// not supported. Either do not link to the Vulkan loader or link to a
  /// dynamic library version.
  ///
  /// This function will fail if there are no working Vulkan drivers
  /// installed.
  ///
  /// See Also: [`SDL_Vulkan_GetVkGetInstanceProcAddr`],
  /// [`SDL_Vulkan_UnloadLibrary`]
  pub fn SDL_Vulkan_LoadLibrary(path: *const c_char) -> c_int;

  /// Get the address of the `vkGetInstanceProcAddr` function.
  ///
  /// This should be called after either calling [`SDL_Vulkan_LoadLibrary`]
  /// or creating an [`SDL_Window`] with the [`SDL_WINDOW_VULKAN`] flag.
  pub fn SDL_Vulkan_GetVkGetInstanceProcAddr() -> *mut c_void;

  /// Unload the Vulkan loader library previously loaded by
  /// [`SDL_Vulkan_LoadLibrary`].
  ///
  /// See Also: [`SDL_Vulkan_LoadLibrary`]
  pub fn SDL_Vulkan_UnloadLibrary();

  /// Get the names of the Vulkan instance extensions needed to create a surface
  /// with [`SDL_Vulkan_CreateSurface`].
  ///
  /// * \[in\] `window` Window for which the required Vulkan instance extensions
  ///   should be retrieved (or `NULL`).
  /// * \[in,out\] `pCount` pointer to a `c_uint` related to the number of
  ///   required Vulkan instance extensions (see below).
  /// * \[out\] `pNames` pointer to an array to be filled with the required
  ///   Vulkan instance extensions (or `NULL`).
  ///
  /// **Returns:** `SDL_TRUE` on success, `SDL_FALSE` on error.
  ///
  /// If `pNames` is `NULL`, then the number of required Vulkan instance
  /// extensions is returned in `pCount`. Otherwise, `pCount` must point to a
  /// variable set to the number of elements in the `pNames` array, and on
  /// return the variable is overwritten with the number of names actually
  /// written to `pNames`. If `pCount` is less than the number of required
  /// extensions, at most `pCount` structures will be written. If `pCount` is
  /// smaller than the number of required extensions, `SDL_FALSE` will be
  /// returned instead of `SDL_TRUE`, to indicate that not all the required
  /// extensions were returned.
  ///
  /// If `window` is not `NULL`, it will be checked against its creation flags
  /// to ensure that the Vulkan flag is present. This parameter will be removed
  /// in a future major release.
  ///
  /// The returned list of extensions will contain `VK_KHR_surface` and zero or
  /// more platform specific extensions
  ///
  /// The extension names queried here must be enabled when calling
  /// [`vkCreateInstance`][1], otherwise surface creation will fail.
  ///
  /// [1]: (https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateInstance.html)
  ///
  /// `window` should have been created with the [`SDL_WINDOW_VULKAN`] flag, or
  /// be `NULL`.
  ///
  /// ```c
  /// unsigned int count;
  /// // get count of required extensions
  /// if(!SDL_Vulkan_GetInstanceExtensions(NULL, &count, NULL))
  ///     handle_error();
  ///
  /// static const char *const additionalExtensions[] =
  /// {
  ///     VK_EXT_DEBUG_REPORT_EXTENSION_NAME, // example additional extension
  /// };
  /// size_t additionalExtensionsCount = sizeof(additionalExtensions) / sizeof(additionalExtensions[0]);
  /// size_t extensionCount = count + additionalExtensionsCount;
  /// const char **names = malloc(sizeof(const char *) * extensionCount);
  /// if(!names)
  ///     handle_error();
  ///
  /// // get names of required extensions
  /// if(!SDL_Vulkan_GetInstanceExtensions(NULL, &count, names))
  ///     handle_error();
  ///
  /// // copy additional extensions after required extensions
  /// for(size_t i = 0; i < additionalExtensionsCount; i++)
  ///     names[i + count] = additionalExtensions[i];
  ///
  /// VkInstanceCreateInfo instanceCreateInfo = {};
  /// instanceCreateInfo.enabledExtensionCount = extensionCount;
  /// instanceCreateInfo.ppEnabledExtensionNames = names;
  /// // fill in rest of instanceCreateInfo
  ///
  /// VkInstance instance;
  /// // create the Vulkan instance
  /// VkResult result = vkCreateInstance(&instanceCreateInfo, NULL, &instance);
  /// free(names);
  /// ```
  ///
  /// See Also: [`SDL_Vulkan_CreateSurface`]
  pub fn SDL_Vulkan_GetInstanceExtensions(
    window: *mut SDL_Window, pCount: *mut c_uint, pNames: *mut *const c_char,
  ) -> SDL_bool;
  // TODO: convert the above example to rust

  /// Create a Vulkan rendering surface for a window.
  ///
  /// * \[in\] `window` SDL_Window to which to attach the rendering surface.
  /// * \[in\] `instance` handle to the Vulkan instance to use.
  /// * \[out\] `surface` pointer to a VkSurfaceKHR handle to receive the handle
  ///   of the newly created surface.
  ///
  /// **Returns:** `SDL_TRUE` on success, `SDL_FALSE` on error.
  ///
  /// ```c
  /// VkInstance instance;
  /// SDL_Window *window;
  ///
  /// // create instance and window
  ///
  /// // create the Vulkan surface
  /// VkSurfaceKHR surface;
  /// if(!SDL_Vulkan_CreateSurface(window, instance, &surface))
  ///     handle_error();
  /// ```
  ///
  /// `window` should have been created with the `SDL_WINDOW_VULKAN` flag.
  ///
  /// `instance` should have been created with the extensions returned by
  /// [`SDL_Vulkan_CreateSurface`] enabled.
  ///
  /// See Also: [`SDL_Vulkan_GetInstanceExtensions`]
  pub fn SDL_Vulkan_CreateSurface(
    window: *mut SDL_Window, instance: VkInstance, surface: *mut VkSurfaceKHR,
  ) -> SDL_bool;

  /// Get the size of a window's underlying drawable in pixels (for use with
  /// setting viewport, scissor & etc).
  ///
  /// * `window` The `SDL_Window` from which the drawable size should be
  ///   queried.
  /// * `w` Pointer to variable for storing the width in pixels, may be `NULL`.
  /// * `h` Pointer to variable for storing the height in pixels, may be `NULL`.
  ///
  /// This may differ from [`SDL_GetWindowSize`] if we're rendering to a
  /// high-DPI drawable, i.e. the window was created with
  /// [`SDL_WINDOW_ALLOW_HIGHDPI`] on a platform with high-DPI support (Apple
  /// calls this "Retina"), and not disabled
  /// by the [`SDL_HINT_VIDEO_HIGHDPI_DISABLED`] hint.
  ///
  /// On macOS high-DPI support must be enabled for an application by
  /// setting `NSHighResolutionCapable` to true in its `Info.plist`.
  ///
  /// See Also: [`SDL_GetWindowSize`] and [`SDL_CreateWindow`]
  pub fn SDL_Vulkan_GetDrawableSize(
    window: *mut SDL_Window, w: *mut c_int, h: *mut c_int,
  );
}
