use gettext_macros::*;
use gettextrs::{getters::*, *};
use lazy_static::lazy_static;

lazy_static! {
    // These tests work with global resource, that is set up here once,
    // and shouldn't be modified in tests.
    static ref SETUP: () = {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");

        bindtextdomain("bound_domain", "/usr/local/share/locale").unwrap();

        bindtextdomain("initialized_domain", "/usr/local/share/locale").unwrap();
        textdomain("initialized_domain").unwrap();

        bind_textdomain_codeset("c_domain", "C").unwrap();
        bind_textdomain_codeset("utf-8_domain", "UTF-8").unwrap();
    };
}

#[test]
fn gettext_macro() {
    let _ = *SETUP;

    assert_eq!(gettext!("Hello, World!"), "Hello, World!");
}
