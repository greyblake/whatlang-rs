use fnv::FnvHashMap;

use lang::*;
use script::*;
use trigrams::*;
use info::Info;
use options::{Options, List};

const MAX_TRIGRAM_DISTANCE : u32 = 300;

// 300 * 300 = 90_000
const MAX_TOTAL_DISTANCE : u32 = 90_000;

/// Detect a language and a script by a given text.
///
/// # Example
/// ```
/// use whatlang::{detect, Lang, Script};
///
/// let info = detect("Ĉu vi ne volas eklerni Esperanton? Bonvolu!").unwrap();
/// assert_eq!(info.lang(), Lang::Epo);
/// assert_eq!(info.script(), Script::Latin);
/// ```
pub fn detect(text: &str) -> Option<Info> {
    detect_with_options(text, &Options::default())
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

pub fn detect_lang_with_options(text: &str, options: &Options) -> Option<Lang> {
    detect_with_options(text, options).map(|info| info.lang)
}

pub fn detect_with_options(text: &str, options: &Options) -> Option<Info> {
    detect_script(text).and_then(|script| {
        detect_lang_based_on_script(text, options, script).map( |(lang, confidence)| {
            Info { lang, script, confidence }
        })
    })
}

fn detect_lang_based_on_script(text: &str, options: &Options, script : Script) -> Option<(Lang, f64)> {
    match script {
        Script::Latin      => detect_lang_in_profiles(text, options, LATIN_LANGS),
        Script::Cyrillic   => detect_lang_in_profiles(text, options, CYRILLIC_LANGS),
        Script::Devanagari => detect_lang_in_profiles(text, options, DEVANAGARI_LANGS),
        Script::Hebrew     => detect_lang_in_profiles(text, options, HEBREW_LANGS),
        Script::Ethiopic   => detect_lang_in_profiles(text, options, ETHIOPIC_LANGS),
        Script::Arabic     => detect_lang_in_profiles(text, options, ARABIC_LANGS),
        Script::Mandarin  => Some((Lang::Cmn, 1.0)),
        Script::Bengali   => Some((Lang::Ben, 1.0)),
        Script::Hangul    => Some((Lang::Kor, 1.0)),
        Script::Georgian  => Some((Lang::Kat, 1.0)),
        Script::Greek     => Some((Lang::Ell, 1.0)),
        Script::Kannada   => Some((Lang::Kan, 1.0)),
        Script::Tamil     => Some((Lang::Tam, 1.0)),
        Script::Thai      => Some((Lang::Tha, 1.0)),
        Script::Gujarati  => Some((Lang::Guj, 1.0)),
        Script::Gurmukhi  => Some((Lang::Pan, 1.0)),
        Script::Telugu    => Some((Lang::Tel, 1.0)),
        Script::Malayalam => Some((Lang::Mal, 1.0)),
        Script::Oriya     => Some((Lang::Ori, 1.0)),
        Script::Myanmar   => Some((Lang::Mya, 1.0)),
        Script::Sinhala   => Some((Lang::Sin, 1.0)),
        Script::Khmer     => Some((Lang::Khm, 1.0)),
        Script::Katakana | Script::Hiragana  => Some((Lang::Jpn, 1.0))
    }
}

fn detect_lang_in_profiles(text: &str, options: &Options, lang_profile_list : LangProfileList) -> Option<(Lang, f64)> {
    let mut lang_distances : Vec<(Lang, u32)> = vec![];
    let trigrams = get_trigrams_with_positions(text);

    for &(ref lang, lang_trigrams) in lang_profile_list {
        match options.list {
            Some(List::White(ref whitelist)) if !whitelist.contains(lang) => continue,
            Some(List::Black(ref blacklist)) if blacklist.contains(lang) => continue,
            _ => {},
        }
        let dist = calculate_distance(lang_trigrams, &trigrams);
        lang_distances.push(((*lang), dist));
    }

    // Sort languages by distance
    lang_distances.sort_by_key(|key| key.1 );

    // Return None if lang_distances is empty
    // Return the only language with is_reliable=true if there is only 1 item
    if lang_distances.len() < 2 {
        return lang_distances.first().map(|pair| (pair.0, 1.0));
    }

    // Calculate is_reliable based on:
    // - number of unique trigrams in the text
    // - rate (diff between score of the first and second languages)
    //
    let lang_dist1 = lang_distances[0];
    let lang_dist2 = lang_distances[1];
    let score1 = MAX_TOTAL_DISTANCE - lang_dist1.1;
    let score2 = MAX_TOTAL_DISTANCE - lang_dist2.1;
    let rate = (score1 - score2) as f64 / (score2 as f64);

    // Hyperbola function. Everything that is above the function has confidence = 1.0
    // If rate is below, confidence is calculated proportionally.
    // Numbers 12.0 and 0.05 are obtained experimentally, so the function represents common sense.
    //
    let confident_rate = (12.0 / trigrams.len() as f64) + 0.05;
    let confidence =
        if rate > confident_rate {
            1.0
        } else {
            rate / confident_rate
        };

    Some((lang_dist1.0, confidence))
}

fn calculate_distance(lang_trigrams: LangProfile,  text_trigrams: &FnvHashMap<String, u32>) -> u32 {
    let mut total_dist = 0u32;

    for (i, &trigram) in lang_trigrams.iter().enumerate() {
        let dist = match text_trigrams.get(trigram) {
            Some(&n) => (n as i32 - i as i32).abs() as u32,
            None => MAX_TRIGRAM_DISTANCE
        };
        total_dist += dist;
    }
    total_dist
}

#[cfg(test)]
mod tests {
    use super::*;
    use script::Script;

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
        let output = detect_with_options(text, &Options::default());
        assert_eq!(output.is_some(), true);
        let info = output.unwrap();
        assert_eq!(info.lang, Lang::Tgl);

        // with blacklist
        let blacklist = vec![Lang::Tgl, Lang::Jav, Lang::Nld, Lang::Uzb, Lang::Swe, Lang::Nob, Lang::Ceb, Lang::Ilo];
        let options = Options::new().set_blacklist(blacklist);
        let output = detect_with_options(text, &options);
        assert_eq!(output.is_some(), true);
        let info = output.unwrap();
        assert_eq!(info.lang, Lang::Eng);
    }

    #[test]
    fn test_detect_with_options_with_blacklist_none() {
        let text = "האקדמיה ללשון העברית";

        // All languages with Hebrew script are in blacklist, so result must be None
        let blacklist = vec![Lang::Heb, Lang::Ydd];
        let options = Options::new().set_blacklist(blacklist);
        let output = detect_with_options(text, &options);
        assert_eq!(output, None);
    }

    #[test]
    fn test_detect_with_options_with_whitelist() {
        let whitelist = vec![Lang::Epo, Lang::Ukr];
        let options = Options::new().set_whitelist(whitelist);

        let text = "Mi ne scias!";
        let output = detect_with_options(text, &options);
        assert_eq!(output.is_some(), true);
        let info = output.unwrap();
        assert_eq!(info.lang, Lang::Epo);
    }
}
