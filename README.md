[![License:Zlib](https://img.shields.io/badge/License-Zlib-brightgreen.svg)](https://opensource.org/licenses/Zlib)
[![AppVeyor](https://ci.appveyor.com/api/projects/status/lqvi8qbjayf35v8m/branch/master?svg=true)](https://ci.appveyor.com/project/Lokathor/fermium/branch/master)
[![TravisCI](https://travis-ci.org/Lokathor/fermium.svg?branch=master](https://travis-ci.org/Lokathor/fermium)

# fermium

## Setup

* This library uses
  [bindgen](https://rust-lang.github.io/rust-bindgen/requirements.html), which
  requires a working copy of `clang` (3.9 or later). Their page has instructions
  for installing clang, based on your system.
  * Win32 note: after installing LLVM you must make an environment variable for
    `LIBCLANG_PATH` pointing to the install directory that has `libclang.dll`
    (eg: `D:\programs\LLVM\bin`)
* To **build the library** you need the SDL2 libraries to link against.
  * On Window this is provided for you by the build script.
  * On Mac and Linux it's assumed that you've already installed them via some
    appropriate method (make sure you have `2.0.9` or later). There's a Mac
    installer file on [the SDL2 download
    page](https://www.libsdl.org/download-2.0.php) if you like. It's also fairly
    trivial to build from source in the normal style for a C project:

```sh
curl -L https://www.libsdl.org/release/SDL2-2.0.9.tar.gz | tar xz
cd SDL2-2.0.9
./configure
make
sudo make install
```

* To **run a program** using this library you'll also need the appropriate SDL2
  runtime files in a place the OS can find them. These are available for Windows
  and Mac on [the SDL2 download page](http://libsdl.org/download-2.0.php), and
  on Linux it's trivial to build from source.
  * On Windows, you should generally ship the `SDL2.dll` along side your
    `program.exe` file. The dynamic library lookup starts in the same folder the
    exe lives in, so you shipping a copy of SDL2.dll won't get mixed up with any
    other SDL2.dll they have installed from anywhere else. If a user needs to
    override your version they can just replace the file, but they probably
    won't even need to.
  * On Mac and Linux you should generally _not_ attempt to ship an SDL2 library
    along with your game. Assume that the user already has an appropriate
    library configured and installed with the best settings for their particular
    system (there's just too many fiddly different systems out there). Simply be
    clear that your program requires SDL2-2.0.9 (or later) and direct them to
    the SDL2 download page (Mac users) or remind them how to build it from
    source (Linux users).

In the future the crate will likely have a feature to enable a static link with
SDL2 (necessary on some platforms), but you should avoid a static link if
possible. Using a dynamic link lets users get a newer versions of SDL2 with bug
fixes and patches and it'll "Just Work" with the copy of your program they
already have.

## Features

* `force_bindgen`: By default, the crate will check for a `bindings.rs` output
  already being on disk in its output folder, and not run if it sees one. If you
  need to force the issue for whatever reason, use this feature.
