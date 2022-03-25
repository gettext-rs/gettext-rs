#[macro_export]
macro_rules! fake {
    (gettext, $msgstr:literal) => {
        mod gettextrs {
            pub use ::gettextrs::formatter;
            pub fn gettext<T: Into<String>>(_msgid: T) -> String {
                $msgstr.into()
            }
        }
    };
    (dgettext, $msgstr:literal) => {
        mod gettextrs {
            pub use ::gettextrs::formatter;
            pub fn dgettext<T, U>(_domainname: T, _msgid: U) -> String
            where
                T: Into<String>,
                U: Into<String>,
            {
                $msgstr.into()
            }
        }
    };
}
