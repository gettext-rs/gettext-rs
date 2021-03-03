extern crate gettextrs;
#[macro_use]
extern crate lazy_static;

use gettextrs::{getters::*, *};
use std::sync::Mutex;

lazy_static! {
    // "Current text domain" is a global resource which all tests modify. This mutex serializes
    // access to that resource.
    static ref TEXTDOMAIN_MUTEX: Mutex<()> = Mutex::new(());
}

#[test]
fn test_current_textdomain() {
    let _lock = TEXTDOMAIN_MUTEX.lock().unwrap();

    textdomain("just_testing").unwrap();
    assert_eq!(current_textdomain().unwrap(), "just_testing".as_bytes());

    textdomain("test_current_textdomain").unwrap();
    assert_eq!(
        current_textdomain().unwrap(),
        "test_current_textdomain".as_bytes()
    );
}

#[test]
fn test_domain_directory() {
    use std::path::PathBuf;

    static TEXTDOMAIN: &'static str = "test_domain_directory";

    {
        let _lock = TEXTDOMAIN_MUTEX.lock().unwrap();
        textdomain(TEXTDOMAIN).unwrap();
    }

    bindtextdomain(TEXTDOMAIN, "/tmp").unwrap();
    assert_eq!(domain_directory(TEXTDOMAIN).unwrap(), PathBuf::from("/tmp"));

    let path = "/some/nonexistent path (hopefully)";
    bindtextdomain(TEXTDOMAIN, path).unwrap();
    assert_eq!(domain_directory(TEXTDOMAIN).unwrap(), PathBuf::from(path));
}

#[test]
fn test_textdomain_codeset() {
    static TEXTDOMAIN: &'static str = "test_textdomain_codeset";

    {
        let _lock = TEXTDOMAIN_MUTEX.lock().unwrap();
        textdomain(TEXTDOMAIN).unwrap();
    }

    bind_textdomain_codeset(TEXTDOMAIN, "C").unwrap();
    assert_eq!(
        textdomain_codeset(TEXTDOMAIN).unwrap(),
        Some("C".to_string())
    );

    bind_textdomain_codeset(TEXTDOMAIN, "UTF-8").unwrap();
    assert_eq!(
        textdomain_codeset(TEXTDOMAIN).unwrap(),
        Some("UTF-8".to_string())
    );
}
