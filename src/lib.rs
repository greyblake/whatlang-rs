mod lang;
mod script;
mod info;
mod utils;
mod trigrams;
mod detect;
mod detector;
mod options;

pub use lang::Lang;
pub use script::Script;
pub use info::Info;
pub use detector::Detector;

pub use detect::detect;
pub use detect::detect_lang;
pub use script::detect_script;
