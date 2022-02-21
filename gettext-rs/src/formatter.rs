pub fn format(haystack: &str, pos_args: &[String]) -> Option<String> {
    let mut result = String::new();
    let mut chars = haystack.chars().peekable();
    let mut pos_args = pos_args.iter();

    while let Some(c) = chars.next() {
        if c == '{' && chars.next_if_eq(&'}').is_some() {
            match pos_args.next() {
                Some(arg) => result.push_str(arg),
                None => return None,
            }
        } else if (c == '{' || c == '}') && chars.next_if_eq(&c).is_some() {
            result.push(c);
        } else if c == '{' || c == '}' {
            return None;
        } else {
            result.push(c);
        }
    }

    if pos_args.next().is_some() {
        return None;
    }

    Some(result)
}
