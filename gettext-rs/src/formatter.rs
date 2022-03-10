use macros_shared::*;

pub fn format(haystack: &str, pos_args: &[String]) -> Option<String> {
    let mut result = String::new();
    let mut cursor = 0;
    let mut pos_args = pos_args.iter();

    for m in directives_parser().find_iter(haystack) {
        result.push_str(&haystack[cursor..m.start()]);
        cursor = m.end();

        match DIRECTIVES_PATTERNS[m.pattern()] {
            "{}" => match pos_args.next() {
                Some(v) => result.push_str(v),
                None => return None,
            },
            "{{" => result.push('{'),
            "}}" => result.push('}'),
            _ => return None,
        }
    }

    if pos_args.next().is_some() {
        return None;
    }

    result.push_str(&haystack[cursor..]);
    Some(result)
}
