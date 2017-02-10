use lang::Lang;
use script::Script;
use script::detect_script;
use info::Info;
use options;
use options::Options;

use detect;

/// Configurable structure that holds detection options and provides functions
/// to detect language and script.
/// # Examples
/// Specifying a whitelist:
///
/// ```
/// use whatlang::{Detector, Lang};
///
/// const WHITELIST : &'static [Lang] = &[Lang::Eng, Lang::Rus];
///
/// // Create detector with whitelist
/// let detector = Detector::with_whitelist(WHITELIST);
/// let lang = detector.detect_lang("That is not Russian");
/// assert_eq!(lang, Some(Lang::Eng));
/// ```
///
/// Specifying a blacklist:
///
/// ```
/// use whatlang::{Detector, Lang};
///
/// const BLACKLIST: &'static [Lang] = &[Lang::Eng, Lang::Ita];
///
/// let detector = Detector::with_blacklist(BLACKLIST);
/// let lang = detector.detect_lang("Jen la trinkejo fermitis, ni iras tra mallumo kaj pluvo.");
/// assert_eq!(lang, Some(Lang::Epo));
/// ```
pub struct Detector<'a> {
    options: Options<'a>,
}

impl<'a> Detector<'a> {
    pub fn new() -> Self {
        Detector { options: options::DEFAULT }
    }

    pub fn with_whitelist(whitelist: &'a [Lang]) -> Self {
        let opts = Options { whitelist: Some(whitelist), blacklist: None };
        Detector { options: opts }
    }

    pub fn with_blacklist(blacklist: &'a [Lang]) -> Self {
        let opts = Options { whitelist: None, blacklist: Some(blacklist) };
        Detector { options: opts }
    }

    pub fn detect(&self, text: &str) -> Option<Info> {
        detect::detect_with_options(text, &self.options)
    }

    pub fn detect_lang(&self, text: &str) -> Option<Lang> {
        detect::detect_lang_with_options(text, &self.options)
    }

    pub fn detect_script(&self, text: &str) -> Option<Script> {
        detect_script(text)
    }
}

#[cfg(test)]
mod tests {
    use super::Detector;
    use super::Script;
    use super::Lang;

    #[test]
    fn test_detect_script() {
        // Russian, Cyrillic
        assert_eq!(Detector::new().detect_script("Кириллица"), Some(Script::Cyrillic));
    }

    #[test]
    fn test_detect_lang() {
        // Esperanto
        let text = "Ĉiuj redaktantoj de Esperanta Vikipedio estas volontuloj. Ili partoprenas en la kunlaborema komunumo, sen estro, kie la anoj kunordigas siajn strebojn kadre de temaj projektoj kaj pluraj diskutejoj. Ili sekvas la bazajn regulojn establitaj de la komunumo, ekzemple kontrolebleco de la informo aŭ la menciindeco de la temo.";
        assert_eq!(Detector::new().detect_lang(text), Some(Lang::Epo));
    }

    #[test]
    fn test_detect() {
        // Esperanto
        let text = "Ĉiuj redaktantoj de Esperanta Vikipedio estas volontuloj. Ili partoprenas en la kunlaborema komunumo, sen estro, kie la anoj kunordigas siajn strebojn kadre de temaj projektoj kaj pluraj diskutejoj. Ili sekvas la bazajn regulojn establitaj de la komunumo, ekzemple kontrolebleco de la informo aŭ la menciindeco de la temo.";
        let output = Detector::new().detect(text);
        assert_eq!(output.is_some(), true);

        let info = output.unwrap();
        assert_eq!(info.lang, Lang::Epo);
        assert_eq!(info.script, Script::Latin);
    }
}
