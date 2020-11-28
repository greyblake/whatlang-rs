use hashbrown::HashMap;

use crate::constants::TEXT_TRIGRAMS_SIZE;
use crate::utils::is_stop_char;

const MAX_INITIAL_HASH_CAPACITY: usize = 2048;

#[derive(Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Clone, Copy)]
pub struct Trigram(pub(crate) char, pub(crate) char, pub(crate) char);

#[allow(clippy::unnecessary_sort_by)]
pub fn get_trigrams_with_positions(text: &str) -> HashMap<Trigram, u32> {
    // Sort in descending order by number of occurrences and trigrams
    let mut count_vec: Vec<_> = count(text)
        .into_iter()
        .map(|(trigram, count)| (count, trigram))
        .collect();
    count_vec.sort_by(|a, b| b.cmp(a));

    count_vec
        .into_iter()
        .take(TEXT_TRIGRAMS_SIZE)
        .enumerate()
        .map(|(i, (_, trigram))| (trigram, i as u32))
        .collect()
}

fn count(text: &str) -> HashMap<Trigram, u32> {
    let hash_capacity = calculate_initial_hash_capacity(text);
    let mut counter_hash: HashMap<Trigram, u32> = HashMap::with_capacity(hash_capacity);

    // iterate through the string and count trigrams
    let mut chars_iter = text
        .chars()
        .map(to_trigram_char)
        .flat_map(char::to_lowercase)
        .chain(Some(' '));
    let mut c1 = ' ';
    // unwrap is safe, because we always chain a space character on the end of the iterator
    let mut c2 = chars_iter.next().unwrap();
    for cur_char in chars_iter {
        let c3 = cur_char;
        if !(c2 == ' ' && (c1 == ' ' || c3 == ' ')) {
            let trigram = Trigram(c1, c2, c3);
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
fn to_trigram_char(ch: char) -> char {
    if is_stop_char(ch) {
        ' '
    } else {
        ch
    }
}

// In order to improve performance, define the initial capacity for trigrams hash map,
// based on the size of the input text.
fn calculate_initial_hash_capacity(text: &str) -> usize {
    let len = text.len();
    if len > MAX_INITIAL_HASH_CAPACITY {
        MAX_INITIAL_HASH_CAPACITY
    } else {
        len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_valuable_trigram_chars(chars: &[char]) {
        for &ch in chars.iter() {
            assert_eq!(to_trigram_char(ch), ch);
        }
    }

    fn assert_not_valuable_trigram_chars(chars: &[char]) {
        for &ch in chars.iter() {
            assert_eq!(to_trigram_char(ch), ' ');
        }
    }

    #[test]
    fn test_to_trigram_char() {
        // valuable chars, that matters
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

    fn assert_count(text: &str, pairs: &[(&str, u32)]) {
        let result = count(text);
        for &(trigram_str, expected_n) in pairs.iter() {
            let chars: Vec<char> = trigram_str.clone().chars().collect();
            let trigram = Trigram(chars[0], chars[1], chars[2]);
            let actual_n = result[&trigram];
            assert_eq!(
                actual_n, expected_n,
                "trigram '{:?}' expected to occur {} times, got {}",
                trigram, expected_n, actual_n
            );
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
        assert_count(
            "Give - IT...",
            &[
                (" gi", 1),
                ("giv", 1),
                ("ive", 1),
                ("ve ", 1),
                (" it", 1),
                ("it ", 1),
            ],
        );
    }

    #[test]
    fn test_get_trigrams_with_positions() {
        let res = get_trigrams_with_positions("xaaaaabbbbd");
        assert_eq!(res[&Trigram('a', 'a', 'a')], 0);
        assert_eq!(res[&Trigram('b', 'b', 'b')], 1);
    }
}
