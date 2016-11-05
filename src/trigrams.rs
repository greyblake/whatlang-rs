use std::collections::HashMap;

pub fn get_trigrams_with_positions(text : &String) -> HashMap<String, u32> {
    let counter_hash = count(text);

    let mut count_vec: Vec<_> = counter_hash.iter().collect();
    count_vec.sort_by_key(|key| key.1 );
    count_vec.reverse();

    let mut result: HashMap<String, u32> = HashMap::new();

    // TODO: extract 600 as LANG_PROFILE_LENGTH * 2
    for (i, trigram) in count_vec.iter().take(600).map(|x| x.0).enumerate() {
        // TODO: find a way not to clone it
        result.insert((*trigram).clone(), i as u32);
    }

    result
}

#[inline(always)]
fn count(text : &String) -> HashMap<String, u32> {
    let mut s = text.to_lowercase();
    s.push(' '); // add space to the end

    let mut counter_hash : HashMap<String, u32> = HashMap::new();

    // iterate through the string and count trigrams
    let mut chars_iter = s.chars();
    let mut c1 = ' ';
    let mut c2 = to_trigram_char(chars_iter.next().unwrap());
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

    counter_hash
}

// Convert punctuations and digits to a space.
#[inline(always)]
fn to_trigram_char(ch : char) -> char {
    match ch {
        '\u{0000}'...'\u{0040}' | '\u{005B}'...'\u{0060}' | '\u{007B}'...'\u{007E}' => ' ',
        _ => ch
    }
}



#[cfg(test)]
mod tests {
    use super::to_trigram_char;
    use super::count;
    use super::get_trigrams_with_positions;

    fn assert_valuable_trigram_chars(chars : &[char]) {
        for &ch in chars.iter() {
            assert_eq!(to_trigram_char(ch), ch);
        }
    }

    fn assert_not_valuable_trigram_chars(chars : &[char]) {
        for &ch in chars.iter() {
            assert_eq!(to_trigram_char(ch), ' ');
        }
    }

    #[test]
    fn test_to_trigram_char() {
        // valuable chars, that metters
        assert_valuable_trigram_chars(&['a', 'z', 'A', 'Z', 'Ж', 'ß']);

        // punctuations, digits, etc..
        //
        // 0x00 - 0x40
        assert_not_valuable_trigram_chars(&['\t', '\n', ' ', '.', '0', '9', ',', '@']);
        // 0x5B - 0x60
        assert_not_valuable_trigram_chars(&['[', ']', '^', '\\', '`']);
        // 0x7B - 0x7E
        assert_not_valuable_trigram_chars(&['[', '|', '{', '}', '~']);
    }



    fn assert_count(text : &str, pairs : &[(&str, u32)]) {
        let result = count(&text.to_string());
        for &(k, v) in pairs.iter() {
            let &actual_val = result.get(k).unwrap_or(&0);
            assert_eq!(actual_val, v, "trigram '{}' expected to occur {} times, got {}", k, v, actual_val);
        }
        assert_eq!(result.len(), pairs.len());
    }

    #[test]
    fn test_count() {
        assert_count("", &[]);
        assert_count(",", &[]);
        assert_count("a", &[(" a ", 1)]);
        assert_count("-a-", &[(" a ", 1)]);
        assert_count("yes", &[(" ye", 1), ("yes", 1), ("es ", 1)]);
        assert_count("Give - IT...", &[(" gi", 1), ("giv", 1), ("ive", 1), ("ve ", 1), (" it", 1), ("it ", 1)]);
    }

    #[test]
    fn test_get_trigrams_with_positions() {
        let res = get_trigrams_with_positions(&"xaaaaabbbbd".to_string());
        println!("positions:  {:?}", res);
        assert_eq!(*res.get("aaa").unwrap(), 0);
        assert_eq!(*res.get("bbb").unwrap(), 1);
    }
}
