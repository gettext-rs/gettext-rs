extern crate gettextrs;

use gettextrs::*;

#[test]
fn gettext_no_formatting() {
    setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
    bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();
    textdomain("hellorust").unwrap();

    assert_eq!(gettext!("Hello, World!"), "Hello, World!");
}

#[test]
fn dgettext_no_formatting() {
    setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
    bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();

    assert_eq!(dgettext!("hellorust", "Hello, World!"), "Hello, World!");
}

#[test]
fn dcgettext_no_formatting() {
    setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
    bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();

    assert_eq!(
        dcgettext!("hellorust", LocaleCategory::LcAll, "Hello, World!"),
        "Hello, World!"
    );
}

#[test]
fn ngettext_no_formatting() {
    setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
    bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();
    textdomain("hellorust").unwrap();

    assert_eq!(
        ngettext!("There is one result", "There are few results", 2),
        "There are few results"
    );
}

#[test]
fn dngettext_no_formatting() {
    setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
    bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();

    assert_eq!(
        dngettext!(
            "hellorust",
            "There is one result",
            "There are few results",
            2
        ),
        "There are few results"
    );
}

#[test]
fn dcngettext_no_formatting() {
    setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
    bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();

    assert_eq!(
        dcngettext!(
            "hellorust",
            LocaleCategory::LcAll,
            "There is one result",
            "There are few results",
            2
        ),
        "There are few results"
    );
}

#[test]
fn pgettext_no_formatting() {
    setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
    bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();
    textdomain("hellorust").unwrap();

    assert_eq!(pgettext!("context", "Hello, World!"), "Hello, World!");
}

#[test]
fn npgettext_no_formatting() {
    setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
    bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();
    textdomain("hellorust").unwrap();

    assert_eq!(
        npgettext!("context", "There is one result", "There are few results", 2),
        "There are few results"
    );
}

#[test]
fn gettext_basic_formatting() {
    setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
    bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();
    textdomain("hellorust").unwrap();

    assert_eq!(
        gettext!("Hello, {}! {}", "World", "UwU"),
        "Hello, World! UwU"
    );
}

#[test]
fn dgettext_basic_formatting() {
    setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
    bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();

    assert_eq!(
        dgettext!("hellorust", "Hello, {}! {}", "World", "UwU"),
        "Hello, World! UwU"
    );
}

#[test]
fn dcgettext_basic_formatting() {
    setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
    bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();

    assert_eq!(
        dcgettext!(
            "hellorust",
            LocaleCategory::LcAll,
            "Hello, {}! {}",
            "World",
            "UwU"
        ),
        "Hello, World! UwU"
    );
}

#[test]
fn ngettext_basic_formatting() {
    setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
    bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();
    textdomain("hellorust").unwrap();

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
fn dngettext_basic_formatting() {
    setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
    bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();

    assert_eq!(
        dngettext!(
            "hellorust",
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
fn dcngettext_basic_formatting() {
    setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
    bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();

    assert_eq!(
        dcngettext!(
            "hellorust",
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
fn pgettext_basic_formatting() {
    setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
    bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();
    textdomain("hellorust").unwrap();

    assert_eq!(
        pgettext!("context", "Hello, {}! {}", "World", "UwU"),
        "Hello, World! UwU"
    );
}

#[test]
fn npgettext_basic_formatting() {
    setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
    bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();
    textdomain("hellorust").unwrap();

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
fn gettext_fewer_arguments_than_parameters() {
    setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
    bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();
    textdomain("hellorust").unwrap();

    assert_eq!(gettext!("Hello, {}! {}", "World"), "Hello, World! {}");
}

#[test]
#[cfg_attr(
    debug_assertions,
    should_panic = "There are fewer arguments than format directives"
)]
fn dgettext_fewer_arguments_than_parameters() {
    setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
    bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();

    assert_eq!(
        dgettext!("hellorust", "Hello, {}! {}", "World"),
        "Hello, World! {}"
    );
}

#[test]
#[cfg_attr(
    debug_assertions,
    should_panic = "There are fewer arguments than format directives"
)]
fn dcgettext_fewer_arguments_than_parameters() {
    setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
    bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();

    assert_eq!(
        dcgettext!("hellorust", LocaleCategory::LcAll, "Hello, {}! {}", "World"),
        "Hello, World! {}"
    );
}

#[test]
#[cfg_attr(
    debug_assertions,
    should_panic = "There are fewer arguments than format directives"
)]
fn ngettext_fewer_arguments_than_parameters() {
    setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
    bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();
    textdomain("hellorust").unwrap();

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
fn dngettext_fewer_arguments_than_parameters() {
    setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
    bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();

    assert_eq!(
        dngettext!(
            "hellorust",
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
fn dcngettext_fewer_arguments_than_parameters() {
    setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
    bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();

    assert_eq!(
        dcngettext!(
            "hellorust",
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
fn pgettext_fewer_arguments_than_parameters() {
    setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
    bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();
    textdomain("hellorust").unwrap();

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
fn npgettext_fewer_arguments_than_parameters() {
    setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
    bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();
    textdomain("hellorust").unwrap();

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
fn gettext_more_arguments_than_parameters() {
    setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
    bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();
    textdomain("hellorust").unwrap();

    assert_eq!(gettext!("Hello, {}!", "World", "UwU"), "Hello, World!");
}

#[test]
#[cfg_attr(
    debug_assertions,
    should_panic = "There are more arguments than format directives"
)]
fn dgettext_more_arguments_than_parameters() {
    setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
    bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();

    assert_eq!(
        dgettext!("hellorust", "Hello, {}!", "World", "UwU"),
        "Hello, World!"
    );
}

#[test]
#[cfg_attr(
    debug_assertions,
    should_panic = "There are more arguments than format directives"
)]
fn dcgettext_more_arguments_than_parameters() {
    setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
    bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();

    assert_eq!(
        dcgettext!(
            "hellorust",
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
fn ngettext_more_arguments_than_parameters() {
    setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
    bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();
    textdomain("hellorust").unwrap();

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
fn dngettext_more_arguments_than_parameters() {
    setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
    bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();

    assert_eq!(
        dngettext!(
            "hellorust",
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
fn dcngettext_more_arguments_than_parameters() {
    setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
    bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();

    assert_eq!(
        dcngettext!(
            "hellorust",
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
fn pgettext_more_arguments_than_parameters() {
    setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
    bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();
    textdomain("hellorust").unwrap();

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
fn npgettext_more_arguments_than_parameters() {
    setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
    bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();
    textdomain("hellorust").unwrap();

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
fn ngettext_special_n_formatting() {
    setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
    bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();
    textdomain("hellorust").unwrap();

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
fn dngettext_special_n_formatting() {
    setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
    bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();

    assert_eq!(
        dngettext!(
            "hellorust",
            "There is {n} apple! Only {n}!",
            "There are {n} apples! Only {n}!",
            2
        ),
        "There are 2 apples! Only 2!"
    );
    assert_eq!(
        dngettext!(
            "hellorust",
            "There is one apple! Only one!",
            "There are {n} apples! Only {n}!",
            1
        ),
        "There is one apple! Only one!"
    );
    assert_eq!(
        dngettext!(
            "hellorust",
            "There is one apple! Only one!",
            "There are {n} apples! Only {n}!",
            2
        ),
        "There are 2 apples! Only 2!"
    );
    assert_eq!(
        dngettext!(
            "hellorust",
            "There is {n} \"{}\" in text! Only {n}!",
            "There are {n} \"{}\" in text! Only {n}!",
            2,
            "UwU"
        ),
        "There are 2 \"UwU\" in text! Only 2!"
    );
}

#[test]
fn dcngettext_special_n_formatting() {
    setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
    bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();

    assert_eq!(
        dcngettext!(
            "hellorust",
            LocaleCategory::LcAll,
            "There is {n} apple! Only {n}!",
            "There are {n} apples! Only {n}!",
            2
        ),
        "There are 2 apples! Only 2!"
    );
    assert_eq!(
        dcngettext!(
            "hellorust",
            LocaleCategory::LcAll,
            "There is one apple! Only one!",
            "There are {n} apples! Only {n}!",
            1
        ),
        "There is one apple! Only one!"
    );
    assert_eq!(
        dcngettext!(
            "hellorust",
            LocaleCategory::LcAll,
            "There is one apple! Only one!",
            "There are {n} apples! Only {n}!",
            2
        ),
        "There are 2 apples! Only 2!"
    );
    assert_eq!(
        dcngettext!(
            "hellorust",
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
fn npgettext_special_n_formatting() {
    setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
    bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();
    textdomain("hellorust").unwrap();

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
fn gettext_trailing_comma() {
    setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
    bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();
    textdomain("hellorust").unwrap();

    assert_eq!(gettext!("Hello, World!",), "Hello, World!");
    assert_eq!(
        gettext!("Hello, {}! {}", "World", "UwU",),
        "Hello, World! UwU"
    );
}

#[test]
fn dgettext_trailing_comma() {
    setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
    bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();

    assert_eq!(dgettext!("hellorust", "Hello, World!",), "Hello, World!");
    assert_eq!(
        dgettext!("hellorust", "Hello, {}! {}", "World", "UwU",),
        "Hello, World! UwU"
    );
}

#[test]
fn dcgettext_trailing_comma() {
    setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
    bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();

    assert_eq!(
        dcgettext!("hellorust", LocaleCategory::LcAll, "Hello, World!"),
        "Hello, World!"
    );
    assert_eq!(
        dcgettext!(
            "hellorust",
            LocaleCategory::LcAll,
            "Hello, {}! {}",
            "World",
            "UwU",
        ),
        "Hello, World! UwU"
    );
}

#[test]
fn ngettext_trailing_comma() {
    setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
    bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();
    textdomain("hellorust").unwrap();

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
fn dngettext_trailing_comma() {
    setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
    bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();

    assert_eq!(
        dngettext!(
            "hellorust",
            "There is one result",
            "There are few results",
            2,
        ),
        "There are few results"
    );
    assert_eq!(
        dngettext!(
            "hellorust",
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
fn dcngettext_trailing_comma() {
    setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
    bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();

    assert_eq!(
        dcngettext!(
            "hellorust",
            LocaleCategory::LcAll,
            "There is one result",
            "There are few results",
            2,
        ),
        "There are few results"
    );
    assert_eq!(
        dcngettext!(
            "hellorust",
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
fn pgettext_trailing_comma() {
    setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
    bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();
    textdomain("hellorust").unwrap();

    assert_eq!(pgettext!("context", "Hello, World!",), "Hello, World!");
    assert_eq!(
        pgettext!("context", "Hello, {}! {}", "World", "UwU",),
        "Hello, World! UwU"
    );
}

#[test]
fn npgettext_trailing_comma() {
    setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
    bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();
    textdomain("hellorust").unwrap();

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
