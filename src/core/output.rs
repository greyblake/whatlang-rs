use crate::{Lang, Script};

// TODO:
// Find a better name?:
// * Response
// * Info
// * DetectionResult ?
pub struct Output {
    script: Script,
    lang: Lang,
}

impl Output {
    pub fn new(script: Script, lang: Lang) -> Self {
        Self { script, lang }
    }
}
