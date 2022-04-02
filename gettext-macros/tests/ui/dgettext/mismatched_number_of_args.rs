use gettext_macros::*;

fn main() {
    dgettext!("domainname", "{}");
    dgettext!("domainname", "{}", 1, 2);
    dgettext!("domainname", "{}{}");
    dgettext!("domainname", "{}{}", 1);
    dgettext!("domainname", "{}{}{}", 1, 2);
}
