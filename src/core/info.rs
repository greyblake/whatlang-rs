use crate::{Lang, Script};

// TODO:
// Find a better name?:
// * Response
// * Info
// * DetectionResult ?
pub struct Info {
    script: Script,
    lang: Lang,
}

impl Info {
    pub fn new(script: Script, lang: Lang) -> Self {
        Self { script, lang }
    }

    pub fn lang(&self) -> Lang {
        self.lang
    }

    pub fn script(&self) -> Script {
        self.script
    }
}
