use hashbrown::HashMap;

use crate::core::{LangScores, InternalQuery, Output, AllowList, Text, LowercaseText};
use crate::scripts::grouping::MultiLangScript;
use crate::Lang;
use super::{LangProfile, LangProfileList};
use super::{LATIN_LANGS, CYRILLIC_LANGS, DEVANAGARI_LANGS, HEBREW_LANGS, ARABIC_LANGS};
use super::utils::get_trigrams_with_positions;
use super::{Trigram, MAX_TOTAL_DISTANCE, MAX_TRIGRAM_DISTANCE};

pub struct RawOutcome {
    pub trigrams_count: usize,
    pub lang_scores: LangScores
}

pub fn detect(iquery: &mut InternalQuery) -> Option<Output> {
    let lang_scores = raw_detect(iquery).lang_scores;
    lang_scores.scores.first().map( |&(lang, _)| {
        let script = iquery.multi_lang_script.to_script();
        Output::new(script, lang)
    })
}

pub fn raw_detect(iquery: &mut InternalQuery) -> RawOutcome {
    let lang_profile_list = script_to_lang_profile_list(iquery.multi_lang_script);
    calculate_scores_in_profiles(&mut iquery.text, &iquery.allow_list, lang_profile_list)
}

fn script_to_lang_profile_list(script: MultiLangScript) -> LangProfileList {
    use MultiLangScript as MLS;
    match script {
        MLS::Latin => LATIN_LANGS,
        MLS::Cyrillic => CYRILLIC_LANGS,
        MLS::Arabic => ARABIC_LANGS,
        MLS::Devanagari => DEVANAGARI_LANGS,
        MLS::Hebrew => HEBREW_LANGS,
    }
}

fn calculate_scores_in_profiles(
    text: &mut Text,
    allow_list: &AllowList,
    lang_profile_list: LangProfileList,
) -> RawOutcome {
    let mut lang_distances: Vec<(Lang, u32)> = vec![];

    let trigrams = get_trigrams_with_positions(&text.lowercase());
    let trigrams_count = trigrams.len();

    for &(lang, lang_trigrams) in lang_profile_list {
        if !allow_list.is_allowed(lang) {
            continue;
        }
        let dist = calculate_distance(lang_trigrams, &trigrams);
        lang_distances.push(((lang), dist));
    }

    // Sort languages by distance
    lang_distances.sort_by_key(|key| key.1);

    let scores = lang_distances
        .iter()
        .map(|&(lang, distance)| (lang, distance_to_score(distance)))
        .collect();

    RawOutcome {
        trigrams_count,
        lang_scores: LangScores::new(scores)
    }
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

fn distance_to_score(distance: u32) -> f64 {
    let similarity = MAX_TOTAL_DISTANCE - distance;
    similarity as f64 / MAX_TRIGRAM_DISTANCE as f64
}
