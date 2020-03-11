#!/bin/sh

cargo clean
cargo build --features="dynamic_link, use_bindgen_bin"
