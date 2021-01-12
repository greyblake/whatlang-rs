mod alphabet;
mod combined;
//mod core;

use crate::Lang;
use crate::core::Method;

pub trait NormalizedOutcome {
    fn normalized_scores(&self) -> &[(Lang, f64)];
}

pub fn detect_by_method(text: &str, method: Method) -> Option<Lang> {
    if let Some(script) = crate::detect_script(text) {
        match method {
            Method::Trigram => crate::detect_lang(text),
            Method::Alphabet => alphabet::detect_by_alphabet(text, script),
            Method::Combined => combined::detect_by_combined(text, script),
        }
    } else {
        None
    }
}
