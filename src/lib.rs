#[derive(PartialEq, Eq, Debug)]
pub enum Script {
    Unknown,
    Cyrillic,
    Latin
}

pub fn detect_script(text : String) -> Script {
    let mut latin_count = 0;
    let mut cyrillic_count = 0;

    for ch in text.chars() {
        if is_cyrillic(ch) { cyrillic_count += 1; }
        if is_latin(ch) {  latin_count += 1; }
        println!("Char: {:?}", ch);

    }

    if latin_count > 0 && latin_count > cyrillic_count {
        Script::Latin
    } else if cyrillic_count > 0 && cyrillic_count > latin_count {
        Script::Cyrillic
    } else {
        Script::Unknown
    }
}

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

// TODO: finish implementation, with all the Latin chars.
fn is_latin(ch : char) -> bool {
    match ch {
        'a'...'z' | 'A'...'Z' => true,
        _ => false
    }
}

#[cfg(test)]
mod tests {
    use super::Script;
    use super::is_cyrillic;
    use super::is_latin;
    use super::detect_script;

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
    fn test_is_latin() {
        assert_eq!(is_latin('z'), true);
        assert_eq!(is_latin('A'), true);

        assert_eq!(is_latin('ж'), false);
    }

    #[test]
    fn test_detect_script() {
        assert_eq!(detect_script("1234567890-,;!".to_string()), Script::Unknown);
        assert_eq!(detect_script("Hello!".to_string()), Script::Latin);
        assert_eq!(detect_script("Привет всем!".to_string()), Script::Cyrillic);
        assert_eq!(detect_script("Привет всем! Этот текст на русском with some English.".to_string()), Script::Cyrillic);
        assert_eq!(detect_script("Russian word любовь means love.".to_string()), Script::Latin);
    }
}
