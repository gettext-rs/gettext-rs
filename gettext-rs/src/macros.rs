//! Macros that translate the message and then replace placeholders in it.

/// This is an implementation detail for replacing arguments in the gettext macros.
/// Don't call this directly.
#[doc(hidden)]
#[allow(dead_code)]
pub fn rt_format<T: std::fmt::Display>(msgstr: &str, pat: &str, arg: T) -> String {
    match msgstr.split_once(pat) {
        Some((pre, suf)) => { format!("{}{}{}", pre, arg, suf) }
        None => {
            debug_assert!(false, "There are more arguments than format directives");
            msgstr.to_string()
        }
    }
}

/// This is an implementation detail for replacing arguments in the gettext macros.
/// Don't call this directly.
#[macro_export]
#[doc(hidden)]
macro_rules! rt_format {
	( $msgstr:expr, $pat:expr, ) => {{
        debug_assert!(
            !$msgstr.contains("{}"),
            "There are fewer arguments than format directives"
        );
		$msgstr
	}};
	( $msgstr:expr, $pat:expr, $arg:expr $(, $rest: expr)* ) => {{
		$crate::rt_format!(
            $crate::macros::rt_format(&$msgstr, $pat, $arg),
            $pat,
            $($rest),*
        )
	}};
}

/// Like [`gettext`], but allows for formatting.
///
/// It calls [`gettext`] on `msgid`, and then replaces each occurrence of `{}` with the next value
/// out of `args`.
///
/// # Panics
///
/// If compiled with debug assertions enabled (as in "dev" profile),
/// will panic if the number of arguments doesn't match the number of format directives.
///
/// [`gettext`]: fn.gettext.html
#[macro_export]
macro_rules! gettext {
    ( $msgid:expr $(,)? ) => {
        $crate::gettext($msgid)
    };
    ( $msgid:expr, $($args:expr),+ $(,)? ) => {{
        $crate::rt_format!(
            $crate::gettext($msgid),
            "{}",
            $($args),+
        )
    }};
}

/// Like [`dgettext`], but allows for formatting.
///
/// It calls [`dgettext`] on `domainname` and `msgid`, and then replaces each occurrence of `{}`
/// with the next value out of `args`.
///
/// # Panics
///
/// If compiled with debug assertions enabled (as in "dev" profile),
/// will panic if the number of arguments doesn't match the number of format directives.
///
/// [`dgettext`]: fn.dgettext.html
#[macro_export]
macro_rules! dgettext {
    ( $domainname:expr, $msgid:expr $(,)? ) => {
        $crate::dgettext($domainname, $msgid)
    };
    ( $domainname:expr, $msgid:expr, $($args:expr),+ $(,)? ) => {{
        $crate::rt_format!(
            $crate::dgettext($domainname, $msgid),
            "{}",
            $($args),+
        )
    }};
}

/// Like [`dcgettext`], but allows for formatting.
///
/// It calls [`dcgettext`] on `domainname`, `category`, and `msgid`, and then replaces each
/// occurrence of `{}` with the next value out of `args`.
///
/// # Panics
///
/// If compiled with debug assertions enabled (as in "dev" profile),
/// will panic if the number of arguments doesn't match the number of format directives.
///
/// [`dcgettext`]: fn.dcgettext.html
#[macro_export]
macro_rules! dcgettext {
    ( $domainname:expr, $category:expr, $msgid:expr $(,)? ) => {
        $crate::dcgettext($domainname, $msgid, $category)
    };
    ( $domainname:expr, $category:expr, $msgid:expr, $($args:expr),+ $(,)? ) => {{
        $crate::rt_format!(
            $crate::dcgettext($domainname, $msgid, $category),
            "{}",
            $($args),+
        )
    }};
}

/// Like [`ngettext`], but allows for formatting.
///
/// It calls [`ngettext`] on `msgid`, `msgid_plural`, and `n`, and then replaces each occurrence of
/// `{}` with the next value out of `args`, and `{n}` with `n`.
///
/// # Panics
///
/// If compiled with debug assertions enabled (as in "dev" profile),
/// will panic if the number of arguments doesn't match the number of format directives.
///
/// [`ngettext`]: fn.ngettext.html
#[macro_export]
macro_rules! ngettext {
    ( $msgid:expr, $msgid_plural:expr, $n:expr $(,)? ) => {{
        let mut msgstr = $crate::ngettext($msgid, $msgid_plural, $n);
        while msgstr.contains("{n}") {
            msgstr = $crate::macros::rt_format(&msgstr, "{n}", $n);
        }
        msgstr
    }};
    ( $msgid:expr, $msgid_plural:expr, $n:expr, $($args:expr),+ $(,)? ) => {{
        let mut msgstr = $crate::ngettext($msgid, $msgid_plural, $n);
        while msgstr.contains("{n}") {
            msgstr = $crate::macros::rt_format(&msgstr, "{n}", $n);
        }
        $crate::rt_format!(msgstr, "{}", $($args),+)
    }};
}

/// Like [`dngettext`], but allows for formatting.
///
/// It calls [`dngettext`] on `domainname`, `msgid`, `msgid_plural`, and `n`, and then replaces
/// each occurrence of `{}` with the next value out of `args`, and `{n}` with `n`.
///
/// # Panics
///
/// If compiled with debug assertions enabled (as in "dev" profile),
/// will panic if the number of arguments doesn't match the number of format directives.
///
/// [`dngettext`]: fn.dngettext.html
#[macro_export]
macro_rules! dngettext {
    ( $domainname:expr, $msgid:expr, $msgid_plural:expr, $n:expr $(,)? ) => {{
        let mut msgstr = $crate::dngettext($domainname, $msgid, $msgid_plural, $n);
        while msgstr.contains("{n}") {
            msgstr = $crate::macros::rt_format(&msgstr, "{n}", $n);
        }
        msgstr
    }};
    ( $domainname:expr, $msgid:expr, $msgid_plural:expr, $n:expr, $($args:expr),+ $(,)? ) => {{
        let mut msgstr = $crate::dngettext($domainname, $msgid, $msgid_plural, $n);
        while msgstr.contains("{n}") {
            msgstr = $crate::macros::rt_format(&msgstr, "{n}", $n);
        }
        $crate::rt_format!(msgstr, "{}", $($args),+)
    }};
}

/// Like [`dcngettext`], but allows for formatting.
///
/// It calls [`dcngettext`] on `domainname`, `category`, `msgid`, `msgid_plural`, and `n`, and then
/// replaces each occurrence of `{}` with the next value out of `args`, and `{n}` with `n`.
///
/// # Panics
///
/// If compiled with debug assertions enabled (as in "dev" profile),
/// will panic if the number of arguments doesn't match the number of format directives.
///
/// [`dcngettext`]: fn.dcngettext.html
#[macro_export]
macro_rules! dcngettext {
    ( $domainname:expr, $category:expr, $msgid:expr, $msgid_plural:expr, $n:expr $(,)? ) => {{
        let mut msgstr = $crate::dcngettext($domainname, $msgid, $msgid_plural, $n, $category);
        while msgstr.contains("{n}") {
            msgstr = $crate::macros::rt_format(&msgstr, "{n}", $n);
        }
        msgstr
    }};
    ( $domainname:expr, $category:expr,
      $msgid:expr, $msgid_plural:expr, $n:expr, $($args:expr),+ $(,)? ) => {{
        let mut msgstr = $crate::dcngettext($domainname, $msgid, $msgid_plural, $n, $category);
        while msgstr.contains("{n}") {
            msgstr = $crate::macros::rt_format(&msgstr, "{n}", $n);
        }
        $crate::rt_format!(msgstr, "{}", $($args),+)
    }};
}

/// Like [`pgettext`], but allows for formatting.
///
/// It calls [`pgettext`] on `msgctxt` and `msgid`, and then replaces each occurrence of `{}` with
/// the next value out of `args`.
///
/// # Panics
///
/// If compiled with debug assertions enabled (as in "dev" profile),
/// will panic if the number of arguments doesn't match the number of format directives.
///
/// [`pgettext`]: fn.pgettext.html
#[macro_export]
macro_rules! pgettext {
    ( $msgctxt:expr, $msgid:expr $(,)? ) => {
        $crate::pgettext($msgctxt, $msgid)
    };
    ( $msgctxt:expr, $msgid:expr, $($args:expr),+ $(,)? ) => {{
        $crate::rt_format!(
            $crate::pgettext($msgctxt, $msgid),
            "{}",
            $($args),+
        )
    }};
}

/// Like [`npgettext`], but allows for formatting.
///
/// It calls [`npgettext`] on `msgctxt`, `msgid`, `msgid_plural`, and `n`, and then replaces each
/// occurrence of `{}` with the next value out of `args`, and `{n}` with `n`.
///
/// # Panics
///
/// If compiled with debug assertions enabled (as in "dev" profile),
/// will panic if the number of arguments doesn't match the number of format directives.
///
/// [`npgettext`]: fn.npgettext.html
#[macro_export]
macro_rules! npgettext {
    ( $msgctxt:expr, $msgid:expr, $msgid_plural:expr, $n:expr $(,)? ) => {{
        let mut msgstr = $crate::npgettext($msgctxt, $msgid, $msgid_plural, $n);
        while msgstr.contains("{n}") {
            msgstr = $crate::macros::rt_format(&msgstr, "{n}", $n);
        }
        msgstr
    }};
    ( $msgctxt:expr, $msgid:expr, $msgid_plural:expr, $n:expr, $($args:expr),+ $(,)? ) => {{
        let mut msgstr = $crate::npgettext($msgctxt, $msgid, $msgid_plural, $n);
        while msgstr.contains("{n}") {
            msgstr = $crate::macros::rt_format(&msgstr, "{n}", $n);
        }
        $crate::rt_format!(msgstr, "{}", $($args),+)
    }};
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn gettext_no_formatting() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();
        textdomain("hellorust").unwrap();

        assert_eq!(
            gettext!("Hello, World!"),
            "Hello, World!"
        );
    }

    #[test]
    fn dgettext_no_formatting() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();

        assert_eq!(
            dgettext!("hellorust",
                "Hello, World!"),
            "Hello, World!"
        );
    }

    #[test]
    fn dcgettext_no_formatting() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();

        assert_eq!(
            dcgettext!("hellorust", LocaleCategory::LcAll,
                "Hello, World!"),
            "Hello, World!"
        );
    }

    #[test]
    fn ngettext_no_formatting() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();
        textdomain("hellorust").unwrap();

        assert_eq!(
            ngettext!(
                "There is one result",
                "There are few results",
                2
            ),
            "There are few results"
        );
    }

    #[test]
    fn dngettext_no_formatting() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();

        assert_eq!(
            dngettext!("hellorust",
                "There is one result",
                "There are few results",
                2
            ),
            "There are few results"
        );
    }

    #[test]
    fn dcngettext_no_formatting() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();

        assert_eq!(
            dcngettext!("hellorust", LocaleCategory::LcAll,
                "There is one result",
                "There are few results",
                2
            ),
            "There are few results"
        );
    }

    #[test]
    fn pgettext_no_formatting() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();
        textdomain("hellorust").unwrap();

        assert_eq!(
            pgettext!("context",
                "Hello, World!"),
            "Hello, World!"
        );
    }

    #[test]
    fn npgettext_no_formatting() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();
        textdomain("hellorust").unwrap();

        assert_eq!(
            npgettext!("context",
                "There is one result",
                "There are few results",
                2
            ),
            "There are few results"
        );
    }


    #[test]
    fn gettext_basic_formatting() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();
        textdomain("hellorust").unwrap();

        assert_eq!(
            gettext!("Hello, {}! {}", "World", "UwU"),
            "Hello, World! UwU"
        );
    }

    #[test]
    fn dgettext_basic_formatting() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();

        assert_eq!(
            dgettext!("hellorust",
                "Hello, {}! {}", "World", "UwU"),
            "Hello, World! UwU"
        );
    }

    #[test]
    fn dcgettext_basic_formatting() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();

        assert_eq!(
            dcgettext!("hellorust", LocaleCategory::LcAll,
                "Hello, {}! {}", "World", "UwU"),
            "Hello, World! UwU"
        );
    }

    #[test]
    fn ngettext_basic_formatting() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();
        textdomain("hellorust").unwrap();

        assert_eq!(
            ngettext!(
                "There is one \"{}\" in text! {}",
                "There are few \"{}\" in text! {}",
                2, "UwU", "OwO"
            ),
            "There are few \"UwU\" in text! OwO"
        );
    }

    #[test]
    fn dngettext_basic_formatting() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();

        assert_eq!(
            dngettext!("hellorust",
                "There is one \"{}\" in text! {}",
                "There are few \"{}\" in text! {}",
                2, "UwU", "OwO"
            ),
            "There are few \"UwU\" in text! OwO"
        );
    }

    #[test]
    fn dcngettext_basic_formatting() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();

        assert_eq!(
            dcngettext!("hellorust", LocaleCategory::LcAll,
                "There is one \"{}\" in text! {}",
                "There are few \"{}\" in text! {}",
                2, "UwU", "OwO"
            ),
            "There are few \"UwU\" in text! OwO"
        );
    }

    #[test]
    fn pgettext_basic_formatting() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();
        textdomain("hellorust").unwrap();

        assert_eq!(
            pgettext!("context",
                "Hello, {}! {}", "World", "UwU"),
            "Hello, World! UwU"
        );
    }

    #[test]
    fn npgettext_basic_formatting() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();
        textdomain("hellorust").unwrap();

        assert_eq!(
            npgettext!("context",
                "There is one \"{}\" in text! {}",
                "There are few \"{}\" in text! {}",
                2, "UwU", "OwO"
            ),
            "There are few \"UwU\" in text! OwO"
        );
    }


    #[test]
    #[cfg_attr(debug_assertions, should_panic="There are fewer arguments than format directives")]
    fn gettext_fewer_arguments_than_parameters() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();
        textdomain("hellorust").unwrap();

        assert_eq!(
            gettext!("Hello, {}! {}", "World"),
            "Hello, World! {}"
        );
    }

    #[test]
    #[cfg_attr(debug_assertions, should_panic="There are fewer arguments than format directives")]
    fn dgettext_fewer_arguments_than_parameters() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();

        assert_eq!(
            dgettext!("hellorust",
                "Hello, {}! {}", "World"),
            "Hello, World! {}"
        );
    }

    #[test]
    #[cfg_attr(debug_assertions, should_panic="There are fewer arguments than format directives")]
    fn dcgettext_fewer_arguments_than_parameters() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();

        assert_eq!(
            dcgettext!("hellorust", LocaleCategory::LcAll,
                "Hello, {}! {}", "World"),
            "Hello, World! {}"
        );
    }

    #[test]
    #[cfg_attr(debug_assertions, should_panic="There are fewer arguments than format directives")]
    fn ngettext_fewer_arguments_than_parameters() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();
        textdomain("hellorust").unwrap();

        assert_eq!(
            ngettext!(
                "There is one \"{}\" in text! {}",
                "There are few \"{}\" in text! {}",
                2, "UwU"
            ),
            "There are few \"UwU\" in text! {}"
        );
    }

    #[test]
    #[cfg_attr(debug_assertions, should_panic="There are fewer arguments than format directives")]
    fn dngettext_fewer_arguments_than_parameters() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();

        assert_eq!(
            dngettext!("hellorust",
                "There is one \"{}\" in text! {}",
                "There are few \"{}\" in text! {}",
                2, "UwU"
            ),
            "There are few \"UwU\" in text! {}"
        );
    }

    #[test]
    #[cfg_attr(debug_assertions, should_panic="There are fewer arguments than format directives")]
    fn dcngettext_fewer_arguments_than_parameters() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();

        assert_eq!(
            dcngettext!("hellorust", LocaleCategory::LcAll,
                "There is one \"{}\" in text! {}",
                "There are few \"{}\" in text! {}",
                2, "UwU"
            ),
            "There are few \"UwU\" in text! {}"
        );
    }

    #[test]
    #[cfg_attr(debug_assertions, should_panic="There are fewer arguments than format directives")]
    fn pgettext_fewer_arguments_than_parameters() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();
        textdomain("hellorust").unwrap();

        assert_eq!(
            pgettext!("context",
                "Hello, {}! {}", "World"),
            "Hello, World! {}"
        );
    }

    #[test]
    #[cfg_attr(debug_assertions, should_panic="There are fewer arguments than format directives")]
    fn npgettext_fewer_arguments_than_parameters() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();
        textdomain("hellorust").unwrap();

        assert_eq!(
            npgettext!("context",
                "There is one \"{}\" in text! {}",
                "There are few \"{}\" in text! {}",
                2, "UwU"
            ),
            "There are few \"UwU\" in text! {}"
        );
    }


    #[test]
    #[cfg_attr(debug_assertions, should_panic="There are more arguments than format directives")]
    fn gettext_more_arguments_than_parameters() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();
        textdomain("hellorust").unwrap();

        assert_eq!(
            gettext!("Hello, {}!", "World", "UwU"),
            "Hello, World!"
        );
    }

    #[test]
    #[cfg_attr(debug_assertions, should_panic="There are more arguments than format directives")]
    fn dgettext_more_arguments_than_parameters() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();

        assert_eq!(
            dgettext!("hellorust",
                "Hello, {}!", "World", "UwU"),
            "Hello, World!"
        );
    }

    #[test]
    #[cfg_attr(debug_assertions, should_panic="There are more arguments than format directives")]
    fn dcgettext_more_arguments_than_parameters() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();

        assert_eq!(
            dcgettext!("hellorust", LocaleCategory::LcAll,
                "Hello, {}!", "World", "UwU"),
            "Hello, World!"
        );
    }

    #[test]
    #[cfg_attr(debug_assertions, should_panic="There are more arguments than format directives")]
    fn ngettext_more_arguments_than_parameters() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();
        textdomain("hellorust").unwrap();

        assert_eq!(
            ngettext!(
                "There is one \"{}\" in text!",
                "There are few \"{}\" in text!",
                2, "UwU", "OwO"
            ),
            "There are few \"UwU\" in text!"
        );
    }

    #[test]
    #[cfg_attr(debug_assertions, should_panic="There are more arguments than format directives")]
    fn dngettext_more_arguments_than_parameters() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();

        assert_eq!(
            dngettext!("hellorust",
                "There is one \"{}\" in text!",
                "There are few \"{}\" in text!",
                2, "UwU", "OwO"
            ),
            "There are few \"UwU\" in text!"
        );
    }

    #[test]
    #[cfg_attr(debug_assertions, should_panic="There are more arguments than format directives")]
    fn dcngettext_more_arguments_than_parameters() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();

        assert_eq!(
            dcngettext!("hellorust", LocaleCategory::LcAll,
                "There is one \"{}\" in text!",
                "There are few \"{}\" in text!",
                2, "UwU", "OwO"
            ),
            "There are few \"UwU\" in text!"
        );
    }

    #[test]
    #[cfg_attr(debug_assertions, should_panic="There are more arguments than format directives")]
    fn pgettext_more_arguments_than_parameters() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();
        textdomain("hellorust").unwrap();

        assert_eq!(
            pgettext!("context",
                "Hello, {}!", "World", "UwU"),
            "Hello, World!"
        );
    }

    #[test]
    #[cfg_attr(debug_assertions, should_panic="There are more arguments than format directives")]
    fn npgettext_more_arguments_than_parameters() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();
        textdomain("hellorust").unwrap();

        assert_eq!(
            npgettext!("context",
                "There is one \"{}\" in text!",
                "There are few \"{}\" in text!",
                2, "UwU", "OwO"
            ),
            "There are few \"UwU\" in text!"
        );
    }


    #[test]
    fn ngettext_special_n_formatting() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();
        textdomain("hellorust").unwrap();

        assert_eq!(
            ngettext!(
                "There is {n} apple! Only {n}!",
                "There are {n} apples! Only {n}!",
                2
            ),
            "There are 2 apples! Only 2!"
        );
        assert_eq!(
            ngettext!(
                "There is one apple! Only one!",
                "There are {n} apples! Only {n}!",
                1
            ),
            "There is one apple! Only one!"
        );
        assert_eq!(
            ngettext!(
                "There is one apple! Only one!",
                "There are {n} apples! Only {n}!",
                2
            ),
            "There are 2 apples! Only 2!"
        );
        assert_eq!(
            ngettext!(
                "There is {n} \"{}\" in text! Only {n}!",
                "There are {n} \"{}\" in text! Only {n}!",
                2, "UwU"
            ),
            "There are 2 \"UwU\" in text! Only 2!"
        );
    }

    #[test]
    fn dngettext_special_n_formatting() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();

        assert_eq!(
            dngettext!("hellorust",
                "There is {n} apple! Only {n}!",
                "There are {n} apples! Only {n}!",
                2
            ),
            "There are 2 apples! Only 2!"
        );
        assert_eq!(
            dngettext!("hellorust",
                "There is one apple! Only one!",
                "There are {n} apples! Only {n}!",
                1
            ),
            "There is one apple! Only one!"
        );
        assert_eq!(
            dngettext!("hellorust",
                "There is one apple! Only one!",
                "There are {n} apples! Only {n}!",
                2
            ),
            "There are 2 apples! Only 2!"
        );
        assert_eq!(
            dngettext!("hellorust",
                "There is {n} \"{}\" in text! Only {n}!",
                "There are {n} \"{}\" in text! Only {n}!",
                2, "UwU"
            ),
            "There are 2 \"UwU\" in text! Only 2!"
        );
    }

    #[test]
    fn dcngettext_special_n_formatting() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();

        assert_eq!(
            dcngettext!("hellorust", LocaleCategory::LcAll,
                "There is {n} apple! Only {n}!",
                "There are {n} apples! Only {n}!",
                2
            ),
            "There are 2 apples! Only 2!"
        );
        assert_eq!(
            dcngettext!("hellorust", LocaleCategory::LcAll,
                "There is one apple! Only one!",
                "There are {n} apples! Only {n}!",
                1
            ),
            "There is one apple! Only one!"
        );
        assert_eq!(
            dcngettext!("hellorust", LocaleCategory::LcAll,
                "There is one apple! Only one!",
                "There are {n} apples! Only {n}!",
                2
            ),
            "There are 2 apples! Only 2!"
        );
        assert_eq!(
            dcngettext!("hellorust", LocaleCategory::LcAll,
                "There is {n} \"{}\" in text! Only {n}!",
                "There are {n} \"{}\" in text! Only {n}!",
                2, "UwU"
            ),
            "There are 2 \"UwU\" in text! Only 2!"
        );
    }

    #[test]
    fn npgettext_special_n_formatting() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();
        textdomain("hellorust").unwrap();

        assert_eq!(
            npgettext!("context",
                "There is {n} apple! Only {n}!",
                "There are {n} apples! Only {n}!",
                2
            ),
            "There are 2 apples! Only 2!"
        );
        assert_eq!(
            npgettext!("context",
                "There is one apple! Only one!",
                "There are {n} apples! Only {n}!",
                1
            ),
            "There is one apple! Only one!"
        );
        assert_eq!(
            npgettext!("context",
                "There is one apple! Only one!",
                "There are {n} apples! Only {n}!",
                2
            ),
            "There are 2 apples! Only 2!"
        );
        assert_eq!(
            npgettext!("context",
                "There is {n} \"{}\" in text! Only {n}!",
                "There are {n} \"{}\" in text! Only {n}!",
                2, "UwU"
            ),
            "There are 2 \"UwU\" in text! Only 2!"
        );
    }


    #[test]
    fn gettext_trailing_comma() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();
        textdomain("hellorust").unwrap();

        assert_eq!(
            gettext!("Hello, World!",),
            "Hello, World!"
        );
        assert_eq!(
            gettext!("Hello, {}! {}", "World", "UwU",),
            "Hello, World! UwU"
        );
    }

    #[test]
    fn dgettext_trailing_comma() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();

        assert_eq!(
            dgettext!("hellorust",
                "Hello, World!",),
            "Hello, World!"
        );
        assert_eq!(
            dgettext!("hellorust",
                "Hello, {}! {}", "World", "UwU",),
            "Hello, World! UwU"
        );
    }

    #[test]
    fn dcgettext_trailing_comma() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();

        assert_eq!(
            dcgettext!("hellorust", LocaleCategory::LcAll,
                "Hello, World!"),
            "Hello, World!"
        );
        assert_eq!(
            dcgettext!("hellorust", LocaleCategory::LcAll,
                "Hello, {}! {}", "World", "UwU",),
            "Hello, World! UwU"
        );
    }

    #[test]
    fn ngettext_trailing_comma() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();
        textdomain("hellorust").unwrap();

        assert_eq!(
            ngettext!(
                "There is one result",
                "There are few results",
                2,
            ),
            "There are few results"
        );
        assert_eq!(
            ngettext!(
                "There is one \"{}\" in text! {}",
                "There are few \"{}\" in text! {}",
                2, "UwU", "OwO",
            ),
            "There are few \"UwU\" in text! OwO"
        );
    }

    #[test]
    fn dngettext_trailing_comma() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();

        assert_eq!(
            dngettext!("hellorust",
                "There is one result",
                "There are few results",
                2,
            ),
            "There are few results"
        );
        assert_eq!(
            dngettext!("hellorust",
                "There is one \"{}\" in text! {}",
                "There are few \"{}\" in text! {}",
                2, "UwU", "OwO",
            ),
            "There are few \"UwU\" in text! OwO"
        );
    }

    #[test]
    fn dcngettext_trailing_comma() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();

        assert_eq!(
            dcngettext!("hellorust", LocaleCategory::LcAll,
                "There is one result",
                "There are few results",
                2,
            ),
            "There are few results"
        );
        assert_eq!(
            dcngettext!("hellorust", LocaleCategory::LcAll,
                "There is one \"{}\" in text! {}",
                "There are few \"{}\" in text! {}",
                2, "UwU", "OwO",
            ),
            "There are few \"UwU\" in text! OwO"
        );
    }

    #[test]
    fn pgettext_trailing_comma() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();
        textdomain("hellorust").unwrap();

        assert_eq!(
            pgettext!("context",
                "Hello, World!",),
            "Hello, World!"
        );
        assert_eq!(
            pgettext!("context",
                "Hello, {}! {}", "World", "UwU",),
            "Hello, World! UwU"
        );
    }

    #[test]
    fn npgettext_trailing_comma() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();
        textdomain("hellorust").unwrap();

        assert_eq!(
            npgettext!("context",
                "There is one result",
                "There are few results",
                2,
            ),
            "There are few results"
        );
        assert_eq!(
            npgettext!("context",
                "There is one \"{}\" in text! {}",
                "There are few \"{}\" in text! {}",
                2, "UwU", "OwO",
            ),
            "There are few \"UwU\" in text! OwO"
        );
    }
}
