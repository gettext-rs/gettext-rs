use aho_corasick::{AhoCorasick, AhoCorasickBuilder, MatchKind};

pub static DIRECTIVES_PATTERNS: [&str; 5] = ["{}", "{{", "}}", "{", "}"];

pub fn directives_parser() -> AhoCorasick {
    AhoCorasickBuilder::new()
        .match_kind(MatchKind::LeftmostLongest)
        .build(DIRECTIVES_PATTERNS)
}
