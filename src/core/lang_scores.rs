use crate::Lang;

pub struct LangScores {
    pub(crate) scores: Vec<(Lang, f64)>
}

impl LangScores {
    pub fn new(scores: Vec<(Lang, f64)>) -> Self {
        Self { scores }
    }
}
