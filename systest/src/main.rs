#![allow(bad_style)]
#![allow(unused_imports)]
extern crate gettext_sys;

#[cfg(not(feature = "cross_compiling"))]
use gettext_sys::*;

#[cfg(not(cross_compiling))]
include!(concat!(env!("OUT_DIR"), "/all.rs"));

#[cfg(feature = "cross_compiling")]
fn main() {
    println!("Cross-compilation detected. Skipping system tests.");
}

#[cfg(feature = "cross_compiling")]
#[test]
fn dummy_cross_compile_test() {
    println!("Cross-compilation detected. Skipping system tests.");
}
