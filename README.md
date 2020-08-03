[![License:Zlib](https://img.shields.io/badge/License-Zlib-brightgreen.svg)](https://opensource.org/licenses/Zlib)
![Minimum Rust Version](https://img.shields.io/badge/Min%20Rust-1.33-green.svg)
[![crates.io](https://img.shields.io/crates/v/fermium.svg)](https://crates.io/crates/fermium)
[![docs.rs](https://docs.rs/fermium/badge.svg)](https://docs.rs/fermium/)

# fermium

The `fermium` crate is raw bindings to the SDL2 C API.

Currently this targets `SDL2-2.0.14`.

It uses a bundled copy of SDL2 on Windows, and the system version on Mac/Linux.

## Versioning

SDL2 doesn't follow SemVer, so this crate can't easily follow SemVer.

Instead, this crate uses the entire SDL2 version targeted as the major version,
with the minor and patch versions being "normal semver" from there.
* `SDL2-2.0.14` --> `fermium-20014.0.0`

## Project Logo

[![birth of fermium](https://upload.wikimedia.org/wikipedia/commons/5/58/Ivy_Mike_-_mushroom_cloud.jpg)](https://en.wikipedia.org/wiki/Fermium)
