use macros_shared::*;

pub struct PosArguments<'a> {
    args: &'a [String],
    state: Vec<bool>,
    index: usize,
}

impl PosArguments<'_> {
    pub fn get(&mut self, i: usize) -> Option<&String> {
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

impl<'a, T: AsRef<[String]>> From<&'a T> for PosArguments<'a> {
    fn from(args: &'a T) -> Self {
        let args = args.as_ref();
        let mut state = vec![];
        for _ in args {
            state.push(false)
        }
        Self {
            args,
            state,
            index: 0,
        }
    }
}

impl<'a> Iterator for PosArguments<'a> {
    type Item = &'a String;

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

pub fn format<'a, T: Into<PosArguments<'a>>>(haystack: &str, pos_args: T) -> Option<String> {
    let mut pos_args = pos_args.into();
    let mut result = String::new();
    let mut cursor = 0;
    let mut inside = (false, 0);

    for m in directives_parser().find_iter(haystack) {
        if inside.0 {
            inside.1 = cursor;
        } else {
            result.push_str(&haystack[cursor..m.start()]);
        }
        cursor = m.end();

        match DIRECTIVES_PATTERNS[m.pattern()] {
            "{}" => match pos_args.next() {
                Some(v) => result.push_str(v),
                _ => return None,
            },
            "{{" => result.push('{'),
            "}}" => result.push('}'),
            "{" if !inside.0 => inside.0 = true,
            "}" if inside.0 => match haystack[inside.1..m.start()].parse::<usize>() {
                Ok(n) => {
                    inside.0 = false;
                    match pos_args.get(n) {
                        Some(v) => result.push_str(v),
                        _ => return None,
                    }
                }
                _ => return None,
            },
            _ => return None,
        }
    }

    if !pos_args.is_empty() || inside.0 {
        return None;
    }

    result.push_str(&haystack[cursor..]);
    Some(result)
}
