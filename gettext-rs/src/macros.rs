//! Macros that translate the message and then replace placeholders in it.

/// This is an implementation detail for counting arguments in the gettext macros. Don't call this directly.
#[macro_export]
#[doc(hidden)]
macro_rules! count_args {
    () => { 0 };
    ($_e: expr $(, $rest: expr)*) => { 1 + $crate::count_args!($($rest),*) }
}

/// This is an implementation detail for replacing arguments in the gettext macros. Don't call this directly.
#[macro_export]
#[doc(hidden)]
macro_rules! freplace {
    ($format:expr, $($args:expr),+ $(,)?) => {{
        let mut parts = $format.split("{}");
        debug_assert_eq!(parts.clone().count() - 1, $crate::count_args!($($args),*), "Argument count has to match number of format directives ({{}})");

        let mut output = parts.next().unwrap_or_default().to_string();
        $(
            output += &format!("{}{}", $args, &parts.next().expect("Argument count has to match number of format directives ({})"));
        )*
        output
    }};
}

/// Like [`gettext`], but allows for formatting.
///
/// It calls [`gettext`] on `msgid`, and then replaces each occurrence of `{}` with the next value
/// out of `args`.
///
/// [`gettext`]: fn.gettext.html
#[macro_export]
macro_rules! gettext {
    ($msgid:expr, $($args:expr),+ $(,)?) => {{
        let format = $crate::gettext($msgid);
        $crate::freplace!(format, $($args),*)
    }};
}

/// Like [`dgettext`], but allows for formatting.
///
/// It calls [`dgettext`] on `domainname` and `msgid`, and then replaces each occurrence of `{}`
/// with the next value out of `args`.
///
/// [`dgettext`]: fn.dgettext.html
#[macro_export]
macro_rules! dgettext {
    ($domainname:expr, $msgid:expr, $($args:expr),+ $(,)?) => {{
        let format = $crate::dgettext($domainname, $msgid);
        $crate::freplace!(format, $($args),*)
    }};
}

/// Like [`dcgettext`], but allows for formatting.
///
/// It calls [`dcgettext`] on `domainname`, `category`, and `msgid`, and then replaces each
/// occurrence of `{}` with the next value out of `args`.
///
/// [`dcgettext`]: fn.dcgettext.html
#[macro_export]
macro_rules! dcgettext {
    ($domainname:expr, $category:expr, $msgid:expr, $($args:expr),+ $(,)?) => {{
        let format = $crate::dcgettext($domainname, $msgid, $category);
        $crate::freplace!(format, $($args),*)
    }};
}

/// Like [`ngettext`], but allows for formatting.
///
/// It calls [`ngettext`] on `msgid`, `msgid_plural`, and `n`, and then replaces each occurrence of
/// `{}` with the next value out of `args`.
///
/// [`ngettext`]: fn.ngettext.html
#[macro_export]
macro_rules! ngettext {
    ($msgid:expr, $msgid_plural:expr, $n:expr, $($args:expr),+ $(,)?) => {{
        let format = $crate::ngettext($msgid, $msgid_plural, $n);
        $crate::freplace!(format, $($args),*)
    }}
}

/// Like [`dngettext`], but allows for formatting.
///
/// It calls [`dngettext`] on `domainname`, `msgid`, `msgid_plural`, and `n`, and then replaces
/// each occurrence of `{}` with the next value out of `args`.
///
/// [`dngettext`]: fn.dngettext.html
#[macro_export]
macro_rules! dngettext {
    ($domainname:expr, $msgid:expr, $msgid_plural:expr, $n:expr, $($args:expr),+ $(,)?) => {{
        let format = $crate::dngettext($domainname, $msgid, $msgid_plural, $n);
        $crate::freplace!(format, $($args),*)
    }}
}

/// Like [`dcngettext`], but allows for formatting.
///
/// It calls [`dcngettext`] on `domainname`, `category`, `msgid`, `msgid_plural`, and `n`, and then
/// replaces each occurrence of `{}` with the next value out of `args`.
///
/// [`dcngettext`]: fn.dcngettext.html
#[macro_export]
macro_rules! dcngettext {
    ($domainname:expr, $category:expr, $msgid:expr, $msgid_plural:expr, $n:expr, $($args:expr),+ $(,)?) => {{
        let format = $crate::dcngettext($domainname, $msgid, $msgid_plural, $n, $category);
        $crate::freplace!(format, $($args),*)
    }}
}

/// Like [`pgettext`], but allows for formatting.
///
/// It calls [`pgettext`] on `msgctxt` and `msgid`, and then replaces each occurrence of `{}` with
/// the next value out of `args`.
///
/// [`pgettext`]: fn.pgettext.html
#[macro_export]
macro_rules! pgettext {
    ($msgctxt:expr, $msgid:expr, $($args:expr),+ $(,)?) => {{
        let format = $crate::pgettext($msgctxt, $msgid);
        $crate::freplace!(format, $($args),*)
    }}
}

/// Like [`npgettext`], but allows for formatting.
///
/// It calls [`npgettext`] on `msgctxt`, `msgid`, `msgid_plural`, and `n`, and then replaces each
/// occurrence of `{}` with the next value out of `args`.
///
/// [`npgettext`]: fn.npgettext.html
#[macro_export]
macro_rules! npgettext {
    ($msgctxt:expr, $msgid:expr, $msgid_plural:expr, $n:expr, $($args:expr),+ $(,)?) => {{
        let format = $crate::npgettext($msgctxt, $msgid, $msgid_plural, $n);
        $crate::freplace!(format, $($args),*)
    }}
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_gettext_macro() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");

        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();
        textdomain("hellorust").unwrap();

        assert_eq!(gettext!("Hello, {}!", "world"), "Hello, world!");
        assert_eq!(
            gettext!("Hello, {} {}!", "small", "world"),
            "Hello, small world!"
        );
    }

    #[test]
    fn test_ngettext_macro() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");

        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();
        textdomain("hellorust").unwrap();

        assert_eq!(
            ngettext!("Singular {}!", "Multiple {}!", 2, "Worlds"),
            "Multiple Worlds!"
        );
    }

    #[test]
    fn test_pgettext_macro() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");

        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();
        textdomain("hellorust").unwrap();

        assert_eq!("Hello, world!", pgettext!("context", "Hello, {}!", "world"));
    }

    #[test]
    fn test_npgettext_macro() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");

        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();
        textdomain("hellorust").unwrap();

        assert_eq!(
            "Multiple Worlds!",
            npgettext!("context", "Singular {}!", "Multiple {}!", 2, "Worlds")
        );
    }

    #[test]
    fn test_dgettext_macro() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");

        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();

        assert_eq!(
            "Hello, world!",
            dgettext!("hellorust", "Hello, {}!", "world")
        );
    }

    #[test]
    fn test_dcgettext_macro() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");

        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();

        assert_eq!(
            "Hello, world!",
            dcgettext!("hellorust", LocaleCategory::LcAll, "Hello, {}!", "world")
        );
    }

    #[test]
    fn test_dcngettext_macro() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");

        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();

        assert_eq!(
            "Singular World",
            dcngettext!(
                "hellorust",
                LocaleCategory::LcAll,
                "Singular {}",
                "Multiple {}",
                1,
                "World"
            )
        )
    }

    #[test]
    fn test_dngettext_macro() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");

        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();

        assert_eq!(
            "Singular World!",
            dngettext!("hellrust", "Singular {}!", "Multiple {}!", 1, "World")
        )
    }
}
