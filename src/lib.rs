//! # gettext C library FFI binding for Rust

//! Usage:
//!
//!```
//! use gettext_rs::*;
//! setlocale(LocaleCategory::LC_ALL, "en_US.UTF-8");
//! bindtextdomain("hellorust", "/usr/local/share/locale");
//! textdomain("hellorust");
//! println!("Translated: {}", gettext("Hello, world!"));
//!```

/// Raw FFI interface for gettext library
mod gettext_raw {
	use std::os::raw::{c_char, c_int};

	extern {
	    pub fn gettext(s: *const c_char) -> *const c_char;
	    pub fn bindtextdomain(domain : *const c_char, dir : *const c_char) -> *const c_char;
	    pub fn textdomain(domain : *const c_char) -> *const c_char;
	    pub fn setlocale(category: c_int, locale: *const c_char) -> *const c_char;
	}
}

/// Safe wrapper for gettext library
mod gettext {
	use gettext_raw;
	use std::ffi::CString;
	use std::ffi::CStr;

	/// Locale category enum ported from locale.h
	pub enum LocaleCategory {
		LC_CTYPE               = 0,
		LC_NUMERIC             = 1,
		LC_TIME                = 2,
		LC_COLLATE             = 3,
		LC_MONETARY            = 4,
		LC_MESSAGES            = 5,
		LC_ALL                 = 6,
		LC_PAPER               = 7,
		LC_NAME                = 8,
		LC_ADDRESS             = 9,
		LC_TELEPHONE           = 10,
		LC_MEASUREMENT         = 11,
		LC_IDENTIFICATION      = 12		
	}


	/// Translate msgid to localized message	
	pub fn gettext<T: Into<Vec<u8>>>(s:T) -> String {
		unsafe {
			CStr::from_ptr(gettext_raw::gettext(CString::new(s).unwrap().as_ptr())).to_string_lossy().into_owned()
		}
	}

	/// Switch to specific text domain
	pub fn textdomain<T: Into<Vec<u8>>>(domain:T) -> String {
		unsafe {
			CStr::from_ptr(gettext_raw::textdomain(CString::new(domain).unwrap().as_ptr())).to_string_lossy().into_owned()
		}
	}
	
	/// Bind text domain to some directory containing gettext MO files
	pub fn bindtextdomain<T: Into<Vec<u8>>>(domian:T, dir:T) -> String {
                unsafe {
                	CStr::from_ptr(gettext_raw::bindtextdomain(CString::new(domian).unwrap().as_ptr(), CString::new(dir).unwrap().as_ptr())).to_string_lossy().into_owned()
		}
        }

	/// Set current locale for translations
	pub fn setlocale<T: Into<Vec<u8>>>(category: LocaleCategory, locale:T) -> String {
                unsafe {
                        CStr::from_ptr(gettext_raw::setlocale(category as i32, CString::new(locale).unwrap().as_ptr())).to_string_lossy().into_owned()
                }
        }

}

pub use gettext::*;

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn smoke_test() {
	    setlocale(LocaleCategory::LC_ALL, "en_US.UTF-8");
	    bindtextdomain("hellorust", "/usr/local/share/locale");
	    textdomain("hellorust");
	    assert_eq!("Hello, world!", gettext("Hello, world!"));
	}
}
