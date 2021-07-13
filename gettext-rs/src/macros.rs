//! Macros that translate the message and then replace placeholders in it.

/// This is an implementation detail for replacing arguments in the gettext macros.
/// Don't call this directly.
#[doc(hidden)]
#[allow(dead_code)]
pub fn rt_format<T: std::fmt::Display>(msgstr: &str, arg: T, pat: &str) -> String {
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
	( $msgstr:expr, $pat:expr, ) => {
		$msgstr
	};
	( $msgstr:expr, $pat:expr, $arg:expr $(, $rest: expr)* ) => {{
		$crate::rt_format!(
            $crate::macros::rt_format(&$msgstr, $arg, $pat),
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
/// When building the `dev` profile,
/// it panics if the number of arguments doesn't match the number of format directives.
///
/// [`gettext`]: fn.gettext.html
#[macro_export]
macro_rules! gettext {
    ( $msgid:expr $(,)? ) => {
        $crate::gettext($msgid)
    };
    ( $msgid:expr, $($args:expr),+ $(,)? ) => {{
        let mut msgstr = $crate::gettext($msgid);
        msgstr = $crate::rt_format!(msgstr, "{}", $($args),+);
        debug_assert!(
            !msgstr.contains("{}"),
            "There are less arguments than format directives"
        );

        msgstr
    }};
}

/// Like [`dgettext`], but allows for formatting.
///
/// It calls [`dgettext`] on `domainname` and `msgid`, and then replaces each occurrence of `{}`
/// with the next value out of `args`.
///
/// # Panics
///
/// When building the `dev` profile,
/// it panics if the number of arguments doesn't match the number of format directives.
///
/// [`dgettext`]: fn.dgettext.html
#[macro_export]
macro_rules! dgettext {
    ( $domainname:expr, $msgid:expr $(,)? ) => {
        $crate::dgettext($domainname, $msgid)
    };
    ( $domainname:expr, $msgid:expr, $($args:expr),+ $(,)? ) => {{
        let mut msgstr = $crate::dgettext($domainname, $msgid);
        msgstr = $crate::rt_format!(msgstr, "{}", $($args),+);
        debug_assert!(
            !msgstr.contains("{}"),
            "There are less arguments than format directives"
        );

        msgstr
    }};
}

/// Like [`dcgettext`], but allows for formatting.
///
/// It calls [`dcgettext`] on `domainname`, `category`, and `msgid`, and then replaces each
/// occurrence of `{}` with the next value out of `args`.
///
/// # Panics
///
/// When building the `dev` profile,
/// it panics if the number of arguments doesn't match the number of format directives.
///
/// [`dcgettext`]: fn.dcgettext.html
#[macro_export]
macro_rules! dcgettext {
    ( $domainname:expr, $category:expr, $msgid:expr $(,)? ) => {
        $crate::dcgettext($domainname, $msgid, $category)
    };
    ( $domainname:expr, $category:expr, $msgid:expr, $($args:expr),+ $(,)? ) => {{
        let mut msgstr = $crate::dcgettext($domainname, $msgid, $category);
        msgstr = $crate::rt_format!(msgstr, "{}", $($args),+);
        debug_assert!(
            !msgstr.contains("{}"),
            "There are less arguments than format directives"
        );

        msgstr
    }};
}

/// Like [`ngettext`], but allows for formatting.
///
/// It calls [`ngettext`] on `msgid`, `msgid_plural`, and `n`, and then replaces each occurrence of
/// `{}` with the next value out of `args`.
///
/// # Panics
///
/// When building the `dev` profile,
/// it panics if the number of arguments doesn't match the number of format directives.
///
/// [`ngettext`]: fn.ngettext.html
#[macro_export]
macro_rules! ngettext {
    ( $msgid:expr, $msgid_plural:expr, $n:expr $(,)? ) => {
        $crate::ngettext($msgid, $msgid_plural, $n)
    };
    ( $msgid:expr, $msgid_plural:expr, $n:expr, $($args:expr),+ $(,)? ) => {{
        let mut msgstr = $crate::ngettext($msgid, $msgid_plural, $n);
        if msgstr.contains("{n}") == true {
            msgstr = $crate::macros::rt_format(&msgstr, $n, "{n}");
        }
        msgstr = $crate::rt_format!(msgstr, "{}", $($args),+);
        debug_assert!(
            !msgstr.contains("{}"),
            "There are less arguments than format directives"
        );

        msgstr
    }};
}

/// Like [`dngettext`], but allows for formatting.
///
/// It calls [`dngettext`] on `domainname`, `msgid`, `msgid_plural`, and `n`, and then replaces
/// each occurrence of `{}` with the next value out of `args`.
///
/// # Panics
///
/// When building the `dev` profile,
/// it panics if the number of arguments doesn't match the number of format directives.
///
/// [`dngettext`]: fn.dngettext.html
#[macro_export]
macro_rules! dngettext {
    ( $domainname:expr, $msgid:expr, $msgid_plural:expr, $n:expr $(,)? ) => {
        $crate::dngettext($domainname, $msgid, $msgid_plural, $n)
    };
    ( $domainname:expr, $msgid:expr, $msgid_plural:expr, $n:expr, $($args:expr),+ $(,)? ) => {{
        let mut msgstr = $crate::dngettext($domainname, $msgid, $msgid_plural, $n);
        if msgstr.contains("{n}") == true {
            msgstr = $crate::macros::rt_format(&msgstr, $n, "{n}");
        }
        msgstr = $crate::rt_format!(msgstr, "{}", $($args),+);
        debug_assert!(
            !msgstr.contains("{}"),
            "There are less arguments than format directives"
        );

        msgstr
    }};
}

/// Like [`dcngettext`], but allows for formatting.
///
/// It calls [`dcngettext`] on `domainname`, `category`, `msgid`, `msgid_plural`, and `n`, and then
/// replaces each occurrence of `{}` with the next value out of `args`.
///
/// # Panics
///
/// When building the `dev` profile,
/// it panics if the number of arguments doesn't match the number of format directives.
///
/// [`dcngettext`]: fn.dcngettext.html
#[macro_export]
macro_rules! dcngettext {
    ( $domainname:expr, $category:expr, $msgid:expr, $msgid_plural:expr, $n:expr $(,)? ) => {
        $crate::dcngettext($domainname, $msgid, $msgid_plural, $n, $category)
    };
    ( $domainname:expr, $category:expr, $msgid:expr, $msgid_plural:expr, $n:expr, $($args:expr),+ $(,)? ) => {{
        let mut msgstr = $crate::dcngettext($domainname, $msgid, $msgid_plural, $n, $category);
        if msgstr.contains("{n}") == true {
            msgstr = $crate::macros::rt_format(&msgstr, $n, "{n}");
        }
        msgstr = $crate::rt_format!(msgstr, "{}", $($args),+);
        debug_assert!(
            !msgstr.contains("{}"),
            "There are less arguments than format directives"
        );

        msgstr
    }};
}

/// Like [`pgettext`], but allows for formatting.
///
/// It calls [`pgettext`] on `msgctxt` and `msgid`, and then replaces each occurrence of `{}` with
/// the next value out of `args`.
///
/// # Panics
///
/// When building the `dev` profile,
/// it panics if the number of arguments doesn't match the number of format directives.
///
/// [`pgettext`]: fn.pgettext.html
#[macro_export]
macro_rules! pgettext {
    ( $msgctxt:expr, $msgid:expr $(,)? ) => {
        $crate::pgettext($msgctxt, $msgid)
    };
    ( $msgctxt:expr, $msgid:expr, $($args:expr),+ $(,)? ) => {{
        let mut msgstr = $crate::pgettext($msgctxt, $msgid);
        msgstr = $crate::rt_format!(msgstr, "{}", $($args),+);
        debug_assert!(
            !msgstr.contains("{}"),
            "There are less arguments than format directives"
        );

        msgstr
    }};
}

/// Like [`npgettext`], but allows for formatting.
///
/// It calls [`npgettext`] on `msgctxt`, `msgid`, `msgid_plural`, and `n`, and then replaces each
/// occurrence of `{}` with the next value out of `args`.
///
/// # Panics
///
/// When building the `dev` profile,
/// it panics if the number of arguments doesn't match the number of format directives.
///
/// [`npgettext`]: fn.npgettext.html
#[macro_export]
macro_rules! npgettext {
    ( $msgctxt:expr, $msgid:expr, $msgid_plural:expr, $n:expr $(,)? ) => {
        $crate::npgettext($msgctxt, $msgid, $msgid_plural, $n)
    };
    ( $msgctxt:expr, $msgid:expr, $msgid_plural:expr, $n:expr, $($args:expr),+ $(,)? ) => {{
        let mut msgstr = $crate::npgettext($msgctxt, $msgid, $msgid_plural, $n);
        if msgstr.contains("{n}") == true {
            msgstr = $crate::macros::rt_format(&msgstr, $n, "{n}");
        }
        msgstr = $crate::rt_format!(msgstr, "{}", $($args),+);
        debug_assert!(
            !msgstr.contains("{}"),
            "There are less arguments than format directives"
        );

        msgstr
    }};
}

#[cfg(test)]
mod test {
    use crate::*;
    use std::panic::catch_unwind;

    #[test]
    fn no_formatting() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();
        {
            textdomain("hellorust").unwrap();

            assert_eq!(
                gettext!("Hello, World!"),
                "Hello, World!"
            );
            assert_eq!(
                ngettext!(
                    "There is one result",
                    "There are few results",
                    2
                ),
                "There are few results"
            );
            assert_eq!(
                pgettext!("context",
                    "Hello, World!"
                ),
                "Hello, World!"
            );
            assert_eq!(
                npgettext!("context",
                    "There is one result",
                    "There are few results",
                    2
                ),
                "There are few results"
            );
        };
        assert_eq!(
            dgettext!("hellorust",
                "Hello, World!"
            ),
            "Hello, World!"
        );
        assert_eq!(
            dngettext!("hellorust",
                "There is one result",
                "There are few results",
                2
            ),
            "There are few results"
        );
        assert_eq!(
            dcgettext!("hellorust", LocaleCategory::LcAll,
                "Hello, World!"
            ),
            "Hello, World!"
        );
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
    fn basic_formatting() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();
        {
            textdomain("hellorust").unwrap();

            assert_eq!(
                gettext!("Hello, {}! {}", "World", "UwU"),
                "Hello, World! UwU"
            );
            assert_eq!(
                ngettext!(
                    "There is one \"{}\" in text! {}",
                    "There are few \"{}\" in text! {}",
                    2, "UwU", "OwO"
                ),
                "There are few \"UwU\" in text! OwO"
            );
            assert_eq!(
                pgettext!("context",
                    "Hello, {}! {}", "World", "UwU"
                ),
                "Hello, World! UwU"
            );
            assert_eq!(
                npgettext!("context",
                    "There is one \"{}\" in text! {}",
                    "There are few \"{}\" in text! {}",
                    2, "UwU", "OwO"
                ),
                "There are few \"UwU\" in text! OwO"
            );
        };
        assert_eq!(
            dgettext!("hellorust",
                "Hello, {}! {}", "World", "UwU"
            ),
            "Hello, World! UwU"
        );
        assert_eq!(
            dngettext!("hellorust",
                "There is one \"{}\" in text! {}",
                "There are few \"{}\" in text! {}",
                2, "UwU", "OwO"
            ),
            "There are few \"UwU\" in text! OwO"
        );
        assert_eq!(
            dcgettext!("hellorust", LocaleCategory::LcAll,
                "Hello, {}! {}", "World", "UwU"
            ),
            "Hello, World! UwU"
        );
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
    fn less_arguments_than_parameters() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();
        {
            textdomain("hellorust").unwrap();

            match catch_unwind(|| {
                gettext!("Hello, {}! {}", "World")
            }) {
                Ok(msgstr) => { assert_eq!(msgstr, "Hello, World! {}") }
                Err(cause) => {
                    assert_eq!(
                        cause.downcast_ref::<&str>().unwrap(),
                        &"There are less arguments than format directives"
                    );
                }
            }

            match catch_unwind(|| {
                ngettext!(
                    "There is one \"{}\" in text! {}",
                    "There are few \"{}\" in text! {}",
                    2, "UwU"
                )
            }) {
                Ok(msgstr) => { assert_eq!(msgstr, "There are few \"UwU\" in text! {}") }
                Err(cause) => {
                    assert_eq!(
                        cause.downcast_ref::<&str>().unwrap(),
                        &"There are less arguments than format directives"
                    );
                }
            }

            match catch_unwind(|| {
                pgettext!("context",
                    "Hello, {}! {}", "World"
                )
            }) {
                Ok(msgstr) => { assert_eq!(msgstr, "Hello, World! {}") }
                Err(cause) => {
                    assert_eq!(
                        cause.downcast_ref::<&str>().unwrap(),
                        &"There are less arguments than format directives"
                    );
                }
            }

            match catch_unwind(|| {
                npgettext!("context",
                    "There is one \"{}\" in text! {}",
                    "There are few \"{}\" in text! {}",
                    2, "UwU"
                )
            }) {
                Ok(msgstr) => { assert_eq!(msgstr, "There are few \"UwU\" in text! {}") }
                Err(cause) => {
                    assert_eq!(
                        cause.downcast_ref::<&str>().unwrap(),
                        &"There are less arguments than format directives"
                    );
                }
            }
        };

        match catch_unwind(|| {
            dgettext!("hellorust",
                "Hello, {}! {}", "World"
            )
        }) {
            Ok(msgstr) => { assert_eq!(msgstr, "Hello, World! {}") }
            Err(cause) => {
                assert_eq!(
                    cause.downcast_ref::<&str>().unwrap(),
                    &"There are less arguments than format directives"
                );
            }
        }

        match catch_unwind(|| {
            dngettext!("hellorust",
                "There is one \"{}\" in text! {}",
                "There are few \"{}\" in text! {}",
                2, "UwU"
            )
        }) {
            Ok(msgstr) => { assert_eq!(msgstr, "There are few \"UwU\" in text! {}") }
            Err(cause) => {
                assert_eq!(
                    cause.downcast_ref::<&str>().unwrap(),
                    &"There are less arguments than format directives"
                );
            }
        }

        match catch_unwind(|| {
            dcgettext!("hellorust", LocaleCategory::LcAll,
                "Hello, {}! {}", "World"
            )
        }) {
            Ok(msgstr) => { assert_eq!(msgstr, "Hello, World! {}") }
            Err(cause) => {
                assert_eq!(
                    cause.downcast_ref::<&str>().unwrap(),
                    &"There are less arguments than format directives"
                );
            }
        }

        match catch_unwind(|| {
            dcngettext!("hellorust", LocaleCategory::LcAll,
                "There is one \"{}\" in text! {}",
                "There are few \"{}\" in text! {}",
                2, "UwU"
            )
        }) {
            Ok(msgstr) => { assert_eq!(msgstr, "There are few \"UwU\" in text! {}") }
            Err(cause) => {
                assert_eq!(
                    cause.downcast_ref::<&str>().unwrap(),
                    &"There are less arguments than format directives"
                );
            }
        }
    }

    #[test]
    fn more_arguments_than_parameters() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();
        {
            textdomain("hellorust").unwrap();

            match catch_unwind(|| {
                gettext!("Hello, {}!", "World", "UwU")
            }) {
                Ok(msgstr) => { assert_eq!(msgstr, "Hello, World!") }
                Err(cause) => {
                    assert_eq!(
                        cause.downcast_ref::<&str>().unwrap(),
                        &"There are more arguments than format directives"
                    );
                }
            }

            match catch_unwind(|| {
                ngettext!(
                    "There is one \"{}\" in text!",
                    "There are few \"{}\" in text!",
                    2, "UwU", "OwO"
                )
            }) {
                Ok(msgstr) => { assert_eq!(msgstr, "There are few \"UwU\" in text!") }
                Err(cause) => {
                    assert_eq!(
                        cause.downcast_ref::<&str>().unwrap(),
                        &"There are more arguments than format directives"
                    );
                }
            }

            match catch_unwind(|| {
                pgettext!("context",
                    "Hello, {}!", "World", "UwU"
                )
            }) {
                Ok(msgstr) => { assert_eq!(msgstr, "Hello, World!") }
                Err(cause) => {
                    assert_eq!(
                        cause.downcast_ref::<&str>().unwrap(),
                        &"There are more arguments than format directives"
                    );
                }
            }

            match catch_unwind(|| {
                npgettext!("context",
                    "There is one \"{}\" in text!",
                    "There are few \"{}\" in text!",
                    2, "UwU", "OwO"
                )
            }) {
                Ok(msgstr) => { assert_eq!(msgstr, "There are few \"UwU\" in text!") }
                Err(cause) => {
                    assert_eq!(
                        cause.downcast_ref::<&str>().unwrap(),
                        &"There are more arguments than format directives"
                    );
                }
            }
        };

        match catch_unwind(|| {
            dgettext!("hellorust",
                "Hello, {}!", "World", "UwU"
            )
        }) {
            Ok(msgstr) => { assert_eq!(msgstr, "Hello, World!") }
            Err(cause) => {
                assert_eq!(
                    cause.downcast_ref::<&str>().unwrap(),
                    &"There are more arguments than format directives"
                );
            }
        }

        match catch_unwind(|| {
            dngettext!("hellorust",
                "There is one \"{}\" in text!",
                "There are few \"{}\" in text!",
                2, "UwU", "OwO"
            )
        }) {
            Ok(msgstr) => { assert_eq!(msgstr, "There are few \"UwU\" in text!") }
            Err(cause) => {
                assert_eq!(
                    cause.downcast_ref::<&str>().unwrap(),
                    &"There are more arguments than format directives"
                );
            }
        }

        match catch_unwind(|| {
            dcgettext!("hellorust", LocaleCategory::LcAll,
                "Hello, {}!", "World", "UwU"
            )
        }) {
            Ok(msgstr) => { assert_eq!(msgstr, "Hello, World!") }
            Err(cause) => {
                assert_eq!(
                    cause.downcast_ref::<&str>().unwrap(),
                    &"There are more arguments than format directives"
                );
            }
        }

        match catch_unwind(|| {
            dcngettext!("hellorust", LocaleCategory::LcAll,
                "There is one \"{}\" in text!",
                "There are few \"{}\" in text!",
                2, "UwU", "OwO"
            )
        }) {
            Ok(msgstr) => { assert_eq!(msgstr, "There are few \"UwU\" in text!") }
            Err(cause) => {
                assert_eq!(
                    cause.downcast_ref::<&str>().unwrap(),
                    &"There are more arguments than format directives"
                );
            }
        }
    }

    #[test]
    fn special_n_formatting() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();
        {
            textdomain("hellorust").unwrap();
            assert_eq!(
                ngettext!(
                    "There is {n} \"{}\" in text!",
                    "There are {n} \"{}\" in text!",
                    2, "UwU"
                ),
                "There are 2 \"UwU\" in text!"
            );
            assert_eq!(
                npgettext!("context",
                    "There is {n} \"{}\" in text!",
                    "There are {n} \"{}\" in text!",
                    2, "UwU"
                ),
                "There are 2 \"UwU\" in text!"
            );
        };
        assert_eq!(
            dngettext!("hellorust",
                    "There is {n} \"{}\" in text!",
                    "There are {n} \"{}\" in text!",
                    2, "UwU"
                ),
            "There are 2 \"UwU\" in text!"
        );
        assert_eq!(
            dcngettext!("hellorust", LocaleCategory::LcAll,
                    "There is {n} \"{}\" in text!",
                    "There are {n} \"{}\" in text!",
                    2, "UwU"
                ),
            "There are 2 \"UwU\" in text!"
        );
    }

    #[test]
    fn trailing_comma() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
        bindtextdomain("hellorust", "/usr/local/share/locale").unwrap();
        {
            textdomain("hellorust").unwrap();

            assert_eq!(
                gettext!("Hello, World!",),
                "Hello, World!"
            );
            assert_eq!(
                ngettext!(
                    "There is one result",
                    "There are few results",
                    2,
                ),
                "There are few results"
            );
            assert_eq!(
                pgettext!("context",
                    "Hello, World!",
                ),
                "Hello, World!"
            );
            assert_eq!(
                npgettext!("context",
                    "There is one result",
                    "There are few results",
                    2,
                ),
                "There are few results"
            );

            assert_eq!(
                gettext!("Hello, {}! {}", "World", "UwU",),
                "Hello, World! UwU"
            );
            assert_eq!(
                ngettext!(
                    "There is one \"{}\" in text! {}",
                    "There are few \"{}\" in text! {}",
                    2, "UwU", "OwO",
                ),
                "There are few \"UwU\" in text! OwO"
            );
            assert_eq!(
                pgettext!("context",
                    "Hello, {}! {}", "World", "UwU",
                ),
                "Hello, World! UwU"
            );
            assert_eq!(
                npgettext!("context",
                    "There is one \"{}\" in text! {}",
                    "There are few \"{}\" in text! {}",
                    2, "UwU", "OwO",
                ),
                "There are few \"UwU\" in text! OwO"
            );
        };
        assert_eq!(
            dgettext!("hellorust",
                "Hello, World!",
            ),
            "Hello, World!",
        );
        assert_eq!(
            dngettext!("hellorust",
                "There is one result",
                "There are few results",
                2,
            ),
            "There are few results"
        );
        assert_eq!(
            dcgettext!("hellorust", LocaleCategory::LcAll,
                "Hello, World!",
            ),
            "Hello, World!"
        );
        assert_eq!(
            dcngettext!("hellorust", LocaleCategory::LcAll,
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
        assert_eq!(
            dcgettext!("hellorust", LocaleCategory::LcAll,
                "Hello, {}! {}", "World", "UwU",
            ),
            "Hello, World! UwU"
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
}
