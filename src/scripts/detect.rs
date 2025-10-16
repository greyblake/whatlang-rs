use std::cmp::Reverse;

use super::chars;
use super::script::Script;
use crate::utils::is_stop_char;

type ScriptCounter = (Script, fn(char) -> bool, usize);

/// Detect only a script by a given text.
/// Works much faster than a complete detection with `detect`.
///
/// # Example
/// ```
/// use whatlang::{detect_script, Script};
/// let script = detect_script("Благодаря Эсперанто вы обрётете друзей по всему миру!").unwrap();
/// assert_eq!(script, Script::Cyrillic);
/// ```
pub fn detect_script(text: &str) -> Option<Script> {
    let raw_info = raw_detect_script(text);
    raw_info.main_script()
}

#[derive(Debug)]
pub struct RawScriptInfo {
    pub counters: Vec<(Script, usize)>,
}

impl RawScriptInfo {
    fn new(mut counters: Vec<(Script, usize)>) -> Self {
        counters.sort_unstable_by_key(|(_, score)| Reverse(*score));
        Self { counters }
    }

    pub(crate) fn main_script(&self) -> Option<Script> {
        // expect - is safe because self.counters is never expected to be empty
        // See raw_detect_script().
        let pair = self.counters.first().expect("counters must not be empty");
        if pair.1 > 0 { Some(pair.0) } else { None }
    }

    pub(crate) fn count(&self, script: Script) -> usize {
        // expect - is safe because self.counters always have all scripts
        // See raw_detect_script().
        self.counters
            .iter()
            .find(|(s, _count)| *s == script)
            .expect("count() failed because script is not found")
            .1
    }
}

pub fn raw_detect_script(text: &str) -> RawScriptInfo {
    let mut script_counters: [ScriptCounter; 25] = [
        (Script::Latin, chars::is_latin, 0),
        (Script::Cyrillic, chars::is_cyrillic, 0),
        (Script::Arabic, chars::is_arabic, 0),
        (Script::Mandarin, chars::is_mandarin, 0),
        (Script::Devanagari, chars::is_devanagari, 0),
        (Script::Hebrew, chars::is_hebrew, 0),
        (Script::Ethiopic, chars::is_ethiopic, 0),
        (Script::Georgian, chars::is_georgian, 0),
        (Script::Bengali, chars::is_bengali, 0),
        (Script::Hangul, chars::is_hangul, 0),
        (Script::Hiragana, chars::is_hiragana, 0),
        (Script::Katakana, chars::is_katakana, 0),
        (Script::Greek, chars::is_greek, 0),
        (Script::Kannada, chars::is_kannada, 0),
        (Script::Tamil, chars::is_tamil, 0),
        (Script::Thai, chars::is_thai, 0),
        (Script::Gujarati, chars::is_gujarati, 0),
        (Script::Gurmukhi, chars::is_gurmukhi, 0),
        (Script::Telugu, chars::is_telugu, 0),
        (Script::Malayalam, chars::is_malayalam, 0),
        (Script::Oriya, chars::is_oriya, 0),
        (Script::Myanmar, chars::is_myanmar, 0),
        (Script::Sinhala, chars::is_sinhala, 0),
        (Script::Khmer, chars::is_khmer, 0),
        (Script::Armenian, chars::is_armenian, 0),
    ];

    for ch in text.chars() {
        if is_stop_char(ch) {
            continue;
        }

        // For performance reasons, we need to mutate script_counters by calling
        // `swap` function, it would not be possible to do using normal iterator.
        for i in 0..script_counters.len() {
            let found = {
                let (_script, check_fn, ref mut count) = script_counters[i];
                if check_fn(ch) {
                    *count += 1;
                    true
                } else {
                    false
                }
            };
            // Have to let borrow of count fall out of scope before doing swapping, or we could
            // do this above.
            if found {
                // If script was found, move it closer to the front.
                // If the text contains largely 1 or 2 scripts, this will
                // cause these scripts to be eventually checked first.
                if i > 0 {
                    script_counters.swap(i - 1, i);
                }
                break;
            }
        }
    }

    let counters: Vec<(Script, usize)> = script_counters
        .iter()
        .map(|&(script, _, count)| (script, count))
        .collect();

    RawScriptInfo::new(counters)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_script_name() {
        assert_eq!(Script::Cyrillic.name(), "Cyrillic");
        assert_eq!(Script::Katakana.name(), "Katakana");
    }

    #[test]
    fn test_detect_script() {
        assert_eq!(detect_script("1234567890-,;!"), None);

        // One script
        assert_eq!(detect_script("Hello!"), Some(Script::Latin));
        assert_eq!(detect_script("Привет всем!"), Some(Script::Cyrillic));
        assert_eq!(
            detect_script("ქართული ენა მსოფლიო "),
            Some(Script::Georgian)
        );
        assert_eq!(
            detect_script("県見夜上温国阪題富販"),
            Some(Script::Mandarin)
        );
        assert_eq!(
            detect_script(" ككل حوالي 1.6، ومعظم الناس "),
            Some(Script::Arabic)
        );
        assert_eq!(
            detect_script("हिमालयी वन चिड़िया (जूथेरा सालिमअली) चिड़िया की एक प्रजाति है"),
            Some(Script::Devanagari)
        );
        assert_eq!(
            detect_script("היסטוריה והתפתחות של האלפבית העברי"),
            Some(Script::Hebrew)
        );
        assert_eq!(
            detect_script("የኢትዮጵያ ፌዴራላዊ ዴሞክራሲያዊሪፐብሊክ"),
            Some(Script::Ethiopic)
        );

        // Mixed scripts
        assert_eq!(
            detect_script("Привет! Текст на русском with some English."),
            Some(Script::Cyrillic)
        );
        assert_eq!(
            detect_script("Russian word любовь means love."),
            Some(Script::Latin)
        );
    }
}
