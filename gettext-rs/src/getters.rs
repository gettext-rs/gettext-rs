//! Query gettext configuration.
//!
//! There are just a few settings in gettext. The only required one is the message domain, set
//! using [`textdomain`][::textdomain]; the other two are the path where translations are searched
//! for, and the encoding to which the messages should be converted.
//!
//! The underlying C API uses the same functions both as setters and as getters: to get the current
//! value, you just pass `NULL` as an argument. This is ergonomic in C, but not in Rust: wrapping
//! everything in `Option`s is a tad ugly. That's why this crate provides getters as separate
//! functions. They're in a module of their own to prevent them from clashing with any functions
//! that upstream might add in the future.

extern crate gettext_sys as ffi;

use std::ffi::{CStr, CString};
use std::io;
use std::path::PathBuf;
use std::ptr;

/// Get currently set message domain.
///
/// If you want to *set* the domain, rather than getting its current value, use
/// [`textdomain`][::textdomain].
pub fn current_textdomain() -> Result<Vec<u8>, io::Error> {
    unsafe {
        let result = ffi::textdomain(ptr::null());
        if result.is_null() {
            Err(io::Error::last_os_error())
        } else {
            Ok(CStr::from_ptr(result).to_bytes().to_owned())
        }
    }
}

/// Get base directory for the given domain.
///
/// If you want to *set* the directory, rather than querying its current value, use
/// [`bindtextdomain`][::bindtextdomain].
///
/// # Panics
///
/// Panics if `domain` contains an internal 0 byte, as such values can't be passed to the gettext's
/// C API.
pub fn domain_directory<T: Into<Vec<u8>>>(domain: T) -> Result<PathBuf, io::Error> {
    let domain = CString::new(domain).expect("`domain` contains an internal 0 byte");

    #[cfg(windows)]
    {
        use std::ffi::OsString;
        use std::os::windows::ffi::OsStringExt;

        unsafe {
            let mut ptr = ffi::wbindtextdomain(domain.as_ptr(), ptr::null());
            if ptr.is_null() {
                Err(io::Error::last_os_error())
            } else {
                let mut result = vec![];
                while *ptr != 0_u16 {
                    result.push(*ptr);
                    ptr = ptr.offset(1);
                }
                Ok(PathBuf::from(OsString::from_wide(&result)))
            }
        }
    }

    #[cfg(not(windows))]
    {
        use std::ffi::OsString;
        use std::os::unix::ffi::OsStringExt;

        unsafe {
            let result = ffi::bindtextdomain(domain.as_ptr(), ptr::null());
            if result.is_null() {
                Err(io::Error::last_os_error())
            } else {
                let result = CStr::from_ptr(result);
                Ok(PathBuf::from(OsString::from_vec(result.to_bytes().to_vec())))
            }
        }
    }
}

/// Get encoding of translated messages for given domain.
///
/// Returns `None` if encoding is not set.
///
/// If you want to *set* an encoding, rather than get the current one, use
/// [`bind_textdomain_codeset`][::bind_textdomain_codeset].
///
/// # Panics
///
/// Panics if:
/// * `domain` contains an internal 0 byte, as such values can't be passed to the gettext's C API;
/// * the result is not in UTF-8 (which shouldn't happen as the results should always be ASCII, as
///     they're just codeset names).
pub fn textdomain_codeset<T: Into<Vec<u8>>>(domain: T) -> Result<Option<String>, io::Error> {
    let domain = CString::new(domain).expect("`domain` contains an internal 0 byte");
    unsafe {
        let result = ffi::bind_textdomain_codeset(domain.as_ptr(), ptr::null());
        if result.is_null() {
            let error = io::Error::last_os_error();
            if let Some(0) = error.raw_os_error() {
                return Ok(None)
            } else {
                return Err(error)
            }
        } else {
            let result =
                CStr::from_ptr(result)
                .to_str()
                .expect("`bind_textdomain_codeset()` returned non-UTF-8 string")
                .to_owned();
            Ok(Some(result))
        }
    }
}
