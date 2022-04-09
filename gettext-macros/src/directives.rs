use crate::error::UiError;
use macros_shared::*;
use quote::{quote, ToTokens};
use syn::{
    parse::{Error, Result},
    LitStr,
};

pub fn validate(msgid: &LitStr, args: usize) -> Result<Option<Vec<ToFormat>>> {
    let mut params = 0;
    let mut escapes = false;
    let haystack = &msgid.value();
    let mut to_format = Vec::<ToFormat>::new();
    let mut cursor: usize = 0;

    for p in Formatter::new(haystack) {
        let todo: (ToFormat, usize, usize);

        match p {
            Pattern::Argument {
                arg: Argument::Ordered(v @ None),
                start,
                end,
            } => {
                params += 1;
                todo = (ToFormat::Argument(Argument::Ordered(v)), cursor, start);
                cursor = end;
            }
            Pattern::Argument {
                arg: Argument::Ordered(v @ Some(n)),
                start,
                end,
            } => match n < args {
                true => {
                    params += 1;
                    todo = (ToFormat::Argument(Argument::Ordered(v)), cursor, start);
                    cursor = end;
                }
                false => return Err(Error::new(msgid.span(), UiError::InvalidRefToPosArg(n))),
            },
            Pattern::Escaped { brace, start, end } => {
                escapes = true;
                todo = (
                    match brace {
                        Brace::Opening => ToFormat::Char('{'),
                        Brace::Closing => ToFormat::Char('}'),
                    },
                    cursor,
                    start,
                );
                cursor = end;
            }
            Pattern::Unescaped(c) => return Err(Error::new(msgid.span(), UiError::Unmatched(c))),
        }

        if todo.1 != todo.2 {
            to_format.push(ToFormat::String((&haystack[todo.1..todo.2]).to_string()));
        }
        to_format.push(todo.0)
    }

    if params != args {
        return Err(Error::new(
            msgid.span(),
            UiError::MismatchNumOfArgs { params, args },
        ));
    }

    if cursor < haystack.len() {
        to_format.push(ToFormat::String((&haystack[cursor..]).to_string()));
    }

    if params == 0 && !escapes {
        Ok(None)
    } else {
        Ok(Some(to_format))
    }
}

#[cfg_attr(test, derive(PartialEq, Debug))]
pub enum ToFormat {
    Argument(Argument),
    String(String),
    Char(char),
}

impl ToTokens for ToFormat {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        match self {
            ToFormat::Argument(Argument::Ordered(Some(n))) => {
                quote! {std::option::Option::<usize>::Some(#n)}.to_tokens(tokens)
            }
            ToFormat::Argument(Argument::Ordered(None)) => {
                quote! {std::option::Option::<usize>::None}.to_tokens(tokens)
            }
            ToFormat::String(v) => v.to_tokens(tokens),
            ToFormat::Char(v) => v.to_tokens(tokens),
        }
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
            validate(&litstr, 1).unwrap(),
            Some(vec![ToFormat::Argument(Argument::Ordered(None))])
        )
    }

    #[test]
    fn text_around_parameter() {
        let litstr = LitStr!("There is a {} parameter");

        assert_eq!(
            validate(&litstr, 1).unwrap(),
            Some(vec![
                ToFormat::String("There is a ".to_string()),
                ToFormat::Argument(Argument::Ordered(None)),
                ToFormat::String(" parameter".to_string()),
            ])
        )
    }

    #[test]
    fn ordered_parameter() {
        let litstr = LitStr!("{0}");

        assert_eq!(
            validate(&litstr, 1).unwrap(),
            Some(vec![ToFormat::Argument(Argument::Ordered(Some(0)))])
        )
    }

    #[test]
    fn text_around_ordered_parameter() {
        let litstr = LitStr!("There is a {0} parameter");

        assert_eq!(
            validate(&litstr, 1).unwrap(),
            Some(vec![
                ToFormat::String("There is a ".to_string()),
                ToFormat::Argument(Argument::Ordered(Some(0))),
                ToFormat::String(" parameter".to_string()),
            ])
        )
    }

    #[test]
    fn escaped_opening_brace() {
        let litstr = LitStr!("{{");

        assert_eq!(
            validate(&litstr, 0).unwrap(),
            Some(vec![ToFormat::Char('{')])
        )
    }

    #[test]
    fn text_around_escaped_opening_brace() {
        let litstr = LitStr!("There is an {{ escape");

        assert_eq!(
            validate(&litstr, 0).unwrap(),
            Some(vec![
                ToFormat::String("There is an ".to_string()),
                ToFormat::Char('{'),
                ToFormat::String(" escape".to_string()),
            ])
        )
    }

    #[test]
    fn escaped_closing_brace() {
        let litstr = LitStr!("}}");

        assert_eq!(
            validate(&litstr, 0).unwrap(),
            Some(vec![ToFormat::Char('}')])
        )
    }

    #[test]
    fn text_around_escaped_closing_brace() {
        let litstr = LitStr!("There is an }} escape");

        assert_eq!(
            validate(&litstr, 0).unwrap(),
            Some(vec![
                ToFormat::String("There is an ".to_string()),
                ToFormat::Char('}'),
                ToFormat::String(" escape".to_string()),
            ])
        )
    }

    #[test]
    fn no_parameters() {
        let litstr = LitStr!("There are no parameters");

        assert_eq!(validate(&litstr, 0).unwrap(), None)
    }
}
