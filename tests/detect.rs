extern crate serde_json;
extern crate whatlang;

use whatlang::{detect, detect_lang, Lang, Script};

use std::collections::HashMap;

#[test]
fn test_with_multiple_examples() {
    let example_data = include_str!("examples.json");

    let examples: HashMap<String, String> = serde_json::from_str(example_data).unwrap();

    for (lang_code, text) in examples {
        print!("Test {} ... ", lang_code);

        let lang = Lang::from_code(lang_code).expect("Unknown language code");
        let detected_lang = detect_lang(&text).unwrap();
        assert_eq!(detected_lang, lang);
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
    assert_eq!(info.script(), Script::Cyrillic);
    assert_eq!(info.script().name(), "Cyrillic");
    assert_eq!(info.lang(), Lang::Rus);
    assert_eq!(info.lang().code(), "rus");
    assert_eq!(info.lang().eng_name(), "Russian");
    assert_eq!(info.lang().name(), "Русский");

    assert_eq!(info.confidence(), 1.0);
    assert!(info.is_reliable());
}

#[test]
fn test_japanese_with_mandarin_chars() {
    let text = r#"
        この間、川越城や松井田城などの諸城を拡張・改修 河越城の三の丸と八幡郭など拡張、松井田城の大道寺郭構築など
    "#;

    let info = detect(text).unwrap();
    assert_eq!(info.script(), Script::Mandarin);
    assert_eq!(info.lang(), Lang::Jpn);
    assert!(info.is_reliable());
}
