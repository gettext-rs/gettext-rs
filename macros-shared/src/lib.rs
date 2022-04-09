use aho_corasick::{AhoCorasick, AhoCorasickBuilder, FindIter, MatchKind};
use once_cell::sync::Lazy;

static AC: Lazy<AhoCorasick> = Lazy::new(|| {
    AhoCorasickBuilder::new()
        .match_kind(MatchKind::LeftmostLongest)
        .build(["{}", "{{", "}}", "{", "}"])
});

#[cfg_attr(not(release), derive(PartialEq, Debug))]
pub enum Argument {
    Ordered(Option<usize>),
}

#[cfg_attr(not(release), derive(PartialEq, Debug))]
pub enum Pattern {
    Argument {
        arg: Argument,
        start: usize,
        end: usize,
    },
    Escaped {
        brace: Brace,
        start: usize,
        end: usize,
    },
    Unescaped(Brace),
}

#[cfg_attr(not(release), derive(PartialEq, Debug))]
pub enum Brace {
    Opening,
    Closing,
}

pub struct Formatter<'a, 'b> {
    haystack: &'a str,
    find_iter: FindIter<'b, 'a, usize>,
    patterns: [&'a str; 5],
    inside: (bool, usize, usize),
}

impl<'a> Formatter<'a, '_> {
    pub fn new(haystack: &'a str) -> Self {
        let patterns = ["{}", "{{", "}}", "{", "}"];
        Formatter {
            haystack,
            find_iter: AC.find_iter(haystack),
            patterns,
            inside: (false, 0, 0),
        }
    }
}

impl Iterator for Formatter<'_, '_> {
    type Item = Pattern;

    fn next(&mut self) -> Option<Self::Item> {
        use Brace::*;
        // use Pattern::*;

        for m in &mut self.find_iter {
            match self.patterns[m.pattern()] {
                "{}" => {
                    return Some(Pattern::Argument {
                        arg: Argument::Ordered(None),
                        start: m.start(),
                        end: m.end(),
                    })
                }
                "{{" => {
                    return Some(Pattern::Escaped {
                        brace: Opening,
                        start: m.start(),
                        end: m.end(),
                    })
                }
                "}}" => {
                    return Some(Pattern::Escaped {
                        brace: Closing,
                        start: m.start(),
                        end: m.end(),
                    })
                }
                "{" if !self.inside.0 => {
                    self.inside.0 = true;
                    self.inside.1 = m.start();
                    self.inside.2 = m.end();
                }
                "}" if self.inside.0 => {
                    match self.haystack[self.inside.2..m.start()].parse::<usize>() {
                        Ok(n) => {
                            self.inside.0 = false;
                            return Some(Pattern::Argument {
                                arg: Argument::Ordered(Some(n)),
                                start: self.inside.1,
                                end: m.end(),
                            });
                        }
                        _ => return Some(Pattern::Unescaped(Opening)),
                    }
                }
                "{" => return Some(Pattern::Unescaped(Opening)),
                "}" => return Some(Pattern::Unescaped(Closing)),
                _ => unreachable!(),
            }
        }

        if self.inside.0 {
            self.inside.0 = false;
            Some(Pattern::Unescaped(Opening))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_ordered_none() {
        let haystack = "{}";
        let expected = [Pattern::Argument {
            arg: Argument::Ordered(None),
            start: 0,
            end: 2,
        }];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }

    #[test]
    fn text_around_single_ordered_none() {
        let haystack = "Text {} text";
        let expected = [Pattern::Argument {
            arg: Argument::Ordered(None),
            start: 5,
            end: 7,
        }];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }

    #[test]
    fn multiple_ordered_none() {
        let haystack = "{}{}";
        let expected = [
            Pattern::Argument {
                arg: Argument::Ordered(None),
                start: 0,
                end: 2,
            },
            Pattern::Argument {
                arg: Argument::Ordered(None),
                start: 2,
                end: 4,
            },
        ];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }

    #[test]
    fn text_around_multiple_ordered_none() {
        let haystack = "Text {} text {} text";
        let expected = [
            Pattern::Argument {
                arg: Argument::Ordered(None),
                start: 5,
                end: 7,
            },
            Pattern::Argument {
                arg: Argument::Ordered(None),
                start: 13,
                end: 15,
            },
        ];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }

    #[test]
    fn unicode_haystack_ordered_none() {
        let haystack = "私は{}です!";
        let expected = [Pattern::Argument {
            arg: Argument::Ordered(None),
            start: 6,
            end: 8,
        }];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }

    #[test]
    fn escaped_opening() {
        let haystack = "{{";
        let expected = [Pattern::Escaped {
            brace: Brace::Opening,
            start: 0,
            end: 2,
        }];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }

    #[test]
    fn text_around_escaped_opening() {
        let haystack = "Text {{ text";
        let expected = [Pattern::Escaped {
            brace: Brace::Opening,
            start: 5,
            end: 7,
        }];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }

    #[test]
    fn multiple_escaped_opening() {
        let haystack = "{{{{";
        let expected = [
            Pattern::Escaped {
                brace: Brace::Opening,
                start: 0,
                end: 2,
            },
            Pattern::Escaped {
                brace: Brace::Opening,
                start: 2,
                end: 4,
            },
        ];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }

    #[test]
    fn text_around_multiple_escaped_opening() {
        let haystack = "Text {{ text {{ text";
        let expected = [
            Pattern::Escaped {
                brace: Brace::Opening,
                start: 5,
                end: 7,
            },
            Pattern::Escaped {
                brace: Brace::Opening,
                start: 13,
                end: 15,
            },
        ];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }

    #[test]
    fn escaped_closing() {
        let haystack = "}}";
        let expected = [Pattern::Escaped {
            brace: Brace::Closing,
            start: 0,
            end: 2,
        }];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }

    #[test]
    fn text_around_escaped_closing() {
        let haystack = "Text }} text";
        let expected = [Pattern::Escaped {
            brace: Brace::Closing,
            start: 5,
            end: 7,
        }];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }

    #[test]
    fn multiple_escaped_closing() {
        let haystack = "}}}}";
        let expected = [
            Pattern::Escaped {
                brace: Brace::Closing,
                start: 0,
                end: 2,
            },
            Pattern::Escaped {
                brace: Brace::Closing,
                start: 2,
                end: 4,
            },
        ];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }

    #[test]
    fn text_around_multiple_escaped_closing() {
        let haystack = "Text }} text }} text";
        let expected = [
            Pattern::Escaped {
                brace: Brace::Closing,
                start: 5,
                end: 7,
            },
            Pattern::Escaped {
                brace: Brace::Closing,
                start: 13,
                end: 15,
            },
        ];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }

    #[test]
    fn escaped_pair() {
        let haystack = "{{}}";
        let expected = [
            Pattern::Escaped {
                brace: Brace::Opening,
                start: 0,
                end: 2,
            },
            Pattern::Escaped {
                brace: Brace::Closing,
                start: 2,
                end: 4,
            },
        ];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }

    #[test]
    fn text_around_escaped_pair() {
        let haystack = "Text {{}} text";
        let expected = [
            Pattern::Escaped {
                brace: Brace::Opening,
                start: 5,
                end: 7,
            },
            Pattern::Escaped {
                brace: Brace::Closing,
                start: 7,
                end: 9,
            },
        ];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }

    #[test]
    fn unicode_haystack_escaped_pair() {
        let haystack = "私は{{}}です!";
        let expected = [
            Pattern::Escaped {
                brace: Brace::Opening,
                start: 6,
                end: 8,
            },
            Pattern::Escaped {
                brace: Brace::Closing,
                start: 8,
                end: 10,
            },
        ];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }

    #[test]
    fn unescaped_opening() {
        let haystack = "{";
        let expected = [Pattern::Unescaped(Brace::Opening)];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }

    #[test]
    fn text_around_unescaped_opening() {
        let haystack = "Text { text";
        let expected = [Pattern::Unescaped(Brace::Opening)];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }

    #[test]
    fn multiple_unescaped_opening() {
        let haystack = "{ {";
        let expected = [
            Pattern::Unescaped(Brace::Opening),
            Pattern::Unescaped(Brace::Opening),
        ];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }

    #[test]
    fn text_around_multiple_unescaped_opening() {
        let haystack = "Text { text { text";
        let expected = [
            Pattern::Unescaped(Brace::Opening),
            Pattern::Unescaped(Brace::Opening),
        ];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }

    #[test]
    fn unescaped_closing() {
        let haystack = "}";
        let expected = [Pattern::Unescaped(Brace::Closing)];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }

    #[test]
    fn text_around_unescaped_closing() {
        let haystack = "Text } text";
        let expected = [Pattern::Unescaped(Brace::Closing)];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }

    #[test]
    fn multiple_unescaped_closing() {
        let haystack = "} }";
        let expected = [
            Pattern::Unescaped(Brace::Closing),
            Pattern::Unescaped(Brace::Closing),
        ];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }

    #[test]
    fn text_around_multiple_unescaped_closing() {
        let haystack = "Text } text } text";
        let expected = [
            Pattern::Unescaped(Brace::Closing),
            Pattern::Unescaped(Brace::Closing),
        ];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }

    #[test]
    fn unescaped_braces() {
        let haystack = "}{";
        let expected = [
            Pattern::Unescaped(Brace::Closing),
            Pattern::Unescaped(Brace::Opening),
        ];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }

    #[test]
    fn text_around_unescaped_braces() {
        let haystack = "Text } text { text";
        let expected = [
            Pattern::Unescaped(Brace::Closing),
            Pattern::Unescaped(Brace::Opening),
        ];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }

    #[test]
    fn unicode_haystack_unescaped_braces() {
        let haystack = "私は}{です!";
        let expected = [
            Pattern::Unescaped(Brace::Closing),
            Pattern::Unescaped(Brace::Opening),
        ];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }

    #[test]
    fn single_ordered_some() {
        let haystack = "{0}";
        let expected = [Pattern::Argument {
            arg: Argument::Ordered(Some(0)),
            start: 0,
            end: 3,
        }];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }

    #[test]
    fn text_around_single_ordered_some() {
        let haystack = "Text {0} text";
        let expected = [Pattern::Argument {
            arg: Argument::Ordered(Some(0)),
            start: 5,
            end: 8,
        }];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }

    #[test]
    fn multiple_ordered_some() {
        let haystack = "{0}{1}";
        let expected = [
            Pattern::Argument {
                arg: Argument::Ordered(Some(0)),
                start: 0,
                end: 3,
            },
            Pattern::Argument {
                arg: Argument::Ordered(Some(1)),
                start: 3,
                end: 6,
            },
        ];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }

    #[test]
    fn text_around_multiple_ordered_some() {
        let haystack = "Text {0} text {1} text";
        let expected = [
            Pattern::Argument {
                arg: Argument::Ordered(Some(0)),
                start: 5,
                end: 8,
            },
            Pattern::Argument {
                arg: Argument::Ordered(Some(1)),
                start: 14,
                end: 17,
            },
        ];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }

    #[test]
    fn unicode_haystack_ordered_some() {
        let haystack = "私は{0}です!";
        let expected = [Pattern::Argument {
            arg: Argument::Ordered(Some(0)),
            start: 6,
            end: 9,
        }];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }
}
