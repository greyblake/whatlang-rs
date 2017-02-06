extern crate whatlang;
extern crate rustc_serialize;

use whatlang::detect_lang;
use whatlang::Lang;
use whatlang::Query;

use rustc_serialize::json;
use std::collections::HashMap;


#[test]
fn test_with_multiple_examples() {
    let example_data = include_str!("examples.json");

    let examples: HashMap<String, String> = json::decode(example_data).unwrap();

    for (lang_code, text) in examples {
        print!("Test {} ... ", lang_code);

        let lang = Lang::from_code(lang_code).expect("Unknown language code");
        let query = Query::new(&text);
        let result = detect_lang(query).unwrap();
        assert_eq!(result.lang, lang);

        println!("OK");

    }
}

