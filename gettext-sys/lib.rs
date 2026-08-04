use std::os::raw::{c_char, c_int, c_ulong};

#[cfg(windows)]
#[allow(non_camel_case_types)]
type wchar_t = u16;

unsafe extern "C" {
    pub fn gettext(s: *const c_char) -> *mut c_char;
    pub fn dgettext(domain: *const c_char, s: *const c_char) -> *mut c_char;
    pub fn dcgettext(domain: *const c_char, s: *const c_char, category: c_int) -> *mut c_char;

    pub fn ngettext(s1: *const c_char, s2: *const c_char, n: c_ulong) -> *mut c_char;
    pub fn dngettext(
        domain: *const c_char,
        s1: *const c_char,
        s2: *const c_char,
        n: c_ulong,
    ) -> *mut c_char;
    pub fn dcngettext(
        domain: *const c_char,
        s1: *const c_char,
        s2: *const c_char,
        n: c_ulong,
        category: c_int,
    ) -> *mut c_char;

    pub fn bindtextdomain(domain: *const c_char, dir: *const c_char) -> *mut c_char;
    #[cfg(windows)]
    // The "wbindtextdomain" symbol is not exposed directly in the compiled
    // .DLL file when building using MinGW. See: https://github.com/Koka/gettext-rs/pull/79
    fn libintl_wbindtextdomain(domain: *const c_char, dir: *const wchar_t) -> *mut wchar_t;

    pub fn textdomain(domain: *const c_char) -> *mut c_char;

    pub fn bind_textdomain_codeset(domain: *const c_char, codeset: *const c_char) -> *mut c_char;

    /// Set the current locale.
    ///
    /// # Safety
    ///
    /// In POSIX parlance, this function is `MT-Unsafe const:locale env`, meaning that:
    ///
    /// * it non-atomically modifies the locale object which is better regarded as constant;
    /// * it accesses the program environment without any synchronization.
    ///
    /// Canonical way to ensure safety is to call `setlocale()` as early as possible, prior to
    /// starting any more threads or enabling any POSIX signals.
    ///
    /// See also [`setlocale(3)`](https://www.man7.org/linux/man-pages/man3/setlocale.3.html).
    pub fn setlocale(category: c_int, locale: *const c_char) -> *mut c_char;
}

#[cfg(windows)]
pub unsafe fn wbindtextdomain(domain: *const c_char, dir: *const wchar_t) -> *mut wchar_t {
    libintl_wbindtextdomain(domain, dir)
}
