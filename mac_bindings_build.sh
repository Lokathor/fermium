#!/bin/sh

export PATH="/usr/local/opt/llvm/bin:$PATH"
cargo build --features="link_dynamic, use_bindgen_bin"
