use gettext_macros::*;

fn main() {
    dgettext!("domainname", "Hello, {!");
    dgettext!("domainname", "Hello, }!");
}
