use gettext_macros::*;

fn main() {
	gettext!("{}");
	gettext!("{}", 1, 2);
	gettext!("{}{}");
	gettext!("{}{}", 1);
	gettext!("{}{}{}", 1, 2);
}
