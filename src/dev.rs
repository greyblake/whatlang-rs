//! This mod exposes some internal API.
//! It exists only to enable tuning of the library with extra supporting tools (e.g. benchmarks).
//! Developers are advised against relying on API.
//!
pub use crate::core::{detect, detect_lang, detect_with_options, Detector, Info, Method, Options};
pub use crate::lang::Lang;
pub use crate::scripts::{detect_script, Script};
