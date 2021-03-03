# Changelog

## 0.21.0 - 2021-03-03

### Added
- A note regarding GNU gettext's LGPL license (Alexander Batischev)
- Checks for build tools required by GNU gettext (Dean Leggo)
- Bindings for `wbindtextdomain` (only available on Windows) (Alexander
    Batischev)
- Build-time dependency on `tempfile` (Alexander Batischev)

### Changed
- Bump bundled GNU gettext to 0.21 (Alexander Batischev)

### Fixed
- Build failure when a path contains spaces (Alexander Batischev)



## 0.19.9 - 2019-07-26

### Added
- Support for Windows+GNU (François Laignel)
- Support for musl libc (Rasmus Thomsen)
- `gettext-system` feature which asks the crate to use gettext that's built into
    libc (if available) (François Laignel)
- Use xz to compress the bundled GNU gettext tarball, to save space (Daniel
    García Moreno)



## 0.19.8 - 2018-05-23

Initial release (Konstantin V. Salikhov, Faizaan).
