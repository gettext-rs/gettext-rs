extern crate ctest;

use std::env;
use std::process::{self, Command};
use std::path::{Path, PathBuf};

fn main() {
    let mut cfg = ctest::TestGenerator::new();

    cfg.header("locale.h");
    cfg.header("libintl.h");
    cfg.generate("../gettext-sys/lib.rs", "all.rs");
}
