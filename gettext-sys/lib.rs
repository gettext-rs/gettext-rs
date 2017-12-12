use std::os::raw::{c_char, c_int, c_uint};

extern "C" {
    pub fn gettext(s: *const c_char) -> *const c_char;
    pub fn dgettext(domain: *const c_char, s: *const c_char) -> *const c_char;
    pub fn dcgettext(domain: *const c_char, s: *const c_char, category: c_int) -> *const c_char;

    pub fn ngettext(s1: *const c_char, s2: *const c_char, n: c_uint) -> *const c_char;
    pub fn dngettext(domain: *const c_char, s1: *const c_char, s2: *const c_char, n: c_uint) -> *const c_char;
    pub fn dcngettext(domain: *const c_char, s1: *const c_char, s2: *const c_char, n: c_uint, category: c_int) -> *const c_char;

    pub fn bindtextdomain(domain: *const c_char, dir: *const c_char) -> *const c_char;
    pub fn textdomain(domain: *const c_char) -> *const c_char;

    pub fn bind_textdomain_codeset(domain: *const c_char, codeset: *const c_char) -> *const c_char;

    pub fn setlocale(category: c_int, locale: *const c_char) -> *const c_char;
}
