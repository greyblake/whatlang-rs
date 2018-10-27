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
const TEMPLATE_LANG_RS_PATH: &'static str = "templates/lang.rs";
const TRIGRAM_COUNT: usize = 300;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
struct LangInfo {
    code: String,
    eng_name: String,
    name: String,
    native_speakers: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct Lang {
    info: LangInfo,
    script: String,
    trigrams: Vec<String>,
}

fn main() {
    println!("cargo:rerun-if-changed={}", DATA_PATH);
    println!("cargo:rerun-if-changed={}", SUPPORTED_LANG_PATH);
    println!("cargo:rerun-if-changed={}", TEMPLATE_LANG_RS_PATH);

    generate_source_files();
    skeptic::generate_doc_tests(&["README.md"]);
}

fn generate_source_files() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("lang.rs");
    let mut lang_def = BufWriter::new(File::create(&dest_path).unwrap());

    let (lang_infos, scripts) = load_data();

    render_lang_rs(&mut lang_def, &lang_infos, &scripts);
}

fn load_data() -> (Vec<LangInfo>, HashMap<String, Vec<Lang>>) {
    let data_file = BufReader::new(File::open(DATA_PATH).unwrap());
    let mut lang_reader = csv::ReaderBuilder::new().flexible(true).from_path(SUPPORTED_LANG_PATH).unwrap();

    let mut lang_infos: Vec<LangInfo> = lang_reader.deserialize().map(Result::unwrap).collect();
    lang_infos.sort_by(|left, right| left.code.cmp(&right.code));

    let supported_lang_codes: HashMap<String, LangInfo> = lang_infos.iter()
        .map(|lang| (lang.code.clone(), lang.clone()))
        .collect();

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
                info: (*info).clone(),
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

    (lang_infos, scripts)
}

fn render_lang_rs(buf: &mut BufWriter<File>, lang_infos: &[LangInfo], scripts: &HashMap<String, Vec<Lang>>) {
    let mut tera = tera::Tera::default();
    tera.add_template_file(TEMPLATE_LANG_RS_PATH, Some("lang.rs"));

    let mut ctx = tera::Context::new();
    ctx.insert("lang_infos", lang_infos);
    ctx.insert("scripts", scripts);

    let code = tera.render("lang.rs", &ctx).unwrap();
    writeln!(buf, "{}", code).unwrap();
}
