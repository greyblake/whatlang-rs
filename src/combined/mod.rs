use crate::alphabets;
use crate::core::{calculate_confidence, Info, InternalQuery};
use crate::trigrams;
use crate::Lang;

#[derive(Debug)]
pub struct RawOutcome {
    pub scores: Vec<(Lang, f64)>,
    #[allow(dead_code)]
    pub alphabet_raw_outcome: alphabets::RawOutcome,
    pub trigram_raw_outcome: trigrams::RawOutcome,
}

pub fn detect(iquery: &InternalQuery) -> Option<Info> {
    let raw_outcome = raw_detect(iquery);

    let count = raw_outcome.trigram_raw_outcome.trigrams_count;
    let mut normalized_scores_iter = raw_outcome.scores.into_iter();

    let opt_lang_score1 = normalized_scores_iter.next();
    let opt_lang_score2 = normalized_scores_iter.next();

    // TODO: Logic is duplicated in alphabets and trigrams. Consider refactoring
    opt_lang_score1.map(|(lang1, score1)| {
        let script = iquery.multi_lang_script.to_script();
        let confidence = if let Some((_, score2)) = opt_lang_score2 {
            calculate_confidence(score1, score2, count)
        } else {
            1.0
        };
        Info::new(script, lang1, confidence)
    })
}

// TODO: optimize!
pub fn raw_detect(iquery: &InternalQuery) -> RawOutcome {
    let alphabet_raw_outcome: alphabets::RawOutcome = alphabets::raw_detect(iquery);
    let trigram_raw_outcome: trigrams::RawOutcome = trigrams::raw_detect(iquery);

    let alphabet_scores: &Vec<(Lang, f64)> = &alphabet_raw_outcome.scores;
    let trigram_scores: &Vec<(Lang, f64)> = &trigram_raw_outcome.scores;

    let mut all_langs: Vec<Lang> = alphabet_scores.iter().map(|x| x.0).collect();
    for (lang, _) in trigram_scores.iter() {
        if !all_langs.contains(lang) {
            all_langs.push(*lang);
        }
    }

    let count = alphabet_raw_outcome.count;

    let alphabet_weight = calc_alphabet_weight(count);
    let trigram_weight = 1.0 - alphabet_weight;

    let mut scores = Vec::with_capacity(all_langs.len());

    for lang in all_langs {
        let a: f64 = alphabet_scores
            .iter()
            .find(|(l, _)| l == &lang)
            .map(|x| x.1)
            .unwrap_or(0.0);
        let t: f64 = trigram_scores
            .iter()
            .find(|(l, _)| l == &lang)
            .map(|x| x.1)
            .unwrap_or(0.0);

        debug_assert!(a >= 0.0);
        debug_assert!(a <= 1.0);
        debug_assert!(t >= 0.0);
        debug_assert!(t <= 1.0);

        let score = a * alphabet_weight + t * trigram_weight;

        debug_assert!(score >= 0.0);
        debug_assert!(score <= 1.0);

        scores.push((lang, score));
    }

    scores.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Less));

    RawOutcome {
        scores,
        alphabet_raw_outcome,
        trigram_raw_outcome,
    }
}

// Function that calculates weight of alphabet score depending on number of characters in the given
// text. The longer text the less significant alphabet weight is (and the more significant is
// trigram weight).
//
// y = -(x/300) + 2/3
// where:
//   x - number of characters
//   y = alphabet weight
//
//
//          alphabet weight
//          ^
//          |
//     2/3 -* (0; 2/3)
//          | \_
//          |   \_
//          |     \_
//          |       \_
//          |         \  (100; 1/3)
//     1/3 -|          *------------
//          |
//          +----------|------------> count
//         0          100
//
fn calc_alphabet_weight(count: usize) -> f64 {
    let weight = -(count as f64 / 300.0) + 2.0 / 3.0;
    weight.clamp(1.0 / 3.0, 2.0 / 3.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_alphabet_weight() {
        assert_eq!(calc_alphabet_weight(0), 2.0 / 3.0);
        assert_eq!(calc_alphabet_weight(50), 0.5);
        assert_eq!(calc_alphabet_weight(100), 1.0 / 3.0);
        assert_eq!(calc_alphabet_weight(200), 1.0 / 3.0);
    }
}
