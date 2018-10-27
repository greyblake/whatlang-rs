extern crate csv;
extern crate skeptic;
extern crate serde_json;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate tera;

use std::io::{Write, BufReader, BufWriter};
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::env;

const DATA_PATH: &'static str = "misc/data.json";
const SUPPORTED_LANG_PATH: &'static str = "misc/supported_languages.csv";
const TRIGRAM_COUNT: usize = 300;

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
    trigrams: Vec<String>,
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

    let lang_data: HashMap<String, HashMap<String, String>> = serde_json::from_reader(data_file).unwrap();

    let mut scripts: HashMap<String, Vec<Lang>> = HashMap::with_capacity(lang_data.len());
    let mut all_langs: Vec<Lang> = Vec::new();
    for (script, langs) in &lang_data {
        for (code, trigrams) in langs {
            let info = match supported_lang_codes.get(&**code) {
                Some(info) => info,
                None => continue,
            };
            let lang = Lang {
                info: info,
                script: script.clone(),
                trigrams: trigrams.split('|').map(Into::into).collect()
            };
            if lang.trigrams.len() != TRIGRAM_COUNT {
                panic!("Language {} has {} trigrams, instead of {}", code, lang.trigrams.len(), TRIGRAM_COUNT);
            }

            all_langs.push(lang.clone());
            scripts.entry(script.clone()).or_insert_with(Vec::new).push(lang);
        }
    }


    define_enum_lang(&mut lang_def, &supported_langs);

    define_fn_lang_from_code(&mut lang_def, &supported_langs);
    define_fn_lang_to_code(&mut lang_def, &supported_langs);
    define_fn_lang_to_name(&mut lang_def, &supported_langs);
    define_fn_lang_to_eng_name(&mut lang_def, &supported_langs);

    define_const_script_langs(&mut lang_def, &scripts);


    // generates doc tests for `README.md`.
    skeptic::generate_doc_tests(&["README.md"]);
}

fn capitalize(origin: &str) -> String {
    let mut value = String::from(origin);
    value[0..1].make_ascii_uppercase();
    value
}

fn define_enum_lang(buf: &mut BufWriter<File>, supported_langs: &[SupportedLang]) {
    writeln!(buf, "/// Represents a language following [ISO 639-3](https://en.wikipedia.org/wiki/ISO_639-3) standard.").unwrap();
    writeln!(buf, "#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]").unwrap();
    writeln!(buf, "pub enum Lang {{").unwrap();
    for (i, supported_lang) in supported_langs.iter().enumerate() {
        writeln!(buf, "  /// {} ({})", supported_lang.name, supported_lang.eng_name).unwrap();
        // Number starting at 1, to allow enum optimizations to occur (e.g. Option<Lang> will be smaller)
        writeln!(buf, "  {} = {},", capitalize(&supported_lang.code), i + 1).unwrap();
    }
    writeln!(buf, "}}").unwrap();
}

fn define_fn_lang_from_code(buf: &mut BufWriter<File>, supported_langs: &[SupportedLang]) {
    writeln!(buf, "#[inline] fn lang_from_code<S: Into<String>>(code: S) -> Option<Lang> {{").unwrap();
    writeln!(buf, "  match code.into().to_lowercase().as_ref() {{").unwrap();
    for supported_lang in supported_langs.iter() {
        writeln!(buf, "    \"{}\" => Some(Lang::{}),", supported_lang.code, capitalize(&supported_lang.code));
    }
    writeln!(buf, "    _ => None,").unwrap();
    writeln!(buf, "  }}").unwrap();
    writeln!(buf, "}}").unwrap();
}

fn define_fn_lang_to_code(buf: &mut BufWriter<File>, supported_langs: &[SupportedLang]) {
    writeln!(buf, "#[inline] fn lang_to_code(lang: Lang) -> &'static str {{").unwrap();
    writeln!(buf, "  match lang {{").unwrap();
    for supported_lang in supported_langs.iter() {
        writeln!(buf, "    Lang::{} => \"{}\",", capitalize(&supported_lang.code), supported_lang.code).unwrap();
    }
    writeln!(buf, "  }}").unwrap();
    writeln!(buf, "}}").unwrap();
}

fn define_fn_lang_to_name(buf: &mut BufWriter<File>, supported_langs: &[SupportedLang]) {
    writeln!(buf, "#[inline] fn lang_to_name(lang: Lang) -> &'static str {{").unwrap();
    writeln!(buf, "  match lang {{").unwrap();
    for supported_lang in supported_langs.iter() {
        writeln!(buf, "    Lang::{} => \"{}\",", capitalize(&supported_lang.code), supported_lang.name).unwrap();
    }
    writeln!(buf, "  }}").unwrap();
    writeln!(buf, "}}").unwrap();
}

fn define_fn_lang_to_eng_name(buf: &mut BufWriter<File>, supported_langs: &[SupportedLang]) {
    writeln!(buf, "#[inline] fn lang_to_eng_name(lang: Lang) -> &'static str {{").unwrap();
    writeln!(buf, "  match lang {{").unwrap();
    for supported_lang in supported_langs.iter() {
        writeln!(buf, "    Lang::{} => \"{}\",", capitalize(&supported_lang.code), supported_lang.eng_name).unwrap();
    }
    writeln!(buf, "  }}").unwrap();
    writeln!(buf, "}}").unwrap();
}

fn define_const_script_langs(buf: &mut BufWriter<File>, scripts: &HashMap<String, Vec<Lang>>) {
    for (script, script_langs) in scripts {
        writeln!(buf, "/// Languages for script '{}'", script).unwrap();
        writeln!(buf, "pub static {}_LANGS: LangProfileList = &[", script.to_ascii_uppercase()).unwrap();
        for lang in script_langs {
            writeln!(buf, "  (Lang::{}, &{:?}),", capitalize(&lang.info.code), lang.trigrams).unwrap();
        }
        writeln!(buf, "];").unwrap();
    }
}
