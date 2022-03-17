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
