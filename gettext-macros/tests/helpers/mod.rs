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
    (ngettext, $msgstr:literal, $msgstr_plural:literal) => {
        mod gettextrs {
            pub use ::gettextrs::formatter;
            pub fn ngettext<T, S>(_msgid: T, _msgid_plural: S, n: u32) -> String
            where
                T: Into<String>,
                S: Into<String>,
            {
                if n == 1 {
                    $msgstr.into()
                } else {
                    $msgstr_plural.into()
                }
            }
        }
    };
}
