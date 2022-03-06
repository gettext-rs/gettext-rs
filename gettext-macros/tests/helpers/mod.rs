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
}
