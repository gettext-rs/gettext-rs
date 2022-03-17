use gettext_macros::*;

fn main() {
    gettext!("{0}");
    gettext!("{}{1}", 1);
    gettext!("{}{}{2}", 1, 2);
}
