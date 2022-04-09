#![allow(clippy::into_iter_on_ref)]

#[cfg(test)]
mod tests;

use macros_shared::*;

pub fn format<A, const N: usize, const M: usize>(
    haystack: &str,
    fallback: [ToFormat; N],
    args: A,
) -> String
where
    A: Into<Arguments<M>>,
{
    let args = args.into();
    match try_format(haystack, args.into_iter()) {
        Some(v) => v,
        None => {
            let mut args = args.into_iter();
            IntoIterator::into_iter(fallback).fold(String::new(), |mut accum, thing| {
                // Safe to unwrap iterator values, because the function
                // is always called with right number of args
                match thing {
                    ToFormat::Argument(Argument::Ordered(None)) => {
                        accum.push_str(args.next().unwrap())
                    }
                    ToFormat::Argument(Argument::Ordered(Some(n))) => {
                        accum.push_str(args.get(n).unwrap())
                    }
                    ToFormat::String(v) => accum.push_str(v),
                    ToFormat::Char(v) => accum.push(v),
                }
                accum
            })
        }
    }
}

fn try_format<const N: usize>(
    haystack: &str,
    mut args: private::ArgumentsIter<N>,
) -> Option<String> {
    let mut result = String::new();
    let mut cursor = 0;

    for p in Formatter::new(haystack) {
        let todo: (&str, usize, usize);

        match p {
            Pattern::Argument {
                arg: Argument::Ordered(None),
                start,
                end,
            } => todo = (args.next()?, start, end),
            Pattern::Argument {
                arg: Argument::Ordered(Some(n)),
                start,
                end,
            } => todo = (args.get(n)?, start, end),
            Pattern::Escaped {
                brace: Brace::Opening,
                start,
                end,
            } => todo = ("{", start, end),
            Pattern::Escaped {
                brace: Brace::Closing,
                start,
                end,
            } => todo = ("}", start, end),
            Pattern::Unescaped(_) => return None,
        }

        result.push_str(&haystack[cursor..todo.1]);
        result.push_str(todo.0);
        cursor = todo.2;
    }

    if !args.is_empty() {
        return None;
    }

    result.push_str(&haystack[cursor..]);
    Some(result)
}

pub enum ToFormat {
    Argument(Argument),
    String(&'static str),
    Char(char),
}

impl From<Option<usize>> for ToFormat {
    fn from(v: Option<usize>) -> Self {
        ToFormat::Argument(Argument::Ordered(v))
    }
}

impl From<&'static str> for ToFormat {
    fn from(v: &'static str) -> Self {
        ToFormat::String(v)
    }
}

impl From<char> for ToFormat {
    fn from(v: char) -> Self {
        ToFormat::Char(v)
    }
}

pub struct Arguments<const N: usize>([String; N]);

impl<const N: usize> From<[String; N]> for Arguments<N> {
    fn from(args: [String; N]) -> Self {
        Self(args)
    }
}

mod private {
    use super::Arguments;

    impl<'a, const N: usize> IntoIterator for &'a Arguments<N> {
        type Item = &'a str;
        type IntoIter = ArgumentsIter<'a, N>;

        fn into_iter(self) -> Self::IntoIter {
            Self::IntoIter {
                args: &self.0,
                state: [false; N],
                index: 0,
            }
        }
    }

    pub struct ArgumentsIter<'a, const N: usize> {
        args: &'a [String; N],
        state: [bool; N],
        index: usize,
    }

    impl<'a, const N: usize> Iterator for ArgumentsIter<'a, N> {
        type Item = &'a str;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.state.len() {
                let result = &self.args[self.index];
                self.state[self.index] = true;
                self.index += 1;
                Some(result)
            } else {
                None
            }
        }
    }

    impl<const N: usize> ArgumentsIter<'_, N> {
        pub fn get(&mut self, i: usize) -> Option<&str> {
            if i < self.state.len() {
                let result = &self.args[i];
                self.state[i] = true;
                Some(result)
            } else {
                None
            }
        }

        pub fn is_empty(&self) -> bool {
            for used in &self.state {
                if !used {
                    return false;
                }
            }

            true
        }
    }
}
