pub use lang::Lang;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Script {
    Cyrillic,
    Latin,

    Arabic,

    //Devanagari,
    //Ethiopic,
    //Hebrew,

    Cmn,
    Kat,
}

static FUNCS : &'static [(Script, fn(char) -> bool)] = &[
    (Script::Cyrillic, is_cyrillic),
    (Script::Latin   , is_latin),
    (Script::Arabic  , is_arabic),
    (Script::Kat     , is_kat),
    (Script::Cmn     , is_cmn)
];

pub fn detect_script(text : String) -> Option<Script> {
    let mut max_score = 0;
    let mut result = None;

    for &(script, func) in FUNCS {
        let mut current_score = 0;
        for ch in text.chars() {
            if func(ch) { current_score += 1; }
        }
        if current_score > max_score {
            max_score = current_score;
            result = Some(script);
        }
    }

    result
}

// TODO: Use http://jrgraphix.net/research/unicode_blocks.php
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

// Based on https://en.wikipedia.org/wiki/Arabic_script_in_Unicode
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

// Is Georgian char?
fn is_kat(ch : char) -> bool {
   match ch {
       '\u{10A0}'...'\u{10FF}' => true,
       _ => false
   }
}

// TODO: likely not the full set of possible chars..
fn is_cmn(ch : char) -> bool {
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


#[cfg(test)]
mod tests {
   use super::Script;
   use super::is_cyrillic;
   use super::is_latin;
   use super::is_kat;
   use super::detect_script;

   #[test]
   fn test_detect_script() {
       assert_eq!(detect_script("1234567890-,;!".to_string()), None);

       // One script
       assert_eq!(detect_script("Hello!".to_string()), Some(Script::Latin));
       assert_eq!(detect_script("Привет всем!".to_string()), Some(Script::Cyrillic));
       assert_eq!(detect_script("ქართული ენა მსოფლიო ".to_string()), Some(Script::Kat));
       assert_eq!(detect_script("県見夜上温国阪題富販".to_string()), Some(Script::Cmn));
       assert_eq!(detect_script(" ككل حوالي 1.6، ومعظم الناس ".to_string()), Some(Script::Arabic));


       // Mixed scripts
       assert_eq!(detect_script("Привет всем! Этот текст на русском with some English.".to_string()), Some(Script::Cyrillic));
       assert_eq!(detect_script("Russian word любовь means love.".to_string()), Some(Script::Latin));
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
   fn test_is_latin() {
       assert_eq!(is_latin('z'), true);
       assert_eq!(is_latin('A'), true);

       assert_eq!(is_latin('ж'), false);
   }

   #[test]
   fn test_is_kat() {
       assert_eq!(is_kat('რ'), true);
       assert_eq!(is_kat('ж'), false);
   }
}
