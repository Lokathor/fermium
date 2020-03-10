#!/bin/sh

cargo clean
cargo build --features="link_dynamic, use_bindgen_bin"
