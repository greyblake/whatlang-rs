use core::fmt;
use core::str::FromStr;

use super::lang_mapping;
use crate::error::Error;
use crate::Lang;

#[cfg(feature = "enum-map")]
use enum_map::Enum;

/// Represents a writing system (Latin, Cyrillic, Arabic, etc).
#[cfg_attr(feature = "enum-map", derive(Enum))]
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Script {
    // Keep this in alphabetic order (for C bindings)
    Arabic,
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
const VALUES: [Script; 24] = [
    Script::Arabic,
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
    /// for script in Script::values() {
    ///     println!("{}", script);
    /// }
    /// ```
    pub fn values() -> &'static [Script] {
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
        }
    }

    pub fn langs(&self) -> &[Lang] {
        lang_mapping::script_langs(*self)
    }
}

impl fmt::Display for Script {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl FromStr for Script {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        #[cfg(not(feature = "std"))]
        use crate::alloc::string::ToString;
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
            _ => Err(Error::ParseScript(s.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_values_iter() {
        assert_eq!(Script::values().len(), 24);
        let values = Script::values();
        assert!(values.contains(&Script::Cyrillic));
        assert!(values.contains(&Script::Arabic));
        assert!(values.contains(&Script::Latin));
    }

    #[test]
    fn test_from_str() {
        for &script in Script::values() {
            let s = script.name();
            assert_eq!(s.parse::<Script>().unwrap(), script);
            assert_eq!(s.to_lowercase().parse::<Script>().unwrap(), script);
            assert_eq!(s.to_uppercase().parse::<Script>().unwrap(), script);
        }

        let result = "foobar".parse::<Script>();
        assert!(matches!(result, Err(Error::ParseScript(_))));
    }

    #[test]
    fn test_langs() {
        // Vec of all langs obtained with script.langs()
        let script_langs: Vec<Lang> = Script::values()
            .iter()
            .map(|script| script.langs())
            .flatten()
            .map(|lang| *lang)
            .collect();

        // Ensure all langs belong at least to one script
        for lang in Lang::values() {
            assert!(script_langs.contains(&lang));
        }
    }
}
