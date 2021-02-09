use crate::alphabets;
use crate::core::{Info, InternalQuery, LangScores};
use crate::trigrams;
use crate::Lang;

pub fn detect(iquery: &mut InternalQuery) -> Option<Info> {
    let lang_scores = raw_detect(iquery);
    lang_scores.scores.first().map(|&(lang, _)| {
        let script = iquery.multi_lang_script.to_script();
        Info::new(script, lang)
    })
}

// TODO: optimize!
pub fn raw_detect(iquery: &mut InternalQuery) -> LangScores {
    let alphabet_scores = alphabets::raw_detect(iquery).scores;
    let trigram_scores = trigrams::raw_detect(iquery).lang_scores.scores;

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
