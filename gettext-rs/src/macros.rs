//! Macros that translate the message and then replace placeholders in it.

/// This is an implementation detail for replacing arguments in the gettext macros. Don't call this directly.
#[doc(hidden)]
#[allow(dead_code)]
fn freplace(format: &str, args: &[&str]) -> String {
    let mut parts = format.split("{}");
    debug_assert_eq!(parts.clone().count() - 1, args.len(), "Argument count has to match number of format directives ({{}})");

    let mut output = parts.next().unwrap_or_default().to_string();
    for (arg, part) in args.into_iter().zip(parts) {
        output += &format!("{}{}", arg, part);
    }

    output
}

/// Like [`gettext`], but allows for formatting.
///
/// It calls [`gettext`] on `msgid`, and then replaces each occurrence of `{}` with the next value
/// out of `args`.
///
/// [`gettext`]: fn.gettext.html
#[macro_export]
macro_rules! gettext {
    (domain = $domainname:expr, category = $category:expr, $msgid:expr, $msgid_plural:expr, $n:expr, [$($args:expr),+]) => {{
        let format = $crate::dcngettext($domainname, $msgid, $msgid_plural, $n, $category);
        $crate::macros::freplace(&format, &[$($args),*])
    }};

    (domain = $domainname:expr, category = $category:expr, $msgid:expr, $msgid_plural:expr, $n:expr) => {
        $crate::dcngettext($domainname, $msgid, $msgid_plural, $n, $category)
    };

    (domain = $domainname:expr, category = $category:expr, $msgid:expr, [$($args:expr),+]) => {{
        let format = $crate::dcgettext($domainname, $msgid, $category);
        $crate::macros::freplace(&format, &[$($args),*])
    }};

    (domain = $domainname:expr, category = $category:expr, $msgid:expr) => {
        $crate::dcgettext($domainname, $msgid, $category)
    };

    (domain = $domainname:expr, $msgid:expr, $msgid_plural:expr, $n:expr, [$($args:expr),+]) => {{
        let format = $crate::dngettext($domainname, $msgid, $msgid_plural, $n);
        $crate::macros::freplace(&format, &[$($args),*])
    }};

    (domain = $domainname:expr, $msgid:expr, $msgid_plural:expr, $n:expr) => {
        $crate::dngettext($domainname, $msgid, $msgid_plural, $n)
    };

    (domain = $domainname:expr, $msgid:expr, [$($args:expr),+]) => {{
        let format = $crate::dgettext($domainname, $msgid);
        $crate::macros::freplace(&format, &[$($args),*])
    }};

    (domain = $domainname:expr, $msgid:expr) => {
        $crate::dgettext($domainname, $msgid)
    };

    ($msgid:expr, $msgid_plural:expr, $n:expr, [$($args:expr),+]) => {{
        let format = $crate::ngettext($msgid, $msgid_plural, $n);
        $crate::macros::freplace(&format, &[$($args),*])
    }};

    ($msgid:expr, $msgid_plural:expr, $n:expr) => {
        $crate::ngettext($msgid, $msgid_plural, $n)
    };

    ($msgid:expr, [$($args:expr),+]) => {{
        let format = $crate::gettext($msgid);
        $crate::macros::freplace(&format, &[$($args),*])
    }};

    ($msgid:expr) => {
        $crate::gettext($msgid)
    };
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
        $crate::macros::freplace(&format, &[$($args),*])
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
        $crate::macros::freplace(&format, &[$($args),*])
    }}
}

#[cfg(test)]
mod test {
    use crate::*;
    use super::freplace;

    #[test]
    fn freplace_works_correctly() {
        assert_eq!(
            freplace("No params", &[]),
            "No params"
        );
        assert_eq!(
            freplace("Middle {} param", &["working"]),
            "Middle working param"
        );
        assert_eq!(
            freplace("{} working param", &["Beginning"]),
            "Beginning working param"
        );
        assert_eq!(
            freplace("Working param {}", &["at the end"]),
            "Working param at the end"
        );
        assert_eq!(
            freplace("{} param, {} param, and param {}", &["First", "second", "at the end"]),
            "First param, second param, and param at the end"
        );
    }

    #[test]
    #[should_panic(expected = "Argument count has to match number of format directives")]
    fn freplace_panics_too_many_args() {
        freplace("No params", &["But there is one"]);
    }

    #[test]
    #[should_panic(expected = "Argument count has to match number of format directives")]
    fn freplace_panics_no_args() {
        freplace("{} params", &[]);
    }

    #[test]
    fn gettext_singular() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");

        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();
        textdomain("hellorust").unwrap();

        assert_eq!(
            gettext!("Hello, world!"),
            "Hello, world!"
        );

        assert_eq!(
            gettext!("Hello, {}!", ["world"]),
            "Hello, world!"
        );
    }

    #[test]
    fn gettext_plural() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");

        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();
        textdomain("hellorust").unwrap();

        assert_eq!(
            gettext!("Singular!", "Multiple!", 1),
            "Singular!"
        );
        assert_eq!(
            gettext!("Singular!", "Multiple!", 2),
            "Multiple!"
        );

        assert_eq!(
            gettext!("Singular {}!", "Multiple {}!", 1, ["Worlds"]),
            "Singular Worlds!"
        );
        assert_eq!(
            gettext!("Singular {}!", "Multiple {}!", 2, ["Worlds"]),
            "Multiple Worlds!"
        );
    }

    #[test]
    fn gettext_domain_singular() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");

        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();

        assert_eq!(
            gettext!(domain = "hellorust", "Hello, world!"),
            "Hello, world!"
        );

        assert_eq!(
            gettext!(domain = "hellorust", "Hello, {}!", ["world"]),
            "Hello, world!"
        );
    }

    #[test]
    fn gettext_domain_plural() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");

        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();

        assert_eq!(
            gettext!(domain = "hellrust",
                "Singular!", "Multiple!", 1
            ),
            "Singular!"
        );
        assert_eq!(
            gettext!(domain = "hellrust",
                "Singular!", "Multiple!", 2
            ),
            "Multiple!"
        );

        assert_eq!(
            gettext!(domain = "hellrust",
                "Singular {}!", "Multiple {}!", 1, ["World"]
            ),
            "Singular World!"
        );
        assert_eq!(
            gettext!(domain = "hellrust",
                "Singular {}!", "Multiple {}!", 2, ["World"]
            ),
            "Multiple World!"
        );
    }

    #[test]
    fn gettext_domain_category_singular() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");

        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();

        assert_eq!(
            gettext!(domain = "hellorust", category = LocaleCategory::LcAll,
                "Hello, world!"
            ),
            "Hello, world!"
        );

        assert_eq!(
            gettext!(domain = "hellorust", category = LocaleCategory::LcAll,
                "Hello, {}!", ["world"]
            ),
            "Hello, world!"
        );
    }

    #[test]
    fn gettext_domain_category_plural() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");

        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();

        assert_eq!(
            gettext!(domain = "hellorust", category = LocaleCategory::LcAll,
                "Singular!", "Multiple!", 1
            ),
            "Singular!",
        );
        assert_eq!(
            gettext!(domain = "hellorust", category = LocaleCategory::LcAll,
                "Singular!", "Multiple!", 2
            ),
            "Multiple!",
        );

        assert_eq!(
            gettext!(domain = "hellorust", category = LocaleCategory::LcAll,
                "Singular {}!", "Multiple {}!", 1, ["World"]
            ),
            "Singular World!",
        );
        assert_eq!(
            gettext!(domain = "hellorust", category = LocaleCategory::LcAll,
                "Singular {}!", "Multiple {}!", 2, ["World"]
            ),
            "Multiple World!",
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

}
