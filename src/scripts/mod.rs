mod detect;
mod lang_mapping;
mod script;
pub(crate) mod grouping;

pub use self::detect::detect_script;
pub use self::script::Script;
