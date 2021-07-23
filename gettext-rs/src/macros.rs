//! Macros that translate the message and then replace placeholders in it.

/// This is an implementation detail for replacing arguments in the gettext macros.
/// Don't call this directly.
#[doc(hidden)]
#[allow(dead_code)]
pub fn rt_format<T: std::fmt::Display>(msgstr: &str, pat: &str, arg: T) -> String {
    match msgstr.split_once(pat) {
        Some((pre, suf)) => format!("{}{}{}", pre, arg, suf),
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
