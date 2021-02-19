//! # gettext C library FFI binding for Rust
//!
//! Usage:
//!
//! ```
//! use gettextrs::*;
//!
//! setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
//!
//! bindtextdomain("hellorust", "/usr/local/share/locale");
//! textdomain("hellorust");
//!
//! println!("Translated: {}", gettext("Hello, world!"));
//! println!("Singular: {}", ngettext("One thing", "Multiple things", 1));
//! println!("Plural: {}", ngettext("One thing", "Multiple things", 2));
//! println!("With placeholder: {}", gettext!("Hello, {user}!", user = "Example User"));
//! ```
//!
//! Alternatively, you can initialize the locale and text domain using the [`TextDomain`] builder.
//! By default, a translation of the specified text domain in current language is searched in
//! the system's data paths. See [`TextDomain`]'s documentation for other options.
//!
//! ```no_run
//! use gettextrs::TextDomain;
//!
//! TextDomain::new("hellorust")
//!            .init()
//!            .unwrap();
//! ```
//!
//! [`TextDomain`]: struct.TextDomain.html

extern crate locale_config;

extern crate gettext_sys as ffi;

use std::ffi::CString;
use std::ffi::CStr;
use std::os::raw::c_ulong;

mod macros;
pub use macros::*;
mod text_domain;
pub use text_domain::{TextDomain, TextDomainError};

/// Locale category enum ported from locale.h.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LocaleCategory {
    /// Character classification and case conversion.
    LcCType = 0,
    /// Non-monetary numeric formats.
    LcNumeric = 1,
    /// Date and time formats.
    LcTime = 2,
    /// Collation order.
    LcCollate = 3,
    /// Monetary formats.
    LcMonetary = 4,
    /// Formats of informative and diagnostic messages and interactive responses.
    LcMessages = 5,
    /// For all.
    LcAll = 6,
    /// Paper size.
    LcPaper = 7,
    /// Name formats.
    LcName = 8,
    /// Address formats and location information.
    LcAddress = 9,
    /// Telephone number formats.
    LcTelephone = 10,
    /// Measurement units (Metric or Other).
    LcMeasurement = 11,
    /// Metadata about the locale information.
    LcIdentification = 12,
}

/// Translate msgid to localized message from default domain.
///
/// # Panics
///
/// Panics if `s` contains an internal 0 byte, as such values can't be passed to the gettext's
/// C API.
pub fn gettext<T: Into<Vec<u8>>>(s: T) -> String {
    let s = CString::new(s).expect("`s` contains an internal 0 byte");
    unsafe {
        CStr::from_ptr(ffi::gettext(s.as_ptr()))
            .to_string_lossy()
            .into_owned()
    }
}

/// Translate msgid to localized message from specified domain.
///
/// # Panics
///
/// Panics if `domain` or `s` contain an internal 0 byte, as such values can't be passed to the
/// gettext's C API.
pub fn dgettext<T: Into<Vec<u8>>>(domain: T, s: T) -> String {
    let domain = CString::new(domain).expect("`domain` contains an internal 0 byte");
    let s = CString::new(s).expect("`s` contains an internal 0 byte");
    unsafe {
        CStr::from_ptr(ffi::dgettext(domain.as_ptr(), s.as_ptr()))
            .to_string_lossy()
            .into_owned()
    }
}

/// Translate msgid to localized message from specified domain using custom locale category.
///
/// # Panics
///
/// Panics if `domain` or `s` contain an internal 0 byte, as such values can't be passed to the
/// gettext's C API.
pub fn dcgettext<T: Into<Vec<u8>>>(domain: T, s: T, category: LocaleCategory) -> String {
    let domain = CString::new(domain).expect("`domain` contains an internal 0 byte");
    let s = CString::new(s).expect("`s` contains an internal 0 byte");
    unsafe {
        CStr::from_ptr(ffi::dcgettext(domain.as_ptr(), s.as_ptr(), category as i32))
            .to_string_lossy()
            .into_owned()
    }
}

/// Translate msgid to localized message from default domain (with plural support).
///
/// # Panics
///
/// Panics if `singular` or `plural` contain an internal 0 byte, as such values can't be passed to
/// the gettext's C API.
pub fn ngettext<T: Into<Vec<u8>>>(singular: T, plural : T, n : u32) -> String {
    let singular = CString::new(singular).expect("`singular` contains an internal 0 byte");
    let plural = CString::new(plural).expect("`plural` contains an internal 0 byte");
    unsafe {
        CStr::from_ptr(ffi::ngettext(singular.as_ptr(), plural.as_ptr(), n as c_ulong))
            .to_string_lossy()
            .into_owned()
    }
}

/// Translate msgid to localized message from specified domain (with plural support).
///
/// # Panics
///
/// Panics if `domain`, `singular`, or `plural` contain an internal 0 byte, as such values can't be
/// passed to the gettext's C API.
pub fn dngettext<T: Into<Vec<u8>>>(domain: T, singular: T, plural: T, n : u32) -> String {
    let domain = CString::new(domain).expect("`domain` contains an internal 0 byte");
    let singular = CString::new(singular).expect("`singular` contains an internal 0 byte");
    let plural = CString::new(plural).expect("`plural` contains an internal 0 byte");
    unsafe {
        CStr::from_ptr(ffi::dngettext(domain.as_ptr(), singular.as_ptr(), plural.as_ptr(), n as c_ulong))
            .to_string_lossy()
            .into_owned()
    }
}

/// Translate msgid to localized message from specified domain using custom locale category (with plural support).
///
/// # Panics
///
/// Panics if `domain`, `singular`, or `plural` contain an internal 0 byte, as such values can't be
/// passed to the gettext's C API.
pub fn dcngettext<T: Into<Vec<u8>>>(domain: T, singular: T, plural: T, n : u32, category: LocaleCategory) -> String {
    let domain = CString::new(domain).expect("`domain` contains an internal 0 byte");
    let singular = CString::new(singular).expect("`singular` contains an internal 0 byte");
    let plural = CString::new(plural).expect("`plural` contains an internal 0 byte");
    unsafe {
        CStr::from_ptr(ffi::dcngettext(domain.as_ptr(), singular.as_ptr(), plural.as_ptr(), n as c_ulong, category as i32))
            .to_string_lossy()
            .into_owned()
    }
}

/// Switch to specific text domain.
///
/// # Panics
///
/// Panics if `domain` contains an internal 0 byte, as such values can't be passed to the gettext's
/// C API.
pub fn textdomain<T: Into<Vec<u8>>>(domain: T) -> String {
    let domain = CString::new(domain).expect("`domain` contains an internal 0 byte");
    unsafe {
        CStr::from_ptr(ffi::textdomain(domain.as_ptr()))
            .to_string_lossy()
            .into_owned()
    }
}

/// Bind text domain to some directory containing gettext MO files.
///
/// # Panics
///
/// Panics if `domain` or `dir` contain an internal 0 byte, as such values can't be passed to the
/// gettext's C API.
pub fn bindtextdomain<T: Into<Vec<u8>>>(domain: T, dir: T) -> String {
    let domain = CString::new(domain).expect("`domain` contains an internal 0 byte");
    let dir = CString::new(dir).expect("`dir` contains an internal 0 byte");
    unsafe {
        CStr::from_ptr(ffi::bindtextdomain(domain.as_ptr(), dir.as_ptr()))
            .to_string_lossy()
            .into_owned()
    }
}

/// Set current locale for translations.
///
/// # Panics
///
/// Panics if `locale` contains an internal 0 byte, as such values can't be passed to the gettext's
/// C API.
pub fn setlocale<T: Into<Vec<u8>>>(category: LocaleCategory, locale: T) -> Option<String> {
    let c = CString::new(locale).expect("`locale` contains an internal 0 byte");
    unsafe {
        let ret = ffi::setlocale(category as i32, c.as_ptr());
        if ret.is_null() {
            None
        } else {
            Some(CStr::from_ptr(ret).to_string_lossy().into_owned())
        }
    }
}

/// Set encoding of translated messages.
///
/// # Panics
///
/// Panics if `domain` or `codeset` contain an internal 0 byte, as such values can't be passed to
/// the gettext's C API.
pub fn bind_textdomain_codeset<T: Into<Vec<u8>>>(domain: T, codeset: T) -> String {
    let domain = CString::new(domain).expect("`domain` contains an internal 0 byte");
    let codeset = CString::new(codeset).expect("`codeset` contains an internal 0 byte");
    unsafe {
        CStr::from_ptr(ffi::bind_textdomain_codeset(domain.as_ptr(),
                                                   codeset.as_ptr()))
            .to_string_lossy()
            .into_owned()
    }
}

static CONTEXT_SEPARATOR: u8 = b'\x04';

fn build_context_id(ctx: &Vec<u8>, s: &Vec<u8>) -> Vec<u8> {
    let mut text: Vec<u8> = vec![];
    text.extend(ctx.iter().cloned());
    text.push(CONTEXT_SEPARATOR);
    text.extend(s.iter().cloned());
    text
}

fn panic_on_zero_in_ctx(string: &Vec<u8>) {
    if string.contains(&0) {
        panic!("`ctx` contains an internal 0 byte");
    }
}

/// Translate msgid to localized message from default domain (with context support).
///
/// # Panics
///
/// Panics if `ctx` or `s` contain an internal 0 byte, as such values can't be passed to the
/// gettext's C API.
pub fn pgettext<T: Into<Vec<u8>>>(ctx: T, s: T) -> String {
    let ctx = ctx.into();
    panic_on_zero_in_ctx(&ctx);

    let msgid = s.into();
    let text = build_context_id(&ctx, &msgid);

    let trans = gettext(text);
    if trans.contains(CONTEXT_SEPARATOR as char) {
        return gettext(msgid);
    }

    trans
}

/// Translate msgid to localized message from default domain (with plural support and context
/// support).
///
/// # Panics
///
/// Panics if `ctx`, `singular`, or `plural` contain an internal 0 byte, as such values can't be
/// passed to the gettext's C API.
pub fn npgettext<T: Into<Vec<u8>>>(ctx: T, singular: T, plural: T, n: u32) -> String {
    let ctx = ctx.into();
    panic_on_zero_in_ctx(&ctx);

    let singular_msgid = singular.into();
    let plural_msgid = plural.into();
    let singular_ctx = build_context_id(&ctx, &singular_msgid);
    let plural_ctx = build_context_id(&ctx, &plural_msgid);

    let trans = ngettext(singular_ctx, plural_ctx, n);
    if trans.contains(CONTEXT_SEPARATOR as char) {
        return ngettext(singular_msgid, plural_msgid, n);
    }

    trans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_test() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");

        bindtextdomain("hellorust", "/usr/local/share/locale");
        textdomain("hellorust");

        assert_eq!("Hello, world!", gettext("Hello, world!"));
    }

    #[test]
    fn plural_test() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");

        bindtextdomain("hellorust", "/usr/local/share/locale");
        textdomain("hellorust");

        assert_eq!("Hello, world!", ngettext("Hello, world!", "Hello, worlds!", 1));
        assert_eq!("Hello, worlds!", ngettext("Hello, world!", "Hello, worlds!", 2));
    }

    #[test]
    fn context_test() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");

        bindtextdomain("hellorust", "/usr/local/share/locale");
        textdomain("hellorust");

        assert_eq!("Hello, world!", pgettext("context", "Hello, world!"));
    }

    #[test]
    fn plural_context_test() {
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");

        bindtextdomain("hellorust", "/usr/local/share/locale");
        textdomain("hellorust");

        assert_eq!("Hello, world!", npgettext("context", "Hello, world!", "Hello, worlds!", 1));
        assert_eq!("Hello, worlds!", npgettext("context", "Hello, world!", "Hello, worlds!", 2));
    }

    #[test]
    #[should_panic(expected = "`s` contains an internal 0 byte")]
    fn gettext_panics() {
        gettext("input string\0");
    }

    #[test]
    #[should_panic(expected = "`domain` contains an internal 0 byte")]
    fn dgettext_panics_on_zero_in_domain() {
        dgettext("hello\0world!", "hi");
    }

    #[test]
    #[should_panic(expected = "`s` contains an internal 0 byte")]
    fn dgettext_panics_on_zero_in_s() {
        dgettext("hello world", "another che\0ck");
    }

    #[test]
    #[should_panic(expected = "`domain` contains an internal 0 byte")]
    fn dcgettext_panics_on_zero_in_domain() {
        dcgettext("a diff\0erent input", "hello", LocaleCategory::LcAll);
    }

    #[test]
    #[should_panic(expected = "`s` contains an internal 0 byte")]
    fn dcgettext_panics_on_zero_in_s() {
        dcgettext("world", "yet \0 another\0 one", LocaleCategory::LcMessages);
    }

    #[test]
    #[should_panic(expected = "`singular` contains an internal 0 byte")]
    fn ngettext_panics_on_zero_in_singular() {
        ngettext("singular\0form", "plural form", 10);
    }

    #[test]
    #[should_panic(expected = "`plural` contains an internal 0 byte")]
    fn ngettext_panics_on_zero_in_plural() {
        ngettext("singular form", "plural\0form", 0);
    }

    #[test]
    #[should_panic(expected = "`domain` contains an internal 0 byte")]
    fn dngettext_panics_on_zero_in_domain() {
        dngettext("do\0main", "one", "many", 0);
    }

    #[test]
    #[should_panic(expected = "`singular` contains an internal 0 byte")]
    fn dngettext_panics_on_zero_in_singular() {
        dngettext("domain", "just a\0 single one", "many", 100);
    }

    #[test]
    #[should_panic(expected = "`plural` contains an internal 0 byte")]
    fn dngettext_panics_on_zero_in_plural() {
        dngettext("d", "1", "many\0many\0many more", 10000);
    }

    #[test]
    #[should_panic(expected = "`domain` contains an internal 0 byte")]
    fn dcngettext_panics_on_zero_in_domain() {
        dcngettext("doma\0in", "singular", "plural", 42, LocaleCategory::LcCType);
    }

    #[test]
    #[should_panic(expected = "`singular` contains an internal 0 byte")]
    fn dcngettext_panics_on_zero_in_singular() {
        dcngettext("domain", "\0ne", "plural", 13, LocaleCategory::LcNumeric);
    }

    #[test]
    #[should_panic(expected = "`plural` contains an internal 0 byte")]
    fn dcngettext_panics_on_zero_in_plural() {
        dcngettext("d-o-m-a-i-n", "one", "a\0few", 0, LocaleCategory::LcTime);
    }

    #[test]
    #[should_panic(expected = "`domain` contains an internal 0 byte")]
    fn textdomain_panics_on_zero_in_domain() {
        textdomain("this is \0 my domain");
    }

    #[test]
    #[should_panic(expected = "`domain` contains an internal 0 byte")]
    fn bindtextdomain_panics_on_zero_in_domain() {
        bindtextdomain("\0bind this", "/usr/share/locale");
    }

    #[test]
    #[should_panic(expected = "`dir` contains an internal 0 byte")]
    fn bindtextdomain_panics_on_zero_in_dir() {
        bindtextdomain("my_domain", "/opt/locales\0");
    }

    #[test]
    #[should_panic(expected = "`locale` contains an internal 0 byte")]
    fn setlocale_panics_on_zero_in_locale() {
        setlocale(LocaleCategory::LcCollate, "en_\0US");
    }

    #[test]
    #[should_panic(expected = "`domain` contains an internal 0 byte")]
    fn bind_textdomain_codeset_panics_on_zero_in_domain() {
        bind_textdomain_codeset("doma\0in", "UTF-8");
    }

    #[test]
    #[should_panic(expected = "`codeset` contains an internal 0 byte")]
    fn bind_textdomain_codeset_panics_on_zero_in_codeset() {
        bind_textdomain_codeset("name", "K\0I8-R");
    }

    #[test]
    #[should_panic(expected = "`ctx` contains an internal 0 byte")]
    fn pgettext_panics_on_zero_in_ctx() {
        pgettext("context\0", "string");
    }

    #[test]
    #[should_panic(expected = "`s` contains an internal 0 byte")]
    fn pgettext_panics_on_zero_in_s() {
        pgettext("ctx", "a message\0to be translated");
    }

    #[test]
    #[should_panic(expected = "`ctx` contains an internal 0 byte")]
    fn npgettext_panics_on_zero_in_ctx() {
        npgettext("c\0tx", "singular", "plural", 0);
    }

    #[test]
    #[should_panic(expected = "`singular` contains an internal 0 byte")]
    fn npgettext_panics_on_zero_in_singular() {
        npgettext("ctx", "sing\0ular", "many many more", 135626);
    }

    #[test]
    #[should_panic(expected = "`plural` contains an internal 0 byte")]
    fn npgettext_panics_on_zero_in_plural() {
        npgettext("context", "uno", "one \0fewer", 10585);
    }
}
