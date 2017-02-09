use lang::Lang;
use script::Script;
use script::detect_script;
use info::Info;

pub struct Detector<'a> {
    whitelist: Option<&'a [Lang]>,
    blacklist: Option<&'a [Lang]>
}

impl<'a> Detector<'a> {
    pub fn new() -> Self {
        Self { whitelist: None, blacklist: None }
    }

    pub fn with_whitelist(whitelist: &'a [Lang]) -> Self {
        Self { whitelist: Some(whitelist), blacklist: None }
    }

    pub fn with_blacklist(blacklist: &'a [Lang]) -> Self {
        Self { whitelist: None, blacklist: Some(blacklist) }
    }

    pub fn detect(text: &str) -> Option<Info> {
        None
    }

    pub fn detect_lang(text: &str) -> Option<Lang> {
        None
    }

    pub fn detect_script(&self, text: &str) -> Option<Script> {
        detect_script(text)
    }
}

#[cfg(test)]
mod tests {
    use super::Detector;
    use super::Script;

    #[test]
    fn test_detect_script() {
        assert_eq!(Detector::new().detect_script("Кириллица"), Some(Script::Cyrillic));
    }
}
