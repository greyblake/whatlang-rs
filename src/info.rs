use lang::Lang;
use script::Script;

/// Represents a full outcome of language detection.
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Info {
    pub lang: Lang,
    pub script: Script,
    pub is_reliable: bool
}
