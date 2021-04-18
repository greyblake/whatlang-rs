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
