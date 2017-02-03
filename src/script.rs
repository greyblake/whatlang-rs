#![allow(unused_assignments)]

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Script {
    Latin,
    Cyrillic,
    Arabic,
    Devanagari,
    Hiragana,
    Katakana,
    Ethiopic,
    Hebrew,
    Bengali,
    Georgian,
    Mandarin,
    Hangul,
    Greek,
    Kannada,
    Tamil,
    Thai,
    Gujarati,
    Gurmukhi
}

macro_rules! check_scripts {
    ( $text:ident, $( $script:expr => $is_script:ident ),* ) => {
        {
            let mut max_score = 0;
            let mut result = None;
            let mut current_score = 0;
            let half = $text.len() / 2;

            $(
                current_score = 0;
                for ch in $text.chars() {
                    if $is_script(ch) { current_score += 1; }
                }
                if current_score > half {
                    return Some($script);
                }
                if current_score > max_score {
                    max_score = current_score;
                    result = Some($script);
                }
            )*
            result
        }
    };
}

pub fn detect_script(text: &String) -> Option<Script> {
    check_scripts!(
        text,
        Script::Latin      => is_latin,
        Script::Cyrillic   => is_cyrillic,
        Script::Arabic     => is_arabic,
        Script::Mandarin   => is_mandarin,
        Script::Devanagari => is_devanagari,
        Script::Hebrew     => is_hebrew,
        Script::Ethiopic   => is_ethiopic,
        Script::Georgian   => is_georgian,
        Script::Bengali    => is_bengali,
        Script::Hangul     => is_hangul,
        Script::Hiragana   => is_hiragana,
        Script::Katakana   => is_katakana,
        Script::Greek      => is_greek,
        Script::Kannada    => is_kannada,
        Script::Tamil      => is_tamil,
        Script::Thai       => is_thai,
        Script::Gujarati   => is_gujarati,
        Script::Gurmukhi   => is_gurmukhi
    )
}

#[inline(always)]
fn is_cyrillic(ch: char) -> bool {
   match ch {
       '\u{0400}'...'\u{0484}' |
       '\u{0487}'...'\u{052F}' |
       '\u{2DE0}'...'\u{2DFF}' |
       '\u{A640}'...'\u{A69D}' |
       '\u{1D2B}' |
       '\u{1D78}' |
       '\u{A69F}' => true,
       _ => false
   }
}

// https://en.wikipedia.org/wiki/Latin_script_in_Unicode
#[inline(always)]
fn is_latin(ch : char) -> bool {
    match ch {
        'a'...'z' |
        'A'...'Z' |
        '\u{0080}'...'\u{00FF}' |
        '\u{0100}'...'\u{017F}' |
        '\u{0180}'...'\u{024F}' |
        '\u{0250}'...'\u{02AF}' |
        '\u{1D00}'...'\u{1D7F}' |
        '\u{1D80}'...'\u{1DBF}' |
        '\u{1E00}'...'\u{1EFF}' |
        '\u{2100}'...'\u{214F}' |
        '\u{2C60}'...'\u{2C7F}' |
        '\u{A720}'...'\u{A7FF}' |
        '\u{AB30}'...'\u{AB6F}' => true,
        _ => false
    }
}

// Based on https://en.wikipedia.org/wiki/Arabic_script_in_Unicode
#[inline(always)]
fn is_arabic(ch : char) -> bool {
    match ch {
        '\u{0600}'...'\u{06FF}' |
        '\u{0750}'...'\u{07FF}' |
        '\u{08A0}'...'\u{08FF}' |
        '\u{FB50}'...'\u{FDFF}' |
        '\u{FE70}'...'\u{FEFF}' |
        '\u{10E60}'...'\u{10E7F}' |
        '\u{1EE00}'...'\u{1EEFF}' => true,
        _ => false
    }
}

// Based on https://en.wikipedia.org/wiki/Devanagari#Unicode
#[inline(always)]
fn is_devanagari(ch : char) -> bool {
    match ch {
        '\u{0900}'...'\u{097F}' |
        '\u{A8E0}'...'\u{A8FF}' |
        '\u{1CD0}'...'\u{1CFF}' => true,
        _ => false
    }
}

// Based on https://www.key-shortcut.com/en/writing-systems/ethiopian-script/
#[inline(always)]
fn is_ethiopic(ch : char) -> bool {
    match ch {
        '\u{1200}'...'\u{139F}' |
        '\u{2D80}'...'\u{2DDF}' |
        '\u{AB00}'...'\u{AB2F}' => true,
        _ => false
    }
}

// Based on https://en.wikipedia.org/wiki/Hebrew_(Unicode_block)
#[inline(always)]
fn is_hebrew(ch : char) -> bool {
    match ch {
        '\u{0590}'...'\u{05FF}' => true,
        _ => false
    }
}

#[inline(always)]
fn is_georgian(ch : char) -> bool {
   match ch {
       '\u{10A0}'...'\u{10FF}' => true,
       _ => false
   }
}

#[inline(always)]
fn is_mandarin(ch : char) -> bool {
    match ch {
        '\u{2E80}'...'\u{2E99}' |
        '\u{2E9B}'...'\u{2EF3}' |
        '\u{2F00}'...'\u{2FD5}' |
        '\u{3005}' |
        '\u{3007}' |
        '\u{3021}'...'\u{3029}' |
        '\u{3038}'...'\u{303B}' |
        '\u{3400}'...'\u{4DB5}' |
        '\u{4E00}'...'\u{9FCC}' |
        '\u{F900}'...'\u{FA6D}' |
        '\u{FA70}'...'\u{FAD9}' => true,
        _ => false
    }
}

#[inline(always)]
fn is_bengali(ch : char) -> bool {
   match ch {
       '\u{0980}'...'\u{09FF}' => true,
       _ => false
   }
}

#[inline(always)]
fn is_hiragana(ch : char) -> bool {
   match ch {
       '\u{3040}'...'\u{309F}' => true,
       _ => false
   }
}

#[inline(always)]
fn is_katakana(ch : char) -> bool {
   match ch {
       '\u{30A0}'...'\u{30FF}' => true,
       _ => false
    }
}


// Hangul is Korean Alphabet. Unicode ranges are taken from: https://en.wikipedia.org/wiki/Hangul
#[inline(always)]
fn is_hangul(ch : char) -> bool {
    match ch {
        '\u{AC00}'...'\u{D7AF}' |
        '\u{1100}'...'\u{11FF}' |
        '\u{3130}'...'\u{318F}' |
        '\u{3200}'...'\u{32FF}' |
        '\u{A960}'...'\u{A97F}' |
        '\u{D7B0}'...'\u{D7FF}' |
        '\u{FF00}'...'\u{FFEF}' => true,
        _ => false
    }
}

// Taken from: https://en.wikipedia.org/wiki/Greek_and_Coptic
#[inline(always)]
fn is_greek(ch : char) -> bool {
    match ch {
        '\u{0370}'...'\u{03FF}' => true,
        _ => false
    }
}

// Based on: https://en.wikipedia.org/wiki/Kannada_(Unicode_block)
#[inline(always)]
fn is_kannada(ch : char) -> bool {
    match ch {
        '\u{0C80}'...'\u{0CFF}' => true,
        _ => false
    }
}

// Based on: https://en.wikipedia.org/wiki/Tamil_(Unicode_block)
#[inline(always)]
fn is_tamil(ch: char) -> bool {
    match ch {
        '\u{0B80}'...'\u{0BFF}' => true,
        _ => false
    }
}

// Based on: https://en.wikipedia.org/wiki/Thai_(Unicode_block)
#[inline(always)]
fn is_thai(ch: char) -> bool {
    match ch {
        '\u{0E00}'...'\u{0E7F}' => true,
        _ => false
    }
}

// Based on: https://en.wikipedia.org/wiki/Gujarati_(Unicode_block)
#[inline(always)]
fn is_gujarati(ch: char) -> bool {
    match ch {
        '\u{0A80}'...'\u{0AFF}' => true,
        _ => false
    }
}

// Gurmukhi is the script for Punjabi language.
// Based on: https://en.wikipedia.org/wiki/Gurmukhi_(Unicode_block)
#[inline(always)]
fn is_gurmukhi(ch: char) -> bool {
    match ch {
        '\u{0A00}'...'\u{0A7F}' => true,
        _ => false
    }
}

#[cfg(test)]
mod tests {
    use super::Script;
    use super::is_cyrillic;
    use super::is_latin;
    use super::is_georgian;
    use super::is_ethiopic;
    use super::is_bengali;
    use super::is_katakana;
    use super::is_hiragana;
    use super::is_hangul;
    use super::is_greek;
    use super::is_kannada;
    use super::is_tamil;
    use super::is_thai;
    use super::is_gujarati;
    use super::is_gurmukhi;
    use super::detect_script;

    #[test]
    fn test_detect_script() {
        assert_eq!(detect_script(&"1234567890-,;!".to_string()), None);

        // One script
        assert_eq!(detect_script(&"Hello!".to_string()), Some(Script::Latin));
        assert_eq!(detect_script(&"Привет всем!".to_string()), Some(Script::Cyrillic));
        assert_eq!(detect_script(&"ქართული ენა მსოფლიო ".to_string()), Some(Script::Georgian));
        assert_eq!(detect_script(&"県見夜上温国阪題富販".to_string()), Some(Script::Mandarin));
        assert_eq!(detect_script(&" ككل حوالي 1.6، ومعظم الناس ".to_string()), Some(Script::Arabic));
        assert_eq!(detect_script(&"हिमालयी वन चिड़िया (जूथेरा सालिमअली) चिड़िया की एक प्रजाति है".to_string()), Some(Script::Devanagari));
        assert_eq!(detect_script(&"היסטוריה והתפתחות של האלפבית העברי".to_string()), Some(Script::Hebrew));
        assert_eq!(detect_script(&"የኢትዮጵያ ፌዴራላዊ ዴሞክራሲያዊሪፐብሊክ".to_string()), Some(Script::Ethiopic));

        // Mixed scripts
        assert_eq!(detect_script(&"Привет! Текст на русском with some English.".to_string()), Some(Script::Cyrillic));
        assert_eq!(detect_script(&"Russian word любовь means love.".to_string()), Some(Script::Latin));
    }

    #[test]
    fn test_is_latin() {
        assert_eq!(is_latin('z'), true);
        assert_eq!(is_latin('A'), true);
        assert_eq!(is_latin('č'), true);
        assert_eq!(is_latin('š'), true);
        assert_eq!(is_latin('Ĵ'), true);

        assert_eq!(is_latin('ж'), false);
    }

    #[test]
    fn test_is_cyrillic() {
        assert_eq!(is_cyrillic('а'), true);
        assert_eq!(is_cyrillic('Я'), true);
        assert_eq!(is_cyrillic('Ґ'), true);
        assert_eq!(is_cyrillic('ї'), true);
        assert_eq!(is_cyrillic('Ꙕ'), true);

        assert_eq!(is_cyrillic('L'), false);
    }

    #[test]
    fn test_is_ethiopic() {
        assert_eq!(is_ethiopic('ፚ'), true);
        assert_eq!(is_ethiopic('ᎀ'), true);

        assert_eq!(is_ethiopic('а'), false);
        assert_eq!(is_ethiopic('L'), false);
    }

    #[test]
    fn test_is_georgian() {
        assert_eq!(is_georgian('რ'), true);
        assert_eq!(is_georgian('ж'), false);
    }

    #[test]
    fn test_is_bengali() {
        assert_eq!(is_bengali('ই'), true);
        assert_eq!(is_bengali('z'), false);
    }

    #[test]
    fn test_is_katakana() {
        assert_eq!(is_katakana('カ'), true);
        assert_eq!(is_katakana('f'), false);
    }

    #[test]
    fn test_is_hiragana() {
        assert_eq!(is_hiragana('ひ'), true);
        assert_eq!(is_hiragana('a'), false);
    }

    #[test]
    fn test_is_hangul() {
        assert_eq!(is_hangul('ᄁ'), true);
        assert_eq!(is_hangul('t'), false);
    }

    #[test]
    fn test_is_greek() {
        assert_eq!(is_greek('φ'), true);
        assert_eq!(is_greek('ф'), false);
    }

    #[test]
    fn test_is_kannada() {
        assert_eq!(is_kannada('ಡ'), true);
        assert_eq!(is_kannada('S'), false);
    }

    #[test]
    fn test_is_tamil() {
        assert_eq!(is_tamil('ஐ'), true);
        assert_eq!(is_tamil('Ж'), false);
    }

    #[test]
    fn test_is_thai() {
        assert_eq!(is_thai('ก'), true);
        assert_eq!(is_thai('๛'), true);
        assert_eq!(is_thai('Ж'), false);
    }

    #[test]
    fn test_is_gujarati() {
        assert_eq!(is_gujarati('ઁ'), true);
        assert_eq!(is_gujarati('૱'), true);
        assert_eq!(is_gujarati('Ж'), false);
    }

    #[test]
    fn test_is_gurmukhi() {
        assert_eq!(is_gurmukhi('ਁ'), true);
        assert_eq!(is_gurmukhi('ੴ'), true);
        assert_eq!(is_gurmukhi('Ж'), false);
    }
}
