//! # gettext C library FFI binding for Rust
//!
//! Usage:
//!
//! ```
//! use gettext_rs::*;
//!
//! set_locale(LocaleCategory::LcAll, "en_US.UTF-8");
//!
//! bind_text_domain("hellorust", "/usr/local/share/locale");
//! text_domain("hellorust");
//!
//! println!("Translated: {}", get_text("Hello, world!"));
//! ```

/// Raw FFI interface for gettext library
mod gettext_raw {
    use std::os::raw::{c_char, c_int};

    extern "C" {
        pub fn gettext(s: *const c_char) -> *const c_char;
        pub fn bindtextdomain(domain: *const c_char, dir: *const c_char) -> *const c_char;
        pub fn textdomain(domain: *const c_char) -> *const c_char;
        pub fn setlocale(category: c_int, locale: *const c_char) -> *const c_char;
    }
}

/// Safe wrapper for gettext library
mod get_text {
    use gettext_raw;
    use std::ffi::CString;
    use std::ffi::CStr;

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

    /// Translate msgid to localized message
    pub fn get_text<T: Into<Vec<u8>>>(s: T) -> String {
        unsafe {
            CStr::from_ptr(gettext_raw::gettext(CString::new(s).unwrap().as_ptr()))
                .to_string_lossy()
                .into_owned()
        }
    }

    /// Switch to specific text domain
    pub fn text_domain<T: Into<Vec<u8>>>(domain: T) -> String {
        unsafe {
            CStr::from_ptr(gettext_raw::textdomain(CString::new(domain).unwrap().as_ptr()))
                .to_string_lossy()
                .into_owned()
        }
    }

    /// Bind text domain to some directory containing gettext MO files
    pub fn bind_text_domain<T: Into<Vec<u8>>>(domian: T, dir: T) -> String {
        unsafe {
            CStr::from_ptr(gettext_raw::bindtextdomain(CString::new(domian).unwrap().as_ptr(),
                                                       CString::new(dir).unwrap().as_ptr()))
                .to_string_lossy()
                .into_owned()
        }
    }

    /// Set current locale for translations
    pub fn set_locale<T: Into<Vec<u8>>>(category: LocaleCategory, locale: T) -> String {
        unsafe {
            CStr::from_ptr(gettext_raw::setlocale(category as i32,
                                                  CString::new(locale).unwrap().as_ptr()))
                .to_string_lossy()
                .into_owned()
        }
    }

}

pub use get_text::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_test() {
        set_locale(LocaleCategory::LcAll, "en_US.UTF-8");

        bind_text_domain("hellorust", "/usr/local/share/locale");
        text_domain("hellorust");

        assert_eq!("Hello, world!", get_text("Hello, world!"));
    }
}
