use std::fmt;
use std::str::FromStr;

use super::lang_mapping;
use crate::Lang;
use crate::error::ParseError;

/// Represents a writing system (Latin, Cyrillic, Arabic, etc).
#[cfg_attr(feature = "enum-map", derive(::enum_map::Enum))]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub enum Script {
    // Keep this in alphabetic order (for C bindings)
    Arabic,
    Armenian,
    Bengali,
    Cyrillic,
    Devanagari,
    Ethiopic,
    Georgian,
    Greek,
    Gujarati,
    Gurmukhi,
    Hangul,
    Hebrew,
    Hiragana,
    Kannada,
    Katakana,
    Khmer,
    Latin,
    Malayalam,
    Mandarin,
    Myanmar,
    Oriya,
    Sinhala,
    Tamil,
    Telugu,
    Thai,
}

// Array of all existing Script values.
const VALUES: [Script; 25] = [
    Script::Arabic,
    Script::Armenian,
    Script::Bengali,
    Script::Cyrillic,
    Script::Devanagari,
    Script::Ethiopic,
    Script::Georgian,
    Script::Greek,
    Script::Gujarati,
    Script::Gurmukhi,
    Script::Hangul,
    Script::Hebrew,
    Script::Hiragana,
    Script::Kannada,
    Script::Katakana,
    Script::Khmer,
    Script::Latin,
    Script::Malayalam,
    Script::Mandarin,
    Script::Myanmar,
    Script::Oriya,
    Script::Sinhala,
    Script::Tamil,
    Script::Telugu,
    Script::Thai,
];

impl Script {
    /// Get all existing scripts.
    ///
    /// # Example
    /// ```
    /// use whatlang::Script;
    /// for script in Script::all() {
    ///     println!("{}", script);
    /// }
    /// ```
    pub fn all() -> &'static [Script] {
        &VALUES
    }

    pub fn name(&self) -> &str {
        match *self {
            Script::Latin => "Latin",
            Script::Cyrillic => "Cyrillic",
            Script::Arabic => "Arabic",
            Script::Devanagari => "Devanagari",
            Script::Hiragana => "Hiragana",
            Script::Katakana => "Katakana",
            Script::Ethiopic => "Ethiopic",
            Script::Hebrew => "Hebrew",
            Script::Bengali => "Bengali",
            Script::Georgian => "Georgian",
            Script::Mandarin => "Mandarin",
            Script::Hangul => "Hangul",
            Script::Greek => "Greek",
            Script::Kannada => "Kannada",
            Script::Tamil => "Tamil",
            Script::Thai => "Thai",
            Script::Gujarati => "Gujarati",
            Script::Gurmukhi => "Gurmukhi",
            Script::Telugu => "Telugu",
            Script::Malayalam => "Malayalam",
            Script::Oriya => "Oriya",
            Script::Myanmar => "Myanmar",
            Script::Sinhala => "Sinhala",
            Script::Khmer => "Khmer",
            Script::Armenian => "Armenian",
        }
    }

    pub fn langs(&self) -> &[Lang] {
        lang_mapping::script_langs(*self)
    }

    /// Returns the [bcp47] subtag as [registerd with IANA][registry] for the given script.
    ///
    /// [bcp47]: https://datatracker.ietf.org/doc/bcp47/
    /// [registry]: https://www.iana.org/assignments/language-subtag-registry/language-subtag-registry
    pub fn bcp47_subtag(&self) -> &str {
        match *self {
            Script::Latin => "Latn",
            Script::Cyrillic => "Cyrl",
            Script::Arabic => "Arab",
            Script::Devanagari => "Deva",
            Script::Hiragana => "Hira",
            Script::Katakana => "Kana",
            Script::Ethiopic => "Ethi",
            Script::Hebrew => "Hebr",
            Script::Bengali => "Beng",
            Script::Georgian => "Geor",
            Script::Mandarin => "Hani",
            Script::Hangul => "Hang",
            Script::Greek => "Grek",
            Script::Kannada => "Knda",
            Script::Tamil => "Taml",
            Script::Thai => "Thai",
            Script::Gujarati => "Gujr",
            Script::Gurmukhi => "Guru",
            Script::Telugu => "Telu",
            Script::Malayalam => "Mlym",
            Script::Oriya => "Orya",
            Script::Myanmar => "Mymr",
            Script::Sinhala => "Sinh",
            Script::Khmer => "Khmr",
            Script::Armenian => "Armn",
        }
    }

    /// Takes the a [script subtag as registerd with IANA][registry] and returns the matching variant if available. Only exact matches and subsets are recognized.
    ///
    /// The following subsets are recognized:
    /// * `hans`, `hant` -> [Script::Mandarin] (`hani`)
    /// * `jamo` -> [Script::Hangul] (`hang`)
    ///
    /// [registry]: https://www.iana.org/assignments/language-subtag-registry/language-subtag-registry
    pub fn from_bcp47_subtag(subtag: &str) -> Result<Self, ParseError> {
        match subtag.to_lowercase().trim() {
            "latn" => Ok(Script::Latin),
            "cyrl" => Ok(Script::Cyrillic),
            "arab" => Ok(Script::Arabic),
            "deva" => Ok(Script::Devanagari),
            "hira" => Ok(Script::Hiragana),
            "kana" => Ok(Script::Katakana),
            "ethi" => Ok(Script::Ethiopic),
            "hebr" => Ok(Script::Hebrew),
            "beng" => Ok(Script::Bengali),
            "geor" => Ok(Script::Georgian),
            "hani" | "hans" | "hant" => Ok(Script::Mandarin),
            "hang" | "jamo" => Ok(Script::Hangul),
            "grek" => Ok(Script::Greek),
            "knda" => Ok(Script::Kannada),
            "taml" => Ok(Script::Tamil),
            "thai" => Ok(Script::Thai),
            "gujr" => Ok(Script::Gujarati),
            "guru" => Ok(Script::Gurmukhi),
            "telu" => Ok(Script::Telugu),
            "mlym" => Ok(Script::Malayalam),
            "orya" => Ok(Script::Oriya),
            "mymr" => Ok(Script::Myanmar),
            "sinh" => Ok(Script::Sinhala),
            "khmr" => Ok(Script::Khmer),
            "armn" => Ok(Script::Armenian),
            _ => Err(ParseError::ScriptFromBcp47(subtag.to_string())),
        }
    }
}

impl fmt::Display for Script {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl FromStr for Script {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().trim() {
            "latin" => Ok(Script::Latin),
            "cyrillic" => Ok(Script::Cyrillic),
            "arabic" => Ok(Script::Arabic),
            "devanagari" => Ok(Script::Devanagari),
            "hiragana" => Ok(Script::Hiragana),
            "katakana" => Ok(Script::Katakana),
            "ethiopic" => Ok(Script::Ethiopic),
            "hebrew" => Ok(Script::Hebrew),
            "bengali" => Ok(Script::Bengali),
            "georgian" => Ok(Script::Georgian),
            "mandarin" => Ok(Script::Mandarin),
            "hangul" => Ok(Script::Hangul),
            "greek" => Ok(Script::Greek),
            "kannada" => Ok(Script::Kannada),
            "tamil" => Ok(Script::Tamil),
            "thai" => Ok(Script::Thai),
            "gujarati" => Ok(Script::Gujarati),
            "gurmukhi" => Ok(Script::Gurmukhi),
            "telugu" => Ok(Script::Telugu),
            "malayalam" => Ok(Script::Malayalam),
            "oriya" => Ok(Script::Oriya),
            "myanmar" => Ok(Script::Myanmar),
            "sinhala" => Ok(Script::Sinhala),
            "khmer" => Ok(Script::Khmer),
            "armenian" => Ok(Script::Armenian),
            _ => Err(ParseError::Script(s.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all() {
        assert_eq!(Script::all().len(), 25);
        let all = Script::all();
        assert!(all.contains(&Script::Cyrillic));
        assert!(all.contains(&Script::Arabic));
        assert!(all.contains(&Script::Latin));
    }

    #[test]
    fn test_from_str() {
        for &script in Script::all() {
            let s = script.name();
            assert_eq!(s.parse::<Script>().unwrap(), script);
            assert_eq!(s.to_lowercase().parse::<Script>().unwrap(), script);
            assert_eq!(s.to_uppercase().parse::<Script>().unwrap(), script);
        }

        let result = "foobar".parse::<Script>();
        assert!(matches!(result, Err(ParseError::Script(_))));
    }

    #[test]
    fn test_from_bcp47() {
        for &script in Script::all() {
            let s = script.bcp47_subtag();
            assert_eq!(Script::from_bcp47_subtag(s).unwrap(), script);
            assert_eq!(
                Script::from_bcp47_subtag(&s.to_lowercase()).unwrap(),
                script
            );
            assert_eq!(
                Script::from_bcp47_subtag(&s.to_uppercase()).unwrap(),
                script
            );
        }

        let result = Script::from_bcp47_subtag("foobar");
        assert!(matches!(result, Err(ParseError::ScriptFromBcp47(_))));
    }

    #[test]
    fn test_langs() {
        // Vec of all langs obtained with script.langs()
        let script_langs: Vec<Lang> = Script::all()
            .iter()
            .map(|script| script.langs())
            .flatten()
            .copied()
            .collect();

        // Ensure all langs belong at least to one script
        for lang in Lang::all() {
            assert!(script_langs.contains(&lang));
        }
    }

    #[test]
    fn test_script_display() {
        assert_eq!(Script::Georgian.to_string(), "Georgian");
        assert_eq!(Script::Cyrillic.to_string(), "Cyrillic");
        assert_eq!(Script::Arabic.to_string(), "Arabic");
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_and_deserialize() {
        let scripts = vec![Script::Georgian, Script::Cyrillic];
        let json_scripts = serde_json::to_string(&scripts).unwrap();
        assert_eq!(json_scripts, r#"["Georgian","Cyrillic"]"#);
        let parsed_scripts: Vec<Script> = serde_json::from_str(&json_scripts).unwrap();
        assert_eq!(parsed_scripts, scripts);
    }
}
