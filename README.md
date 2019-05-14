[![License:Zlib](https://img.shields.io/badge/License-Zlib-brightgreen.svg)](https://opensource.org/licenses/Zlib)
[![AppVeyor](https://ci.appveyor.com/api/projects/status/lqvi8qbjayf35v8m/branch/master?svg=true)](https://ci.appveyor.com/project/Lokathor/fermium/branch/master)
[![TravisCI](https://travis-ci.org/Lokathor/fermium.svg?branch=master)](https://travis-ci.org/Lokathor/fermium)
[![crates.io](https://img.shields.io/crates/v/fermium.svg)](https://crates.io/crates/fermium)
[![docs.rs](https://docs.rs/fermium/badge.svg)](https://docs.rs/fermium/)

# fermium

This is always `no_std`.The bindings use `libc` for the C type declarations.

Currently only supports the desktop platforms: Windows (MSVC), Mac, and Linux.

## Using This Crate

* **Before Building:** Since this crate is about SDL2 bindings, it obviously
  requires SDL2. You will need the latest stable version of SDL2 (currently
  `2.0.9`). Theoretically this crate could be configured for an older version
  with minimal effort, but this has not been tested.
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

* **Building:** There are pre-generated bindings files for common platforms. By
  default, with no features enabled, it will pull in the pre-generated bindings
  file of the current platform and use that. If this fails, you will need to
  generated some bindings yourself using `bindgen`.
  * If you need to make your own bindings, you can install the `bindgen` binary
    and have this crate run it as a CLI tool, or you can have this crate build
    it as a library and run it that way. Either way please see the [requirements
    page](https://rust-lang.github.io/rust-bindgen/requirements.html) for
    information on how to setup bindgen if you don't already have it. **Windows
    Users:** after installing LLVM as instructed you must manually make an
    environment variable for `LIBCLANG_PATH` pointing to the install directory
    that has `libclang.dll` (eg: `D:\programs\LLVM\bin`).
  * When building you can also activate the `static_link_sdl2_use_with_caution`
    feature if you would like to staticly link SDL2 instead of the normal
    dynamic link.
  * On non-Windows platforms, this feature requires that your copy of SDL2 was
    compiled wih the `-fPIC` flag.
  * On non-Windows platforms this feature will add every directory in the
    `LD_LIBRARY_PATH` list to be searched for the static lib files (which
    _should_ find SDL2 just fine).
  * As the name implies, you should generally _not_ statically link SDL2, unless
    your target platform absolutely requires it.

* **Running:** Once you've compiled your program using this library, you'll need
  to have the SDL2 dynamic library somewhere that the OS can find for your
  program to run (unless you went with a static link).
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

This crate uses the Zlib license (the same license that SDL2 itself uses).
