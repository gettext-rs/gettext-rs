# gettext-rs

GNU Gettext FFI binding for Rust

Usage:

```
 use gettext_rs::*;
 setlocale(LocaleCategory::LC_ALL, "en_US.UTF-8");
 bindtextdomain("hellorust", "/usr/local/share/locale");
 textdomain("hellorust");
 println!("Translated: {}", gettext("Hello, world!"));
```

