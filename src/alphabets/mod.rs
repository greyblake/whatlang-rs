mod cyrillic;
mod detection;
mod latin;

pub use detection::{detect, raw_detect};

use crate::Lang;

pub struct RawOutcome {
    pub count: usize,
    pub raw_scores: Vec<(Lang, usize)>,
    pub scores: Vec<(Lang, f64)>,
}
