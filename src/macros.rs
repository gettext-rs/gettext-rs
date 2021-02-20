
/// Like [gettext](fn.gettext.html), but allows for formatting.
#[macro_export]
macro_rules! gettext {
    ($($arg:tt)*) => ($crate::gettext(std::fmt::format(format_args!($($arg)*))))
}

/// Like [dgettext](fn.dgettext.html), but allows for formatting.
#[macro_export]
macro_rules! dgettext {
    ($domain:expr, $string:expr, $($string_names:ident = $string_values:expr),*) => ($crate::dgettext($domain, format!($string, $( $string_names = $string_values ),*)));
}

/// Like [dcgettext](fn.gettext.html), but allows for formatting.
#[macro_export]
macro_rules! dcgettext {
    ($domain:expr, $category:expr, $string:expr, $($string_names:ident = $string_values:expr),*,) => ($crate::dcgettext($domain, format!($string, $( $string_names = $string_values ),*), $category));
}

/// Like [ngettext](fn.gettext.html), but allows for formatting.
#[macro_export]
macro_rules! ngettext {
    ($singular:expr, $($singular_names:ident = $singular_values:expr),*, $plural:expr, $($plural_names:ident = $plural_values:expr),*, $n:expr) => ($crate::ngettext(format!($singular, $( $singular_names = $singular_values ),*), format!($plural, $( $plural_names = $plural_values ),*), $n));
}

/// Like [dngettext](fn.gettext.html), but allows for formatting.
#[macro_export]
macro_rules! dngettext {
    ($domain:expr, $singular:expr, $($singular_names:ident = $singular_values:expr),*, $plural:expr, $($plural_names:ident = $plural_values:expr),*, $n:expr) => ($crate::dngettext($domain, format!($singular, $( $singular_names = $singular_values ),*), format!($plural, $( $plural_names = $plural_values ),*), $n));
}

/// Like [dcngettext](fn.gettext.html), but allows for formatting.
#[macro_export]
macro_rules! dcngettext {
    ($domain:expr, $singular:expr, $($singular_names:ident = $singular_values:expr),*, $plural:expr, $($plural_names:ident = $plural_values:expr),*, $n:expr, $category:expr) => ($crate::dcngettext($domain, format!($singular, $( $singular_names = $singular_values ),*), format!($plural, $( $plural_names = $plural_values ),*), $n, $category));
}

/// Like [pgettext](fn.gettext.html), but allows for formatting.
#[macro_export]
macro_rules! pgettext {
    ($ctx:expr, $s:expr, $($s_names:ident = $s_values:expr),*) => ($crate::pgettext($ctx, format!($s, $( $s_names = $s_values ),*)));
}

/// Like [npgettext](fn.gettext.html), but allows for formatting.
#[macro_export]
macro_rules! npgettext {
    ($ctx:expr, $singular:expr, $($singular_names:ident = $singular_values:expr),*, $plural:expr, $($plural_names:ident = $plural_values:expr),*, $n:expr) => ($crate::npgettext($ctx, format!($singular, $( $singular_names = $singular_values ),*), format!($plural, $( $plural_names = $plural_values ),*), $n));
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_gettext_macro() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");

        bindtextdomain("hellorust", "/usr/local/share/locale");
        textdomain("hellorust");

        assert_eq!(gettext!("Hello, {name}!", name = "world"), "Hello, world!");
    }

    #[test]
    fn test_ngettext_macro() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");

        bindtextdomain("hellorust", "/usr/local/share/locale");
        textdomain("hellorust");

        assert_eq!(
            ngettext!(
                "Hello, {name}!",
                name = "world",
                "Hello, {names}!",
                names = "worlds",
                2
            ),
            "Hello, worlds!"
        );
    }

    #[test]
    fn test_pgettext_macro() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");

        bindtextdomain("hellorust", "/usr/local/share/locale");
        textdomain("hellorust");

        assert_eq!(
            "Hello, world!",
            pgettext!("context", "Hello, {name}!", name = "world")
        );
    }

    #[test]
    fn test_npgettext_macro() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");

        bindtextdomain("hellorust", "/usr/local/share/locale");
        textdomain("hellorust");

        assert_eq!(
            "Hello, worlds!",
            npgettext!(
                "context",
                "Hello, {name}!",
                name = "world",
                "Hello, {names}!",
                names = "worlds",
                2
            )
        );
    }

    #[test]
    fn test_dgettext_macro() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");

        bindtextdomain("hellorust", "/usr/local/share/locale");

        assert_eq!(
            "Hello, world!",
            dgettext!("hellorust", "Hello, {name}!", name = "world")
        );
    }

    #[test]
    fn test_dcgettext_macro() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");

        bindtextdomain("hellorust", "/usr/local/share/locale");

        assert_eq!(
            "Hello, world!",
            dcgettext!(
                "hellorust",
                LocaleCategory::LcAll,
                "Hello, {name}!",
                name = "world",
            )
        );
    }

    #[test]
    fn test_dcngettext_macro() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");

        bindtextdomain("hellorust", "/usr/local/share/locale");

        assert_eq!(
            "Hello, World!",
            dcngettext!(
                "hellorust",
                "Hello, {name}!",
                name = "World",
                "Hello, {names}!",
                names = "Worlds",
                1,
                LocaleCategory::LcAll
            )
        )
    }

    #[test]
    fn test_dngettext_macro() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");

        bindtextdomain("hellorust", "/usr/local/share/locale");

        assert_eq!(
            "Hello, World!",
            dngettext!(
                "hellrust",
                "Hello, {name}!",
                name = "World",
                "Hello, {names}!",
                names = "Worlds",
                1
            )
        )
    }
}
