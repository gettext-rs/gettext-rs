use super::*;

#[test]
fn parameter() {
    let haystack = "{}";
    let args = ["There was no text"];
    let expected = "There was no text";

    assert_eq!(
        try_format(
            haystack,
            Arguments::from(args.map(|s| s.to_string())).into_iter()
        ),
        Some(expected.to_string())
    );
}

#[test]
fn text_around_parameter() {
    let haystack = "There is some text {} parameter";
    let args = ["around"];
    let expected = "There is some text around parameter";

    assert_eq!(
        try_format(
            haystack,
            Arguments::from(args.map(|s| s.to_string())).into_iter()
        ),
        Some(expected.to_string())
    );
}

#[test]
fn multiple_parameters() {
    let haystack = "{}{}";
    let args = ["There was", " no text"];
    let expected = "There was no text";

    assert_eq!(
        try_format(
            haystack,
            Arguments::from(args.map(|s| s.to_string())).into_iter()
        ),
        Some(expected.to_string())
    );
}

#[test]
fn text_around_multiple_parameters() {
    let haystack = "There is {} text {} parameters";
    let args = ["some", "around"];
    let expected = "There is some text around parameters";

    assert_eq!(
        try_format(
            haystack,
            Arguments::from(args.map(|s| s.to_string())).into_iter()
        ),
        Some(expected.to_string())
    );
}

#[test]
fn text_around_a_bunch_of_parameters() {
    let haystack = "There {} quite {} bunch {} text {} parameters";
    let args = ["is", "a", "of", "around"];
    let expected = "There is quite a bunch of text around parameters";

    assert_eq!(
        try_format(
            haystack,
            Arguments::from(args.map(|s| s.to_string())).into_iter()
        ),
        Some(expected.to_string())
    );
}

#[test]
fn escaped_opening_brace() {
    let haystack = "{{";
    let args = [];
    let expected = "{";

    assert_eq!(
        try_format(haystack, Arguments::from(args).into_iter()),
        Some(expected.to_string())
    );
}

#[test]
fn text_around_escaped_opening_brace() {
    let haystack = "There is an {{ escape";
    let args = [];
    let expected = "There is an { escape";

    assert_eq!(
        try_format(haystack, Arguments::from(args).into_iter()),
        Some(expected.to_string())
    );
}

#[test]
fn escaped_closing_brace() {
    let haystack = "}}";
    let args = [];
    let expected = "}";

    assert_eq!(
        try_format(haystack, Arguments::from(args).into_iter()),
        Some(expected.to_string())
    );
}

#[test]
fn text_around_escaped_closing_brace() {
    let haystack = "There is an }} escape";
    let args = [];
    let expected = "There is an } escape";

    assert_eq!(
        try_format(haystack, Arguments::from(args).into_iter()),
        Some(expected.to_string())
    );
}

#[test]
fn unescaped_opening_brace() {
    let haystack = "{";
    let args = [];

    assert_eq!(
        try_format(haystack, Arguments::from(args).into_iter()),
        None
    );
}

#[test]
fn text_around_unescaped_opening_brace() {
    let haystack = "There is an { escape";
    let args = [];

    assert_eq!(
        try_format(haystack, Arguments::from(args).into_iter()),
        None
    );
}

#[test]
fn unescaped_closing_brace() {
    let haystack = "}";
    let args = [];

    assert_eq!(
        try_format(haystack, Arguments::from(args).into_iter()),
        None
    );
}

#[test]
fn text_around_unescaped_closing_brace() {
    let haystack = "There is an } escape";
    let args = [];

    assert_eq!(
        try_format(haystack, Arguments::from(args).into_iter()),
        None
    );
}

#[test]
// The formatter is to be used on translated strings,
// primarily non-Ascii Unicode characters.
fn unicode_haystack() {
    let haystack = "私は{}です!";
    let args = ["ウクライナ人"];
    let expected = "私はウクライナ人です!";

    assert_eq!(
        try_format(
            haystack,
            Arguments::from(args.map(|s| s.to_string())).into_iter()
        ),
        Some(expected.to_string())
    );
}

#[test]
#[ignore]
// There is no need to require this behavior.
// Formatter shouldn't be called when there is nothing to format.
fn no_formatting() {
    let haystack = "There is nothing to format";
    let args = [];
    let expected = "There is nothing to format";

    assert_eq!(
        try_format(haystack, Arguments::from(args).into_iter()),
        Some(expected.to_string())
    );
}

#[test]
fn ordered_parameters() {
    let haystack = "{0}";
    let args = ["Single"];
    let expected = "Single";

    assert_eq!(
        try_format(
            haystack,
            Arguments::from(args.map(|s| s.to_string())).into_iter()
        ),
        Some(expected.to_string())
    );
}

#[test]
fn text_around_ordered_parameters() {
    let haystack = "There is some text {0} parameter";
    let args = ["around"];
    let expected = "There is some text around parameter";

    assert_eq!(
        try_format(
            haystack,
            Arguments::from(args.map(|s| s.to_string())).into_iter()
        ),
        Some(expected.to_string())
    );
}

#[test]
fn multiple_ordered_parameters() {
    let haystack = "{0}{1}";
    let args = ["There was", " no text"];
    let expected = "There was no text";

    assert_eq!(
        try_format(
            haystack,
            Arguments::from(args.map(|s| s.to_string())).into_iter()
        ),
        Some(expected.to_string())
    );
}

#[test]
fn text_around_multiple_ordered_parameters() {
    let haystack = "There is {0} text {1} parameters";
    let args = ["some", "around"];
    let expected = "There is some text around parameters";

    assert_eq!(
        try_format(
            haystack,
            Arguments::from(args.map(|s| s.to_string())).into_iter()
        ),
        Some(expected.to_string())
    );
}

#[test]
fn text_around_a_bunch_of_ordered_parameters() {
    let haystack = "There {0} quite {1} bunch {2} text {3} parameters";
    let args = ["is", "a", "of", "around"];
    let expected = "There is quite a bunch of text around parameters";

    assert_eq!(
        try_format(
            haystack,
            Arguments::from(args.map(|s| s.to_string())).into_iter()
        ),
        Some(expected.to_string())
    );
}

#[test]
fn multiple_reversely_ordered_parameters() {
    let haystack = "{1}{0}";
    let args = [" there was", "No text"];
    let expected = "No text there was";

    assert_eq!(
        try_format(
            haystack,
            Arguments::from(args.map(|s| s.to_string())).into_iter()
        ),
        Some(expected.to_string())
    );
}

#[test]
fn text_around_multiple_reversely_ordered_parameters() {
    let haystack = "There is {1} text {0} parameters";
    let args = ["around", "some"];
    let expected = "There is some text around parameters";

    assert_eq!(
        try_format(
            haystack,
            Arguments::from(args.map(|s| s.to_string())).into_iter()
        ),
        Some(expected.to_string())
    );
}

#[test]
fn reversely_ordered_parameters() {
    let haystack = "{3}, {2}, {1}, {0}";
    let args = ["First", "Second", "Third", "Fourth"];
    let expected = "Fourth, Third, Second, First";

    assert_eq!(
        try_format(
            haystack,
            Arguments::from(args.map(|s| s.to_string())).into_iter()
        ),
        Some(expected.to_string())
    );
}

#[test]
fn randomly_ordered_parameters() {
    let haystack = "{2}, {1}, {3}, {0}";
    let args = ["First", "Second", "Third", "Fourth"];
    let expected = "Third, Second, Fourth, First";

    assert_eq!(
        try_format(
            haystack,
            Arguments::from(args.map(|s| s.to_string())).into_iter()
        ),
        Some(expected.to_string())
    );
}

#[test]
fn partially_ordered_parameters() {
    let haystack = "{2}, {3}, {}, {}";
    let args = ["First", "Second", "Third", "Fourth"];
    let expected = "Third, Fourth, First, Second";

    assert_eq!(
        try_format(
            haystack,
            Arguments::from(args.map(|s| s.to_string())).into_iter()
        ),
        Some(expected.to_string())
    );
}

#[test]
fn extra_ordered_parameter() {
    let haystack = "{}, {}, {1}";
    let args = ["First", "Second"];
    let expected = "First, Second, Second";

    assert_eq!(
        try_format(
            haystack,
            Arguments::from(args.map(|s| s.to_string())).into_iter()
        ),
        Some(expected.to_string())
    );
}

#[test]
fn unused_ordered_parameter() {
    let haystack = "{0}, {}";
    let args = ["First", "Second"];

    assert_eq!(
        try_format(
            haystack,
            Arguments::from(args.map(|s| s.to_string())).into_iter()
        ),
        None
    );
}

#[test]
fn fallback() {
    let haystack = "Hello, {}! To escape `{` use `{{`. Goodbye, {0}!";
    let fallback = [
        ToFormat::String("Hello, "),
        ToFormat::Argument(Argument::Ordered(None)),
        ToFormat::String("! To escape `"),
        ToFormat::Char('{'),
        ToFormat::String("` use `"),
        ToFormat::Char('{'),
        ToFormat::Char('{'),
        ToFormat::String("`. Goodbye, "),
        ToFormat::Argument(Argument::Ordered(Some(0))),
        ToFormat::String("!"),
    ];
    let args = ["Username"];
    let expected = "Hello, Username! To escape `{` use `{{`. Goodbye, Username!";

    assert_eq!(
        format(haystack, fallback, args.map(|s| s.to_string())),
        expected.to_string()
    );
}
