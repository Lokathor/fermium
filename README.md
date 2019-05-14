[![License:Zlib](https://img.shields.io/badge/License-Zlib-brightgreen.svg)](https://opensource.org/licenses/Zlib)
[![AppVeyor](https://ci.appveyor.com/api/projects/status/lqvi8qbjayf35v8m/branch/master?svg=true)](https://ci.appveyor.com/project/Lokathor/fermium/branch/master)
[![TravisCI](https://travis-ci.org/Lokathor/fermium.svg?branch=master)](https://travis-ci.org/Lokathor/fermium)
[![crates.io](https://img.shields.io/crates/v/fermium.svg)](https://crates.io/crates/fermium)
[![docs.rs](https://docs.rs/fermium/badge.svg)](https://docs.rs/fermium/)

# fermium

* This crate is always `no_std`.
* The bindings use `libc` for the C type declarations.
* There are pre-generated bindings files included for the major platforms
  (Windows MSVC, Mac OS, Linux x86_64, Linux ARM rpi3), and presumably you could
  also run bindgen (via cargo feature) to generate bindings if you're building
  in some other platform. If you do run bindgen for some other platform, PRs are
  welcome (I don't have any Android or iOS expertise).

## Differences From `sdl2-sys`

* Static linking by default (cargo feature for `dynamic_link`)
* Bindings use "scoped constants" instead of "rustified enums".
* I actually pretty much understand the whole build process.

## Using This Crate

### Before Building

The crate supplies the necessary files to build on Windows MSVC without any
previous installation. However, at the moment, it expects that SDL2 developer
files have already been installed to the system if you're building for any other
platform. [Homebrew has SDL2-2.0.9](https://formulae.brew.sh/formula/sdl2), but
most linux package managers have an older version of SDL2 and you'll have to
build from source. You can look in the [travis file](.travis.yml) to see how you
might install SDL2-2.0.9 from source if you decide to go that route. Please note
that you must use `-fPIC` when building SDL2 if you're planning to statically
link to SDL2 (the default mode for this crate).

If you want to run bindgen yourself you'll of course have to [install
bindgen](https://rust-lang.github.io/rust-bindgen/requirements.html). This crate
supports both using bindgen as a library or using bindgen as a CLI application.

If you are not on a platform with a pre-generated file and you don't run bindgen
yourself the build will complete but the crate will be empty. If all the
functions appear to be missing you're probably on an unusual platform and you
didn't run bindgen.

### After Building

By default, `fermium` generates programs that are statically linked to SDL2 with
the [dynamic
API](https://www.reddit.com/r/linux_gaming/comments/1upn39/sdl2_adds_dynamic_api_magic_to_allow_updating_it/)
enabled. This means that the generated binary won't depend on any external SDL2
files. Just send your binary to the user and they can play it out of the box.
However, the user will be able to use the `SDL_DYNAMIC_API` environment variable
to run the program using an alternate SDL2 implementation that they have on
their system (v2.0.9 or later), if they desire.

If you'd link to only have a dynamic link, you can enable the `dynamic_link`
cargo feature. This makes the binary a little smaller (~1.4mb) but then if the
end user doesn't have SDL2 already installed they won't be able to run the
program at all.

## License

This crate uses the Zlib license (the same license that SDL2 itself uses).

## Project Logo

[![birth of fermium](https://upload.wikimedia.org/wikipedia/commons/5/58/Ivy_Mike_-_mushroom_cloud.jpg)](https://en.wikipedia.org/wiki/Fermium)
