mod cyrillic;
pub(crate) mod detection;
mod latin;

pub use detection::{detect, raw_detect};

use crate::Lang;

#[derive(Debug)]
pub struct RawOutcome {
    pub count: usize,
    pub raw_scores: Vec<(Lang, usize)>,
    pub scores: Vec<(Lang, f64)>,
}

fn normalize_score(raw_score: usize, max_raw_score: usize) -> f64 {
    if raw_score == 0 {
        0.0
    } else {
        raw_score as f64 / max_raw_score as f64
    }
}
