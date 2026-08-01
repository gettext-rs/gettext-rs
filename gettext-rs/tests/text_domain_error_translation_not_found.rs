extern crate gettextrs;

use gettextrs::{TextDomain, TextDomainError};

fn main() {
    match TextDomain::new("0_0").locale("en_US").init().err() {
        Some(TextDomainError::TranslationNotFound(message)) => assert_eq!(message, "en"),
        _ => panic!(),
    };
}
