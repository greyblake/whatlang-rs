use lang::Lang;
use script::Script;
use script::detect_script;
use info::Info;
use options;
use options::Options;

use detect;

pub struct Detector<'a> {
    options: Options<'a>,
    //whitelist: Option<&'a [Lang]>,
    //blacklist: Option<&'a [Lang]>
}

impl<'a> Detector<'a> {
    pub fn new() -> Self {
        Self { options: options::DEFAULT }
    }

    //pub fn with_whitelist(whitelist: &'a [Lang]) -> Self {
    //    Self { whitelist: Some(whitelist), blacklist: None }
    //}

    //pub fn with_blacklist(blacklist: &'a [Lang]) -> Self {
    //    Self { whitelist: None, blacklist: Some(blacklist) }
    //}

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
