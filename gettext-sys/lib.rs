extern crate cfg_if;
extern crate libc;

mod gettext_exports;
pub use gettext_exports::*;

pub use libc::setlocale;

pub use libc::LC_CTYPE;
pub use libc::LC_NUMERIC;
pub use libc::LC_TIME;
pub use libc::LC_COLLATE;
pub use libc::LC_MONETARY;
pub use libc::LC_ALL;

cfg_if::cfg_if!{
    if #[cfg(any(target_os = "fuchsia",
                 target_os = "solid_asp3",
                 all(unix, not(target_os = "hermit"))))] {
        pub use libc::LC_MESSAGES;
        pub const LIBC_HAS_LC_MESSAGES: bool = true;
    } else {
        pub const LC_MESSAGES: std::os::raw::c_int = 1729;
        pub const LIBC_HAS_LC_MESSAGES: bool = false;
    }
}

cfg_if::cfg_if!{
    if #[cfg(all(unix,
             any(target_os = "linux",
                 target_os = "l4re",
                 target_os = "android",
                 target_os = "emscripten"),
             any(target_env = "ohos",
                 target_env = "gnu",
                 target_os = "android")))] {
        pub use libc::LC_PAPER;
        pub use libc::LC_NAME;
        pub use libc::LC_ADDRESS;
        pub use libc::LC_TELEPHONE;
        pub use libc::LC_MEASUREMENT;
        pub use libc::LC_IDENTIFICATION;
    }
}
