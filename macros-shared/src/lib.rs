use aho_corasick::{AhoCorasick, AhoCorasickBuilder, FindIter, MatchKind};
use once_cell::sync::Lazy;

static AC: Lazy<AhoCorasick> = Lazy::new(|| {
    AhoCorasickBuilder::new()
        .match_kind(MatchKind::LeftmostLongest)
        .build(["{}", "{{", "}}", "{", "}"])
});

#[cfg_attr(test, derive(PartialEq, Debug))]
pub enum Pattern {
    Ordered(Option<usize>, usize, usize),
    Escaped(Brace, usize, usize),
    Unescaped(Brace),
}

#[cfg_attr(test, derive(PartialEq, Debug))]
pub enum Brace {
    Opening,
    Closing,
}

pub struct Formatter<'a, 'b> {
    haystack: &'a str,
    find_iter: FindIter<'b, 'a, usize>,
    patterns: [&'a str; 5],
    cursor: usize,
    inside: (bool, usize, usize),
}

impl<'a> Formatter<'a, '_> {
    pub fn new(haystack: &'a str) -> Self {
        let patterns = ["{}", "{{", "}}", "{", "}"];
        Formatter {
            haystack,
            find_iter: AC.find_iter(haystack),
            patterns,
            cursor: 0,
            inside: (false, 0, 0),
        }
    }
}

impl Iterator for Formatter<'_, '_> {
    type Item = Pattern;

    fn next(&mut self) -> Option<Self::Item> {
        use Brace::*;
        use Pattern::*;

        for m in &mut self.find_iter {
            self.cursor = m.end();

            match self.patterns[m.pattern()] {
                "{}" => return Some(Ordered(None, m.start(), m.end())),
                "{{" => return Some(Escaped(Opening, m.start(), m.end())),
                "}}" => return Some(Escaped(Closing, m.start(), m.end())),
                "{" if !self.inside.0 => {
                    self.inside.0 = true;
                    self.inside.1 = m.start();
                    self.inside.2 = m.end();
                }
                "}" if self.inside.0 => {
                    match self.haystack[self.inside.2..m.start()].parse::<usize>() {
                        Ok(n) => {
                            self.inside.0 = false;
                            return Some(Ordered(Some(n), self.inside.1, m.end()));
                        }
                        _ => return Some(Unescaped(Opening)),
                    }
                }
                "{" => return Some(Unescaped(Opening)),
                "}" => return Some(Unescaped(Closing)),
                _ => unreachable!(),
            }
        }

        if self.inside.0 {
            self.inside.0 = false;
            Some(Unescaped(Opening))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Brace::*, Formatter, Pattern::*};

    #[test]
    fn single_ordered_none() {
        let haystack = "{}";
        let expected = [Ordered(None, 0, 2)];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }

    #[test]
    fn text_around_single_ordered_none() {
        let haystack = "Text {} text";
        let expected = [Ordered(None, 5, 7)];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }

    #[test]
    fn multiple_ordered_none() {
        let haystack = "{}{}";
        let expected = [Ordered(None, 0, 2), Ordered(None, 2, 4)];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }

    #[test]
    fn text_around_multiple_ordered_none() {
        let haystack = "Text {} text {} text";
        let expected = [Ordered(None, 5, 7), Ordered(None, 13, 15)];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }

    #[test]
    fn unicode_haystack_ordered_none() {
        let haystack = "私は{}です!";
        let expected = [Ordered(None, 6, 8)];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }

    #[test]
    fn escaped_opening() {
        let haystack = "{{";
        let expected = [Escaped(Opening, 0, 2)];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }

    #[test]
    fn text_around_escaped_opening() {
        let haystack = "Text {{ text";
        let expected = [Escaped(Opening, 5, 7)];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }

    #[test]
    fn multiple_escaped_opening() {
        let haystack = "{{{{";
        let expected = [Escaped(Opening, 0, 2), Escaped(Opening, 2, 4)];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }

    #[test]
    fn text_around_multiple_escaped_opening() {
        let haystack = "Text {{ text {{ text";
        let expected = [Escaped(Opening, 5, 7), Escaped(Opening, 13, 15)];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }

    #[test]
    fn escaped_closing() {
        let haystack = "}}";
        let expected = [Escaped(Closing, 0, 2)];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }

    #[test]
    fn text_around_escaped_closing() {
        let haystack = "Text }} text";
        let expected = [Escaped(Closing, 5, 7)];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }

    #[test]
    fn multiple_escaped_closing() {
        let haystack = "}}}}";
        let expected = [Escaped(Closing, 0, 2), Escaped(Closing, 2, 4)];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }

    #[test]
    fn text_around_multiple_escaped_closing() {
        let haystack = "Text }} text }} text";
        let expected = [Escaped(Closing, 5, 7), Escaped(Closing, 13, 15)];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }

    #[test]
    fn escaped_pair() {
        let haystack = "{{}}";
        let expected = [Escaped(Opening, 0, 2), Escaped(Closing, 2, 4)];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }

    #[test]
    fn text_around_escaped_pair() {
        let haystack = "Text {{}} text";
        let expected = [Escaped(Opening, 5, 7), Escaped(Closing, 7, 9)];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }

    #[test]
    fn unicode_haystack_escaped_pair() {
        let haystack = "私は{{}}です!";
        let expected = [Escaped(Opening, 6, 8), Escaped(Closing, 8, 10)];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }

    #[test]
    fn unescaped_opening() {
        let haystack = "{";
        let expected = [Unescaped(Opening)];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }

    #[test]
    fn text_around_unescaped_opening() {
        let haystack = "Text { text";
        let expected = [Unescaped(Opening)];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }

    #[test]
    fn multiple_unescaped_opening() {
        let haystack = "{ {";
        let expected = [Unescaped(Opening), Unescaped(Opening)];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }

    #[test]
    fn text_around_multiple_unescaped_opening() {
        let haystack = "Text { text { text";
        let expected = [Unescaped(Opening), Unescaped(Opening)];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }

    #[test]
    fn unescaped_closing() {
        let haystack = "}";
        let expected = [Unescaped(Closing)];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }

    #[test]
    fn text_around_unescaped_closing() {
        let haystack = "Text } text";
        let expected = [Unescaped(Closing)];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }

    #[test]
    fn multiple_unescaped_closing() {
        let haystack = "} }";
        let expected = [Unescaped(Closing), Unescaped(Closing)];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }

    #[test]
    fn text_around_multiple_unescaped_closing() {
        let haystack = "Text } text } text";
        let expected = [Unescaped(Closing), Unescaped(Closing)];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }

    #[test]
    fn unescaped_braces() {
        let haystack = "}{";
        let expected = [Unescaped(Closing), Unescaped(Opening)];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }

    #[test]
    fn text_around_unescaped_braces() {
        let haystack = "Text } text { text";
        let expected = [Unescaped(Closing), Unescaped(Opening)];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }

    #[test]
    fn unicode_haystack_unescaped_braces() {
        let haystack = "私は}{です!";
        let expected = [Unescaped(Closing), Unescaped(Opening)];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }

    #[test]
    fn single_ordered_some() {
        let haystack = "{0}";
        let expected = [Ordered(Some(0), 0, 3)];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }

    #[test]
    fn text_around_single_ordered_some() {
        let haystack = "Text {0} text";
        let expected = [Ordered(Some(0), 5, 8)];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }

    #[test]
    fn multiple_ordered_some() {
        let haystack = "{0}{1}";
        let expected = [Ordered(Some(0), 0, 3), Ordered(Some(1), 3, 6)];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }

    #[test]
    fn text_around_multiple_ordered_some() {
        let haystack = "Text {0} text {1} text";
        let expected = [Ordered(Some(0), 5, 8), Ordered(Some(1), 14, 17)];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }

    #[test]
    fn unicode_haystack_ordered_some() {
        let haystack = "私は{0}です!";
        let expected = [Ordered(Some(0), 6, 9)];

        let actual: Vec<_> = Formatter::new(haystack).collect();

        assert_eq!(expected, actual[..])
    }
}
