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
    ($input:expr, $($args:expr),+ $(,)?) => {{
        let mut parts = $input.split("{}");
        debug_assert_eq!(parts.clone().count() - 1, $crate::count_args!($($args),*), "Argument count has to match number of format directives ({{}})");

        let mut output = parts.next().unwrap_or_default().to_string();
        $(
            output += &format!("{}{}", $args, &parts.next().expect("Argument count has to match number of format directives ({})"));
        )*
        output
    }};
}

/// Like [gettext](fn.gettext.html), but allows for formatting.
#[macro_export]
macro_rules! gettext {
    ($format:expr, $($args:expr),+ $(,)?) => {{
        let s = $crate::gettext($format);
        $crate::freplace!(s, $($args),*)
    }};
}

/// Like [dgettext](fn.dgettext.html), but allows for formatting.
#[macro_export]
macro_rules! dgettext {
    ($domain:expr, $format:expr, $($args:expr),+ $(,)?) => {{
        let s = $crate::dgettext($domain, $format);
        $crate::freplace!(s, $($args),*)
    }};
}

/// Like [dcgettext](fn.dcgettext.html), but allows for formatting.
#[macro_export]
macro_rules! dcgettext {
    ($domain:expr, $category:expr, $format:expr, $($args:expr),+ $(,)?) => {{
        let s = $crate::dcgettext($domain, $format, $category);
        $crate::freplace!(s, $($args),*)
    }};
}

/// Like [ngettext](fn.ngettext.html), but allows for formatting.
#[macro_export]
macro_rules! ngettext {
    ($singular:expr, $plural:expr, $n:expr, $($args:expr),+ $(,)?) => {{
        let s = $crate::ngettext($singular, $plural, $n);
        $crate::freplace!(s, $($args),*)
    }}
}

/// Like [dngettext](fn.dngettext.html), but allows for formatting.
#[macro_export]
macro_rules! dngettext {
    ($domain:expr, $singular:expr, $plural:expr, $n:expr, $($args:expr),+ $(,)?) => {{
        let s = $crate::dngettext($domain, $singular, $plural, $n);
        $crate::freplace!(s, $($args),*)
    }}
}

/// Like [dcngettext](fn.dcngettext.html), but allows for formatting.
#[macro_export]
macro_rules! dcngettext {
    ($domain:expr, $category:expr, $singular:expr, $plural:expr, $n:expr, $($args:expr),+ $(,)?) => {{
        let s = $crate::dcngettext($domain, $singular, $plural, $n, $category);
        $crate::freplace!(s, $($args),*)
    }}
}

/// Like [pgettext](fn.pgettext.html), but allows for formatting.
#[macro_export]
macro_rules! pgettext {
    ($ctx:expr, $format:expr, $($args:expr),+ $(,)?) => {{
        let s = $crate::pgettext($ctx, $format);
        $crate::freplace!(s, $($args),*)
    }}
}

/// Like [npgettext](fn.npgettext.html), but allows for formatting.
#[macro_export]
macro_rules! npgettext {
    ($ctx:expr, $singular:expr, $plural:expr, $n:expr, $($args:expr),+ $(,)?) => {{
        let s = $crate::npgettext($ctx, $singular, $plural, $n);
        $crate::freplace!(s, $($args),*)
    }}
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_gettext_macro() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");

        bindtextdomain("hellorust", "/usr/local/share/locale");
        textdomain("hellorust");

        assert_eq!(gettext!("Hello, {}!", "world"), "Hello, world!");
        assert_eq!(
            gettext!("Hello, {} {}!", "small", "world"),
            "Hello, small world!"
        );
    }

    #[test]
    fn test_ngettext_macro() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");

        bindtextdomain("hellorust", "/usr/local/share/locale");
        textdomain("hellorust");

        assert_eq!(
            ngettext!("Singular {}!", "Multiple {}!", 2, "Worlds"),
            "Multiple Worlds!"
        );
    }

    #[test]
    fn test_pgettext_macro() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");

        bindtextdomain("hellorust", "/usr/local/share/locale");
        textdomain("hellorust");

        assert_eq!("Hello, world!", pgettext!("context", "Hello, {}!", "world"));
    }

    #[test]
    fn test_npgettext_macro() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");

        bindtextdomain("hellorust", "/usr/local/share/locale");
        textdomain("hellorust");

        assert_eq!(
            "Multiple Worlds!",
            npgettext!("context", "Singular {}!", "Multiple {}!", 2, "Worlds")
        );
    }

    #[test]
    fn test_dgettext_macro() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");

        bindtextdomain("hellorust", "/usr/local/share/locale");

        assert_eq!(
            "Hello, world!",
            dgettext!("hellorust", "Hello, {}!", "world")
        );
    }

    #[test]
    fn test_dcgettext_macro() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");

        bindtextdomain("hellorust", "/usr/local/share/locale");

        assert_eq!(
            "Hello, world!",
            dcgettext!("hellorust", LocaleCategory::LcAll, "Hello, {}!", "world")
        );
    }

    #[test]
    fn test_dcngettext_macro() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");

        bindtextdomain("hellorust", "/usr/local/share/locale");

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

        bindtextdomain("hellorust", "/usr/local/share/locale");

        assert_eq!(
            "Singular World!",
            dngettext!("hellrust", "Singular {}!", "Multiple {}!", 1, "World")
        )
    }
}
