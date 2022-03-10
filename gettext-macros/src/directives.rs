use macros_shared::*;
use syn::{
    parse::{Error, Result},
    LitStr,
};

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Directives {
    pub amount: usize,
    pub escapes: bool,
}

impl TryFrom<&LitStr> for Directives {
    type Error = Error;

    fn try_from(msgid: &LitStr) -> Result<Self> {
        let mut result = Self {
            amount: 0,
            escapes: false,
        };

        for m in directives_parser().find_iter(&msgid.value()) {
            match DIRECTIVES_PATTERNS[m.pattern()] {
                "{}" => result.amount += 1,
                "{{" | "}}" => result.escapes = true,
                c @ ("{" | "}") => return Err(
                    Error::new(
                        msgid.span(),
                        format!(
                            "Unmatched `{0}` in format string. If you intended to print `{0}`, you can escape it using `{1}`",
                            c,
                            {
                                match c {
                                    "{" => "{{",
                                    "}" => "}}",
                                    _ => unreachable!()
                                }
                            }
                        )
                    )
                ),
                _ => unreachable!()
            }
        }

        Ok(result)
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

        assert_eq!(
            Directives::try_from(&litstr).unwrap(),
            Directives {
                amount: 1,
                escapes: false
            }
        );
    }

    #[test]
    fn text_before_parameter() {
        let litstr = LitStr!("Text {}");

        assert_eq!(
            Directives::try_from(&litstr).unwrap(),
            Directives {
                amount: 1,
                escapes: false
            }
        );
    }

    #[test]
    fn text_after_parameter() {
        let litstr = LitStr!("{} text");

        assert_eq!(
            Directives::try_from(&litstr).unwrap(),
            Directives {
                amount: 1,
                escapes: false
            }
        );
    }

    #[test]
    fn text_around_parameter() {
        let litstr = LitStr!("There is a {} parameter");

        assert_eq!(
            Directives::try_from(&litstr).unwrap(),
            Directives {
                amount: 1,
                escapes: false
            }
        );
    }

    #[test]
    fn text_around_multiple_parameters() {
        let litstr = LitStr!("There are {} multiple {} parameters");

        assert_eq!(
            Directives::try_from(&litstr).unwrap(),
            Directives {
                amount: 2,
                escapes: false
            }
        );
    }

    #[test]
    fn text_around_a_bunch_of_parameters() {
        let litstr = LitStr!("There is {} quite {} a bunch {} of text {} around parameters");

        assert_eq!(
            Directives::try_from(&litstr).unwrap(),
            Directives {
                amount: 4,
                escapes: false
            }
        );
    }

    #[test]
    fn escaped_opening_brace() {
        let litstr = LitStr!("{{");

        assert_eq!(
            Directives::try_from(&litstr).unwrap(),
            Directives {
                amount: 0,
                escapes: true
            }
        );
    }

    #[test]
    fn escaped_closing_brace() {
        let litstr = LitStr!("}}");

        assert_eq!(
            Directives::try_from(&litstr).unwrap(),
            Directives {
                amount: 0,
                escapes: true
            }
        );
    }

    #[test]
    fn escaped_pair_of_braces() {
        let litstr = LitStr!("{{}}");

        assert_eq!(
            Directives::try_from(&litstr).unwrap(),
            Directives {
                amount: 0,
                escapes: true
            }
        );
    }

    #[test]
    fn text_before_escaped_opening_brace() {
        let litstr = LitStr!("Text {{");

        assert_eq!(
            Directives::try_from(&litstr).unwrap(),
            Directives {
                amount: 0,
                escapes: true
            }
        );
    }

    #[test]
    fn text_before_escaped_closing_brace() {
        let litstr = LitStr!("Text }}");

        assert_eq!(
            Directives::try_from(&litstr).unwrap(),
            Directives {
                amount: 0,
                escapes: true
            }
        );
    }

    #[test]
    fn text_before_escaped_pair_of_braces() {
        let litstr = LitStr!("Text {{}}");

        assert_eq!(
            Directives::try_from(&litstr).unwrap(),
            Directives {
                amount: 0,
                escapes: true
            }
        );
    }

    #[test]
    fn text_after_escaped_opening_brace() {
        let litstr = LitStr!("{{ text");

        assert_eq!(
            Directives::try_from(&litstr).unwrap(),
            Directives {
                amount: 0,
                escapes: true
            }
        );
    }

    #[test]
    fn text_after_escaped_closing_brace() {
        let litstr = LitStr!("}} text");

        assert_eq!(
            Directives::try_from(&litstr).unwrap(),
            Directives {
                amount: 0,
                escapes: true
            }
        );
    }

    #[test]
    fn text_after_escaped_pair_of_braces() {
        let litstr = LitStr!("{{}} text");

        assert_eq!(
            Directives::try_from(&litstr).unwrap(),
            Directives {
                amount: 0,
                escapes: true
            }
        );
    }

    #[test]
    fn text_around_escaped_opening_brace() {
        let litstr = LitStr!("There is an {{ escape");

        assert_eq!(
            Directives::try_from(&litstr).unwrap(),
            Directives {
                amount: 0,
                escapes: true
            }
        );
    }

    #[test]
    fn text_around_escaped_closing_brace() {
        let litstr = LitStr!("There is an }} escape");

        assert_eq!(
            Directives::try_from(&litstr).unwrap(),
            Directives {
                amount: 0,
                escapes: true
            }
        );
    }

    #[test]
    fn text_around_escaped_pair_of_braces() {
        let litstr = LitStr!("There are {{}} escapes");

        assert_eq!(
            Directives::try_from(&litstr).unwrap(),
            Directives {
                amount: 0,
                escapes: true
            }
        );
    }
}
