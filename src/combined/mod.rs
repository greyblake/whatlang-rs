use crate::alphabets;
use crate::core::{calculate_confidence, Info, InternalQuery};
use crate::trigrams;
use crate::Lang;

#[derive(Debug)]
pub struct RawOutcome {
    scores: Vec<(Lang, f64)>,
    alphabet_raw_outcome: alphabets::RawOutcome,
    trigram_raw_outcome: trigrams::RawOutcome,
}

pub fn detect(iquery: &mut InternalQuery) -> Option<Info> {
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
pub fn raw_detect(iquery: &mut InternalQuery) -> RawOutcome {
    let alphabet_raw_outcome = alphabets::raw_detect(iquery);
    let trigram_raw_outcome = trigrams::raw_detect(iquery);

    let alphabet_scores = &alphabet_raw_outcome.scores;

    // TODO: Use normalized scores here
    let trigram_scores = &trigram_raw_outcome.scores;

    let mut all_langs: Vec<Lang> = alphabet_scores.iter().map(|x| x.0).collect();
    trigram_scores.iter().for_each(|(lang, _)| {
        if !all_langs.contains(lang) {
            all_langs.push(*lang);
        }
    });

    let mut scores = vec![];

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
        // NOTE: Magically adding big multiplier to trigram score
        // improves overall result;
        // * For cyrillic: 1000.0
        // * For latin: 100.0
        let score = a * (t + 100.0);
        scores.push((lang, score));
    }

    scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Less));

    RawOutcome {
        scores,
        alphabet_raw_outcome,
        trigram_raw_outcome,
    }
}
