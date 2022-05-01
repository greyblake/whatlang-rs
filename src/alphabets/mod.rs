//! Alphabet method is very inaccurate by itself to determine a language, but it supports
//! other methods (trigrams) and allows to improve accuracy on very short texts.
//!
//! It's based on the fact, that some languages may use characters that are not present in others.
//! (e.g. character `ö` is common for German or Finish texts, but not really expected to be
//! seen in English or Spanish).
//!
//! ## Alogirthm
//!
//! * For every single character `c` in a lowecased text:
//!   * If `c` is relevant for the script?
//!     * count += 1
//!     * for every language where `c` is a commonly used character
//!       * raw_lang_score += 1
//! * Normalize scores for every language:
//!   * lang_score = raw_lang_score / count
//!
//! ## Pros and cons
//!
//! This algorithm is very simple and fast, but has also comes with some disadvantages:
//! * Character frequencies are not respected (e.g. letter `O` has 7.16% occurrence rate in English and only
//! 2.24% in German, but according to this alphabet model, both English and German would get +1
//!   score for letter `O`).
//! * It does not work always well: here are some examples:
//!   * `Can you tell me where is Schönheitstraße?` - it's cleary an English sentence with a German
//!   proper name `Schönheitstraße`, but the model gives 34 scores to German and only 30 to
//!   English, because of the letters `ö` and `ß`, which are present in German, but not in English.
//!   * `Façade` - is a valid English word. But English gets panished because of the untypical
//!   character `ç` that was inherited from French.
//!
//! The generic (agnostic to a script) implementation and algorithm can be found in the [common] module.

pub(crate) mod common;
pub(crate) mod cyrillic;
pub(crate) mod detection;
pub(crate) mod latin;

pub use detection::{detect, raw_detect};
use crate::Lang;

#[derive(Debug)]
pub struct RawOutcome {
    pub count: usize,
    pub raw_scores: Vec<(Lang, usize)>,
    pub scores: Vec<(Lang, f64)>,
}
