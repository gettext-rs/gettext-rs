# gettext-rs

GNU [Gettext](https://www.gnu.org/software/gettext/) FFI binding for Rust

[![https://travis-ci.org/Koka/gettext-rs](https://travis-ci.org/Koka/gettext-rs.svg?branch=master)](https://travis-ci.org/Koka/gettext-rs)

Docs are available [here](http://koka.github.io/gettext-rs/gettext_rs/)

Usage:

```
 use gettext_rs::*;
 setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
 bindtextdomain("hellorust", "/usr/local/share/locale");
 textdomain("hellorust");
 println!("Translated: {}", gettext("Hello, world!"));
```

