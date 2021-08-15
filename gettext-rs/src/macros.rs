//! Macros that translate the message and then replace placeholders in it.

/// This is an implementation detail for replacing arguments in the gettext macros.
/// Don't call this directly.
#[doc(hidden)]
#[allow(dead_code)]
pub fn rt_format(
    msgstr: String,
    args: Vec<&dyn std::string::ToString>,
    n_arg: Option<&dyn std::string::ToString>,
) -> String {
    let mut args = args.iter().peekable();
    let mut formatted = String::new();
    let mut chars = msgstr.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            ch @ ('{' | '}') => {
                if let Some(ch) = chars.next_if_eq(&ch) {
                    formatted.push(ch);
                } else {
                    match ch {
                        '{' => {
                            if let Some(_) = chars.next_if_eq(&'}') {
                                match args.next() {
                                    Some(arg) => formatted.push_str(&arg.to_string()),
                                    None => {
                                        debug_assert!(
                                            false,
                                            "There are fewer arguments than format directives"
                                        );
                                        formatted.push_str("{}");
                                    }
                                }
                            } else if let Some(_) = chars.next_if_eq(&'n') {
                                if let Some(_) = chars.next_if_eq(&'}') {
                                    match n_arg {
                                        Some(arg) => formatted.push_str(&arg.to_string()),
                                        None => {
                                            debug_assert!(
                                                false,
                                                "{}",
                                                "Using '{n}' format directive in non-plural form"
                                            );
                                            formatted.push_str("{n}");
                                        }
                                    }
                                }
                            }
                        }
                        '}' => {}
                        _ => unreachable!(),
                    }
                }
            }
            _ => formatted.push(ch),
        }
    }

    #[cfg(debug_assertions)]
    if let Some(_) = args.peek() {
        debug_assert!(false, "There are more arguments than format directives")
    }

    formatted
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
    ( $msgid:expr $(, $args:expr)* $(,)? ) => {{
        $crate::macros::rt_format(
            $crate::gettext($msgid),
            vec![$(&$args),*],
            Option::None
        )
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
    ( $msgctxt:expr, $msgid:expr $(, $args:expr)* $(,)? ) => {{
        $crate::macros::rt_format(
            $crate::pgettext($msgctxt, $msgid),
            vec![$(&$args),*],
            Option::None
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
    ( $domainname:expr, $msgid:expr $(, $args:expr)* $(,)? ) => {{
        $crate::macros::rt_format(
            $crate::dgettext($domainname, $msgid),
            vec![$(&$args),*],
            Option::None
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
    ( $domainname:expr, $category:expr, $msgid:expr $(, $args:expr)* $(,)? ) => {{
        $crate::macros::rt_format(
            $crate::dcgettext($domainname, $msgid, $category),
            vec![$(&$args),*],
            Option::None
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
    ( $msgid:expr, $msgid_plural:expr, $n:expr $(, $args:expr)* $(,)? ) => {{
        $crate::macros::rt_format(
            $crate::ngettext($msgid, $msgid_plural, $n),
            vec![$(&$args),*],
            Option::Some(&$n)
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
    ( $msgctxt:expr, $msgid:expr, $msgid_plural:expr, $n:expr $(, $args:expr)* $(,)? ) => {{
        $crate::macros::rt_format(
            $crate::npgettext($msgctxt, $msgid, $msgid_plural, $n),
            vec![$(&$args),*],
            Option::Some(&$n)
        )
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
    ( $domainname:expr, $msgid:expr, $msgid_plural:expr, $n:expr $(, $args:expr)* $(,)? ) => {{
        $crate::macros::rt_format(
            $crate::dngettext($domainname, $msgid, $msgid_plural, $n),
            vec![$(&$args),*],
            Option::Some(&$n)
        )
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
    ( $domainname:expr, $category:expr,
      $msgid:expr, $msgid_plural:expr, $n:expr $(, $args:expr)* $(,)? ) => {{
        $crate::macros::rt_format(
            $crate::dcngettext($domainname, $msgid, $msgid_plural, $n, $category),
            vec![$(&$args),*],
            Option::Some(&$n)
        )
    }};
}

#[cfg(test)]
mod tests {
    use macros::rt_format;

    #[test]
    fn partial_escaping() {
        assert_eq!(
            rt_format(
                String::from("{{}, {}}, {{n}, {n}}"),
                vec![&"smth"],
                Option::Some(&5)
            ),
            "{, smth, {n, 5"
        );
    }
}
