use std::iter::IntoIterator;
use syn::{
    parse::{Error, Result},
    LitStr,
};

pub enum Directive {
    Pos,
}

pub struct Directives {
    pub directives: Vec<Directive>,
    pub escapes: bool,
}

impl TryFrom<&LitStr> for Directives {
    type Error = Error;

    fn try_from(msgid: &LitStr) -> Result<Self> {
        use Directive::*;

        let span = msgid.span();
        let value = msgid.value();
        let mut chars = value.chars().peekable();
        let mut result = Self {
            directives: vec![],
            escapes: false,
        };

        loop {
            while let Some(c) = chars.next() {
                if c == '{' && chars.next_if_eq(&'}').is_some() {
                    result.directives.push(Pos);
                } else if (c == '{' || c == '}') && chars.next_if_eq(&c).is_some() {
                    result.escapes = true;
                } else if (c == '{' || c == '}') && chars.next_if_eq(&c).is_none() {
                    return Err(
                        Error::new(
                            span,
                            format!(
                                "Unmatched `{0}` in format string. If you intended to print `{0}`, you can escape it using `{1}`",
                                c,
                                {
                                    if c == '{' {
                                        "{{"
                                    } else {
                                        "}}"
                                    }
                                }
                            )
                        )
                    );
                }
            }
            break;
        }

        Ok(result)
    }
}

impl IntoIterator for Directives {
    type Item = Directive;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.directives.into_iter()
    }
}

impl<'a> IntoIterator for &'a Directives {
    type Item = &'a Directive;
    type IntoIter = std::slice::Iter<'a, Directive>;

    fn into_iter(self) -> Self::IntoIter {
        (&self.directives).into_iter()
    }
}
