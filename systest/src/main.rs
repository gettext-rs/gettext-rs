#![allow(bad_style)]
#![allow(unused_imports)]
#![allow(unused_macros)]

extern crate gettext_sys;

use gettext_sys::*;

include!(concat!(env!("OUT_DIR"), "/all.rs"));
