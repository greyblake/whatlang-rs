use super::Script;
use crate::Lang;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MultiLangScript {
    Latin,
    Cyrillic,
    Arabic,
    Devanagari,
    Hebrew,
}

impl MultiLangScript {
    pub fn to_script(self) -> Script {
        match self {
            Self::Latin => Script::Latin,
            Self::Cyrillic => Script::Cyrillic,
            Self::Arabic => Script::Arabic,
            Self::Devanagari => Script::Devanagari,
            Self::Hebrew => Script::Hebrew,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScriptLangGroup {
    Multi(MultiLangScript),
    One(Lang),

    // Mandarin writing can be japanese in some cases.
    // For now it's a hack.
    // See https://github.com/greyblake/whatlang-rs/pull/45
    Mandarin,
}

impl Script {
    pub fn to_lang_group(self) -> ScriptLangGroup {
        use MultiLangScript as MLS;
        use ScriptLangGroup::{Mandarin, Multi, One};

        match self {
            Script::Latin => Multi(MLS::Latin),
            Script::Cyrillic => Multi(MLS::Cyrillic),
            Script::Arabic => Multi(MLS::Arabic),
            Script::Devanagari => Multi(MLS::Devanagari),
            Script::Hebrew => Multi(MLS::Hebrew),
            Script::Mandarin => Mandarin,
            Script::Bengali => One(Lang::Ben),
            Script::Hangul => One(Lang::Kor),
            Script::Georgian => One(Lang::Kat),
            Script::Greek => One(Lang::Ell),
            Script::Kannada => One(Lang::Kan),
            Script::Tamil => One(Lang::Tam),
            Script::Thai => One(Lang::Tha),
            Script::Gujarati => One(Lang::Guj),
            Script::Gurmukhi => One(Lang::Pan),
            Script::Telugu => One(Lang::Tel),
            Script::Malayalam => One(Lang::Mal),
            Script::Oriya => One(Lang::Ori),
            Script::Myanmar => One(Lang::Mya),
            Script::Sinhala => One(Lang::Sin),
            Script::Khmer => One(Lang::Khm),
            Script::Ethiopic => One(Lang::Amh),
            Script::Katakana | Script::Hiragana => One(Lang::Jpn),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_script_to_lang_group() {
        assert_eq!(
            Script::Latin.to_lang_group(),
            ScriptLangGroup::Multi(MultiLangScript::Latin)
        );

        assert_eq!(
            Script::Georgian.to_lang_group(),
            ScriptLangGroup::One(Lang::Kat)
        );
    }

    #[test]
    fn test_multi_lang_script_to_script() {
        assert_eq!(MultiLangScript::Latin.to_script(), Script::Latin);
        assert_eq!(MultiLangScript::Hebrew.to_script(), Script::Hebrew);
    }
}
