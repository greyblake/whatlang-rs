mod detect;
pub(crate) mod grouping;
mod lang_mapping;
mod script;

pub use self::detect::detect_script;
pub use self::script::Script;
