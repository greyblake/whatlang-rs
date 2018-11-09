use lang::Lang;
use script::Script;

const RELIABLE_CONFIDENCE_THRESHOLD: f64 = 0.8;

/// Represents a full outcome of language detection.
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Info {
    pub(crate) lang: Lang,
    pub(crate) script: Script,
    pub(crate) confidence: f64,
}

impl Info {
    pub fn lang(&self) -> Lang {
        self.lang
    }

    pub fn script(&self) -> Script {
        self.script
    }

    pub fn is_reliable(&self) -> bool {
        self.confidence > RELIABLE_CONFIDENCE_THRESHOLD
    }

    pub fn confidence(&self) -> f64 {
        self.confidence
    }
}
