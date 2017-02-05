use std::collections::HashMap;

mod lang;
mod script;
mod query;
mod result;
mod utils;
mod trigrams;

pub use lang::*;
pub use script::*;
pub use trigrams::*;
pub use query::Query;
pub use result::Result;

const MAX_DIST : u32 = 300;

pub fn detect_lang(query : Query) -> Option<Result> {
    let text = query.text;

    if let Some(script) = detect_script(text) {
        detect_lang_based_on_script(query, script).map( |lang| {
            Result { lang: lang, script: script }
        })
    } else {
        None
    }
}

fn detect_lang_based_on_script(query : Query, script : Script) -> Option<Lang> {
    match script {
        Script::Latin      => detect(query, LATIN_LANGS),
        Script::Cyrillic   => detect(query, CYRILLIC_LANGS),
        Script::Devanagari => detect(query, DEVANAGARI_LANGS),
        Script::Hebrew     => detect(query, HEBREW_LANGS),
        Script::Ethiopic   => detect(query, ETHIOPIC_LANGS),
        Script::Arabic     => detect(query, ARABIC_LANGS),
        Script::Mandarin  => Some(Lang::Cmn),
        Script::Bengali   => Some(Lang::Ben),
        Script::Hangul    => Some(Lang::Kor),
        Script::Georgian  => Some(Lang::Kat),
        Script::Greek     => Some(Lang::Ell),
        Script::Kannada   => Some(Lang::Kan),
        Script::Tamil     => Some(Lang::Tam),
        Script::Thai      => Some(Lang::Tha),
        Script::Gujarati  => Some(Lang::Guj),
        Script::Gurmukhi  => Some(Lang::Pan),
        Script::Telugu    => Some(Lang::Tel),
        Script::Malayalam => Some(Lang::Mal),
        Script::Oriya     => Some(Lang::Ori),
        Script::Myanmar   => Some(Lang::Mya),
        Script::Katakana | Script::Hiragana  => Some(Lang::Jpn)
    }
}

fn detect(query : Query, lang_profile_list : LangProfileList) -> Option<Lang> {
    let mut lang_distances : Vec<(Lang, u32)> = vec![];
    let trigrams = get_trigrams_with_positions(&query.text);

    for &(ref lang, lang_trigrams) in lang_profile_list {
        if let Some(ref whitelist) = query.whitelist {
            // Skip non-whitelisted languages
            if !whitelist.contains(lang) { continue; }
        } else if let Some(ref blacklist) = query.blacklist {
            // Skip blacklisted languages
            if blacklist.contains(lang) { continue; }
        }
        let dist = calculate_distance(lang_trigrams, &trigrams);
        lang_distances.push(((*lang).clone(), dist));
    }

    lang_distances.sort_by_key(|key| key.1 );
    lang_distances.iter().nth(0).map(|pair| pair.0)
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
        let query = Query::new("Además de todo lo anteriormente dicho, también encontramos...");
        let res = detect_lang(query).unwrap();
        assert_eq!(res.lang, Lang::Spa);
        assert_eq!(res.script, Script::Latin);

        let query = Query::new("English does not suit well for the role of international language");
        let res = detect_lang(query).unwrap();
        assert_eq!(res.lang, Lang::Eng);
        assert_eq!(res.script, Script::Latin);

        let query = Query::new("Та нічого, все нормально. А в тебе як?");
        let res = detect_lang(query).unwrap();
        assert_eq!(res.lang, Lang::Ukr);
        assert_eq!(res.script, Script::Cyrillic);

        let query = Query::new("ইউনিকোডে বাংলা লিপি");
        let res = detect_lang(query).unwrap();
        assert_eq!(res.lang, Lang::Ben);
        assert_eq!(res.script, Script::Bengali);
    }

    #[test]
    fn test_detect_lang_with_blacklist() {
        let text = "I am begging pardon";

        // without blacklist
        let query = Query::new(text);
        let result = detect_lang(query).unwrap();
        assert_eq!(result.lang, Lang::Jav);

        // with blacklist
        let blacklist = [Lang::Jav, Lang::Nld, Lang::Uzb, Lang::Swe, Lang::Nob];
        let query = Query::new(text).blacklist(&blacklist);
        let result = detect_lang(query).unwrap();
        assert_eq!(result.lang, Lang::Eng);
    }

    #[test]
    fn test_detect_lang_with_blacklist_none() {
        let text = String::from("האקדמיה ללשון העברית");

        // All languages with Hebrew script are in blacklist, so result must be None
        let blacklist = vec![Lang::Heb, Lang::Ydd];
        let query = Query::new(&text).blacklist(&blacklist);
        let result = detect_lang(query);
        assert_eq!(result, None);
    }

    #[test]
    fn test_detect_lang_with_whitelist() {
        let whitelist = vec![Lang::Epo, Lang::Ukr];

        let text = String::from("Mi ne scias! Ne demandu min plu!");
        let query = Query::new(&text).whitelist(&whitelist);
        let result = detect_lang(query).unwrap();
        assert_eq!(result.lang, Lang::Epo);

        let text = String::from("Тут все.");
        let query = Query::new(&text).whitelist(&whitelist);
        let result = detect_lang(query).unwrap();
        assert_eq!(result.lang, Lang::Ukr);
    }
}
