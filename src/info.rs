use lang::Lang;
use script::Script;

/// Represents a full outcome of language detection.
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Info {
    pub(crate) lang: Lang,
    pub(crate) script: Script,
    pub(crate) is_reliable: bool
}

impl Info {
    pub fn lang(&self) -> Lang {
        self.lang
    }

    pub fn script(&self) -> Script {
        self.script
    }

    pub fn is_reliable(&self) -> bool {
        self.is_reliable
    }
}
