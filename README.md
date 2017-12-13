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

## Manual configuration

gettext.rs's build script will by default build its own version of gettext and
statically link against that. If that is not what you want the build script can
be configured via environment variables:

  GETTEXT_SYSTEM - If specified gettext-sys uses the gettext that is part of glibc. This only works on linux

  GETTEXT_DIR - If specified, a directory that will be used to find gettext installation. It's expected that under this directory the include folder has header files, the bin folder has gettext binary and a lib folder has the runtime libraries.

  GETTEXT_LIB_DIR - If specified, a directory that will be used to find gettext libraries. Overrides the lib folder implied by GETTEXT_DIR (if specified).

  GETTEXT_INCLUDE_DIR - If specified, a directory that will be used to find gettext header files. Overrides the include folder implied by GETTEXT_DIR (if specified).

  GETTEXT_BIN_DIR - If specified, a directory that will be used to find gettext binaries. Overrides the bin folder implied by GETTEXT_DIR (if specified).
  
  GETTEXT_STATIC - If specified, gettext libraries will be statically rather than dynamically linked.

For target-specific configuration, each of these environment variables can be prefixed by an upper-cased target, for example, X86_64_UNKNOWN_LINUX_GNU_GETTEXT_DIR. This can be useful in cross compilation contexts.
