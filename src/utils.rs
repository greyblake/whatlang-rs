// Is it space, punctuation or digit?
// Stop character is a character that does not give any value for script
// or language detection.
#[inline]
pub fn is_stop_char(ch : char) -> bool {
    match ch {
        '\u{0000}'...'\u{0040}' | '\u{005B}'...'\u{0060}' | '\u{007B}'...'\u{007E}' => true,
        _ => false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_top_char() {
        // stop chars
        assert!(is_stop_char(' '));
        assert!(is_stop_char(','));
        assert!(is_stop_char('-'));
        assert!(is_stop_char('-'));
        assert!(is_stop_char('9'));
        assert!(is_stop_char('0'));

        // non-stop chars
        assert!(!is_stop_char('a'));
        assert!(!is_stop_char('z'));
        assert!(!is_stop_char('A')); // latin A
        assert!(!is_stop_char('Z'));
        assert!(!is_stop_char('я'));
        assert!(!is_stop_char('А')); // cyrillic A
    }
}
