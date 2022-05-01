pub(crate) fn is_cyrillic(ch: char) -> bool {
    matches!(ch,
        '\u{0400}'..='\u{0484}'
        | '\u{0487}'..='\u{052F}'
        | '\u{2DE0}'..='\u{2DFF}'
        | '\u{A640}'..='\u{A69D}'
        | '\u{1D2B}'
        | '\u{1D78}'
        | '\u{A69F}'
    )
}

// https://en.wikipedia.org/wiki/Latin_script_in_Unicode
pub(crate) fn is_latin(ch: char) -> bool {
    matches!(ch,
        'a'..='z'
        | 'A'..='Z'
        | '\u{0080}'..='\u{00FF}'
        | '\u{0100}'..='\u{017F}'
        | '\u{0180}'..='\u{024F}'
        | '\u{0250}'..='\u{02AF}'
        | '\u{1D00}'..='\u{1D7F}'
        | '\u{1D80}'..='\u{1DBF}'
        | '\u{1E00}'..='\u{1EFF}'
        | '\u{2100}'..='\u{214F}'
        | '\u{2C60}'..='\u{2C7F}'
        | '\u{A720}'..='\u{A7FF}'
        | '\u{AB30}'..='\u{AB6F}'
    )
}

// Based on https://en.wikipedia.org/wiki/Arabic_script_in_Unicode
pub(crate) fn is_arabic(ch: char) -> bool {
    matches!(ch,
        '\u{0600}'..='\u{06FF}'
        | '\u{0750}'..='\u{07FF}'
        | '\u{08A0}'..='\u{08FF}'
        | '\u{FB50}'..='\u{FDFF}'
        | '\u{FE70}'..='\u{FEFF}'
        | '\u{10E60}'..='\u{10E7F}'
        | '\u{1EE00}'..='\u{1EEFF}'
    )
}

// Based on https://en.wikipedia.org/wiki/Devanagari#Unicode
pub(crate) fn is_devanagari(ch: char) -> bool {
    matches!(ch, '\u{0900}'..='\u{097F}' | '\u{A8E0}'..='\u{A8FF}' | '\u{1CD0}'..='\u{1CFF}')
}

// Based on https://www.key-shortcut.com/en/writing-systems/ethiopian-script/
pub(crate) fn is_ethiopic(ch: char) -> bool {
    matches!(ch, '\u{1200}'..='\u{139F}' | '\u{2D80}'..='\u{2DDF}' | '\u{AB00}'..='\u{AB2F}')
}

// Based on https://en.wikipedia.org/wiki/Hebrew_(Unicode_block)
pub(crate) fn is_hebrew(ch: char) -> bool {
    matches!(ch, '\u{0590}'..='\u{05FF}')
}

pub(crate) fn is_georgian(ch: char) -> bool {
    matches!(ch, '\u{10A0}'..='\u{10FF}')
}

pub(crate) fn is_mandarin(ch: char) -> bool {
    matches!(ch,
        '\u{2E80}'..='\u{2E99}'
        | '\u{2E9B}'..='\u{2EF3}'
        | '\u{2F00}'..='\u{2FD5}'
        | '\u{3005}'
        | '\u{3007}'
        | '\u{3021}'..='\u{3029}'
        | '\u{3038}'..='\u{303B}'
        | '\u{3400}'..='\u{4DB5}'
        | '\u{4E00}'..='\u{9FCC}'
        | '\u{F900}'..='\u{FA6D}'
        | '\u{FA70}'..='\u{FAD9}'
    )
}

pub(crate) fn is_bengali(ch: char) -> bool {
    matches!(ch, '\u{0980}'..='\u{09FF}')
}

pub(crate) fn is_hiragana(ch: char) -> bool {
    matches!(ch, '\u{3040}'..='\u{309F}')
}

pub(crate) fn is_katakana(ch: char) -> bool {
    matches!(ch, '\u{30A0}'..='\u{30FF}')
}

// Hangul is Korean Alphabet. Unicode ranges are taken from: https://en.wikipedia.org/wiki/Hangul
pub(crate) fn is_hangul(ch: char) -> bool {
    matches!(ch,
        '\u{AC00}'..='\u{D7AF}'
        | '\u{1100}'..='\u{11FF}'
        | '\u{3130}'..='\u{318F}'
        | '\u{3200}'..='\u{32FF}'
        | '\u{A960}'..='\u{A97F}'
        | '\u{D7B0}'..='\u{D7FF}'
        | '\u{FF00}'..='\u{FFEF}'
    )
}

// Taken from: https://en.wikipedia.org/wiki/Greek_and_Coptic
pub(crate) fn is_greek(ch: char) -> bool {
    matches!(ch, '\u{0370}'..='\u{03FF}')
}

// Based on: https://en.wikipedia.org/wiki/Kannada_(Unicode_block)
pub(crate) fn is_kannada(ch: char) -> bool {
    matches!(ch, '\u{0C80}'..='\u{0CFF}')
}

// Based on: https://en.wikipedia.org/wiki/Tamil_(Unicode_block)
pub(crate) fn is_tamil(ch: char) -> bool {
    matches!(ch, '\u{0B80}'..='\u{0BFF}')
}

// Based on: https://en.wikipedia.org/wiki/Thai_(Unicode_block)
pub(crate) fn is_thai(ch: char) -> bool {
    matches!(ch, '\u{0E00}'..='\u{0E7F}')
}

// Based on: https://en.wikipedia.org/wiki/Gujarati_(Unicode_block)
pub(crate) fn is_gujarati(ch: char) -> bool {
    matches!(ch, '\u{0A80}'..='\u{0AFF}')
}

// Gurmukhi is the script for Punjabi language.
// Based on: https://en.wikipedia.org/wiki/Gurmukhi_(Unicode_block)
pub(crate) fn is_gurmukhi(ch: char) -> bool {
    matches!(ch, '\u{0A00}'..='\u{0A7F}')
}

pub(crate) fn is_telugu(ch: char) -> bool {
    matches!(ch, '\u{0C00}'..='\u{0C7F}')
}

// Based on: https://en.wikipedia.org/wiki/Malayalam_(Unicode_block)
pub(crate) fn is_malayalam(ch: char) -> bool {
    matches!(ch, '\u{0D00}'..='\u{0D7F}')
}

// Based on: https://en.wikipedia.org/wiki/Malayalam_(Unicode_block)
pub(crate) fn is_oriya(ch: char) -> bool {
    matches!(ch, '\u{0B00}'..='\u{0B7F}')
}

// Based on: https://en.wikipedia.org/wiki/Myanmar_(Unicode_block)
pub(crate) fn is_myanmar(ch: char) -> bool {
    matches!(ch, '\u{1000}'..='\u{109F}')
}

// Based on: https://en.wikipedia.org/wiki/Sinhala_(Unicode_block)
pub(crate) fn is_sinhala(ch: char) -> bool {
    matches!(ch, '\u{0D80}'..='\u{0DFF}')
}

// Based on: https://en.wikipedia.org/wiki/Khmer_alphabet
pub(crate) fn is_khmer(ch: char) -> bool {
    matches!(ch, '\u{1780}'..='\u{17FF}' | '\u{19E0}'..='\u{19FF}')
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_is_telugu() {
        assert_eq!(is_telugu('ఁ'), true);
        assert_eq!(is_telugu('౿'), true);
        assert_eq!(is_telugu('Ж'), false);
    }

    #[test]
    fn test_is_oriya() {
        assert_eq!(is_oriya('ଐ'), true);
        assert_eq!(is_oriya('୷'), true);
        assert_eq!(is_oriya('౿'), false);
    }
}
