use hashbrown::HashMap;

use super::Trigram;
use super::TEXT_TRIGRAMS_SIZE;
use crate::core::LowercaseText;
use crate::utils::is_stop_char;

const MAX_INITIAL_HASH_CAPACITY: usize = 2048;

pub struct TrigramsWithPositions {
    pub(crate) _total_trigrams: u32,
    pub(crate) trigram_positions: HashMap<Trigram, u32>,
}

pub fn get_trigrams_with_positions(text: &LowercaseText) -> TrigramsWithPositions {
    let CountResult {
        total_trigrams,
        trigram_occurances,
    } = count(text);
    let trigram_positions = trigram_occurances_to_positions(trigram_occurances);
    TrigramsWithPositions {
        _total_trigrams: total_trigrams,
        trigram_positions,
    }
}

#[allow(clippy::unnecessary_sort_by)]
fn trigram_occurances_to_positions(
    trigram_occurances: HashMap<Trigram, u32>,
) -> HashMap<Trigram, u32> {
    // Sort in descending order by number of occurrences and trigrams
    let mut count_vec: Vec<_> = trigram_occurances
        .into_iter()
        .map(|(trigram, count)| (count, trigram))
        .collect();
    count_vec.sort_by(|a, b| b.cmp(a));

    count_vec
        .into_iter()
        .take(TEXT_TRIGRAMS_SIZE) // we're interested only in the first 600 (2 * MAX_TRIGRAM_DISTANCE)
        .enumerate()
        .map(|(i, (_, trigram))| (trigram, i as u32))
        .collect()
}

struct CountResult {
    total_trigrams: u32,
    trigram_occurances: HashMap<Trigram, u32>,
}

fn count(text: &LowercaseText) -> CountResult {
    let hash_capacity = calculate_initial_hash_capacity(text);
    let mut trigram_occurances: HashMap<Trigram, u32> = HashMap::with_capacity(hash_capacity);
    let mut total_trigrams = 0;

    // iterate through the string and count trigrams
    let mut chars_iter = text
        .chars()
        .map(to_trigram_char)
        //.flat_map(char::to_lowercase)
        .chain(Some(' '));
    let mut c1 = ' ';
    // unwrap is safe, because we always chain a space character on the end of the iterator
    let mut c2 = chars_iter.next().unwrap();
    for cur_char in chars_iter {
        let c3 = cur_char;
        if !(c2 == ' ' && (c1 == ' ' || c3 == ' ')) {
            let trigram = Trigram(c1, c2, c3);
            let count = trigram_occurances.entry(trigram).or_insert(0);
            *count += 1;
            total_trigrams += 1;
        }
        c1 = c2;
        c2 = c3;
    }

    CountResult {
        total_trigrams,
        trigram_occurances,
    }
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
        let lowercase_text = LowercaseText::new(text);
        let CountResult {
            total_trigrams: _,
            trigram_occurances,
        } = count(&lowercase_text);
        for &(trigram_str, expected_n) in pairs.iter() {
            let chars: Vec<char> = trigram_str.clone().chars().collect();
            let trigram = Trigram(chars[0], chars[1], chars[2]);
            let actual_n = trigram_occurances[&trigram];
            assert_eq!(
                actual_n, expected_n,
                "trigram '{:?}' expected to occur {} times, got {}",
                trigram, expected_n, actual_n
            );
        }
        assert_eq!(trigram_occurances.len(), pairs.len());
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
        let lowercase_text = LowercaseText::new("xaaaaabbbb    d");
        let TrigramsWithPositions {
            _total_trigrams,
            trigram_positions,
        } = get_trigrams_with_positions(&lowercase_text);

        assert_eq!(trigram_positions[&Trigram('a', 'a', 'a')], 0);
        assert_eq!(trigram_positions[&Trigram('b', 'b', 'b')], 1);
        assert_eq!(_total_trigrams, 11);
    }
}
