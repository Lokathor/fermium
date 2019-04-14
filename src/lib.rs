#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// This is bindgen's fault and some day we can hopefully remove it:
// https://github.com/rust-lang/rust-bindgen/issues/1549
#![allow(improper_ctypes)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
