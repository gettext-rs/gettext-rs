use ::gettextrs::*;

mod helpers;

#[test]
fn translate() {
    fake!(dgettext, "Привіт, світ!");

    assert_eq!(dgettext!("domainname", "Hello, World!"), "Привіт, світ!");
}

#[test]
fn format() {
    fake!(dgettext, "Привіт, {}!");

    assert_eq!(
        dgettext!("domainname", "Hello, {}!", "Username"),
        "Привіт, Username!"
    );
}

#[test]
fn fallback() {
    fake!(dgettext, "Привіт, }{!");

    assert_eq!(
        dgettext!("domainname", "Hello, {}!", "Username"),
        "Hello, Username!"
    );
}

#[test]
fn handle_unique_argument() {
    let world = "World".to_string();
    let closure = || world; // This can only be called once

    assert_eq!(
        dgettext!("domainname", "Hello, {}!", closure()),
        "Hello, World!"
    );
}
