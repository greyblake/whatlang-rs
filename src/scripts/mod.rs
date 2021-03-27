mod detect;
pub(crate) mod grouping;
mod lang_mapping;
mod script;

pub use self::detect::detect_script;
pub(crate) use self::detect::{raw_detect_script, RawScriptInfo};
pub use self::script::Script;
