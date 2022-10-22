# [Docs.rs](https://docs.rs/fermium/)

# `fermium`

Rust bindings to the [SDL2](https://libsdl.org/) library.

This crate covers most of SDL2's 2.0.16 API. It can be used compatibly with any
newer version of SDL2. It can also even be used with older versions of SDL2,
though if you call any functions not present you'll get a link error, and if you
call an older version with arguments it doesn't expect it'll generally give you
a runtime error.

By default, the crate links to the system version of SDL2 (or bundled pre-built
dev files on Windows MSVC). Alternately, you can have the crate build SDL2 from
source and statically link to that.

## Versioning

The major version of this crate is basically the bundled version of SDL2 with
all the dots taken out. Each new release of the crate that bundles new SDL2
source is a major version bump. There are *generally* no actual breaks in the
crate code itself (though sometimes small updates happen). There are usually
updates and fixes in how the build script runs, or raising the MSRV of the
crate.

## FAQ

* **Lokathor, why does your crate have such a stupid name?**
  * Well, this started as raw layer for a crate called `beryllium`, to replace its usage of `sdl2-sys` with something new.
    Since `sdl2-sys` was already taken, I needed another name, and I decided to pick another element name.
    Somewhat at random, I just decided to check out element 100 on wikipedia, and it was apparently discovered in the fallout of the first hydrogen bomb.
    I thought to myself, "well that's cool enough, I guess", and now we're here.

## Project Logo

[![birth-of-fermium](https://upload.wikimedia.org/wikipedia/commons/5/58/Ivy_Mike_-_mushroom_cloud.jpg)](https://en.wikipedia.org/wiki/Fermium)
