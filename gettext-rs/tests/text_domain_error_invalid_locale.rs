extern crate gettextrs;

use gettextrs::{TextDomain, TextDomainError};

fn main() {
    let text_domain = TextDomain::new("test").locale("(°_°)");
    let init_result = unsafe {
        // Safety: init() is safe to call because at this point, the program is single-threaded
        text_domain.init()
    };
    match init_result {
        Err(TextDomainError::InvalidLocale(message)) => assert_eq!(message, "(°_°)"),
        _ => panic!(),
    };
}
