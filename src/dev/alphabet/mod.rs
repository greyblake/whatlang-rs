mod cyrillic;

use crate::{Script, Lang};

pub fn detect_by_alphabet(text: &str, script: Script) -> Option<Lang> {
    match script {
        Script::Cyrillic => {
            let mut scores = cyrillic::alphabet_calculate_scores(text);
            scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Less));
            Some(scores[0].0)
        },
        _ => {
            // eprintln!("detect_by_alphabet() for script {} is not yet implemented", script);
            // println!("{}", text);
            None
        }
    }
}
