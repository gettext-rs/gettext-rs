use ::gettextrs::*;

mod helpers;

#[test]
fn translate() {
    fake!(gettext, "Привіт, світ!");

    assert_eq!(gettext!("Hello, World!"), "Привіт, світ!");
}

#[test]
fn format() {
    fake!(gettext, "Привіт, {}!");

    assert_eq!(gettext!("Hello, {}!", "Username"), "Привіт, Username!");
}

#[test]
fn fallback() {
    fake!(gettext, "Привіт, }{");

    assert_eq!(gettext!("Hello, {}!", "Username"), "Hello, Username!");
}
