use ::gettextrs::*;

mod helpers;

// Invocation

#[test]
fn nothing_to_format() {
    assert_eq!(gettext!("Hello, World!"), "Hello, World!");
}

#[test]
fn trailing_comma() {
    assert_eq!(gettext!("Hello, World!",), "Hello, World!");
}

#[test]
fn positional_arg() {
    assert_eq!(gettext!("Hello, {}!", "World"), "Hello, World!");
}

#[test]
fn positional_arg_trailing_comma() {
    assert_eq!(gettext!("Hello, {}!", "World",), "Hello, World!");
}

#[test]
fn positional_args() {
    assert_eq!(gettext!("{}, {}{}", "Hello", "World", '!'), "Hello, World!");
}

#[test]
fn positional_args_trailing_comma() {
    assert_eq!(
        gettext!("{}, {}{}", "Hello", "World", '!',),
        "Hello, World!"
    );
}

// Behaviour

#[test]
fn translate_without_formatting() {
    fake!(gettext, "Привіт, світ!");

    assert_eq!(gettext!("Hello, World!"), "Привіт, світ!");
}

#[test]
fn handle_escapes_only() {
    assert_eq!(gettext!("Hello, {{}}!"), "Hello, {}!");
}

#[test]
fn translate_and_handle_escapes_only() {
    fake!(gettext, "Привіт, {{}}!");

    assert_eq!(gettext!("Hello, {{}}!"), "Привіт, {}!");
}

#[test]
fn translate_with_positional_args() {
    fake!(gettext, "Привіт, {}{}");

    assert_eq!(
        gettext!("Hello, {}{}", "Username", '!'),
        "Привіт, Username!"
    );
}

#[test]
fn fallback_escapes_only() {
    fake!(gettext, "Привіт, {{}!");

    assert_eq!(gettext!("Hello, {{}}!"), "Hello, {}!");
}

#[test]
fn fallback_with_positional_args() {
    fake!(gettext, "Привіт, }{{}");

    assert_eq!(gettext!("Hello, {}{}", "Username", '!'), "Hello, Username!");
}
