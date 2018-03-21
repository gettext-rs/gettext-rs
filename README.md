# gettext-rs

GNU [Gettext](https://www.gnu.org/software/gettext/) FFI binding for Rust

[![https://travis-ci.org/Koka/gettext-rs](https://travis-ci.org/Koka/gettext-rs.svg?branch=master)](https://travis-ci.org/Koka/gettext-rs)
[![https://crates.io/crates/gettext-rs](https://meritbadge.herokuapp.com/gettext-rs?v2)](https://crates.io/crates/gettext-rs)

Docs are available [here](http://koka.github.io/gettext-rs/gettextrs/)

Usage:

```rust
 use gettextrs::*;
 setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
 bindtextdomain("hellorust", "/usr/local/share/locale");
 textdomain("hellorust");
 println!("Translated: {}", gettext("Hello, world!"));
 println!("Singular: {}", ngettext("One thing", "Multiple things", 1));
 println!("Plural: {}", ngettext("One thing", "Multiple things", 2));
```

Alternatively, you can initialize the locale and text domain using the `TextDomain` builder.
By default, a translation of the specified text domain in current language is searched in
the system's data paths. See `TextDomain`'s documentation for other options.

```rust
use gettextrs::TextDomain;

TextDomain::new("hellorust")
           .init()
           .unwrap();
```

## Manual configuration

gettext.rs's build script will by default build its own version of gettext and
statically link against that. If that is not what you want the build script can
be configured via environment variables:

  GETTEXT_SYSTEM - If specified gettext-sys uses the gettext that is part of glibc. This only works on linux and Windows + GNU (e.g. [MSYS2](http://www.msys2.org/)). On Windows, you will need to install `gettext-devel`.
  You can also activate the `gettext-system` feature from your `Cargo.toml` configuration:
  ``` toml
  gettext-rs = { git = "https://github.com/Koka/gettext-rs", features = ["gettext-system"] }
  ```

  GETTEXT_DIR - If specified, a directory that will be used to find gettext installation. It's expected that under this directory the include folder has header files, the bin folder has gettext binary and a lib folder has the runtime libraries.

  GETTEXT_LIB_DIR - If specified, a directory that will be used to find gettext libraries. Overrides the lib folder implied by GETTEXT_DIR (if specified).

  GETTEXT_INCLUDE_DIR - If specified, a directory that will be used to find gettext header files. Overrides the include folder implied by GETTEXT_DIR (if specified).

  GETTEXT_BIN_DIR - If specified, a directory that will be used to find gettext binaries. Overrides the bin folder implied by GETTEXT_DIR (if specified).
  
  GETTEXT_STATIC - If specified, gettext libraries will be statically rather than dynamically linked.

For target-specific configuration, each of these environment variables can be prefixed by an upper-cased target, for example, X86_64_UNKNOWN_LINUX_GNU_GETTEXT_DIR. This can be useful in cross compilation contexts.

Note: on Windows + GNU, if you want to build `gettext-rs` with its own static
version of `getttext`, install the following packages first:
```
pacman --noconfirm -S base-devel mingw-w64-x86_64-gcc libxml2-devel tar
```

The build is quite long. You can speed things up by setting (e.g. for 4 cores):
```
export NUM_JOBS=5
```

This doesn't work on AppVeyor ATM. Use `SET GETTEXT_SYSTEM=true` instead.
