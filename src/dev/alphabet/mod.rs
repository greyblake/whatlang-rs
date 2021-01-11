mod cyrillic;
mod latin;

use crate::dev::NormalizedOutcome;
use crate::{Lang, Script};

impl NormalizedOutcome for Outcome {
    fn normalized_scores(&self) -> &[(Lang, f64)] {
        &self.normalized_scores
    }
}

#[derive(Debug)]
pub struct Outcome {
    max_raw_score: usize,
    raw_scores: Vec<(Lang, usize)>,
    normalized_scores: Vec<(Lang, f64)>,
}

impl Outcome {
    fn new_empty() -> Self {
        Self {
            max_raw_score: 1,
            raw_scores: vec![],
            normalized_scores: vec![],
        }
    }
}

pub fn detect_by_alphabet(text: &str, script: Script) -> Option<Lang> {
    let outcome = raw_detect_by_alphabet(text, script);
    let scores = outcome.normalized_scores();
    if scores.is_empty() {
        None
    } else {
        Some(scores[0].0)
    }
}

pub fn raw_detect_by_alphabet(text: &str, script: Script) -> Outcome {
    match script {
        Script::Cyrillic => cyrillic::alphabet_calculate_scores(text),
        Script::Latin => latin::alphabet_calculate_scores(text),
        _ => Outcome::new_empty(),
    }
}
