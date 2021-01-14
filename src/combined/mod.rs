use crate::trigrams::raw_detect as trigrams_raw_detect;
use crate::alphabets::raw_detect as alphabets_raw_detect;
use crate::core::{Output, InternalQuery, LangScores};
use crate::Lang;

pub fn detect(iquery: &mut InternalQuery) -> Option<Output> {
    let lang_scores = raw_detect(iquery);
    lang_scores.scores.first().map( |&(lang, _)| {
        let script = iquery.multi_lang_script.to_script();
        Output::new(script, lang)
    })
}

// TODO: optimize!
pub fn raw_detect(iquery: &mut InternalQuery) -> LangScores {
    let alphabet_scores = alphabets_raw_detect(iquery).scores;
    let trigram_scores = trigrams_raw_detect(iquery).lang_scores.scores;

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
    LangScores::new(scores)
}
