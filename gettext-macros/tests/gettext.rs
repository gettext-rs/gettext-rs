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
    fake!(gettext, "Привіт, }{!");

    assert_eq!(gettext!("Hello, {}!", "Username"), "Hello, Username!");
}

#[test]
fn handle_unique_argument() {
    let world = "World".to_string();
    let closure = || world; // This can only be called once

    assert_eq!(gettext!("Hello, {}!", closure()), "Hello, World!");
}
