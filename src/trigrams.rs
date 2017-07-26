use utils::is_stop_char;
use std::collections::HashMap;

// Allocating some default for the hash memory slightly improves perfomance (about 3%).
// TODO: ideally initial capacity must be a function of text.len().
const DEFAULT_HASH_CAPACITY : usize = 512;

pub fn get_trigrams_with_positions(text : &str) -> HashMap<String, u32> {
    let counter_hash = count(text);

    // Sort in descending order by number of occurrences and trigrams
    let mut count_vec: Vec<_> = counter_hash.into_iter().map(|(trigram, count)| (count, trigram)).collect();
    count_vec.sort_by(|a, b| b.cmp(a));

    // TODO: extract 600 as LANG_PROFILE_LENGTH * 2
    count_vec.into_iter().take(600).enumerate().map(|(i, (_, trigram))| (trigram, i as u32)).collect()
}

fn count(text : &str) -> HashMap<String, u32> {
    let mut counter_hash : HashMap<String, u32> = HashMap::with_capacity(DEFAULT_HASH_CAPACITY);

    // iterate through the string and count trigrams
    let mut chars_iter = text.chars().map(to_trigram_char).flat_map(char::to_lowercase).chain(Some(' '));
    let mut c1 = ' ';
    // unwrap is safe, because we always chain a space character on the end of the iterator
    let mut c2 = chars_iter.next().unwrap();
    for cur_char in chars_iter {
        let c3 = cur_char;
        if !(c2 == ' ' && (c1 == ' ' || c3 == ' ')) {
            let mut trigram = String::with_capacity(c1.len_utf8() + c2.len_utf8() + c3.len_utf8());
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
#[inline]
fn to_trigram_char(ch : char) -> char {
    if is_stop_char(ch) { ' ' } else { ch }
}



#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(*res.get("aaa").unwrap(), 0);
        assert_eq!(*res.get("bbb").unwrap(), 1);
    }
}
