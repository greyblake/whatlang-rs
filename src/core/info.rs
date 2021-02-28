use crate::{Lang, Script};

const RELIABLE_CONFIDENCE_THRESHOLD: f64 = 0.9;

#[derive(Debug, PartialEq)]
pub struct Info {
    script: Script,
    lang: Lang,
    confidence: f64,
}

impl Info {
    pub fn new(script: Script, lang: Lang, confidence: f64) -> Self {
        Self {
            script,
            lang,
            confidence,
        }
    }

    pub fn lang(&self) -> Lang {
        self.lang
    }

    pub fn script(&self) -> Script {
        self.script
    }

    pub fn confidence(&self) -> f64 {
        self.confidence
    }

    pub fn is_reliable(&self) -> bool {
        self.confidence > RELIABLE_CONFIDENCE_THRESHOLD
    }
}
