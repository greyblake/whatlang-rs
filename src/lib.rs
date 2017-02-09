//! Whatlang is a Rust library to detect(regonize) natural languages.
//! Apart from it, the library also recognizes scripts (writing system).
//! Every language and script are represented by determined list of enums.
//!
//! # Examples
//!
//! Using `detect` function:
//!
//! ```
//! use whatlang::{detect, Lang, Script};
//!
//! let info = detect("Ĉu vi ne volas eklerni Esperanton? Bonvolu!").unwrap();
//! assert_eq!(info.lang, Lang::Epo);
//! assert_eq!(info.script, Script::Latin);
//! ```
//!
//! Using `Detector` with specified blacklist or whitelist:
//!
//! ```
//! use whatlang::{Detector, Lang};
//!
//! const WHITELIST : &'static [Lang] = &[Lang::Eng, Lang::Rus];
//!
//! // You can also create detector using with_blacklist function
//! let detector = Detector::with_whitelist(WHITELIST);
//! let lang = detector.detect_lang("There is no reason not to learn Esperanto.");
//! assert_eq!(lang, Some(Lang::Eng));
//! ```

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
