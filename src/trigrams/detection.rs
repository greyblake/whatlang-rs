use hashbrown::HashMap;

use crate::{Lang, Script, detect_script};
use crate::options::{List, Options};

use super::{LangProfile, LangProfileList};

use crate::trigrams::utils::*;
use crate::trigrams::*;


pub struct Outcome {
   pub normalized_scores: Vec<(Lang, f64)>,
   pub trigram_count: usize,
}

impl Outcome {
    fn new(normalized_scores: Vec<(Lang, f64)>, trigram_count: usize) -> Self {
        Self { normalized_scores, trigram_count }
    }

    fn new_empty() -> Self {
        Self { normalized_scores: vec![], trigram_count: 0 }
    }

    fn from_lang(lang:  Lang) -> Self {
        let normalized_scores = vec![(lang, 1.0)];
        Self { normalized_scores, trigram_count: 1 }
    }
}


pub fn calculate_scores_based_on_script(
    text: &str,
    options: &Options,
    script: Script,
) -> Outcome {
    match script {
        Script::Latin => calculate_scores_in_profiles(text, options, LATIN_LANGS),
        Script::Cyrillic => calculate_scores_in_profiles(text, options, CYRILLIC_LANGS),
        Script::Devanagari => calculate_scores_in_profiles(text, options, DEVANAGARI_LANGS),
        Script::Hebrew => calculate_scores_in_profiles(text, options, HEBREW_LANGS),
        Script::Arabic => calculate_scores_in_profiles(text, options, ARABIC_LANGS),
        Script::Mandarin => detect_mandarin_japanese(options),
        Script::Bengali => Outcome::from_lang(Lang::Ben),
        Script::Hangul => Outcome::from_lang(Lang::Kor),
        Script::Georgian => Outcome::from_lang(Lang::Kat),
        Script::Greek => Outcome::from_lang(Lang::Ell),
        Script::Kannada => Outcome::from_lang(Lang::Kan),
        Script::Tamil => Outcome::from_lang(Lang::Tam),
        Script::Thai => Outcome::from_lang(Lang::Tha),
        Script::Gujarati => Outcome::from_lang(Lang::Guj),
        Script::Gurmukhi => Outcome::from_lang(Lang::Pan),
        Script::Telugu => Outcome::from_lang(Lang::Tel),
        Script::Malayalam => Outcome::from_lang(Lang::Mal),
        Script::Oriya => Outcome::from_lang(Lang::Ori),
        Script::Myanmar => Outcome::from_lang(Lang::Mya),
        Script::Sinhala => Outcome::from_lang(Lang::Sin),
        Script::Khmer => Outcome::from_lang(Lang::Khm),
        Script::Ethiopic => Outcome::from_lang(Lang::Amh),
        Script::Katakana | Script::Hiragana => Outcome::from_lang(Lang::Jpn),
    }
}

fn calculate_scores_in_profiles(
    text: &str,
    options: &Options,
    lang_profile_list: LangProfileList,
) -> Outcome {
    let mut lang_distances: Vec<(Lang, u32)> = vec![];
    let trigrams = get_trigrams_with_positions(text);

    for &(ref lang, lang_trigrams) in lang_profile_list {
        match options.list {
            Some(List::White(ref whitelist)) if !whitelist.contains(lang) => continue,
            Some(List::Black(ref blacklist)) if blacklist.contains(lang) => continue,
            _ => {}
        }
        let dist = calculate_distance(lang_trigrams, &trigrams);
        lang_distances.push(((*lang), dist));
    }

    // Sort languages by distance
    lang_distances.sort_by_key(|key| key.1);

    let lang_scores = lang_distances
        .iter()
        .map(|&(lang, distance)| (lang, distance_to_score(trigrams.len() as u32, distance)))
        .collect();

    Outcome::new(lang_scores, trigrams.len())
}

fn calculate_distance(lang_trigrams: LangProfile, text_trigrams: &HashMap<Trigram, u32>) -> u32 {
    let mut total_dist = 0u32;

    for (i, &trigram) in lang_trigrams.iter().enumerate() {
        let dist = match text_trigrams.get(&trigram) {
            Some(&n) => (n as i32 - i as i32).abs() as u32,
            None => MAX_TRIGRAM_DISTANCE,
        };
        total_dist += dist;
    }
    if total_dist < MAX_TOTAL_DISTANCE {
        total_dist
    } else {
        MAX_TOTAL_DISTANCE
    }

}

fn distance_to_score(trigrams_count: u32, distance: u32) -> f64 {
    let similarity = MAX_TOTAL_DISTANCE - distance;
    similarity as f64 / MAX_TRIGRAM_DISTANCE as f64
}

fn detect_mandarin_japanese(options: &Options) -> Outcome {
    match options.list {
        Some(List::White(ref whitelist)) => {
            if whitelist.contains(&Lang::Jpn) && !whitelist.contains(&Lang::Cmn) {
                Outcome::from_lang(Lang::Jpn)
            } else if whitelist.contains(&Lang::Cmn) {
                Outcome::from_lang(Lang::Cmn)
            } else {
                Outcome::new_empty()
            }
        }
        Some(List::Black(ref blacklist)) => {
            if blacklist.contains(&Lang::Cmn) && !blacklist.contains(&Lang::Jpn) {
                Outcome::from_lang(Lang::Jpn)
            } else if !blacklist.contains(&Lang::Cmn) {
                Outcome::from_lang(Lang::Cmn)
            } else {
                Outcome::new_empty()
            }
        }
        _ => Outcome::from_lang(Lang::Cmn)
    }
}
