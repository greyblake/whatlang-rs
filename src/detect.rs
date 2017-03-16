use std::collections::HashMap;

use lang::*;
use script::*;
use trigrams::*;
use info::Info;
use options::Options;

const MAX_DIST : u32 = 300;

/// Detect a language and a script by a given text.
///
/// # Example
/// ```
/// use whatlang::{detect, Lang, Script};
///
/// let info = detect("Ĉu vi ne volas eklerni Esperanton? Bonvolu!").unwrap();
/// assert_eq!(info.lang, Lang::Epo);
/// assert_eq!(info.script, Script::Latin);
/// ```
pub fn detect(text: &str) -> Option<Info> {
    detect_with_options(text, Options::None)
}

/// Detect only a language by a given text.
///
/// # Example
/// ```
/// use whatlang::{detect_lang, Lang};
/// let lang = detect_lang("There is no reason not to learn Esperanto.").unwrap();
/// assert_eq!(lang, Lang::Eng);
/// ```
pub fn detect_lang(text: &str) -> Option<Lang> {
    detect(text).map(|info| info.lang)
}

pub fn detect_lang_with_options(text: &str, options: Options) -> Option<Lang> {
    detect_with_options(text, options).map(|info| info.lang)
}

pub fn detect_with_options(text: &str, options: Options) -> Option<Info> {
    if let Some(script) = detect_script(text) {
        detect_lang_based_on_script(text, options, script).map( |lang| {
            Info { lang: lang, script: script }
        })
    } else {
        None
    }
}

fn detect_lang_based_on_script(text: &str, options: Options, script : Script) -> Option<Lang> {
    match script {
        Script::Latin      => detect_lang_in_profiles(text, options, LATIN_LANGS),
        Script::Cyrillic   => detect_lang_in_profiles(text, options, CYRILLIC_LANGS),
        Script::Devanagari => detect_lang_in_profiles(text, options, DEVANAGARI_LANGS),
        Script::Hebrew     => detect_lang_in_profiles(text, options, HEBREW_LANGS),
        Script::Ethiopic   => detect_lang_in_profiles(text, options, ETHIOPIC_LANGS),
        Script::Arabic     => detect_lang_in_profiles(text, options, ARABIC_LANGS),
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
        Script::Sinhala   => Some(Lang::Sin),
        Script::Khmer     => Some(Lang::Khm),
        Script::Katakana | Script::Hiragana  => Some(Lang::Jpn)
    }
}

fn detect_lang_in_profiles(text: &str, options: Options, lang_profile_list : LangProfileList) -> Option<Lang> {
    let mut lang_distances : Vec<(Lang, u32)> = vec![];
    let trigrams = get_trigrams_with_positions(text);

    for &(ref lang, lang_trigrams) in lang_profile_list {
        match options {
            // Skip non-whitelisted languages
            Options::Whitelist(whitelist) if !whitelist.contains(lang) => continue,
            // Skip blacklisted languages
            Options::Blacklist(blacklist) if blacklist.contains(lang) => continue,
            _ => {},
        }
        let dist = calculate_distance(lang_trigrams, &trigrams);
        lang_distances.push(((*lang), dist));
    }

    lang_distances.sort_by_key(|key| key.1 );
    lang_distances.first().map(|pair| pair.0)
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
    use super::detect;
    use super::detect_lang;
    use super::detect_with_options;
    use options::Options;

    #[test]
    fn test_detect_spanish() {
        let text = "Además de todo lo anteriormente dicho, también encontramos...";
        let output = detect(text);
        assert_eq!(output.is_some(), true);

        let info = output.unwrap();
        assert_eq!(info.lang, Lang::Spa);
        assert_eq!(info.script, Script::Latin);
    }

    #[test]
    fn test_detect_lang_ukrainian() {
        let text = "Та нічого, все нормально. А в тебе як?";
        assert_eq!(detect_lang(text), Some(Lang::Ukr));
    }

    #[test]
    fn test_detect_with_options_with_blacklist() {
        let text = "I am begging pardon";
        // without blacklist
        let output = detect_with_options(text, Options::None);
        assert_eq!(output.is_some(), true);
        let info = output.unwrap();
        assert_eq!(info.lang, Lang::Tgl);

        // with blacklist
        let blacklist = [Lang::Tgl, Lang::Jav, Lang::Nld, Lang::Uzb, Lang::Swe, Lang::Nob, Lang::Ceb, Lang::Ilo];
        let options = Options::Blacklist(&blacklist);
        let output = detect_with_options(text, options);
        assert_eq!(output.is_some(), true);
        let info = output.unwrap();
        assert_eq!(info.lang, Lang::Eng);
    }

    #[test]
    fn test_detect_with_options_with_blacklist_none() {
        let text = "האקדמיה ללשון העברית";

        // All languages with Hebrew script are in blacklist, so result must be None
        let blacklist = vec![Lang::Heb, Lang::Ydd];
        let options = Options::Blacklist(&blacklist);
        let output = detect_with_options(text, options);
        assert_eq!(output, None);
    }

    #[test]
    fn test_detect_with_options_with_whitelist() {
        let whitelist = vec![Lang::Epo, Lang::Ukr];
        let options = Options::Whitelist(&whitelist);

        let text = "Mi ne scias!";
        let output = detect_with_options(text, options);
        assert_eq!(output.is_some(), true);
        let info = output.unwrap();
        assert_eq!(info.lang, Lang::Epo);
    }
}
