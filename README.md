[![License:Zlib](https://img.shields.io/badge/License-Zlib-brightgreen.svg)](https://opensource.org/licenses/Zlib)
![Minimum Rust Version](https://img.shields.io/badge/Min%20Rust-1.33-green.svg)
[![AppVeyor](https://ci.appveyor.com/api/projects/status/lqvi8qbjayf35v8m/branch/master?svg=true)](https://ci.appveyor.com/project/Lokathor/fermium/branch/master)
[![TravisCI](https://travis-ci.org/Lokathor/fermium.svg?branch=master)](https://travis-ci.org/Lokathor/fermium)
[![crates.io](https://img.shields.io/crates/v/fermium.svg)](https://crates.io/crates/fermium)
[![docs.rs](https://docs.rs/fermium/badge.svg)](https://docs.rs/fermium/)

# fermium

The `fermium` crate is raw bindings to the SDL2 C API.

Compared to the common alternative,
[sdl2-sys](https://crates.io/crates/sdl2-sys), this is has consts instead of
enums, it is slightly more complete, and it works _much_ better on windows MSVC
(no special setup at all).

* There are pre-generated bindings for the following targets:
  * armv7-unknown-linux-gnueabihf
  * i686-pc-windows-msvc
  * x86_64-apple-darwin
  * x86_64-pc-windows-msvc
  * x86_64-unknown-linux-gnu
* If your platform supports SDL2 but isn't on that list, please send in a PR!
  * Install the `clang-3.9`, `libclang-3.9-dev`, and `llvm` packages for your
    platform. Might be under different names, depending on distro and such.
  * Then `cargo install bindgen`
  * Then `cargo build --features="use_bindgen_bin"`
  * All the results go in to the `OUT_DIR`, something like
    `target/debug/build/fermium-LONGHASHCODE/out`.
  * There should be a 2.0.8, 2.0.9, and 2.0.10 version of the bindings for your
    target, it does all three versions in a single build. 
  * Just PR those new files and I'll get it up on crates.io as soon as I can.

You can dynamic link or static link.

* Dynamic linking is the default, and is the officially suggested linking style,
  both by me and also by the SDL2 project.
  * On Windows just grab the "Runtime Binaries" from [The SDL
    Website](https://libsdl.org/download-2.0.php) or [from the fermium
    repository](https://github.com/Lokathor/fermium/blob/master/win32-devel-files/VC/lib/x64).
    Put it in your project directory for development, and ship it with your
    program when you release something.
  * If you're feeling lazy, you can `cargo install fermium` and then just run
    `fermium` and it'll dump `SDL2.dll` into the current directory. It will automatically use either the 32-bit or 64-bit version.
  * On not-windows you should get SDL2 through your distro package manager
    (linux) or homebrew (Mac).
* Static linking is available.
  * On Windows this will automatically build the static library for you. This
    takes some extra time, but is otherwise fully automated and you don't need
    to perform any special steps.
  * On not-Windows please be aware that not all package managers distribute SDL2
    configured for static linking, so you might have to build the library from
    source with static linking enabled.

The default API target level is 2.0.8, but you can enable features to add in
functions from 2.0.9 or 2.0.10.

* Even without the features enabled your program will build and run with any
  newer version of SDL2 as well, and you'll get all relevent bug fixes and such,
  you just can't call the newer functions.
* Please be aware that while Windows and Mac both have easy access to 2.0.10,
  Most of the Linux distributions still have 2.0.8 or 2.0.9 in their package
  managers. Unless you _really_ need those newer functions, you might as well
  stay at the 2.0.8 API level for now.

## License

This crate uses the Zlib license, the same license that SDL2 itself uses.

* The `old-headers-only/` directory contains header files from older versions of
  SDL2 and its add-ons, for use by bindgen when needed.
* The `full-source/` directory contains all the source needed to potentially
  build SDL2 and its add-ons on windows, for use with static linking.
* The `win32-devel-files/` directory has dynamic lib files for use when building
  on Windows. As with the `full-source/` directory, spare files have been
  deleted to save on space.

## Project Logo

[![birth of fermium](https://upload.wikimedia.org/wikipedia/commons/5/58/Ivy_Mike_-_mushroom_cloud.jpg)](https://en.wikipedia.org/wiki/Fermium)
