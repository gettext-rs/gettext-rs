# gettext-rs

Safe bindings for GNU [gettext](https://www.gnu.org/software/gettext/). Please
see [documentation](https://docs.rs/gettext-rs) for details.

## Licensing

Even though this crate is MIT-licensed, GNU gettext itself is licensed under the
GNU LGPL. By default, this crate (through its dependency `gettext-sys`) builds
and statically links GNU gettext, and so you have to abide by LGPL. If you don't
want or can't do that, there are two ways out:

1. if you use glibc or musl libc, enable `gettext-system` feature (see below);
2. dynamically link to GNU gettext library you obtained by some other means,
   like a package manager. For details, see environment variables in
   `gettext-sys` documentation.

## Usage

(If you know how to use gettext and just want a gist of this crate's API, [skip
to the next section](#complete-api-example)).

To internationalize your program with gettext, you have to do four things:

1. wrap translatable strings into calls to appropriate gettext functions;
2. use tools to extract those strings into a so-called "PO template file";
3. translate the strings in the template, getting a so-called "message catalog"
   as a result;
4. compile the message catalog from the plain-text, human-readable PO format to
   the binary MO format, and install them into a well-known location like
   _/usr/local/share/locale_.

This crate only covers the first step, the markup. To extract messages, use
`xtr` (`cargo install xtr`). To translate, you can use desktop tools like
[Poedit][], sites like [Crowdin][], or any text editor. To compile from PO to
MO, use `msgfmt` tool from gettext-tools. The way you install files highly depend
on your distribution method, so it's not covered here either.

[Poedit]: https://poedit.net
[Crowdin]: https://crowdin.com

The best resource on gettext is [GNU gettext manual][]. This crate mimics the
API, so you should have an easy time transferring the advice of that manual to
this crate.

[GNU gettext manual]: https://www.gnu.org/software/gettext/manual/index.html

## Complete API example

```rust
use gettextrs::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    textdomain("hellorust")?;
    bindtextdomain("hellorust", "/usr/local/share/locale")?;

    // It's sufficient to call any one of those two. See "UTF-8 is required" in the
    // rustdocs.
    setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
    bind_textdomain_codeset("hellorust", "UTF-8")?;

    println!("Translated: {}", gettext("Hello, world!"));
    println!("Singular: {}", ngettext("One thing", "Multiple things", 1));
    println!("Plural: {}", ngettext("One thing", "Multiple things", 2));
    println!("With context: {}", pgettext("This is the context", "Hello, world!"));
    println!("Plural with context: {}", npgettext("This is the context", "One thing", "Multiple things", 2));

    Ok(())
}
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

## Features

- `gettext-system`: if enabled, _asks_ the crate to use the gettext
    implementation that's part of glibc or musl libc. This only works on:

    * Linux with glibc or musl libc;
    * Windows + GNU (e.g. [MSYS2](http://www.msys2.org/)) with `gettext-devel`
        installed e.g. using:

        ```
        pacman --noconfirm -S base-devel mingw-w64-x86_64-gcc libxml2-devel tar
        ```

    If none of those conditions hold, the crate will proceed to building and
    statically linking its own copy of GNU gettext!

    This enables `gettext-system` feature of the underlying `gettext-sys` crate.

## Environment variables

This crate doesn't use any. See also the documentation for the underlying
`gettext-sys` crate.
