use macros_shared::{Brace::*, Pattern::*, *};

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

    for p in Formatter::new(haystack) {
        let todo: (&str, usize, usize);

        match p {
            Ordered(None, start, end) => match pos_args.next() {
                Some(v) => todo = (v, start, end),
                _ => return None,
            },
            Ordered(Some(n), start, end) => match pos_args.get(n) {
                Some(v) => todo = (v, start, end),
                _ => return None,
            },
            Escaped(Opening, start, end) => todo = ("{", start, end),
            Escaped(Closing, start, end) => todo = ("}", start, end),
            Unescaped(_) => return None,
        }

        result.push_str(&haystack[cursor..todo.1]);
        result.push_str(todo.0);
        cursor = todo.2;
    }

    if !pos_args.is_empty() {
        return None;
    }

    result.push_str(&haystack[cursor..]);
    Some(result)
}
