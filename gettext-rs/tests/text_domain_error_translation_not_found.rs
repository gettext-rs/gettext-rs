extern crate gettextrs;

use gettextrs::{TextDomain, TextDomainError};

fn main() {
    let text_domain = TextDomain::new("0_0").locale("en_US");
    let init_result = unsafe {
        // Safety: init() is safe to call because at this point, the program is single-threaded
        text_domain.init()
    };
    match init_result {
        Err(TextDomainError::TranslationNotFound(message)) => assert_eq!(message, "en"),
        _ => panic!(),
    };
}
