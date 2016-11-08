use std::collections::HashMap;

mod lang;
mod trigrams;
mod script;

pub use lang::*;
use trigrams::*;
use script::*;

const MAX_DIST : u32 = 300;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Result {
    pub lang: Lang,
    pub script: Script,
    pub is_reliable: bool
}

pub fn detect_lang(text: &String) -> Option<Result> {
    let script_option = detect_script(text);

    // unwrap script option
    let script = match script_option {
        None => { return None; },
        Some(val) => val
    };

    let lang = match script {
        Script::Latin      => detect(text, LATIN_LANGS),
        Script::Cyrillic   => detect(text, CYRILLIC_LANGS),
        Script::Arabic     => detect(text, ARABIC_LANGS),
        Script::Devanagari => detect(text, DEVANAGARI_LANGS),
        Script::Ethiopic   => detect(text, ETHIOPIC_LANGS),
        Script::Hebrew     => detect(text, HEBREW_LANGS),
        Script::Cmn     => Lang::Cmn,
        Script::Kat     => Lang::Kat,
        Script::Bengali => Lang::Ben
    };

    Some(Result { lang: lang, script: script, is_reliable: true })
}

fn detect(text : &String, lang_profile_list : LangProfileList) -> Lang {
    let mut lang_distances : Vec<(Lang, u32)> = vec![];
    let trigrams = get_trigrams_with_positions(&text);

    for &(ref lang, lang_trigrams) in lang_profile_list {
        let dist = calculate_distance(lang_trigrams, &trigrams);
        lang_distances.push(((*lang).clone(), dist));
    }

    lang_distances.sort_by_key(|key| key.1 );
    (lang_distances.iter().nth(0).unwrap().0).clone()
}

fn calculate_distance(lang_trigrams: LangProfile,  text_trigrams: &HashMap<String, u32>) -> u32 {
    let mut total_dist = 0u32;

    for (i, &trigram) in lang_trigrams.iter().enumerate() {
        let dist = match text_trigrams.get(trigram) {
            Some(&n) => (n as i32 - i as i32).abs() as u32,
            None => MAX_DIST
        };
        total_dist += dist;
    }
    total_dist
}

#[cfg(test)]
mod tests {
    use lang::Lang;
    use script::Script;
    use super::detect_lang;

    #[test]
    fn test_detect_lang() {
        let spa_text = &"Además de todo lo anteriormente dicho, también encontramos...".to_string();
        let res = detect_lang(spa_text).unwrap();
        assert_eq!(res.lang, Lang::Spa);
        assert_eq!(res.script, Script::Latin);

        let eng_text = &"English does not suit well for the role of international language".to_string();
        let res = detect_lang(eng_text).unwrap();
        assert_eq!(res.lang, Lang::Eng);
        assert_eq!(res.script, Script::Latin);

        let ukr_text = &"Та нічого, все нормально. А в тебе як?".to_string();
        let res = detect_lang(ukr_text).unwrap();
        assert_eq!(res.lang, Lang::Ukr);
        assert_eq!(res.script, Script::Cyrillic);


        let text = &"ইউনিকোডে বাংলা লিপি".to_string();
        let res = detect_lang(text).unwrap();
        assert_eq!(res.lang, Lang::Ben);
        assert_eq!(res.script, Script::Bengali);
    }
}
