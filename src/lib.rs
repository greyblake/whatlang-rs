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
//! let text = "Ĉu vi ne volas eklerni Esperanton? Bonvolu! Estas unu de la plej bonaj aferoj!";
//! let info = detect(text).unwrap();
//! assert_eq!(info.lang(), Lang::Epo);
//! assert_eq!(info.script(), Script::Latin);
//!
//! // Confidence is in the range from 0 to 1.
//! assert_eq!(info.confidence(), 1.0);
//! assert!(info.is_reliable());
//! ```
//!
//! Using `Detector` with specified blacklist or whitelist:
//!
//! ```
//! use whatlang::{Detector, Lang};
//!
//! let whitelist = vec![Lang::Eng, Lang::Rus];
//!
//! // You can also create detector using with_blacklist function
//! let detector = Detector::with_whitelist(whitelist);
//! let lang = detector.detect_lang("There is no reason not to learn Esperanto.");
//! assert_eq!(lang, Some(Lang::Eng));
//! ```
//!
mod constants;
mod detect;
mod detector;
mod error;
mod info;
mod lang;
mod options;
mod scripts;
mod trigrams;
mod utils;

pub use crate::detector::Detector;
pub use crate::info::Info;
pub use crate::lang::Lang;
pub use crate::options::Options;
pub use crate::scripts::Script;

pub use crate::detect::detect;
pub use crate::detect::detect_lang;
pub use crate::detect::detect_with_options;
pub use crate::scripts::detect_script;
