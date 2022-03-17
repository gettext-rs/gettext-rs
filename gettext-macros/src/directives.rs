use crate::helpers::*;
use macros_shared::*;
use syn::{
    parse::{Error, Result},
    LitStr,
};

pub fn validate(msgid: &LitStr, n_args: usize) -> Result<bool> {
    let mut n_dirs = 0;
    let mut escapes = false;
    let span = msgid.span();
    let haystack = &msgid.value();

    for p in Formatter::new(haystack) {
        use Pattern::*;

        match p {
            Ordered(None, ..) => n_dirs += 1,
            Escaped(..) => escapes = true,
            Ordered(Some(n), ..) => match n < n_args {
                true => n_dirs += 1,
                false => return Err(Error::new(span, "Index out of bounds")),
            },
            Unescaped(c) => {
                let (a, b) = match c {
                    Brace::Opening => ("{", "{{"),
                    Brace::Closing => ("}", "}}"),
                };
                let msg = format!(
                    "Unmatched `{0}` in format string. If you intended to print `{0}`, you can escape it using `{1}`",
                    a, b
                );
                return Err(Error::new(span, msg));
            }
        }
    }

    if let Err(e) = check_amount(n_dirs, n_args) {
        return Err(Error::new(span, e));
    }

    if n_dirs == 0 && !escapes {
        Ok(false)
    } else {
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proc_macro2::Span;

    macro_rules! LitStr {
        ($str: literal) => {
            LitStr::new($str, Span::call_site())
        };
    }

    #[test]
    fn parameter() {
        let litstr = LitStr!("{}");

        assert!(validate(&litstr, 1).unwrap());
    }

    #[test]
    fn text_before_parameter() {
        let litstr = LitStr!("Text {}");

        assert!(validate(&litstr, 1).unwrap());
    }

    #[test]
    fn text_after_parameter() {
        let litstr = LitStr!("{} text");

        assert!(validate(&litstr, 1).unwrap());
    }

    #[test]
    fn text_around_parameter() {
        let litstr = LitStr!("There is a {} parameter");

        assert!(validate(&litstr, 1).unwrap());
    }

    #[test]
    fn text_around_multiple_parameters() {
        let litstr = LitStr!("There are {} multiple {} parameters");

        assert!(validate(&litstr, 2).unwrap());
    }

    #[test]
    fn text_around_a_bunch_of_parameters() {
        let litstr = LitStr!("There is {} quite {} a bunch {} of text {} around parameters");

        assert!(validate(&litstr, 4).unwrap());
    }

    #[test]
    fn escaped_opening_brace() {
        let litstr = LitStr!("{{");

        assert!(validate(&litstr, 0).unwrap());
    }

    #[test]
    fn escaped_closing_brace() {
        let litstr = LitStr!("}}");

        assert!(validate(&litstr, 0).unwrap());
    }

    #[test]
    fn escaped_pair_of_braces() {
        let litstr = LitStr!("{{}}");

        assert!(validate(&litstr, 0).unwrap());
    }

    #[test]
    fn text_before_escaped_opening_brace() {
        let litstr = LitStr!("Text {{");

        assert!(validate(&litstr, 0).unwrap());
    }

    #[test]
    fn text_before_escaped_closing_brace() {
        let litstr = LitStr!("Text }}");

        assert!(validate(&litstr, 0).unwrap());
    }

    #[test]
    fn text_before_escaped_pair_of_braces() {
        let litstr = LitStr!("Text {{}}");

        assert!(validate(&litstr, 0).unwrap());
    }

    #[test]
    fn text_after_escaped_opening_brace() {
        let litstr = LitStr!("{{ text");

        assert!(validate(&litstr, 0).unwrap());
    }

    #[test]
    fn text_after_escaped_closing_brace() {
        let litstr = LitStr!("}} text");

        assert!(validate(&litstr, 0).unwrap());
    }

    #[test]
    fn text_after_escaped_pair_of_braces() {
        let litstr = LitStr!("{{}} text");

        assert!(validate(&litstr, 0).unwrap());
    }

    #[test]
    fn text_around_escaped_opening_brace() {
        let litstr = LitStr!("There is an {{ escape");

        assert!(validate(&litstr, 0).unwrap());
    }

    #[test]
    fn text_around_escaped_closing_brace() {
        let litstr = LitStr!("There is an }} escape");

        assert!(validate(&litstr, 0).unwrap());
    }

    #[test]
    fn text_around_escaped_pair_of_braces() {
        let litstr = LitStr!("There are {{}} escapes");

        assert!(validate(&litstr, 0).unwrap());
    }

    // ========================================================
    //                   Ordered parameters
    // ========================================================

    #[test]
    fn ordered_parameter() {
        let litstr = LitStr!("{0}");

        assert!(validate(&litstr, 1).unwrap());
    }

    #[test]
    fn multiple_ordered_parameters() {
        let litstr = LitStr!("{0}, {1}");

        assert!(validate(&litstr, 2).unwrap());
    }

    #[test]
    fn multiple_reverse_ordered_parameters() {
        let litstr = LitStr!("{1}, {0}");

        assert!(validate(&litstr, 2).unwrap());
    }

    #[test]
    fn a_bunch_of_ordered_parameters() {
        let litstr = LitStr!("{0}, {1}, {2}, {3}");

        assert!(validate(&litstr, 4).unwrap());
    }

    #[test]
    fn a_bunch_of_reverse_ordered_parameters() {
        let litstr = LitStr!("{3}, {2}, {1}, {0}");

        assert!(validate(&litstr, 4).unwrap());
    }

    #[test]
    fn a_bunch_of_randomly_ordered_parameters() {
        let litstr = LitStr!("{2}, {3}, {1}, {0}");

        assert!(validate(&litstr, 4).unwrap());
    }

    #[test]
    fn a_bunch_of_partially_ordered_parameters() {
        let litstr = LitStr!("{2}, {3}, {}, {}");

        assert!(validate(&litstr, 4).unwrap());
    }
}
