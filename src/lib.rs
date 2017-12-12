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
//! ```

extern crate gettext_sys as ffi;

use std::ffi::CString;
use std::ffi::CStr;
use std::os::raw::c_ulong;

/// Locale category enum ported from locale.h
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
    unsafe {
        CStr::from_ptr(ffi::gettext(CString::new(s).unwrap().as_ptr()))
            .to_string_lossy()
            .into_owned()
    }
}

/// Translate msgid to localized message from specified domain
pub fn dgettext<T: Into<Vec<u8>>>(domain: T, s: T) -> String {
    unsafe {
        CStr::from_ptr(ffi::dgettext(CString::new(domain).unwrap().as_ptr(), CString::new(s).unwrap().as_ptr()))
            .to_string_lossy()
            .into_owned()
    }
}

/// Translate msgid to localized message from specified domain using custom locale category
pub fn dcgettext<T: Into<Vec<u8>>>(domain: T, s: T, category: LocaleCategory) -> String {
    unsafe {
        CStr::from_ptr(ffi::dcgettext(CString::new(domain).unwrap().as_ptr(), CString::new(s).unwrap().as_ptr(), category as i32))
            .to_string_lossy()
            .into_owned()
    }
}

/// Translate msgid to localized message from default domain (with plural support)
pub fn ngettext<T: Into<Vec<u8>>>(singular: T, plural : T, n : u32) -> String {
    unsafe {
        CStr::from_ptr(ffi::ngettext(CString::new(singular).unwrap().as_ptr(), CString::new(plural).unwrap().as_ptr(), n as c_ulong))
            .to_string_lossy()
            .into_owned()
    }
}

/// Translate msgid to localized message from specified domain (with plural support)
pub fn dngettext<T: Into<Vec<u8>>>(domain: T, singular: T, plural: T, n : u32) -> String {
    unsafe {
        CStr::from_ptr(ffi::dngettext(CString::new(domain).unwrap().as_ptr(), CString::new(singular).unwrap().as_ptr(), CString::new(plural).unwrap().as_ptr(), n as c_ulong))
            .to_string_lossy()
            .into_owned()
    }
}

/// Translate msgid to localized message from specified domain using custom locale category (with plural support)
pub fn dcngettext<T: Into<Vec<u8>>>(domain: T, singular: T, plural: T, n : u32, category: LocaleCategory) -> String {
    unsafe {
        CStr::from_ptr(ffi::dcngettext(CString::new(domain).unwrap().as_ptr(), CString::new(singular).unwrap().as_ptr(), CString::new(plural).unwrap().as_ptr(), n as c_ulong, category as i32))
            .to_string_lossy()
            .into_owned()
    }
}

/// Switch to specific text domain
pub fn textdomain<T: Into<Vec<u8>>>(domain: T) -> String {
    unsafe {
        CStr::from_ptr(ffi::textdomain(CString::new(domain).unwrap().as_ptr()))
            .to_string_lossy()
            .into_owned()
    }
}

/// Bind text domain to some directory containing gettext MO files
pub fn bindtextdomain<T: Into<Vec<u8>>>(domain: T, dir: T) -> String {
    unsafe {
        CStr::from_ptr(ffi::bindtextdomain(CString::new(domain).unwrap().as_ptr(),
                                                   CString::new(dir).unwrap().as_ptr()))
            .to_string_lossy()
            .into_owned()
    }
}

/// Set current locale for translations
pub fn setlocale<T: Into<Vec<u8>>>(category: LocaleCategory, locale: T) -> String {
    unsafe {
        CStr::from_ptr(ffi::setlocale(category as i32,
                                              CString::new(locale).unwrap().as_ptr()))
            .to_string_lossy()
            .into_owned()
    }
}

pub fn bind_textdomain_codeset<T: Into<Vec<u8>>>(domain: T, codeset: T) -> String {
    unsafe {
        CStr::from_ptr(ffi::bind_textdomain_codeset(CString::new(domain).unwrap().as_ptr(),
                                                   CString::new(codeset).unwrap().as_ptr()))
            .to_string_lossy()
            .into_owned()
    }
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
}
