extern crate whatlang;
extern crate serde_json;

use whatlang::{detect, Lang, Script};

use std::collections::HashMap;

#[test]
fn test_with_multiple_examples() {
    let example_data = include_str!("examples.json");

    let examples: HashMap<String, String> = serde_json::from_str(example_data).unwrap();

    for (lang_code, text) in examples {
        print!("Test {} ... ", lang_code);

        let lang = Lang::from_code(lang_code).expect("Unknown language code");
        let info = detect(&text).unwrap();
        assert_eq!(info.lang(), lang);
        assert!(info.is_reliable());

        println!("OK");
    }
}

#[test]
fn test_with_russian_text() {
    let text = r#"
        Мой дядя самых честных правил,
        Когда не в шутку занемог,
        Он уважать себя заставил
        И лучше выдумать не мог.
    "#;

    let info = detect(text).unwrap();
    assert!(info.is_reliable());
    assert_eq!(info.script(), Script::Cyrillic);
    assert_eq!(info.script().name(), "Cyrillic");
    assert_eq!(info.lang(), Lang::Rus);
    assert_eq!(info.lang().code(), "rus");
    assert_eq!(info.lang().eng_name(), "Russian");
    assert_eq!(info.lang().name(), "Русский");
}
