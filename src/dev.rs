//! This mod exposes some internal API.
//! It exists only to enable tuning of the library with extra supporting tools (e.g. benchmarks).
//! Developers are advised against relying on API.
//!
pub use crate::alphabets::{raw_detect as alphabets_raw_detect, RawOutcome as RawAlphabetsInfo};
pub use crate::combined::{raw_detect as combined_raw_detect, RawOutcome as RawCombinedInfo};
pub use crate::core::{detect, detect_lang, detect_with_options, Detector, Info, Method, Options};
pub use crate::lang::Lang;
pub use crate::scripts::{detect_script, raw_detect_script, RawScriptInfo, Script};
pub use crate::trigrams::{raw_detect as trigrams_raw_detect, RawOutcome as RawTrigramsInfo};

pub use crate::alphabets::cyrillic::alphabet_calculate_scores as alphabet_cyrillic_calculate_scores;
pub use crate::alphabets::latin::alphabet_calculate_scores as alphabet_latin_calculate_scores;
pub use crate::core::{FilterList, LowercaseText};

// private imports
use crate::core::detect::detect_lang_base_on_mandarin_script;
use crate::core::Query;
use crate::scripts::grouping::ScriptLangGroup;

#[derive(Debug)]
pub struct RawInfo {
    pub script_info: RawScriptInfo,
    pub lang_info: Option<RawLangInfo>,
}

#[derive(Debug)]
pub enum RawLangInfo {
    OneScript(Lang),
    MultiScript(RawCombinedInfo),
    Mandarin(Lang),
}

pub fn raw_detect(text: &str) -> RawInfo {
    let script_info = raw_detect_script(text);

    let query = Query {
        text,
        filter_list: &FilterList::default(),
        method: Method::Combined,
    };

    let lang_info = script_info
        .main_script()
        .map(|script| match script.to_lang_group() {
            ScriptLangGroup::One(lang) => RawLangInfo::OneScript(lang),
            ScriptLangGroup::Multi(multi_lang_script) => {
                let iquery = query.to_internal(multi_lang_script);
                let combined = combined_raw_detect(&iquery);
                RawLangInfo::MultiScript(combined)
            }
            ScriptLangGroup::Mandarin => {
                let lang = detect_lang_base_on_mandarin_script(&query, &script_info).lang();
                RawLangInfo::Mandarin(lang)
            }
        });

    RawInfo {
        script_info,
        lang_info,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_raw_detect() {
        let zapovit = r#"
            Як умру, то поховайте
            Мене на могилі,
            Серед степу широкого,
            На Вкраїні милій,
            Щоб лани широкополі,
            І Дніпро, і кручі
            Було видно, було чути,
            Як реве ревучий.
        "#;
        let info = raw_detect(&zapovit);
        assert_eq!(info.script_info.counters[0].0, Script::Cyrillic);
    }
}
