// Is it space, punctuation or digit?
pub fn is_stop_char(ch : char) -> bool {
    match ch {
        '\u{0000}'...'\u{0040}' | '\u{005B}'...'\u{0060}' | '\u{007B}'...'\u{007E}' => true,
        _ => false
    }
}
