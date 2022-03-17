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
fn text_around_parameter() {
    assert_eq!(
        format("There is some text {} parameter", &["around".into()]),
        Some("There is some text around parameter".into())
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
fn text_around_escaped_opening_brace() {
    assert_eq!(
        format("There is an {{ escape", &[]),
        Some("There is an { escape".into())
    );
}

#[test]
fn escaped_closing_brace() {
    assert_eq!(format("}}", &[]), Some("}".into()));
}

#[test]
fn unescaped_opening_brace() {
    assert_eq!(format("{", &[]), None)
}

#[test]
fn text_around_unescaped_opening_brace() {
    assert_eq!(format("There is { to format", &[]), None)
}

#[test]
fn unescaped_closing_brace() {
    assert_eq!(format("}", &[]), None);
}

#[test]
fn text_around_unescaped_closing_brace() {
    assert_eq!(format("There is } to format", &[]), None);
}

#[test]
fn text_around_escaped_closing_brace() {
    assert_eq!(
        format("There is an }} escape", &[]),
        Some("There is an } escape".into())
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

// ============================================================
//                     Ordered parameters
// ============================================================

#[test]
fn ordered_parameters() {
    assert_eq!(
        format("{0}", &["Single".to_string()]),
        Some("Single".into())
    );
}

#[test]
fn multiple_ordered_parameters() {
    assert_eq!(
        format("{0}, {1}", &["First".into(), "Second".into()]),
        Some("First, Second".into())
    );
}

#[test]
fn multiple_reverse_ordered_parameters() {
    assert_eq!(
        format("{1}, {0}", &["First".into(), "Second".into()]),
        Some("Second, First".into())
    );
}

#[test]
fn a_bunch_of_ordered_parameters() {
    assert_eq!(
        format(
            "{0}, {1}, {2}, {3}",
            &[
                "First".into(),
                "Second".into(),
                "Third".into(),
                "Fourth".into()
            ]
        ),
        Some("First, Second, Third, Fourth".into())
    );
}

#[test]
fn a_bunch_of_reverse_ordered_parameters() {
    assert_eq!(
        format(
            "{3}, {2}, {1}, {0}",
            &[
                "First".into(),
                "Second".into(),
                "Third".into(),
                "Fourth".into()
            ]
        ),
        Some("Fourth, Third, Second, First".into())
    );
}

#[test]
fn a_bunch_of_randomly_ordered_parameters() {
    assert_eq!(
        format(
            "{2}, {3}, {1}, {0}",
            &[
                "First".into(),
                "Second".into(),
                "Third".into(),
                "Fourth".into()
            ]
        ),
        Some("Third, Fourth, Second, First".into())
    );
}

#[test]
fn a_bunch_of_partially_ordered_parameters() {
    assert_eq!(
        format(
            "{2}, {3}, {}, {}",
            &[
                "First".into(),
                "Second".into(),
                "Third".into(),
                "Fourth".into()
            ]
        ),
        Some("Third, Fourth, First, Second".into())
    );
}

#[test]
fn extra_ordered_parameter() {
    assert_eq!(
        format("{}, {}, {1}", &["First".into(), "Second".into()]),
        Some("First, Second, Second".into())
    );
}

#[test]
fn ordered_unused_parameter() {
    assert_eq!(format("{0}, {}", &["First".into(), "Second".into()]), None);
}
