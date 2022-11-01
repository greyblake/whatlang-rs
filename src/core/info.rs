use crate::{Lang, Script};

const RELIABLE_CONFIDENCE_THRESHOLD: f64 = 0.9;

/// Represents a full outcome of language detection.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_realiable() {
        let mut info = Info {
            script: Script::Greek,
            lang: Lang::Ell,
            confidence: 0.0,
        };
        assert_eq!(info.is_reliable(), false);

        info.confidence = 1.0;
        assert_eq!(info.is_reliable(), true);
    }
}
