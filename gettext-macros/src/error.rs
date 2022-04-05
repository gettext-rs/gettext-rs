use macros_shared::Brace;
use std::fmt::Display;
use syn::Error;

pub enum UiError {
    InvalidRefToPosArg(usize),
    MismatchNumOfArgs { params: usize, args: usize },
    Unmatched(Brace),
    AtLeastMsgid,
    AtLeastDAndMsgid,
    MissingMsgid,
}

pub fn combine_err(err: &Option<Error>, another: Error) -> Error {
    if let Some(e) = &err {
        let mut e = e.to_owned();
        e.combine(another);
        return e;
    }
    another
}

impl Display for UiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use UiError::*;

        match self {
            InvalidRefToPosArg(n) => {
                let temp;
                let a = match n {
                    0 => "no arguments were given",
                    1 => "there is 1 argument",
                    _ => {
                        temp = format!("there are {} arguments", n);
                        &temp
                    }
                };

                return write!(
                    f,
                    "invalid reference to positional argument {} ({})\nnote: positional arguments are zero-based",
                    n, a
                );
            }
            MismatchNumOfArgs { params, args } => {
                let a = match params {
                    1 => "argument",
                    _ => "arguments",
                };

                let temp;
                let b = match args {
                    0 => "no arguments were given",
                    1 => "there is 1 argument",
                    _ => {
                        temp = format!("there are {} arguments", args);
                        &temp
                    }
                };

                return write!(f, "{} positional {} in format string, but {}", params, a, b);
            }
            Unmatched(c) => {
                let (a, b) = match c {
                    Brace::Opening => ("{", "{{"),
                    Brace::Closing => ("}", "}}"),
                };

                return write!(
                    f,
                    "unmatched `{0}` in format string\nnote: if you intended to print `{0}`, you can escape it using `{1}`",
                    a, b
                );
            }
            AtLeastMsgid => write!(f, "requires at least a msgid string argument"),
            AtLeastDAndMsgid => {
                write!(f, "requires at least domainname and msgid string arguments")
            }
            MissingMsgid => write!(f, "missing msgid string argument"),
        }
    }
}
