[![License:Zlib](https://img.shields.io/badge/License-Zlib-brightgreen.svg)](https://opensource.org/licenses/Zlib)
![Minimum Rust Version](https://img.shields.io/badge/Min%20Rust-1.33-green.svg)
[![AppVeyor](https://ci.appveyor.com/api/projects/status/lqvi8qbjayf35v8m/branch/master?svg=true)](https://ci.appveyor.com/project/Lokathor/fermium/branch/master)
[![TravisCI](https://travis-ci.org/Lokathor/fermium.svg?branch=master)](https://travis-ci.org/Lokathor/fermium)
[![crates.io](https://img.shields.io/crates/v/fermium.svg)](https://crates.io/crates/fermium)
[![docs.rs](https://docs.rs/fermium/badge.svg)](https://docs.rs/fermium/)

# fermium

The `fermium` crate is raw bindings to the SDL2 C API.

Currently binds to `SDL2-2.0.12`.

For the high-level wrapper crate, please see [beryllium](https://github.com/Lokathor/beryllium).

## Platforms

On windows, this crate carries the necessary files to automatically "just work".

On Mac or Linux you'll need to have SDL2 installed via your method of choice.

This does not work with iOS, Android, or Emscripten (PRs accepted if you know how to make it do the thing).

## Versioning Explanation

SDL2 as a library doesn't follow semver, so this crate can't quite follow semver either.

* With standard SemVer, a version is given as `major.minor.patch`
* With SDL2, a version is basically `2.major.minor`
  * Also, starting with `2.0.10`, even minor versions are "release" and odd minor versions are "dev", similar to how the Linux kernel works.
  * They update the final value when there's new functions or improvements to old functions. In this case, there's no actual ABI breaks to any part of the library.
  * _Not that there ever has been_, but if there were to be an ABI break to the library, then they would update the major version.

So, to try and make the `fermium` version indicate the SDL2 version that it's trying to bind to, while also trying to play nice with cargo, we'll have a major version of `200` to reprisent the `2.0.` part of things, and then we'll set the minor version to be the SDL2 minor version (such as `12`), and then we'll use patch releases if we need to put out an update in between when SDL2 does releases.

## Project Logo

[![birth of fermium](https://upload.wikimedia.org/wikipedia/commons/5/58/Ivy_Mike_-_mushroom_cloud.jpg)](https://en.wikipedia.org/wiki/Fermium)
