# fermium

## Setup

* [bindgen](https://rust-lang.github.io/rust-bindgen/requirements.html) has
  instructions for installing clang, based on your system.
  * Win32 note: after installing LLVM you must make a variable for
    `LIBCLANG_PATH` pointing to the install directory that has `libclang.dll`
    (eg: `D:\programs\LLVM\bin`)

## Features

* `force_bindgen`: By default, the crate will check for a `bindings.rs` output
  already being on disk in its output folder, and not run if it sees one. If you
  need to force the issue for whatever reason, use this feature.
