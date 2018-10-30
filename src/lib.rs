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
extern crate hashbrown;

mod lang;
mod script;
mod info;
mod utils;
mod trigrams;
mod detect;
mod detector;
mod options;
mod constants;

pub use lang::Lang;
pub use script::Script;
pub use info::Info;
pub use detector::Detector;
pub use options::Options;

pub use detect::detect;
pub use detect::detect_lang;
pub use detect::detect_with_options;
pub use script::detect_script;
