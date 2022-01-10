extern crate gettextrs;
use gettextrs::formatter::*;

#[test]
fn text_around() {
    assert_eq!(
        format("There is some text {} parameter", &["around".to_string()]).unwrap(),
        "There is some text around parameter"
    );
}

#[test]
fn text_around_escapes() {
    assert_eq!(
        format("There is some text {{}} parameter", &[]).unwrap(),
        "There is some text {} parameter"
    );
}

#[test]
fn text_around_multiple_parameters() {
    assert_eq!(
        format(
            "There is {} text {} parameters",
            &["some".to_string(), "around".to_string()]
        )
        .unwrap(),
        "There is some text around parameters"
    );
}

#[test]
fn text_around_multiple_escapes() {
    assert_eq!(
        format("There is {{}} text {{}} parameters", &[]).unwrap(),
        "There is {} text {} parameters"
    );
}

#[test]
fn no_text() {
    assert_eq!(
        format("{}", &["There was no text".to_string()]).unwrap(),
        "There was no text"
    );
}

#[test]
fn just_escapes() {
    assert_eq!(format("{{}}", &[]).unwrap(), "{}");
}

#[test]
fn no_text_after() {
    assert_eq!(
        format("Some {}", &["text".to_string()]).unwrap(),
        "Some text"
    );
}

#[test]
fn no_text_after_escapes() {
    assert_eq!(format("Some {{}}", &[]).unwrap(), "Some {}");
}

#[test]
fn no_text_before() {
    assert_eq!(
        format("{} text", &["Some".to_string()]).unwrap(),
        "Some text"
    );
}

#[test]
fn no_text_before_escapes() {
    assert_eq!(format("{{}} text", &[]).unwrap(), "{} text");
}

#[test]
fn formatter_no_text_multiple_parameters() {
    assert_eq!(
        format("{}{}", &["There was ".to_string(), "no text".to_string()]).unwrap(),
        "There was no text"
    );
}

#[test]
fn formatter_no_text_multiple_escapes() {
    assert_eq!(format("{{}}{{}}", &[]).unwrap(), "{}{}");
}

#[test]
fn did_not_found() {
    assert_eq!(
        format(
            "There is nothing to format",
            &["but there is an argument".to_string()]
        ),
        None
    )
}

#[test]
fn lacks_args_or_unescaped() {
    assert_eq!(format("There is {} to format", &[]), None)
}

#[test]
fn lacks_args_or_unescaped_no_text() {
    assert_eq!(format("{}", &[]), None)
}

#[test]
fn other_unescaped() {
    assert_eq!(format("There is { to format", &[]), None);
    assert_eq!(format("There is } to format", &[]), None);
    assert_eq!(format("There is }{ to format", &[]), None);
}

#[test]
fn other_unescaped_no_text() {
    assert_eq!(format("{", &[]), None);
    assert_eq!(format("}", &[]), None);
    assert_eq!(format("}{", &[]), None);
}

#[test]
// The formatter is to be used on translated strings,
// primarily non-Ascii Unicode characters.
fn unicode_haystack() {
    assert_eq!(
        format("私は{}です!", &["ウクライナ人".to_string()]).unwrap(),
        "私はウクライナ人です!"
    )
}

#[test]
#[ignore]
// There is no need to require this behavior.
// Formatter shouldn't be called when there is nothing to format.
fn no_formatting() {
    assert_eq!(
        format("There is nothing to format", &[]).unwrap(),
        "There is nothing to format"
    )
}

#[test]
// For consistency, every string have to escape curly braces.
fn no_formatting_but_escapes() {
    assert_eq!(
        format(
            "There is nothing to format, but there are some {{}} to handle",
            &[]
        )
        .unwrap(),
        "There is nothing to format, but there are some {} to handle"
    )
}
