#!/bin/sh

export PATH="/usr/local/opt/llvm/bin:$PATH"
cargo clean
cargo build --features="dynamic_link, use_bindgen_bin"
