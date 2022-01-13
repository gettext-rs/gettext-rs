pub fn check_amount(n_directives: usize, n_arguments: usize) -> Result<usize, String> {
    if n_directives != n_arguments {
        let f;
        Err(format!(
            "{} positional {} in format string, but {}",
            n_directives,
            {
                if n_directives == 1 {
                    "argument"
                } else {
                    "arguments"
                }
            },
            {
                if n_arguments == 0 {
                    "no arguments were given"
                } else if n_arguments == 1 {
                    "there is 1 argument"
                } else {
                    f = format!("there are {} arguments", n_arguments);
                    &f
                }
            }
        ))
    } else {
        Ok(n_directives)
    }
}
