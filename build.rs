extern crate csv;
extern crate serde_json;
extern crate serde;
#[macro_use]
extern crate serde_derive;

// We need AsciiExt for Rust versions below 1.23.
// For Rust 1.23 and higher AsciiExt is no longer needed, so the compiler produces
// a warning, which we suppress.
// For more details please read the Rust 1.23 release note: https://blog.rust-lang.org/2018/01/04/Rust-1.23.html
#[allow(unused_imports)]
use std::ascii::AsciiExt;

use std::io::{Write, BufReader, BufWriter};
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::rc::Rc;
use std::env;

const DATA_PATH: &'static str = "misc/data.json";
const SUPPORTED_LANG_PATH: &'static str = "misc/supported_languages.csv";

const TRIGRAPH_COUNT: usize = 300;

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct SupportedLang {
    code: String,
    eng_name: String,
    name: String,
    native_speakers: Option<f64>,
}

#[derive(Debug, Clone, PartialEq)]
struct Lang<'a> {
    info: &'a SupportedLang,
    script: String,
    trigraphs: Vec<String>,
}

fn main() {
    println!("cargo:rerun-if-changed={}", DATA_PATH);
    println!("cargo:rerun-if-changed={}", SUPPORTED_LANG_PATH);
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("lang.rs");
    let mut lang_def = BufWriter::new(File::create(&dest_path).unwrap());

    let data_file = BufReader::new(File::open(DATA_PATH).unwrap());
    let mut lang_reader = csv::ReaderBuilder::new().flexible(true).from_path(SUPPORTED_LANG_PATH).unwrap();

    let mut supported_langs: Vec<SupportedLang> = lang_reader.deserialize().map(Result::unwrap).collect();
    supported_langs.sort_by(|left, right| left.code.cmp(&right.code));

    let supported_lang_codes: HashMap<&str, &SupportedLang> = supported_langs.iter().map(|lang| (&*lang.code, lang)).collect();

    let capitalized_codes: HashMap<&str, String> = supported_langs.iter().map(|lang| (&*lang.code, capitalize_name(&lang.code))).collect();

    let lang_data: HashMap<String, HashMap<String, String>> = serde_json::from_reader(data_file).unwrap();

    let mut scripts: HashMap<String, Vec<Rc<Lang>>> = HashMap::with_capacity(lang_data.len());
    let mut all_langs: Vec<Rc<Lang>> = Vec::new();
    for (script, langs) in &lang_data {
        for (code, trigraphs) in langs {
            let info = match supported_lang_codes.get(&**code) {
                Some(info) => info,
                None => continue,
            };
            let lang = Rc::new(Lang {
                info: info,
                script: script.clone(),
                trigraphs: trigraphs.split('|').map(Into::into).collect()
            });
            if lang.trigraphs.len() != TRIGRAPH_COUNT {
                panic!("Languate {} has {} trigraphs, instead of {}", code, lang.trigraphs.len(), TRIGRAPH_COUNT);
            }

            all_langs.push(lang.clone());
            scripts.entry(script.clone()).or_insert_with(Vec::new).push(lang);
        }
    }

    writeln!(lang_def, "/// Represents a language following [ISO 639-3](https://en.wikipedia.org/wiki/ISO_639-3) standard.").unwrap();
    writeln!(lang_def, "#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]").unwrap();
    writeln!(lang_def, "pub enum Lang {{").unwrap();
    for (i, supported_lang) in supported_langs.iter().enumerate() {
        writeln!(lang_def, "  /// {} ({})", supported_lang.name, supported_lang.eng_name).unwrap();
        // Number starting at 1, to allow enum optimizations to occur (e.g. Option<Lang> will be smaller)
        writeln!(lang_def, "  {} = {},", capitalize_name(&supported_lang.code), i + 1).unwrap();
    }
    writeln!(lang_def, "}}").unwrap();
    lang_def.flush().unwrap();

    writeln!(lang_def, "#[inline] fn lang_from_code<S: Into<String>>(code: S) -> Option<Lang> {{").unwrap();
    writeln!(lang_def, "  match code.into().to_lowercase().as_ref() {{").unwrap();
    for supported_lang in &supported_langs {
        writeln!(lang_def, "    \"{}\" => Some(Lang::{}),", supported_lang.code, capitalized_codes[&*supported_lang.code]).unwrap();
    }
    writeln!(lang_def, "    _ => None,").unwrap();
    writeln!(lang_def, "  }}").unwrap();
    writeln!(lang_def, "}}").unwrap();

    writeln!(lang_def, "#[inline] fn lang_to_code(lang: Lang) -> &'static str {{").unwrap();
    writeln!(lang_def, "  match lang {{").unwrap();
    for supported_lang in &supported_langs {
        writeln!(lang_def, "    Lang::{} => \"{}\",", capitalized_codes[&*supported_lang.code], supported_lang.code).unwrap();
    }
    writeln!(lang_def, "  }}").unwrap();
    writeln!(lang_def, "}}").unwrap();

    writeln!(lang_def, "#[inline] fn lang_to_name(lang: Lang) -> &'static str {{").unwrap();
    writeln!(lang_def, "  match lang {{").unwrap();
    for supported_lang in &supported_langs {
        writeln!(lang_def, "    Lang::{} => \"{}\",", capitalized_codes[&*supported_lang.code], supported_lang.name).unwrap();
    }
    writeln!(lang_def, "  }}").unwrap();
    writeln!(lang_def, "}}").unwrap();

    writeln!(lang_def, "#[inline] fn lang_to_eng_name(lang: Lang) -> &'static str {{").unwrap();
    writeln!(lang_def, "  match lang {{").unwrap();
    for supported_lang in &supported_langs {
        writeln!(lang_def, "    Lang::{} => \"{}\",", capitalized_codes[&*supported_lang.code], supported_lang.eng_name).unwrap();
    }
    writeln!(lang_def, "  }}").unwrap();
    writeln!(lang_def, "}}").unwrap();

    for (script, script_langs) in &scripts {
        writeln!(lang_def, "/// Languages for script '{}'", script).unwrap();
        writeln!(lang_def, "pub static {}_LANGS: LangProfileList = &[", script.to_ascii_uppercase()).unwrap();
        for lang in script_langs {
            writeln!(lang_def, "  (Lang::{}, &{:?}),", capitalized_codes[&*lang.info.code], lang.trigraphs).unwrap();
        }
        writeln!(lang_def, "];").unwrap();
    }
}

fn capitalize_name(name: &str) -> String {
    let mut name = String::from(name);
    name[0..1].make_ascii_uppercase();
    name
}
