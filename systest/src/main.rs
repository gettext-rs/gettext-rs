#![allow(bad_style)]

extern crate gettext_sys;

use gettext_sys::*;

include!(concat!(env!("OUT_DIR"), "/all.rs"));
