extern crate gettextrs;

use gettextrs::{TextDomain, TextDomainError};

fn main() {
    match TextDomain::new("test").locale("(°_°)").init().err() {
        Some(TextDomainError::InvalidLocale(message)) => assert_eq!(message, "(°_°)"),
        _ => panic!(),
    };
}
