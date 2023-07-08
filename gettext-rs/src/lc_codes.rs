cfg_if::cfg_if! {
    if #[cfg(all(unix,
                 any(target_os = "linux",
                     target_os = "l4re",
                     target_os = "android",
                     target_os = "emscripten"),
                 any(target_env = "ohos",
                     target_env = "gnu",
                     target_os = "android")))] {
        /// Locale category enum ported from locale.h.
        #[derive(Debug, PartialEq, Clone, Copy)]
        pub enum LocaleCategory {
            /// Character classification and case conversion.
            LcCType = ffi::LC_CTYPE as isize,
            /// Non-monetary numeric formats.
            LcNumeric = ffi::LC_NUMERIC as isize,
            /// Date and time formats.
            LcTime = ffi::LC_TIME as isize,
            /// Collation order.
            LcCollate = ffi::LC_COLLATE as isize,
            /// Monetary formats.
            LcMonetary = ffi::LC_MONETARY as isize,
            /// Formats of informative and diagnostic messages and interactive responses.
            /// `setlocale()` will treat this as `LcAll` on non-POSIX systems.
            LcMessages = ffi::LC_MESSAGES as isize,
            /// For all.
            LcAll = ffi::LC_ALL as isize,
            /// Paper size.
            /// Only supported on Linux-like operating systems.
            LcPaper = ffi::LC_PAPER as isize,
            /// Name formats.
            /// Only supported on Linux-like operating systems.
            LcName = ffi::LC_NAME as isize,
            /// Address formats and location information.
            /// Only supported on Linux-like operating systems.
            LcAddress = ffi::LC_ADDRESS as isize,
            /// Telephone number formats.
            /// Only supported on Linux-like operating systems.
            LcTelephone = ffi::LC_TELEPHONE as isize,
            /// Measurement units (Metric or Other).
            /// Only supported on Linux-like operating systems.
            LcMeasurement = ffi::LC_MEASUREMENT as isize,
            /// Metadata about the locale information.
            /// Only supported on Linux-like operating systems.
            LcIdentification = ffi::LC_IDENTIFICATION as isize,
        }
    } else {
        /// Locale category enum ported from locale.h.
        #[derive(Debug, PartialEq, Clone, Copy)]
        pub enum LocaleCategory {
            /// Character classification and case conversion.
            LcCType = ffi::LC_CTYPE as isize,
            /// Non-monetary numeric formats.
            LcNumeric = ffi::LC_NUMERIC as isize,
            /// Date and time formats.
            LcTime = ffi::LC_TIME as isize,
            /// Collation order.
            LcCollate = ffi::LC_COLLATE as isize,
            /// Monetary formats.
            LcMonetary = ffi::LC_MONETARY as isize,
            /// Formats of informative and diagnostic messages and interactive responses.
            /// `setlocale` will treat this as `LcAll` on non-POSIX systems.
            LcMessages = ffi::LC_MESSAGES as isize,
            /// For all.
            LcAll = ffi::LC_ALL as isize,
        }
    }
}
