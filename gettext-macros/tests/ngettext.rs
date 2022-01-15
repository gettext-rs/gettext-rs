use ::gettextrs::*;

mod helpers;

// Invocation

#[test]
fn nothing_to_format() {
    assert_eq!(
        ngettext!("Hello, World!", "Hello, Worlds!", 1),
        "Hello, World!"
    );
}

#[test]
fn trailing_comma() {
    assert_eq!(
        ngettext!("Hello, World!", "Hello, Worlds!", 1,),
        "Hello, World!"
    );
}

#[test]
fn positional_arg() {
    assert_eq!(
        ngettext!("Hello, {}!", "Hello, {}s!", 1, "World"),
        "Hello, World!"
    );
}

#[test]
fn positional_arg_trailing_comma() {
    assert_eq!(
        ngettext!("Hello, {}!", "Hello, {}s!", 1, "World",),
        "Hello, World!"
    );
}

#[test]
fn positional_args() {
    assert_eq!(
        ngettext!("{}, {}{}", "{}, {}s{}", 1, "Hello", "World", '!'),
        "Hello, World!"
    );
}

#[test]
fn positional_args_trailing_comma() {
    assert_eq!(
        ngettext!("{}, {}{}", "{}, {}s{}", 1, "Hello", "World", '!',),
        "Hello, World!"
    );
}

// Behaviour

#[test]
fn translate_without_formatting() {
    fake!(ngettext, "Привіт, світ!", "Привіт, світи!");

    assert_eq!(
        ngettext!("Hello, World!", "Hello, Worlds!", 1),
        "Привіт, світ!"
    );
    assert_eq!(
        ngettext!("Hello, World!", "Hello, Worlds!", 2),
        "Привіт, світи!"
    );
}

#[test]
fn handle_escapes_only() {
    assert_eq!(ngettext!("Hello, {{}}!", "Hello, {{}}s!", 1), "Hello, {}!");
    assert_eq!(ngettext!("Hello, {{}}!", "Hello, {{}}s!", 2), "Hello, {}s!");
}

#[test]
fn translate_and_handle_escapes_only() {
    fake!(ngettext, "Привіт, {{}}!", "Привіт, {{}}и!");

    assert_eq!(ngettext!("Hello, {{}}!", "Hello, {{}}s!", 1), "Привіт, {}!");
    assert_eq!(
        ngettext!("Hello, {{}}!", "Hello, {{}}s!", 2),
        "Привіт, {}и!"
    );
}

#[test]
fn translate_with_positional_args() {
    fake!(ngettext, "Привіт, {}{}", "Привіт, {}s{}");

    assert_eq!(
        ngettext!("Hello, {}{}", "Hello, {}s{}", 1, "Username", '!'),
        "Привіт, Username!"
    );
    assert_eq!(
        ngettext!("Hello, {}{}", "Hello, {}s{}", 2, "Username", '!'),
        "Привіт, Usernames!"
    );
}

#[test]
fn fallback_escapes_only() {
    fake!(ngettext, "Привіт, {{}!", "Привіт, {{}и!");

    assert_eq!(ngettext!("Hello, {{}}!", "Hello, {{}}s!", 1), "Hello, {}!");
    assert_eq!(ngettext!("Hello, {{}}!", "Hello, {{}}s!", 2), "Hello, {}s!");
}

#[test]
fn fallback_with_positional_args() {
    fake!(ngettext, "Привіт, }{{}", "Привіт, }{s{}");

    assert_eq!(
        ngettext!("Hello, {}{}", "Hello, {}s{}", 1, "Username", '!'),
        "Hello, Username!"
    );
    assert_eq!(
        ngettext!("Hello, {}{}", "Hello, {}s{}", 2, "Username", '!'),
        "Hello, Usernames!"
    );
}
