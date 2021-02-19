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

/// Locale category enum ported from locale.h
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

/// Translate msgid to localized message from default domain
pub fn gettext<T: Into<Vec<u8>>>(s: T) -> String {
    let s = CString::new(s).unwrap();
    unsafe {
        CStr::from_ptr(ffi::gettext(s.as_ptr()))
            .to_string_lossy()
            .into_owned()
    }
}

/// Translate msgid to localized message from specified domain
pub fn dgettext<T: Into<Vec<u8>>>(domain: T, s: T) -> String {
    let domain = CString::new(domain).unwrap();
    let s = CString::new(s).unwrap();
    unsafe {
        CStr::from_ptr(ffi::dgettext(domain.as_ptr(), s.as_ptr()))
            .to_string_lossy()
            .into_owned()
    }
}

/// Translate msgid to localized message from specified domain using custom locale category
pub fn dcgettext<T: Into<Vec<u8>>>(domain: T, s: T, category: LocaleCategory) -> String {
    let domain = CString::new(domain).unwrap();
    let s = CString::new(s).unwrap();
    unsafe {
        CStr::from_ptr(ffi::dcgettext(domain.as_ptr(), s.as_ptr(), category as i32))
            .to_string_lossy()
            .into_owned()
    }
}

/// Translate msgid to localized message from default domain (with plural support)
pub fn ngettext<T: Into<Vec<u8>>>(singular: T, plural : T, n : u32) -> String {
    let singular = CString::new(singular).unwrap();
    let plural = CString::new(plural).unwrap();
    unsafe {
        CStr::from_ptr(ffi::ngettext(singular.as_ptr(), plural.as_ptr(), n as c_ulong))
            .to_string_lossy()
            .into_owned()
    }
}

/// Translate msgid to localized message from specified domain (with plural support)
pub fn dngettext<T: Into<Vec<u8>>>(domain: T, singular: T, plural: T, n : u32) -> String {
    let domain = CString::new(domain).unwrap();
    let singular = CString::new(singular).unwrap();
    let plural = CString::new(plural).unwrap();
    unsafe {
        CStr::from_ptr(ffi::dngettext(domain.as_ptr(), singular.as_ptr(), plural.as_ptr(), n as c_ulong))
            .to_string_lossy()
            .into_owned()
    }
}

/// Translate msgid to localized message from specified domain using custom locale category (with plural support)
pub fn dcngettext<T: Into<Vec<u8>>>(domain: T, singular: T, plural: T, n : u32, category: LocaleCategory) -> String {
    let domain = CString::new(domain).unwrap();
    let singular = CString::new(singular).unwrap();
    let plural = CString::new(plural).unwrap();
    unsafe {
        CStr::from_ptr(ffi::dcngettext(domain.as_ptr(), singular.as_ptr(), plural.as_ptr(), n as c_ulong, category as i32))
            .to_string_lossy()
            .into_owned()
    }
}

/// Switch to specific text domain
pub fn textdomain<T: Into<Vec<u8>>>(domain: T) -> String {
    let domain = CString::new(domain).unwrap();
    unsafe {
        CStr::from_ptr(ffi::textdomain(domain.as_ptr()))
            .to_string_lossy()
            .into_owned()
    }
}

/// Bind text domain to some directory containing gettext MO files
pub fn bindtextdomain<T: Into<Vec<u8>>>(domain: T, dir: T) -> String {
    let domain = CString::new(domain).unwrap();
    let dir = CString::new(dir).unwrap();
    unsafe {
        CStr::from_ptr(ffi::bindtextdomain(domain.as_ptr(),
                                                   dir.as_ptr()))
            .to_string_lossy()
            .into_owned()
    }
}

/// Set current locale for translations
pub fn setlocale<T: Into<Vec<u8>>>(category: LocaleCategory, locale: T) -> Option<String> {
    let c = CString::new(locale).unwrap();
    unsafe {
        let ret = ffi::setlocale(category as i32, c.as_ptr());
        if ret.is_null() {
            None
        } else {
            Some(CStr::from_ptr(ret).to_string_lossy().into_owned())
        }
    }
}

pub fn bind_textdomain_codeset<T: Into<Vec<u8>>>(domain: T, codeset: T) -> String {
    let domain = CString::new(domain).unwrap();
    let codeset = CString::new(codeset).unwrap();
    unsafe {
        CStr::from_ptr(ffi::bind_textdomain_codeset(domain.as_ptr(),
                                                   codeset.as_ptr()))
            .to_string_lossy()
            .into_owned()
    }
}

static CONTEXT_SEPARATOR: u8 = b'\x04';

fn build_context_id(ctx: &Vec<u8>, s: &Vec<u8>) -> String {
    let mut text: Vec<u8> = vec![];
    text.extend(ctx.iter().cloned());
    text.push(CONTEXT_SEPARATOR);
    text.extend(s.iter().cloned());
    CString::new(text).unwrap().to_string_lossy().into_owned()
}

/// Translate msgid to localized message from default domain (with context support)
pub fn pgettext<T: Into<Vec<u8>>>(ctx: T, s: T) -> String {
    let msgid = s.into();
    let text = build_context_id(&ctx.into(), &msgid);

    let trans = gettext(text);
    if trans.contains(CONTEXT_SEPARATOR as char) {
        return gettext(msgid);
        //return CString::new(msgid).unwrap().to_string_lossy().into_owned();
    }

    trans
}

/// Translate msgid to localized message from default domain (with plural support and context
/// support)
pub fn npgettext<T: Into<Vec<u8>>>(ctx: T, singular: T, plural: T, n: u32) -> String {
    let ctx = ctx.into();
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
}
