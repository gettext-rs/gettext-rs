use gettext_macros::*;

fn main() {
    dgettext!("domainname");
    dgettext!("domainname", 1, 2);
}
