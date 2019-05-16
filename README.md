[![License:Zlib](https://img.shields.io/badge/License-Zlib-brightgreen.svg)](https://opensource.org/licenses/Zlib)
[![AppVeyor](https://ci.appveyor.com/api/projects/status/lqvi8qbjayf35v8m/branch/master?svg=true)](https://ci.appveyor.com/project/Lokathor/fermium/branch/master)
[![TravisCI](https://travis-ci.org/Lokathor/fermium.svg?branch=master)](https://travis-ci.org/Lokathor/fermium)
[![crates.io](https://img.shields.io/crates/v/fermium.svg)](https://crates.io/crates/fermium)
[![docs.rs](https://docs.rs/fermium/badge.svg)](https://docs.rs/fermium/)

# fermium

* Has pre-generated bindings files for major platforms
  * win32-msvc-x86_64, linux-x86_64, linux-arm32 (rpi3), mac-x86_64,
  * You can build with either the `use_bindgen_bin` or `use_bindgen_lib`
    features to make your own bindings if you need to. PRs are accepted if you
    generate bindings for a new platform!
* Performs static linking to SDL2 by default, with the [Dynamic
  API](https://www.reddit.com/r/linux_gaming/comments/1upn39/sdl2_adds_dynamic_api_magic_to_allow_updating_it/)
  enabled.
  * There is a `dynamic_link` cargo feature if you really want.
* Does not attempt to automatically set up SDL2 itself on non-Windows platforms.
  * On win32-msvc-x86_64 there are provided build artifacts for both a static
    link and dynamic link build.
  * On Mac you should use Homebrew: `brew install sdl2`
  * On Linux you should install SDL2-2.0.9 or later using any desired method.
    This version is not currently available in many package managers, so you
    might have to build from source. There's a [shell script](install-sdl2.sh)
    that I use for the CI builds, if you'd like to use that.
* The whole crate is `no_std` of course.
* The bindings use `libc` for the C type declarations.

## Major Differences From `sdl2-sys`

* Static linking by default (cargo feature for `dynamic_link`)
* Bindings use "scoped constants" instead of "rustified enums".
* The `build.rs` file is simple and easy to understand.

## License

This crate uses the Zlib license, the same license that SDL2 itself uses.

## Project Logo

[![birth of fermium](https://upload.wikimedia.org/wikipedia/commons/5/58/Ivy_Mike_-_mushroom_cloud.jpg)](https://en.wikipedia.org/wiki/Fermium)
