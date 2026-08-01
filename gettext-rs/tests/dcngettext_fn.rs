extern crate gettextrs;

use gettextrs::{getters::*, *};

fn main() {
    setlocale(LocaleCategory::LcAll, "en_US.UTF-8");

    bindtextdomain("bound_domain", "/usr/local/share/locale").unwrap();

    bindtextdomain("initialized_domain", "/usr/local/share/locale").unwrap();
    textdomain("initialized_domain").unwrap();

    bind_textdomain_codeset("c_domain", "C").unwrap();
    bind_textdomain_codeset("utf-8_domain", "UTF-8").unwrap();

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
            LocaleCategory::LcMessages
        ),
        "Hello, World!"
    );
    assert_eq!(
        dcngettext(
            "bound_domain",
            "Hello, World!",
            "Hello, Worlds!",
            2,
            LocaleCategory::LcMessages
        ),
        "Hello, Worlds!"
    );
}
