[![License:Zlib](https://img.shields.io/badge/License-Zlib-brightgreen.svg)](https://opensource.org/licenses/Zlib)
[![AppVeyor](https://ci.appveyor.com/api/projects/status/lqvi8qbjayf35v8m/branch/master?svg=true)](https://ci.appveyor.com/project/Lokathor/fermium/branch/master)
[![TravisCI](https://travis-ci.org/Lokathor/fermium.svg?branch=master)](https://travis-ci.org/Lokathor/fermium)

# fermium

This is always `no_std`. To define the C types involved, the bindings use
`winapi` on Windows and `libc` elsewhere.

## Setup Details

* **Before Building:** Since this crate is about SDL2 bindings, it obviously
  requires SDL2. You will need the latest stable version of SDL2 (currently
  `2.0.9`).
  * For Windows, the crate packages the MSVC development libraries from the
    [SDL2 download page](https://www.libsdl.org/download-2.0.php)
  * For Linux, there are no provided downloads, they tell you to just install
    using your package manager. However, the version that's usually in package
    managers is often not up to date, so honestly just build it from source. The
    [travis file](.travis.yml) of this repository has an example.
  * For Mac you can [download the
    dmg](https://www.libsdl.org/release/SDL2-2.0.9.dmg) installer from the SDL2
    website, or use [Homebrew](https://formulae.brew.sh/formula/sdl2#default),
    or build from source. Note that if you build from source yourself there's a
    few special flags you need to use or you'll get build errors. The [travis
    file](.travis.yml) has an example of how to use the [gcc-fat.sh](gcc-fat.sh)
    script to make things build properly.

* **Building:** This library uses
  [bindgen](https://github.com/rust-lang/rust-bindgen) as a `build-dependency`
  to create the actual bindings via a `build.rs` script. This requires a working
  copy of `clang` (3.9 or later) to be installed. The `bindgen` [requirements
  page](https://rust-lang.github.io/rust-bindgen/requirements.html) has
  instructions for installing clang for each major OS.
  * _Note For Windows Users:_ after installing LLVM you must manually make an
    environment variable for `LIBCLANG_PATH` pointing to the install directory
    that has `libclang.dll` (eg: `D:\programs\LLVM\bin`)

* **Running:** Once you've compiled your program using this library, you'll need
  to have the SDL2 dynamic library somewhere that the OS can find for your
  program to run.
  * For Windows, the SDL2 download page has both a [32-bit
    version](https://www.libsdl.org/release/SDL2-2.0.9-win32-x86.zip) and
    [64-bit version](https://www.libsdl.org/release/SDL2-2.0.9-win32-x86.zip) of
    `SDL2.dll` available. It's also in the [lib directory](lib/) of this
    repository.
  * On Mac and Linux, generally whatever you used to install SDL2 will have
    placed the shared library in the correct system directory for all the shared
    libraries to go. If this is somehow not the case, you can set
    `LD_LIBRARY_PATH` to point to an appropriate directory.

* **Shipping:** When you're ready to send a program out the door, how should you
  package SDL2 along with your program? Well, it depends.
  * For Windows, you should probably just send your `SDL2.dll` file along side
    your `program.exe`. DLL loading starts by checking the same directory as the
    executable, so it's sure to find the correct DLL that way. If the user
    really needs to they can replace the DLL later. Having one copy of
    `SDL2.dll` per program isn't a huge deal, it's only about 1.4 megabytes.
  * For Mac and Linux you _could_ ship a copy of the SDL2 shared library with
    your program, but it's honestly better to assume that the user already has
    it correctly installed they way that they want things setup, and just give
    them instructions to install it if they somehow don't have SDL2 at all.
  * For Steam OS you literally aren't allowed to ship your own version of SDL2,
    they keep the latest version installed as part of the OS setup, and you're
    not allowed to mess with it.

## License

This crate uses the Zlib license (the same license as SDL2 itself uses).
