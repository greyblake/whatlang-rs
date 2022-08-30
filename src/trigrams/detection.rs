use hashbrown::HashMap;

use super::utils::{get_trigrams_with_positions, TrigramsWithPositions};
use super::{LangProfile, LangProfileList};
use super::{Trigram, MAX_TOTAL_DISTANCE, MAX_TRIGRAM_DISTANCE};
use super::{ARABIC_LANGS, CYRILLIC_LANGS, DEVANAGARI_LANGS, HEBREW_LANGS, LATIN_LANGS};
use crate::core::{calculate_confidence, FilterList, Info, InternalQuery, Text};
use crate::scripts::grouping::MultiLangScript;
use crate::Lang;

#[derive(Debug)]
pub struct RawOutcome {
    pub trigrams_count: usize,
    pub raw_distances: Vec<(Lang, u32)>,
    pub scores: Vec<(Lang, f64)>,
}

#[inline]
pub fn detect(iquery: &InternalQuery) -> Option<Info> {
    let raw_outcome = raw_detect(iquery);
    let RawOutcome {
        trigrams_count,
        scores,
        ..
    } = raw_outcome;

    let mut raw_scores_iter = scores.into_iter();

    let opt_lang_score1 = raw_scores_iter.next();
    let opt_lang_score2 = raw_scores_iter.next();

    // TODO: Logic is duplicated in alphabets. Consider refactoring
    opt_lang_score1.map(|(lang1, score1)| {
        let script = iquery.multi_lang_script.to_script();
        let confidence = if let Some((_, score2)) = opt_lang_score2 {
            calculate_confidence(score1, score2, trigrams_count)
        } else {
            1.0
        };
        Info::new(script, lang1, confidence)
    })
}

#[inline]
pub fn raw_detect(iquery: &InternalQuery) -> RawOutcome {
    let lang_profile_list = script_to_lang_profile_list(iquery.multi_lang_script);
    calculate_scores_in_profiles(&iquery.text, iquery.filter_list, lang_profile_list)
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

#[inline]
fn calculate_scores_in_profiles(
    text: &Text,
    filter_list: &FilterList,
    lang_profile_list: LangProfileList,
) -> RawOutcome {
    let mut lang_distances: Vec<(Lang, u32)> = vec![];

    let TrigramsWithPositions {
        trigram_positions, ..
    } = get_trigrams_with_positions(&text.lowercase());
    let unique_trigrams_count = trigram_positions.len();

    for &(lang, lang_trigrams) in lang_profile_list {
        if !filter_list.is_allowed(lang) {
            continue;
        }
        let dist = calculate_distance(lang_trigrams, &trigram_positions);
        lang_distances.push(((lang), dist));
    }

    // Sort languages by distance
    lang_distances.sort_unstable_by_key(|(_, dist)| *dist);

    let max_dist = unique_trigrams_count as u32 * MAX_TRIGRAM_DISTANCE;

    let raw_scores = lang_distances
        .iter()
        .map(|&(lang, distance)| (lang, distance_to_raw_score(distance, max_dist)))
        .collect();

    RawOutcome {
        trigrams_count: unique_trigrams_count,
        scores: raw_scores,
        raw_distances: lang_distances,
    }
}

#[inline]
fn calculate_distance(lang_trigrams: LangProfile, text_trigrams: &HashMap<Trigram, u32>) -> u32 {
    let mut total_dist = 0u32;

    for (i, &trigram) in lang_trigrams.iter().enumerate() {
        let dist = match text_trigrams.get(&trigram) {
            Some(&n) => (n as i32 - i as i32).unsigned_abs(),
            None => MAX_TRIGRAM_DISTANCE,
        };
        total_dist += dist;
    }

    let count = text_trigrams.len() as u32;

    if MAX_TRIGRAM_DISTANCE > count {
        let delta = MAX_TRIGRAM_DISTANCE - count;
        total_dist -= delta * MAX_TRIGRAM_DISTANCE;
    }

    total_dist.clamp(0, MAX_TOTAL_DISTANCE)
}

#[inline]
fn distance_to_raw_score(distance: u32, max_distance: u32) -> f64 {
    let similarity = max_distance - distance;
    similarity as f64 / max_distance as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_when_german_is_given() {
        let text = "Die Ordnung muss für immer in diesem Codebase bleiben";
        let iq = InternalQuery {
            text: Text::new(text),
            filter_list: &FilterList::default(),
            multi_lang_script: MultiLangScript::Latin,
        };
        let raw_outcome = raw_detect(&iq);

        assert_eq!(raw_outcome.trigrams_count, 50);

        let &(first_lang, first_score) = raw_outcome.scores.first().unwrap();
        let &(_last_lang, last_score) = raw_outcome.scores.last().unwrap();

        assert_eq!(first_lang, Lang::Deu);

        assert!(first_score >= 0.0);
        assert!(first_score <= 1.0);

        assert!(last_score >= 0.0);
        assert!(last_score <= 1.0);
    }
}
