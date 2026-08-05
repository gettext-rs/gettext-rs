extern crate gettextrs;

use gettextrs::{getters::*, *};

fn main() {
    unsafe {
        // Safety: `setlocale` is safe to call because at this point, the program is
        // single-threaded.
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
    }

    bindtextdomain("bound_domain", "/usr/local/share/locale").unwrap();

    bindtextdomain("initialized_domain", "/usr/local/share/locale").unwrap();
    textdomain("initialized_domain").unwrap();

    bind_textdomain_codeset("c_domain", "C").unwrap();
    bind_textdomain_codeset("utf-8_domain", "UTF-8").unwrap();

    assert_eq!(
        current_textdomain().unwrap(),
        "initialized_domain".as_bytes()
    );

    assert_eq!(dgettext("bound_domain", "Hello, World!"), "Hello, World!");
}
