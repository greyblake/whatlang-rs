use std::collections::HashMap;

pub fn count_trigrams(s : String) -> HashMap<String, u32> {
    let mut s = s.to_lowercase();
    s.push(' '); // add space to the end

    let mut counter_hash : HashMap<String, u32> = HashMap::new();

    // iterate through the string and count trigrams
    let mut chars_iter = s.chars();
    let mut c1 = chars_iter.next().unwrap();
    let mut c2 = chars_iter.next().unwrap();
    for cur_char in chars_iter {
        let c3 = to_trigram_char(cur_char);
        if !((c1 == ' ' && c2 == ' ') || (c2 == ' ' && c3 == ' ')) {
            let mut trigram = String::with_capacity(3);
            trigram.push(c1);
            trigram.push(c2);
            trigram.push(c3);
            let count = counter_hash.entry(trigram).or_insert(0);
            *count += 1;
        }
        c1 = c2;
        c2 = c3;
    }

    let mut count_vec: Vec<_> = counter_hash.iter().collect();
    count_vec.sort_by_key(|key| key.1 );

    let mut result: HashMap<String, u32> = HashMap::new();

    // TODO: extract 600 as LANG_PROFILE_LENGTH * 2
    for (i, trigram) in count_vec.iter().take(600).map(|x| x.0).enumerate() {
        // TODO: find a way not to clone it
        result.insert((*trigram).clone(), i as u32);
    }

    result
}

// Replace punctuations and digits  with space.
fn to_trigram_char(ch : char) -> char {
    match ch {
        '\u{0000}'...'\u{0040}' | '\u{005B}'...'\u{0060}' | '\u{007B}'...'\u{007E}' => ' ',
        _ => ch
    }
}



#[cfg(test)]
mod tests {
    use super::to_trigram_char;
    //use super::count_trigrams;

    //fn assert_trigram_chars(chars : [char]) {
    //}

    #[test]
    fn test_to_trigram_char() {
        assert_eq!(to_trigram_char('a'), 'a');
        assert_eq!(to_trigram_char('z'), 'z');
        assert_eq!(to_trigram_char('A'), 'A');
        assert_eq!(to_trigram_char('Z'), 'Z');
        assert_eq!(to_trigram_char('Ж'), 'Ж');
        assert_eq!(to_trigram_char('ß'), 'ß');

        // 0x00 - 0x40
        assert_eq!(to_trigram_char('\t'), ' ');
        assert_eq!(to_trigram_char('\n'), ' ');
        assert_eq!(to_trigram_char(' '), ' ');
        assert_eq!(to_trigram_char('.'), ' ');
        assert_eq!(to_trigram_char('0'), ' ');
        assert_eq!(to_trigram_char('9'), ' ');
        assert_eq!(to_trigram_char(','), ' ');
        assert_eq!(to_trigram_char('@'), ' ');

        // 0x5B - 0x60
        assert_eq!(to_trigram_char('['), ' ');
        assert_eq!(to_trigram_char(']'), ' ');
        assert_eq!(to_trigram_char('^'), ' ');
        assert_eq!(to_trigram_char('\\'), ' ');
        assert_eq!(to_trigram_char('`'), ' ');

        // 0x7B - 0x7E
        assert_eq!(to_trigram_char('|'), ' ');
        assert_eq!(to_trigram_char('{'), ' ');
        assert_eq!(to_trigram_char('}'), ' ');
        assert_eq!(to_trigram_char('~'), ' ');
    }
}
