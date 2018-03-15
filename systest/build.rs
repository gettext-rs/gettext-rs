extern crate ctest;

use std::env;
use std::process::{self, Command};
use std::path::{Path, PathBuf};

fn main() {
    let mut cfg = ctest::TestGenerator::new();

    if let Ok(out) = env::var("DEP_GETTEXT_INCLUDE") {
        cfg.include(&out);
    }

    cfg.header("locale.h");
    cfg.header("libintl.h");

    // Skip ptr check because the symbol name is different between glibc
    // implementation and static lib.
    // eg. gettext is libintl_gettext in static lib
    if env::var_os("GETTEXT_SYSTEM").is_none()
        || env::var("TARGET").unwrap().contains("windows")
    {
        println!("Skipping ptr check");
        cfg.skip_fn_ptrcheck(|_| true);
    }

    cfg.generate("../gettext-sys/lib.rs", "all.rs");

    // Check that we can find and run gettext binary
    let cmd = if let Some(bin) = env::var_os("DEP_GETTEXT_BIN") {
        Path::new(&bin).join("gettext")
    } else {
        PathBuf::from("gettext")
    };
    let c = Command::new(&cmd)
        .arg("--version")
        .spawn();
    if let Ok(mut child) = c {
        assert!(child.wait().unwrap().success());
    } else {
        println!("Could not run {}", cmd.display());
        process::exit(1);
    }
}
