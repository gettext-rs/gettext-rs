use gettextrs::*;
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
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");
}

#[test]
fn gettext() {
    let _ = *SETUP;

    assert_eq!(gettext!("Hello, World!"), "Hello, World!");
}

#[test]
fn gettext_trailing_comma() {
    let _ = *SETUP;

    assert_eq!(gettext!("Hello, World!",), "Hello, World!");
}

#[test]
fn gettext_escapes_only() {
    let _ = *SETUP;

    assert_eq!(gettext!("Hello, {{}}!"), "Hello, {}!");
}

#[test]
fn gettext_positional_args() {
    let _ = *SETUP;

    assert_eq!(gettext!("Hello, {}!", "World"), "Hello, World!");
    assert_eq!(gettext!("{}, {}!", "Hello", "World"), "Hello, World!");
    assert_eq!(gettext!("{}, {}{}", "Hello", "World", '!'), "Hello, World!");
}

#[test]
fn gettext_positional_args_and_trailing_comma() {
    let _ = *SETUP;

    assert_eq!(gettext!("Hello, {}!", "World",), "Hello, World!");
    assert_eq!(gettext!("{}, {}!", "Hello", "World",), "Hello, World!");
    assert_eq!(
        gettext!("{}, {}{}", "Hello", "World", '!',),
        "Hello, World!"
    );
}

#[test]
#[ignore]
fn ngettext() {
    let _ = *SETUP;

    assert_eq!(
        ngettext!("Hello, World!", "Hello, Worlds!", 1),
        "Hello, World!"
    );
    assert_eq!(
        ngettext!("Hello, World!", "Hello, Worlds!", 2),
        "Hello, Worlds!"
    );
}
