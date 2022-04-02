use gettext_macros::*;

fn main() {
    dgettext!("domainname", "{0}");
    dgettext!("domainname", "{}{1}", 1);
    dgettext!("domainname", "{}{}{2}", 1, 2);
}
