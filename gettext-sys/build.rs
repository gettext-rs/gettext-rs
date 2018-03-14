extern crate cc;

use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::io::ErrorKind;

fn env(name: &str) -> Option<String> {
    let prefix = env::var("TARGET").unwrap().to_uppercase().replace("-", "_");
    let prefixed = format!("{}_{}", prefix, name);
    println!("cargo:rerun-if-env-changed={}", prefixed);

    if let Ok(var) = env::var(&prefixed) {
        return Some(var);
    }

    println!("cargo:rerun-if-env-changed={}", name);
    env::var(name).ok()
}

fn get_windows_gnu_root() -> String {
    // attempt to find the installation directory for the gnu distribution
    env("MSYSTEM_PREFIX").or_else(||
        env("MINGW_PREFIX")
    ).or_else(|| {
        // AppVeyor env doesn't declare any usable prefix
        let arch = if env::var("TARGET").unwrap().contains("x86_64") {
            "64"
        } else {
            "32"
        };
        let root_test = PathBuf::from(format!("C:/msys64/mingw{}", arch));
        if root_test.is_dir() {
            Some(root_test.to_str().unwrap().to_owned())
        } else {
            None
        }
    }).unwrap_or_else(||
        fail("Failed to get gnu installation root dir")
    )
}

fn main() {
    let target = env::var("TARGET").unwrap();

    if env("GETTEXT_SYSTEM").is_some() {
        if target.contains("linux") && target.contains("-gnu") {
            // intl is part of glibc
            return;
        } else if target.contains("windows") && target.contains("-gnu") {
            // gettext doesn't come with a pkg-config file
            let gnu_root = get_windows_gnu_root();
            println!("cargo:rustc-link-search=native={}/lib", &gnu_root);
            println!("cargo:rustc-link-search=native={}/../usr/lib", &gnu_root);
            println!("cargo:rustc-link-lib=dylib=intl");
            // FIXME: should pthread support be optional?
            // It is needed by `cargo test` while generating doc
            println!("cargo:rustc-link-lib=dylib=pthread");
            println!("cargo:include={}/../usr/include", &gnu_root);
            return;
        }
        // else can't use system gettext on this target
    }

    if target.contains("apple-darwin") {
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=dylib=iconv");
    }

    if let Some(gettext_dir) = env("GETTEXT_DIR") {
        println!("cargo:root={}", gettext_dir);
        if let Some(bin) = env("GETTEXT_BIN_DIR") {
            println!("cargo:bin={}", bin);
        } else {
            println!("cargo:bin={}/bin", gettext_dir);
        }

        if let Some(lib) = env("GETTEXT_LIB_DIR") {
            println!("cargo:lib={}", lib);
            println!("cargo:rustc-link-search=native={}", lib);
        } else {
            println!("cargo:lib={}/lib", gettext_dir);
            println!("cargo:rustc-link-search=native={}/lib", gettext_dir);
        }

        if let Some(include) = env("GETTEXT_INCLUDE_DIR") {
            println!("cargo:include={}", include);
        } else {
            println!("cargo:include={}/include", gettext_dir);
        }

        if env("GETTEXT_STATIC").is_some() {
            println!("cargo:rustc-link-lib=static=intl");
        } else {
            println!("cargo:rustc-link-lib=dylib=intl");
        }

        return
    } else if let (Some(bin), Some(lib), Some(include)) =
        (env("GETTEXT_BIN_DIR"), env("GETTEXT_LIB_DIR"), env("GETTEXT_INCLUDE_DIR"))
    {
        println!("cargo:rustc-link-search=native={}", lib);
        if env("GETTEXT_STATIC").is_some() {
            println!("cargo:rustc-link-lib=static=intl");
        } else {
            println!("cargo:rustc-link-lib=dylib=intl");
        }

        println!("cargo:bin={}", bin);
        println!("cargo:lib={}", lib);
        println!("cargo:include={}", include);
        return
    }

    let host = env::var("HOST").unwrap();
    let src = env::current_dir().unwrap();
    let dst = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    let cfg = cc::Build::new();
    let compiler = cfg.get_compiler();

    let _ = fs::create_dir(&dst.join("build"));
    let _ = fs::create_dir(&dst.join("gettext"));

    let mut cflags = OsString::new();
    for arg in compiler.args() {
        cflags.push(arg);
        cflags.push(" ");
    }

    if !dst.join("gettext/configure").is_file() {
        let mut cmd = Command::new("tar");
        cmd.current_dir(&dst.join("gettext"))
           .arg("xvf")
           .arg(&src.join("gettext-0.19.8.1.tar.gz"))
           .arg("--strip-components")
           .arg("1");
        run(&mut cmd, "tar");
    }

    let mut cmd = Command::new("sh");
    cmd.env("CC", compiler.path())
       .env("CFLAGS", cflags)
       .env("LD", &which("ld").unwrap())
       .env("VERBOSE", "1")
       .current_dir(&dst.join("build"))
       .arg(&dst.join("gettext/configure"));

    cmd.arg("--without-emacs");
    cmd.arg("--disable-java");
    cmd.arg("--disable-csharp");
    cmd.arg("--disable-c++");
    cmd.arg("--disable-shared");
    cmd.arg("--enable-static");
    cmd.arg("--enable-fast-install");
    cmd.arg("--with-included-gettext");
    cmd.arg("--with-included-glib");
    cmd.arg("--with-included-libcroco");
    cmd.arg("--with-included-libunistring");

    cmd.arg(format!("--prefix={}", dst.to_str().unwrap().to_string()));

    if target != host &&
       (!target.contains("windows") || !host.contains("windows")) {
        // NOTE GNU terminology
        // BUILD = machine where we are (cross) compiling curl
        // HOST = machine where the compiled curl will be used
        // TARGET = only relevant when compiling compilers
        if target.contains("windows") {
            // curl's configure can't parse `-windows-` triples when used
            // as `--host`s. In those cases we use this combination of
            // `host` and `target` that appears to do the right thing.
            cmd.arg(format!("--host={}", host));
            cmd.arg(format!("--target={}", target));
        } else {
            cmd.arg(format!("--build={}", host));
            cmd.arg(format!("--host={}", target));
        }
    }
    run(&mut cmd, "sh");
    run(make()
                .arg(&format!("-j{}", env::var("NUM_JOBS").unwrap()))
                .current_dir(&dst.join("build")), "make");
    run(make()
                .arg("install")
                .current_dir(&dst.join("build")), "make");

    println!("cargo:rustc-link-lib=static=intl");
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:lib={}/lib", dst.display());
    println!("cargo:include={}/include", dst.display());
    println!("cargo:bin={}/bin", dst.display());
    println!("cargo:root={}", dst.display());
}

fn run(cmd: &mut Command, program: &str) {
    println!("running: {:?}", cmd);
    let status = match cmd.status() {
        Ok(status) => status,
        Err(ref e) if e.kind() == ErrorKind::NotFound => {
            fail(&format!("failed to execute command: {}\nis `{}` not installed?",
                          e, program));
        }
        Err(e) => fail(&format!("failed to execute command: {}", e)),
    };
    if !status.success() {
        fail(&format!("command did not execute successfully, got: {}", status));
    }
}

fn fail(s: &str) -> ! {
    panic!("\n{}\n\nbuild script failed, must exit now", s)
}

fn which(cmd: &str) -> Option<PathBuf> {
    let cmd = format!("{}{}", cmd, env::consts::EXE_SUFFIX);
    let paths = env::var_os("PATH").unwrap();
    env::split_paths(&paths).map(|p| p.join(&cmd)).find(|p| {
        fs::metadata(p).is_ok()
    })
}

fn make() -> Command {
    let cmd = if cfg!(target_os = "freebsd") {"gmake"} else {"make"};
    let mut cmd = Command::new(cmd);
    // We're using the MSYS make which doesn't work with the mingw32-make-style
    // MAKEFLAGS, so remove that from the env if present.
    if cfg!(windows) {
        cmd.env_remove("MAKEFLAGS").env_remove("MFLAGS");
    }
    return cmd
}
