![Minimum Rust Version](https://img.shields.io/badge/Min%20Rust-1.48-green.svg)
[![crates.io](https://img.shields.io/crates/v/fermium.svg)](https://crates.io/crates/fermium)
[![docs.rs](https://docs.rs/fermium/badge.svg)](https://docs.rs/fermium/)

# `fermium`

Rust bindings to the [SDL2](https://libsdl.org/) library.

This crate bundles the source for SDL2 and then builds it for you.
In other words, you do not need to have installed SDL2 yourself.

## Versioning

SDL2 doesn't follow SemVer, so this crate can't easily follow SemVer either.

Instead, this crate uses the entire SDL2 release version as the major version,
with the minor and patch versions being "normal semver" from there.
* `SDL2-2.0.12` --> `fermium-20012.0.0`

## Project Logo

[![birth of fermium](https://upload.wikimedia.org/wikipedia/commons/5/58/Ivy_Mike_-_mushroom_cloud.jpg)](https://en.wikipedia.org/wiki/Fermium)
