mod cyrillic;

use crate::{Lang, Script};

pub trait NormalizedOutcome {
    fn normalized_scores(&self) -> &[(Lang, f64)];
}

pub fn detect_by_alphabet(text: &str, script: Script) -> Option<Lang> {
    match script {
        Script::Cyrillic => {
            let outcome = cyrillic::alphabet_calculate_scores(text);
            let first = outcome.normalized_scores()[0];
            Some(first.0)
        }
        _ => None,
    }
}
