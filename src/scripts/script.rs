use std::fmt;
use std::str::FromStr;

use crate::error::Error;

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

// Array of all possible Script values that is used to build Script::values() iterator.
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

    /// Get an iterator over all scripts that exist.
    ///
    /// # Example
    /// ```
    /// use whatlang::Script;
    /// for script in Script::values() {
    ///     println!("{}", script);
    /// }
    /// ```
    pub fn values() -> impl Iterator<Item = Script> {
        VALUES.iter().copied()
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
        assert_eq!(Script::values().count(), 24);
        let values: Vec<Script> = Script::values().collect();
        assert!(values.contains(&Script::Cyrillic));
        assert!(values.contains(&Script::Arabic));
        assert!(values.contains(&Script::Latin));
    }

    #[test]
    fn test_from_str() {
        for script in Script::values() {
            let s = script.name();
            assert_eq!(s.parse::<Script>().unwrap(), script);
            assert_eq!(s.to_lowercase().parse::<Script>().unwrap(), script);
            assert_eq!(s.to_uppercase().parse::<Script>().unwrap(), script);
        }

        let result = "foobar".parse::<Script>();
        assert!(matches!(result, Err(Error::ParseScript(_))));
    }
}
