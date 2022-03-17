use macros_shared::*;
use syn::{
    parse::{Error, Result},
    LitStr,
};

pub fn validate(msgid: &LitStr, args: usize) -> Result<bool> {
    let mut params = 0;
    let mut escapes = false;
    let span = msgid.span();
    let haystack = &msgid.value();

    for p in Formatter::new(haystack) {
        use Pattern::*;

        match p {
            Ordered(None, ..) => params += 1,
            Escaped(..) => escapes = true,
            Ordered(Some(n), ..) => match n < args {
                true => params += 1,
                false => {
                    let f;
                    let there_is = match args {
                        0 => "no arguments were given",
                        1 => "there is 1 argument",
                        _ => {
                            f = format!("there are {} arguments", args);
                            &f
                        }
                    };
                    let msg = format!(
                        "invalid reference to positional argument {} ({})\nnote: positional arguments are zero-based",
                        n, there_is
                    );
                    return Err(Error::new(span, msg));
                }
            },
            Unescaped(c) => {
                let (a, b) = match c {
                    Brace::Opening => ("{", "{{"),
                    Brace::Closing => ("}", "}}"),
                };
                let msg = format!(
                    "unmatched `{0}` in format string\nnote: if you intended to print `{0}`, you can escape it using `{1}`",
                    a, b
                );
                return Err(Error::new(span, msg));
            }
        }
    }

    if params != args {
        let arguments = if params == 1 { "argument" } else { "arguments" };

        let f;
        let given = if args == 0 {
            "no arguments were given"
        } else if args == 1 {
            "there is 1 argument"
        } else {
            f = format!("there are {} arguments", args);
            &f
        };

        let msg = format!(
            "{} positional {} in format string, but {}",
            params, arguments, given
        );

        return Err(Error::new(span, msg));
    }

    if params == 0 && !escapes {
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
        let litstr = LitStr!("There is a {} parameter");

        assert!(validate(&litstr, 1).unwrap());
    }

    #[test]
    fn ordered_parameter() {
        let litstr = LitStr!("There is a {0} parameter");

        assert!(validate(&litstr, 1).unwrap());
    }

    #[test]
    fn escaped_opening_brace() {
        let litstr = LitStr!("There is an {{ escape");

        assert!(validate(&litstr, 0).unwrap());
    }

    #[test]
    fn escaped_closing_brace() {
        let litstr = LitStr!("There is an }} escape");

        assert!(validate(&litstr, 0).unwrap());
    }

    #[test]
    fn no_parameters() {
        let litstr = LitStr!("There is no parameters");

        assert!(!validate(&litstr, 0).unwrap());
    }
}
