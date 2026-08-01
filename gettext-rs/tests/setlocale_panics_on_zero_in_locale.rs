extern crate gettextrs;

use gettextrs::*;
use std::panic;

fn main() {
    let msg = panic::catch_unwind(|| setlocale(LocaleCategory::LcCollate, "en_\0US"))
    .expect_err("Expected setlocale call to panic but it did not")
    .downcast::<String>()
    .expect(
        "Expected setlocale call to panic with a `String` error, but it panicked with another type",
    );
    if !msg.starts_with("`locale` contains an internal 0 byte") {
        panic!("Unexpected setlocale error message: {}", msg)
    };
}
