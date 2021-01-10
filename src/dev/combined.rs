use crate::{Lang, Script};
use crate::trigrams::detection::{calculate_scores_based_on_script, Outcome as TrigramOutcome};
use crate::dev::alphabet::raw_detect_by_alphabet;
use crate::dev::NormalizedOutcome;

fn raw_detect_by_trigrams(text: &str, script: Script) -> TrigramOutcome {
    let options = crate::Options::new();
    calculate_scores_based_on_script(text, &options, script)
}

pub fn detect_by_combined(text: &str, script: Script) -> Option<Lang> {
    let alphabet_outcome = raw_detect_by_alphabet(text, script);
    let alphabet_scores = alphabet_outcome.normalized_scores();

    let trigram_outcome = raw_detect_by_trigrams(text, script);
    let trigram_scores = trigram_outcome.normalized_scores;

    let mut all_langs: Vec<Lang> = alphabet_scores.iter().map(|x| x.0).collect();
    trigram_scores.iter().for_each(|(lang, _)| {
        if !all_langs.contains(lang) {
            all_langs.push(*lang);
        }
    });

    let mut scores = vec![];

    for lang in all_langs {
        let a: f64 = alphabet_scores.iter().find(|(l, _)| l == &lang).map(|x| x.1).unwrap_or(0.0);
        let t: f64 = trigram_scores.iter().find(|(l, _)| l == &lang).map(|x| x.1).unwrap_or(0.0);
        let score = a * t;
        scores.push((lang, score));
    }

    scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Less));
    Some(scores[0].0)
}

