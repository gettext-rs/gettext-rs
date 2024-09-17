extern crate ctest2;

use std::env;
use std::path::{Path, PathBuf};
use std::process::{self, Command};

fn main() {
    let target = env::var("TARGET").unwrap();
    let host = env::var("HOST").unwrap();
    let is_cross = target != host;

    // Set the cross_compiling flag before the tests
    if is_cross {
        println!("cargo:rustc-cfg=cross_compiling");
        println!("cargo::rustc-check-cfg=cfg(cross_compiling)");
    }

    let mut cfg = ctest2::TestGenerator::new();

    if target.contains("freebsd") {
        cfg.include("/usr/local/include");
    }

    if let Ok(out) = env::var("DEP_GETTEXT_INCLUDE") {
        cfg.include(&out);
    }

    cfg.header("locale.h");
    cfg.header("libintl.h");

    // Skip ptr check because the symbol name is different between glibc
    // implementation and static lib.
    // eg. gettext is libintl_gettext in static lib
    if env::var_os("GETTEXT_SYSTEM").is_none() || target.contains("windows") {
        println!("Skipping ptr check");
        cfg.skip_fn_ptrcheck(|_| true);
    }

    cfg.generate("../gettext-sys/lib.rs", "all.rs");

    // Check for the existence of the gettext binary
    let cmd = if let Some(bin) = env::var_os("DEP_GETTEXT_BIN") {
        Path::new(&bin).join("gettext")
    } else {
        PathBuf::from("gettext")
    };

    if !cmd.exists() {
        println!(
            "cargo:warning=Gettext binary not found at {}",
            cmd.display()
        );
        process::exit(1);
    }

    // Only run the binary if not cross-compiling
    if !is_cross {
        let c = Command::new(&cmd).arg("--version").spawn();
        if let Ok(mut child) = c {
            assert!(child.wait().unwrap().success());
        } else {
            println!("Could not run {}", cmd.display());
            process::exit(1);
        }
    } else {
        println!("cargo:warning=Cross-compiling detected. Gettext binary found but not executed.");
    }
}
