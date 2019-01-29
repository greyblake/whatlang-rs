extern crate csv;
extern crate serde;
extern crate serde_json;
extern crate skeptic;
#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Write};
use std::path::Path;

const DATA_PATH: &'static str = "misc/data.json";
const SUPPORTED_LANG_PATH: &'static str = "misc/supported_languages.csv";
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

    generate_source_files();
    skeptic::generate_doc_tests(&["README.md"]);
}

fn generate_source_files() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("lang.rs");
    let mut lang_def = BufWriter::new(File::create(&dest_path).unwrap());

    let (lang_infos, scripts) = load_data();

    render_lang_rs(&mut lang_def, &lang_infos, &scripts).unwrap();
}

fn load_data() -> (Vec<LangInfo>, HashMap<String, Vec<Lang>>) {
    let data_file = BufReader::new(File::open(DATA_PATH).unwrap());
    let mut lang_reader = csv::ReaderBuilder::new()
        .flexible(true)
        .from_path(SUPPORTED_LANG_PATH)
        .unwrap();

    let mut lang_infos: Vec<LangInfo> = lang_reader.deserialize().map(Result::unwrap).collect();
    lang_infos.sort_by(|left, right| left.code.cmp(&right.code));

    let supported_lang_codes: HashMap<String, LangInfo> = lang_infos
        .iter()
        .map(|lang| (lang.code.clone(), lang.clone()))
        .collect();

    let lang_data: HashMap<String, HashMap<String, String>> =
        serde_json::from_reader(data_file).unwrap();

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
                trigrams: trigrams.split('|').map(Into::into).collect(),
            };
            if lang.trigrams.len() != TRIGRAM_COUNT {
                panic!(
                    "Language {} has {} trigrams, instead of {}",
                    code,
                    lang.trigrams.len(),
                    TRIGRAM_COUNT
                );
            }

            all_langs.push(lang.clone());
            scripts
                .entry(script.clone())
                .or_insert_with(Vec::new)
                .push(lang);
        }
    }

    (lang_infos, scripts)
}

fn render_lang_rs(
    buf: &mut BufWriter<File>,
    lang_infos: &[LangInfo],
    scripts: &HashMap<String, Vec<Lang>>,
) -> Result<(), io::Error> {
	writeln!(buf, "/// Represents a language following [ISO 639-3](https://en.wikipedia.org/wiki/ISO_639-3) standard.")?;
	writeln!(buf, "#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]")?;
	writeln!(buf, "pub enum Lang {{")?;
    for (index, lang) in lang_infos.iter().enumerate() {
    	writeln!(buf, "{} = {},", capitalize(lang.code.clone()), index)?;
    }
	writeln!(buf, "}}")?;

	writeln!(buf, "fn lang_from_code<S: Into<String>>(code: S) -> Option<Lang> {{")?;
	writeln!(buf, "    match code.into().to_lowercase().as_str() {{")?;
    for lang in lang_infos {
    	writeln!(buf, "\"{}\" => Some(Lang::{}),", lang.code, capitalize(lang.code.clone()))?;
    }
	writeln!(buf, "        _ => None,")?;
	writeln!(buf, "    }}")?;
	writeln!(buf, "}}")?;

	writeln!(buf, "fn lang_to_code(lang: Lang) -> &'static str {{")?;
	writeln!(buf, "	match lang {{")?;
    for lang in lang_infos {
    	writeln!(buf, "Lang::{} => \"{}\",", capitalize(lang.code.clone()), lang.code)?;
    }
	writeln!(buf, "    }}")?;
	writeln!(buf, "}}")?;

	writeln!(buf, "fn lang_to_name(lang: Lang) -> &'static str {{")?;
	writeln!(buf, "    match lang {{")?;
    for lang in lang_infos {
    	writeln!(buf, "Lang::{} => \"{}\",", capitalize(lang.code.clone()), lang.name)?;
    }
	writeln!(buf, "    }}")?;
	writeln!(buf, "}}")?;

	writeln!(buf, "fn lang_to_eng_name(lang: Lang) -> &'static str {{")?;
	writeln!(buf, "	match lang {{")?;
    for lang in lang_infos {
    	writeln!(buf, "Lang::{} => \"{}\",", capitalize(lang.code.clone()), lang.eng_name)?;
    }
	writeln!(buf, "     }}")?;
	writeln!(buf, "}}")?;

	for (script, langs) in scripts {
		writeln!(buf, "pub static {}_LANGS: LangProfileList = &[", script.to_uppercase())?;
		for lang in langs {
			writeln!(buf, "(Lang::{}, &[", capitalize(lang.info.code.clone()))?;
			for trigram in lang.trigrams.clone() {
				writeln!(buf, "\"{}\",", trigram)?;
			}
			writeln!(buf, "]),")?;
		}
		writeln!(buf, "];")?;
	}
	Ok(())
}

fn capitalize(s: String) -> String {
	let mut chars = s.chars();
    match chars.next() {
        None => s,
        Some(c) => {
            let res = c.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase();
            res
        }
}
}
