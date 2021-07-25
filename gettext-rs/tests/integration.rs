extern crate gettextrs;
#[macro_use]
extern crate lazy_static;

use gettextrs::{getters::*, *};

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
fn current_textdomain_works() {
    let _ = *SETUP;

    assert_eq!(
        current_textdomain().unwrap(),
        "initialized_domain".as_bytes()
    );
}

#[test]
fn domain_directory_works() {
    let _ = *SETUP;

    use std::path::PathBuf;

    assert_eq!(
        domain_directory("bound_domain").unwrap(),
        PathBuf::from("/usr/local/share/locale")
    );
}

#[test]
fn test_textdomain_codeset() {
    let _ = *SETUP;

    assert_eq!(
        textdomain_codeset("c_domain").unwrap(),
        Some("C".to_string())
    );

    assert_eq!(
        textdomain_codeset("utf-8_domain").unwrap(),
        Some("UTF-8".to_string())
    );
}

#[test]
fn gettext_fn() {
    let _ = *SETUP;

    assert_eq!(gettext("Hello, World!"), "Hello, World!");
}

#[test]
fn dgettext_fn() {
    let _ = *SETUP;

    assert_eq!(
        current_textdomain().unwrap(),
        "initialized_domain".as_bytes()
    );

    assert_eq!(dgettext("bound_domain", "Hello, World!"), "Hello, World!");
}

#[test]
fn dcgettext_fn() {
    let _ = *SETUP;

    assert_eq!(
        current_textdomain().unwrap(),
        "initialized_domain".as_bytes()
    );

    assert_eq!(
        dcgettext("bound_domain", "Hello, World!", LocaleCategory::LcAll),
        "Hello, World!"
    );
}

#[test]
fn pgettext_fn() {
    let _ = *SETUP;

    assert_eq!(
        current_textdomain().unwrap(),
        "initialized_domain".as_bytes()
    );

    assert_eq!(pgettext("context", "Hello, World!"), "Hello, World!");
}

#[test]
fn ngettext_fn() {
    let _ = *SETUP;

    assert_eq!(
        current_textdomain().unwrap(),
        "initialized_domain".as_bytes()
    );

    assert_eq!(
        ngettext("Hello, World!", "Hello, Worlds!", 1),
        "Hello, World!"
    );
    assert_eq!(
        ngettext("Hello, World!", "Hello, Worlds!", 2),
        "Hello, Worlds!"
    );
}

#[test]
fn dngettext_fn() {
    let _ = *SETUP;

    assert_eq!(
        current_textdomain().unwrap(),
        "initialized_domain".as_bytes()
    );

    assert_eq!(
        dngettext("bound_domain", "Hello, World!", "Hello, Worlds!", 1),
        "Hello, World!"
    );
    assert_eq!(
        dngettext("bound_domain", "Hello, World!", "Hello, Worlds!", 2),
        "Hello, Worlds!"
    );
}

#[test]
fn dcngettext_fn() {
    let _ = *SETUP;

    assert_eq!(
        current_textdomain().unwrap(),
        "initialized_domain".as_bytes()
    );

    assert_eq!(
        dcngettext(
            "bound_domain",
            "Hello, World!",
            "Hello, Worlds!",
            1,
            LocaleCategory::LcAll
        ),
        "Hello, World!"
    );
    assert_eq!(
        dcngettext(
            "bound_domain",
            "Hello, World!",
            "Hello, Worlds!",
            2,
            LocaleCategory::LcAll
        ),
        "Hello, Worlds!"
    );
}

#[test]
fn npgettext_fn() {
    let _ = *SETUP;

    assert_eq!(
        current_textdomain().unwrap(),
        "initialized_domain".as_bytes()
    );

    assert_eq!(
        npgettext("context", "Hello, World!", "Hello, Worlds!", 1),
        "Hello, World!"
    );
    assert_eq!(
        npgettext("context", "Hello, World!", "Hello, Worlds!", 2),
        "Hello, Worlds!"
    );
}

#[test]
fn gettext_macro_no_formatting() {
    let _ = *SETUP;

    assert_eq!(gettext!("Hello, World!"), "Hello, World!");
}

#[test]
fn dgettext_macro_no_formatting() {
    let _ = *SETUP;

    assert_eq!(dgettext!("bound_domain", "Hello, World!"), "Hello, World!");
}

#[test]
fn dcgettext_macro_no_formatting() {
    let _ = *SETUP;

    assert_eq!(
        dcgettext!("bound_domain", LocaleCategory::LcAll, "Hello, World!"),
        "Hello, World!"
    );
}

#[test]
fn ngettext_macro_no_formatting() {
    let _ = *SETUP;

    assert_eq!(
        ngettext!("There is one result", "There are few results", 2),
        "There are few results"
    );
}

#[test]
fn dngettext_macro_no_formatting() {
    let _ = *SETUP;

    assert_eq!(
        dngettext!(
            "bound_domain",
            "There is one result",
            "There are few results",
            2
        ),
        "There are few results"
    );
}

#[test]
fn dcngettext_macro_no_formatting() {
    let _ = *SETUP;

    assert_eq!(
        dcngettext!(
            "bound_domain",
            LocaleCategory::LcAll,
            "There is one result",
            "There are few results",
            2
        ),
        "There are few results"
    );
}

#[test]
fn pgettext_macro_no_formatting() {
    let _ = *SETUP;

    assert_eq!(pgettext!("context", "Hello, World!"), "Hello, World!");
}

#[test]
fn npgettext_macro_no_formatting() {
    let _ = *SETUP;

    assert_eq!(
        npgettext!("context", "There is one result", "There are few results", 2),
        "There are few results"
    );
}

#[test]
fn gettext_macro_basic_formatting() {
    let _ = *SETUP;

    assert_eq!(
        gettext!("Hello, {}! {}", "World", "UwU"),
        "Hello, World! UwU"
    );
}

#[test]
fn dgettext_macro_basic_formatting() {
    let _ = *SETUP;

    assert_eq!(
        dgettext!("bound_domain", "Hello, {}! {}", "World", "UwU"),
        "Hello, World! UwU"
    );
}

#[test]
fn dcgettext_macro_basic_formatting() {
    let _ = *SETUP;

    assert_eq!(
        dcgettext!(
            "bound_domain",
            LocaleCategory::LcAll,
            "Hello, {}! {}",
            "World",
            "UwU"
        ),
        "Hello, World! UwU"
    );
}

#[test]
fn ngettext_macro_basic_formatting() {
    let _ = *SETUP;

    assert_eq!(
        ngettext!(
            "There is one \"{}\" in text! {}",
            "There are few \"{}\" in text! {}",
            2,
            "UwU",
            "OwO"
        ),
        "There are few \"UwU\" in text! OwO"
    );
}

#[test]
fn dngettext_macro_basic_formatting() {
    let _ = *SETUP;

    assert_eq!(
        dngettext!(
            "bound_domain",
            "There is one \"{}\" in text! {}",
            "There are few \"{}\" in text! {}",
            2,
            "UwU",
            "OwO"
        ),
        "There are few \"UwU\" in text! OwO"
    );
}

#[test]
fn dcngettext_macro_basic_formatting() {
    let _ = *SETUP;

    assert_eq!(
        dcngettext!(
            "bound_domain",
            LocaleCategory::LcAll,
            "There is one \"{}\" in text! {}",
            "There are few \"{}\" in text! {}",
            2,
            "UwU",
            "OwO"
        ),
        "There are few \"UwU\" in text! OwO"
    );
}

#[test]
fn pgettext_macro_basic_formatting() {
    let _ = *SETUP;

    assert_eq!(
        pgettext!("context", "Hello, {}! {}", "World", "UwU"),
        "Hello, World! UwU"
    );
}

#[test]
fn npgettext_macro_basic_formatting() {
    let _ = *SETUP;

    assert_eq!(
        npgettext!(
            "context",
            "There is one \"{}\" in text! {}",
            "There are few \"{}\" in text! {}",
            2,
            "UwU",
            "OwO"
        ),
        "There are few \"UwU\" in text! OwO"
    );
}

#[test]
#[cfg_attr(
    debug_assertions,
    should_panic = "There are fewer arguments than format directives"
)]
fn gettext_macro_fewer_arguments_than_parameters() {
    let _ = *SETUP;

    assert_eq!(gettext!("Hello, {}! {}", "World"), "Hello, World! {}");
}

#[test]
#[cfg_attr(
    debug_assertions,
    should_panic = "There are fewer arguments than format directives"
)]
fn dgettext_macro_fewer_arguments_than_parameters() {
    let _ = *SETUP;

    assert_eq!(
        dgettext!("bound_domain", "Hello, {}! {}", "World"),
        "Hello, World! {}"
    );
}

#[test]
#[cfg_attr(
    debug_assertions,
    should_panic = "There are fewer arguments than format directives"
)]
fn dcgettext_macro_fewer_arguments_than_parameters() {
    let _ = *SETUP;

    assert_eq!(
        dcgettext!(
            "bound_domain",
            LocaleCategory::LcAll,
            "Hello, {}! {}",
            "World"
        ),
        "Hello, World! {}"
    );
}

#[test]
#[cfg_attr(
    debug_assertions,
    should_panic = "There are fewer arguments than format directives"
)]
fn ngettext_macro_fewer_arguments_than_parameters() {
    let _ = *SETUP;

    assert_eq!(
        ngettext!(
            "There is one \"{}\" in text! {}",
            "There are few \"{}\" in text! {}",
            2,
            "UwU"
        ),
        "There are few \"UwU\" in text! {}"
    );
}

#[test]
#[cfg_attr(
    debug_assertions,
    should_panic = "There are fewer arguments than format directives"
)]
fn dngettext_macro_fewer_arguments_than_parameters() {
    let _ = *SETUP;

    assert_eq!(
        dngettext!(
            "bound_domain",
            "There is one \"{}\" in text! {}",
            "There are few \"{}\" in text! {}",
            2,
            "UwU"
        ),
        "There are few \"UwU\" in text! {}"
    );
}

#[test]
#[cfg_attr(
    debug_assertions,
    should_panic = "There are fewer arguments than format directives"
)]
fn dcngettext_macro_fewer_arguments_than_parameters() {
    let _ = *SETUP;

    assert_eq!(
        dcngettext!(
            "bound_domain",
            LocaleCategory::LcAll,
            "There is one \"{}\" in text! {}",
            "There are few \"{}\" in text! {}",
            2,
            "UwU"
        ),
        "There are few \"UwU\" in text! {}"
    );
}

#[test]
#[cfg_attr(
    debug_assertions,
    should_panic = "There are fewer arguments than format directives"
)]
fn pgettext_macro_fewer_arguments_than_parameters() {
    let _ = *SETUP;

    assert_eq!(
        pgettext!("context", "Hello, {}! {}", "World"),
        "Hello, World! {}"
    );
}

#[test]
#[cfg_attr(
    debug_assertions,
    should_panic = "There are fewer arguments than format directives"
)]
fn npgettext_macro_fewer_arguments_than_parameters() {
    let _ = *SETUP;

    assert_eq!(
        npgettext!(
            "context",
            "There is one \"{}\" in text! {}",
            "There are few \"{}\" in text! {}",
            2,
            "UwU"
        ),
        "There are few \"UwU\" in text! {}"
    );
}

#[test]
#[cfg_attr(
    debug_assertions,
    should_panic = "There are more arguments than format directives"
)]
fn gettext_macro_more_arguments_than_parameters() {
    let _ = *SETUP;

    assert_eq!(gettext!("Hello, {}!", "World", "UwU"), "Hello, World!");
}

#[test]
#[cfg_attr(
    debug_assertions,
    should_panic = "There are more arguments than format directives"
)]
fn dgettext_macro_more_arguments_than_parameters() {
    let _ = *SETUP;

    assert_eq!(
        dgettext!("bound_domain", "Hello, {}!", "World", "UwU"),
        "Hello, World!"
    );
}

#[test]
#[cfg_attr(
    debug_assertions,
    should_panic = "There are more arguments than format directives"
)]
fn dcgettext_macro_more_arguments_than_parameters() {
    let _ = *SETUP;

    assert_eq!(
        dcgettext!(
            "bound_domain",
            LocaleCategory::LcAll,
            "Hello, {}!",
            "World",
            "UwU"
        ),
        "Hello, World!"
    );
}

#[test]
#[cfg_attr(
    debug_assertions,
    should_panic = "There are more arguments than format directives"
)]
fn ngettext_macro_more_arguments_than_parameters() {
    let _ = *SETUP;

    assert_eq!(
        ngettext!(
            "There is one \"{}\" in text!",
            "There are few \"{}\" in text!",
            2,
            "UwU",
            "OwO"
        ),
        "There are few \"UwU\" in text!"
    );
}

#[test]
#[cfg_attr(
    debug_assertions,
    should_panic = "There are more arguments than format directives"
)]
fn dngettext_macro_more_arguments_than_parameters() {
    let _ = *SETUP;

    assert_eq!(
        dngettext!(
            "bound_domain",
            "There is one \"{}\" in text!",
            "There are few \"{}\" in text!",
            2,
            "UwU",
            "OwO"
        ),
        "There are few \"UwU\" in text!"
    );
}

#[test]
#[cfg_attr(
    debug_assertions,
    should_panic = "There are more arguments than format directives"
)]
fn dcngettext_macro_more_arguments_than_parameters() {
    let _ = *SETUP;

    assert_eq!(
        dcngettext!(
            "bound_domain",
            LocaleCategory::LcAll,
            "There is one \"{}\" in text!",
            "There are few \"{}\" in text!",
            2,
            "UwU",
            "OwO"
        ),
        "There are few \"UwU\" in text!"
    );
}

#[test]
#[cfg_attr(
    debug_assertions,
    should_panic = "There are more arguments than format directives"
)]
fn pgettext_macro_more_arguments_than_parameters() {
    let _ = *SETUP;

    assert_eq!(
        pgettext!("context", "Hello, {}!", "World", "UwU"),
        "Hello, World!"
    );
}

#[test]
#[cfg_attr(
    debug_assertions,
    should_panic = "There are more arguments than format directives"
)]
fn npgettext_macro_more_arguments_than_parameters() {
    let _ = *SETUP;

    assert_eq!(
        npgettext!(
            "context",
            "There is one \"{}\" in text!",
            "There are few \"{}\" in text!",
            2,
            "UwU",
            "OwO"
        ),
        "There are few \"UwU\" in text!"
    );
}

#[test]
fn ngettext_macro_special_n_formatting() {
    let _ = *SETUP;

    assert_eq!(
        ngettext!(
            "There is {n} apple! Only {n}!",
            "There are {n} apples! Only {n}!",
            2
        ),
        "There are 2 apples! Only 2!"
    );
    assert_eq!(
        ngettext!(
            "There is one apple! Only one!",
            "There are {n} apples! Only {n}!",
            1
        ),
        "There is one apple! Only one!"
    );
    assert_eq!(
        ngettext!(
            "There is one apple! Only one!",
            "There are {n} apples! Only {n}!",
            2
        ),
        "There are 2 apples! Only 2!"
    );
    assert_eq!(
        ngettext!(
            "There is {n} \"{}\" in text! Only {n}!",
            "There are {n} \"{}\" in text! Only {n}!",
            2,
            "UwU"
        ),
        "There are 2 \"UwU\" in text! Only 2!"
    );
}

#[test]
fn dngettext_macro_special_n_formatting() {
    let _ = *SETUP;

    assert_eq!(
        dngettext!(
            "bound_domain",
            "There is {n} apple! Only {n}!",
            "There are {n} apples! Only {n}!",
            2
        ),
        "There are 2 apples! Only 2!"
    );
    assert_eq!(
        dngettext!(
            "bound_domain",
            "There is one apple! Only one!",
            "There are {n} apples! Only {n}!",
            1
        ),
        "There is one apple! Only one!"
    );
    assert_eq!(
        dngettext!(
            "bound_domain",
            "There is one apple! Only one!",
            "There are {n} apples! Only {n}!",
            2
        ),
        "There are 2 apples! Only 2!"
    );
    assert_eq!(
        dngettext!(
            "bound_domain",
            "There is {n} \"{}\" in text! Only {n}!",
            "There are {n} \"{}\" in text! Only {n}!",
            2,
            "UwU"
        ),
        "There are 2 \"UwU\" in text! Only 2!"
    );
}

#[test]
fn dcngettext_macro_special_n_formatting() {
    let _ = *SETUP;

    assert_eq!(
        dcngettext!(
            "bound_domain",
            LocaleCategory::LcAll,
            "There is {n} apple! Only {n}!",
            "There are {n} apples! Only {n}!",
            2
        ),
        "There are 2 apples! Only 2!"
    );
    assert_eq!(
        dcngettext!(
            "bound_domain",
            LocaleCategory::LcAll,
            "There is one apple! Only one!",
            "There are {n} apples! Only {n}!",
            1
        ),
        "There is one apple! Only one!"
    );
    assert_eq!(
        dcngettext!(
            "bound_domain",
            LocaleCategory::LcAll,
            "There is one apple! Only one!",
            "There are {n} apples! Only {n}!",
            2
        ),
        "There are 2 apples! Only 2!"
    );
    assert_eq!(
        dcngettext!(
            "bound_domain",
            LocaleCategory::LcAll,
            "There is {n} \"{}\" in text! Only {n}!",
            "There are {n} \"{}\" in text! Only {n}!",
            2,
            "UwU"
        ),
        "There are 2 \"UwU\" in text! Only 2!"
    );
}

#[test]
fn npgettext_macro_special_n_formatting() {
    let _ = *SETUP;

    assert_eq!(
        npgettext!(
            "context",
            "There is {n} apple! Only {n}!",
            "There are {n} apples! Only {n}!",
            2
        ),
        "There are 2 apples! Only 2!"
    );
    assert_eq!(
        npgettext!(
            "context",
            "There is one apple! Only one!",
            "There are {n} apples! Only {n}!",
            1
        ),
        "There is one apple! Only one!"
    );
    assert_eq!(
        npgettext!(
            "context",
            "There is one apple! Only one!",
            "There are {n} apples! Only {n}!",
            2
        ),
        "There are 2 apples! Only 2!"
    );
    assert_eq!(
        npgettext!(
            "context",
            "There is {n} \"{}\" in text! Only {n}!",
            "There are {n} \"{}\" in text! Only {n}!",
            2,
            "UwU"
        ),
        "There are 2 \"UwU\" in text! Only 2!"
    );
}

#[test]
fn gettext_macro_trailing_comma() {
    let _ = *SETUP;

    assert_eq!(gettext!("Hello, World!",), "Hello, World!");
    assert_eq!(
        gettext!("Hello, {}! {}", "World", "UwU",),
        "Hello, World! UwU"
    );
}

#[test]
fn dgettext_macro_trailing_comma() {
    let _ = *SETUP;

    assert_eq!(dgettext!("bound_domain", "Hello, World!",), "Hello, World!");
    assert_eq!(
        dgettext!("bound_domain", "Hello, {}! {}", "World", "UwU",),
        "Hello, World! UwU"
    );
}

#[test]
fn dcgettext_macro_trailing_comma() {
    let _ = *SETUP;

    assert_eq!(
        dcgettext!("bound_domain", LocaleCategory::LcAll, "Hello, World!"),
        "Hello, World!"
    );
    assert_eq!(
        dcgettext!(
            "bound_domain",
            LocaleCategory::LcAll,
            "Hello, {}! {}",
            "World",
            "UwU",
        ),
        "Hello, World! UwU"
    );
}

#[test]
fn ngettext_macro_trailing_comma() {
    let _ = *SETUP;

    assert_eq!(
        ngettext!("There is one result", "There are few results", 2,),
        "There are few results"
    );
    assert_eq!(
        ngettext!(
            "There is one \"{}\" in text! {}",
            "There are few \"{}\" in text! {}",
            2,
            "UwU",
            "OwO",
        ),
        "There are few \"UwU\" in text! OwO"
    );
}

#[test]
fn dngettext_macro_trailing_comma() {
    let _ = *SETUP;

    assert_eq!(
        dngettext!(
            "bound_domain",
            "There is one result",
            "There are few results",
            2,
        ),
        "There are few results"
    );
    assert_eq!(
        dngettext!(
            "bound_domain",
            "There is one \"{}\" in text! {}",
            "There are few \"{}\" in text! {}",
            2,
            "UwU",
            "OwO",
        ),
        "There are few \"UwU\" in text! OwO"
    );
}

#[test]
fn dcngettext_macro_trailing_comma() {
    let _ = *SETUP;

    assert_eq!(
        dcngettext!(
            "bound_domain",
            LocaleCategory::LcAll,
            "There is one result",
            "There are few results",
            2,
        ),
        "There are few results"
    );
    assert_eq!(
        dcngettext!(
            "bound_domain",
            LocaleCategory::LcAll,
            "There is one \"{}\" in text! {}",
            "There are few \"{}\" in text! {}",
            2,
            "UwU",
            "OwO",
        ),
        "There are few \"UwU\" in text! OwO"
    );
}

#[test]
fn pgettext_macro_trailing_comma() {
    let _ = *SETUP;

    assert_eq!(pgettext!("context", "Hello, World!",), "Hello, World!");
    assert_eq!(
        pgettext!("context", "Hello, {}! {}", "World", "UwU",),
        "Hello, World! UwU"
    );
}

#[test]
fn npgettext_macro_trailing_comma() {
    let _ = *SETUP;

    assert_eq!(
        npgettext!("context", "There is one result", "There are few results", 2,),
        "There are few results"
    );
    assert_eq!(
        npgettext!(
            "context",
            "There is one \"{}\" in text! {}",
            "There are few \"{}\" in text! {}",
            2,
            "UwU",
            "OwO",
        ),
        "There are few \"UwU\" in text! OwO"
    );
}
