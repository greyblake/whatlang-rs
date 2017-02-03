use std::collections::HashMap;

mod lang;
mod script;
mod query;
mod result;
mod trigrams;

pub use lang::*;
pub use script::*;
pub use trigrams::*;
pub use query::Query;
pub use result::Result;

const MAX_DIST : u32 = 300;

pub fn detect_lang(query : Query) -> Option<Result> {
    let text = query.text;
    detect_script(text).map( |script| {
        let lang = detect_lang_based_on_script(text, script);
        Result { lang: lang, script: script }
    })
}

fn detect_lang_based_on_script(text: &String, script : Script) -> Lang {
    match script {
        Script::Latin      => detect(text, LATIN_LANGS),
        Script::Cyrillic   => detect(text, CYRILLIC_LANGS),
        Script::Devanagari => detect(text, DEVANAGARI_LANGS),
        Script::Hebrew     => detect(text, HEBREW_LANGS),
        Script::Ethiopic   => detect(text, ETHIOPIC_LANGS),
        Script::Arabic     => detect(text, ARABIC_LANGS),
        Script::Mandarin => Lang::Cmn,
        Script::Bengali  => Lang::Ben,
        Script::Hangul   => Lang::Kor,
        Script::Georgian => Lang::Kat,
        Script::Greek    => Lang::Ell,
        Script::Kannada  => Lang::Kan,
        Script::Tamil    => Lang::Tam,
        Script::Katakana | Script::Hiragana  => Lang::Jpn
    }
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
    use super::Query;

    #[test]
    fn test_detect_lang() {
        let text = &"Además de todo lo anteriormente dicho, también encontramos...".to_string();
        let query = Query { text: text };
        let res = detect_lang(query).unwrap();
        assert_eq!(res.lang, Lang::Spa);
        assert_eq!(res.script, Script::Latin);

        let text = &"English does not suit well for the role of international language".to_string();
        let query = Query { text: text };
        let res = detect_lang(query).unwrap();
        assert_eq!(res.lang, Lang::Eng);
        assert_eq!(res.script, Script::Latin);

        let text = &"Та нічого, все нормально. А в тебе як?".to_string();
        let query = Query { text: text };
        let res = detect_lang(query).unwrap();
        assert_eq!(res.lang, Lang::Ukr);
        assert_eq!(res.script, Script::Cyrillic);

        let text = &"ইউনিকোডে বাংলা লিপি".to_string();
        let query = Query { text: text };
        let res = detect_lang(query).unwrap();
        assert_eq!(res.lang, Lang::Ben);
        assert_eq!(res.script, Script::Bengali);
    }
}
