extern crate gettextrs;
use gettextrs::formatter::*;

#[test]
fn parameter() {
    assert_eq!(
        format("{}", &["There was no text".into()]),
        Some("There was no text".into())
    );
}

#[test]
fn multiple_parameters() {
    assert_eq!(
        format("{}{}", &["There was".into(), " no text".into()]),
        Some("There was no text".into())
    );
}

#[test]
fn text_before_parameter() {
    assert_eq!(
        format("Some {}", &["text".into()]),
        Some("Some text".into())
    );
}

#[test]
fn text_after_parameter() {
    assert_eq!(
        format("{} text", &["Some".into()]),
        Some("Some text".into())
    );
}

#[test]
fn text_around_parameter() {
    assert_eq!(
        format("There is some text {} parameter", &["around".into()]),
        Some("There is some text around parameter".into())
    );
}

#[test]
fn text_around_multiple_parameters() {
    assert_eq!(
        format(
            "There is {} text {} parameters",
            &["some".into(), "around".into()]
        ),
        Some("There is some text around parameters".into())
    );
}

#[test]
fn text_around_a_bunch_of_parameters() {
    assert_eq!(
        format(
            "There {} quite {} bunch {} text {} parameters",
            &["is".into(), "a".into(), "of".into(), "around".into()]
        ),
        Some("There is quite a bunch of text around parameters".into())
    );
}

#[test]
fn escaped_opening_brace() {
    assert_eq!(format("{{", &[]), Some("{".into()));
}

#[test]
fn escaped_closing_brace() {
    assert_eq!(format("}}", &[]), Some("}".into()));
}

#[test]
fn escaped_pair_of_braces() {
    assert_eq!(format("{{}}", &[]), Some("{}".into()));
}

#[test]
fn text_before_escaped_opening_brace() {
    assert_eq!(format("Text {{", &[]), Some("Text {".into()));
}

#[test]
fn text_before_escaped_closing_brace() {
    assert_eq!(format("Text }}", &[]), Some("Text }".into()));
}

#[test]
fn text_before_escaped_pair_of_braces() {
    assert_eq!(format("Text {{}}", &[]), Some("Text {}".into()));
}

#[test]
fn text_after_escaped_opening_brace() {
    assert_eq!(format("{{ text", &[]), Some("{ text".into()));
}

#[test]
fn text_after_escaped_closing_brace() {
    assert_eq!(format("}} text", &[]), Some("} text".into()));
}

#[test]
fn text_after_escaped_pair_of_braces() {
    assert_eq!(format("{{}} text", &[]), Some("{} text".into()));
}

#[test]
fn text_around_escaped_opening_brace() {
    assert_eq!(
        format("There is an {{ escape", &[]),
        Some("There is an { escape".into())
    );
}

#[test]
fn text_around_escaped_closing_brace() {
    assert_eq!(
        format("There is an }} escape", &[]),
        Some("There is an } escape".into())
    );
}

#[test]
fn text_around_escaped_pair_of_braces() {
    assert_eq!(
        format("There is an {{}} escape", &[]),
        Some("There is an {} escape".into())
    );
}

#[test]
fn lacks_parameter() {
    assert_eq!(
        format(
            "There is nothing to format",
            &["but there is an argument".into()]
        ),
        None
    )
}

#[test]
fn extra_parameter() {
    assert_eq!(format("There is extra {} to (not) format", &[]), None)
}

#[test]
fn escapes_before_argument() {
    assert_eq!(
        format(
            "There is {{}} to escape, and {} to format",
            &["something".into()]
        ),
        Some("There is {} to escape, and something to format".into())
    )
}

#[test]
fn escapes_after_argument() {
    assert_eq!(
        format(
            "There is {} to format, and a {{}} to escape",
            &["something".into()]
        ),
        Some("There is something to format, and a {} to escape".into())
    )
}

#[test]
fn unescaped_opening_brace() {
    assert_eq!(format("{", &[]), None)
}

#[test]
fn unescaped_closing_brace() {
    assert_eq!(format("}", &[]), None);
}

#[test]
fn unescaped_pair_of_braces() {
    assert_eq!(format("}{", &[]), None);
}

#[test]
fn text_before_unescaped_opening_brace() {
    assert_eq!(format("Text {", &[]), None);
}

#[test]
fn text_before_unescaped_closing_brace() {
    assert_eq!(format("Text }", &[]), None);
}

#[test]
fn text_before_unescaped_pair_of_braces() {
    assert_eq!(format("Text }{", &[]), None);
}

#[test]
fn text_after_unescaped_opening_brace() {
    assert_eq!(format("{ text", &[]), None);
}

#[test]
fn text_after_unescaped_closing_brace() {
    assert_eq!(format("} text", &[]), None);
}

#[test]
fn text_after_unescaped_pair_of_braces() {
    assert_eq!(format("}{ text", &[]), None);
}

#[test]
fn text_around_unescaped_opening_brace() {
    assert_eq!(format("There is { to format", &[]), None)
}

#[test]
fn text_around_unescaped_closing_brace() {
    assert_eq!(format("There is } to format", &[]), None);
}

#[test]
fn text_around_unescaped_pair_of_braces() {
    assert_eq!(format("There is }{ to format", &[]), None);
}

#[test]
// The formatter is to be used on translated strings,
// primarily non-Ascii Unicode characters.
fn unicode_haystack() {
    assert_eq!(
        format("私は{}です!", &["ウクライナ人".into()]),
        Some("私はウクライナ人です!".into())
    )
}

#[test]
#[ignore]
// There is no need to require this behavior.
// Formatter shouldn't be called when there is nothing to format.
fn no_formatting() {
    assert_eq!(
        format("There is nothing to format", &[]),
        Some("There is nothing to format".into())
    )
}
