extern crate whatlang;
extern crate rustc_serialize;

use whatlang::detect_lang;
use whatlang::Lang;
use whatlang::Query;

use std::fs::File;
use std::io::*;

use rustc_serialize::json;
use std::collections::HashMap;


#[test]
fn test_with_multiple_examples() {
    let file_path = "./tests/examples.json";

    let mut file = File::open(file_path).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let examples: HashMap<String, String> = json::decode(data.as_ref()).unwrap();

    for (lang_code, text) in examples {
        print!("Test {} ... ", lang_code);

        let lang = Lang::from_code(lang_code).expect("Unknown language code");
        let query = Query::new(&text);
        let result = detect_lang(query).unwrap();
        assert_eq!(result.lang, lang);

        println!("OK");

    }
}

