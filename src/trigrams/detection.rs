use hashbrown::HashMap;

use super::utils::get_trigrams_with_positions;
use super::{LangProfile, LangProfileList};
use super::{Trigram, MAX_TOTAL_DISTANCE, MAX_TRIGRAM_DISTANCE};
use super::{ARABIC_LANGS, CYRILLIC_LANGS, DEVANAGARI_LANGS, HEBREW_LANGS, LATIN_LANGS};
use crate::core::{calculate_confidence, AllowList, Info, InternalQuery, Text};
use crate::scripts::grouping::MultiLangScript;
use crate::Lang;

#[derive(Debug)]
pub struct RawOutcome {
    pub trigrams_count: usize,
    pub raw_distances: Vec<(Lang, u32)>,
    pub scores: Vec<(Lang, f64)>,
}

pub fn detect(iquery: &mut InternalQuery) -> Option<Info> {
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

    let raw_scores = lang_distances
        .iter()
        .map(|&(lang, distance)| (lang, distance_to_raw_score(distance)))
        .collect();

    // TODO: CALCULATE NORMALIZED SCORES

    RawOutcome {
        trigrams_count,
        scores: raw_scores,
        raw_distances: lang_distances,
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

fn distance_to_raw_score(distance: u32) -> f64 {
    let similarity = MAX_TOTAL_DISTANCE - distance;
    similarity as f64 / MAX_TOTAL_DISTANCE as f64
}

// fn min_perfect_distance(count_trigrams: usize) -> usize {
//     // 1 = 0 + 299*300
//     // 2 = 0 + 1 + 298*300
//     // 3 = 0 + 0 + 298*300
//     // 4 = 3 + 2 + 1
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_when_german_is_given() {
        let text = "Die Ordnung muss für immer in diesem Codebase bleiben";
        let mut iq = InternalQuery {
            text: Text::new(text),
            allow_list: &AllowList::all(),
            multi_lang_script: MultiLangScript::Latin,
        };
        let raw_outcome = raw_detect(&mut iq);

        assert_eq!(raw_outcome.trigrams_count, 50);

        let &(first_lang, first_score) = raw_outcome.scores.first().unwrap();
        let &(_last_lang, last_score) = raw_outcome.scores.last().unwrap();

        assert_eq!(first_lang, Lang::Deu);

        assert!(first_score >= 0.0);
        assert!(first_score <= 1.0);

        assert!(last_score >= 0.0);
        assert!(last_score <= 1.0);

        //println!("{:#?}", raw_outcome);
    }
}
