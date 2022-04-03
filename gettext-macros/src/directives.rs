use crate::error::UiError;
use macros_shared::*;
use syn::{
    parse::{Error, Result},
    LitStr,
};

pub fn validate(msgid: &LitStr, args: usize) -> Result<bool> {
    let mut params = 0;
    let mut escapes = false;
    let haystack = &msgid.value();

    for p in Formatter::new(haystack) {
        use Pattern::*;

        match p {
            Ordered(None, ..) => params += 1,
            Escaped(..) => escapes = true,
            Ordered(Some(n), ..) => match n < args {
                true => params += 1,
                false => return Err(Error::new(msgid.span(), UiError::InvalidRefToPosArg(n))),
            },
            Unescaped(c) => return Err(Error::new(msgid.span(), UiError::Unmatched(c))),
        }
    }

    if params != args {
        return Err(Error::new(
            msgid.span(),
            UiError::MismatchNumOfArgs { params, args },
        ));
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
