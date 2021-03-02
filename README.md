![Minimum Rust Version](https://img.shields.io/badge/Min%20Rust-1.48-green.svg)
[![crates.io](https://img.shields.io/crates/v/fermium.svg)](https://crates.io/crates/fermium)
[![docs.rs](https://docs.rs/fermium/badge.svg)](https://docs.rs/fermium/)

# `fermium`

Rust bindings to the [SDL2](https://libsdl.org/) library, version 2.0.14

This crate bundles the source for SDL2 and then builds it for you.
In other words, you do not need to have installed SDL2 yourself.
You do need a C compiler, but everyone has one of those, even on Windows.

## Versioning

SDL2 does **not** follow SemVer.

Under SDL2's version scheme, the numbers are what SemVer would call `major.major.minor`.
In other words, 2.0.10 is a compatible API upgrade from 2.0.9, and 2.1.0 is a *breaking* upgrade from 2.0.9.

Also, as of 2.0.10, they moved to a Linux style "even numbers are releases" version.
The current stable version is 2.0.14, and the next stable version will be 2.0.16.

Because SDL2 doesn't follow SemVer, it's slightly harder for `fermium` to follow SemVer.

Since I want it to be very easy to identify what version of SDL2 is bundled in what version of `fermium`,
we use the full SDL2 release version as the major version number of this crate:

> `SDL2-2.0.14` --> `fermium-20014.0.0`

Updates *within* a `fermium` major version will all target the same SDL2 version,
and follow the normal SemVer rules.

## FAQ

* **Lokathor, why does your crate have such a stupid name?**
  * Well, this started as raw layer for a crate called `beryllium`, to replace its usage of `sdl2-sys` with something new.
    Since `sdl2-sys` was already taken, I needed another name, and I decided to pick another element to go with the first element.
    Somewhat at random, I just decided to check out element 100 on wikipedia, and it was apparently discovered in the fallout of the first hydrogen bomb.
    I thought to myself, "well that's cool enough, I guess.", and now we're here.

## Project Logo

[![birth of fermium](https://upload.wikimedia.org/wikipedia/commons/5/58/Ivy_Mike_-_mushroom_cloud.jpg)](https://en.wikipedia.org/wiki/Fermium)
