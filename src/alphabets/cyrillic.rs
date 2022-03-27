use super::RawOutcome;
use crate::core::{FilterList, LowercaseText};
use crate::{alphabets, Lang, Script};
use std::cmp;

const BUL: &str = "абвгдежзийклмнопрстуфхцчшщъьюя";
const RUS: &str = "абвгдежзийклмнопрстуфхцчшщъыьэюяё";
const UKR: &str = "абвгдежзийклмнопрстуфхцчшщьюяєіїґ";
const BEL: &str = "абвгдежзйклмнопрстуфхцчшыьэюяёіў";
const SRP: &str = "абвгдежзиклмнопрстуфхцчшђјљњћџ";
const MKD: &str = "абвгдежзиклмнопрстуфхцчшѓѕјљњќџ";

const ALL: &str = "абвгдежзийклмнопрстуфхцчшщъыьэюяёєіїґўђјљњћџѓѕќ";

fn calculate_char_score(ch: char, alphabet: &Vec<char>) -> i32 {
    if !is_relevant(ch) {
        0
    } else if alphabet.contains(&ch) {
        1
    } else {
        -1
    }
}
fn calculate_lang_score(lang: &Lang, text: &LowercaseText) -> usize {
    let alphabet = get_lang_chars(*lang);
    let score: i32 = text
        .chars()
        .map(|ch| calculate_char_score(ch, &alphabet))
        .sum();

    cmp::max(score, 0) as usize
}

fn is_relevant(ch: char) -> bool {
    ALL.chars().any(|c| c == ch)
}

fn get_lang_chars(lang: Lang) -> Vec<char> {
    let alphabet = match lang {
        Lang::Bul => BUL,
        Lang::Rus => RUS,
        Lang::Ukr => UKR,
        Lang::Bel => BEL,
        Lang::Srp => SRP,
        Lang::Mkd => MKD,

        _ => panic!("No alphabet for {}", lang),
    };
    alphabet.chars().collect()
}

pub fn alphabet_calculate_scores(text: &LowercaseText, filter_list: &FilterList) -> RawOutcome {
    let max_raw_score = text.chars().filter(|&ch| is_relevant(ch)).count();

    let raw_scores: Vec<(Lang, usize)> = Script::Cyrillic
        .langs()
        .iter()
        .filter(|&&lang| filter_list.is_allowed(lang))
        .map(|&lang| (lang, calculate_lang_score(&lang, text)))
        .collect();

    // FIXME: You never use the fact that vector is sorted, at least tests don't fail
    // raw_scores.sort_by(|a, b| b.1.cmp(&a.1));

    let normalized_scores = raw_scores
        .iter()
        .map(|&(lang, raw_score)| (lang, alphabets::normalize_score(raw_score, max_raw_score)))
        .collect();

    RawOutcome {
        count: max_raw_score,
        raw_scores,
        scores: normalized_scores,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CYRILLIC_LANGS: [Lang; 6] = [
        Lang::Rus,
        Lang::Ukr,
        Lang::Srp,
        Lang::Bel,
        Lang::Mkd,
        Lang::Bul,
    ];

    fn fetch<T: Copy>(lang: &Lang, scores: &[(Lang, T)]) -> T {
        scores.iter().find(|(l, _)| l == lang).unwrap().1
    }

    #[test]
    fn test_when_latin_is_given() {
        let text = LowercaseText::new("Foobar, hoh");
        let RawOutcome {
            count,
            raw_scores,
            scores,
        } = alphabet_calculate_scores(&text, &FilterList::default());

        assert_eq!(count, 0);
        assert_eq!(raw_scores.len(), CYRILLIC_LANGS.len());
        assert_eq!(scores.len(), CYRILLIC_LANGS.len());

        for lang in &CYRILLIC_LANGS {
            let raw_score = fetch(lang, &raw_scores);
            assert_eq!(raw_score, 0);
        }

        for lang in &CYRILLIC_LANGS {
            let score = fetch(lang, &scores);
            assert_eq!(score, 0.0);
        }
    }

    #[test]
    fn test_when_common_cyrllic_is_given() {
        let text = LowercaseText::new("абвг ww");
        let RawOutcome {
            count,
            raw_scores,
            scores,
        } = alphabet_calculate_scores(&text, &FilterList::default());

        assert_eq!(count, 4);

        for lang in &CYRILLIC_LANGS {
            let raw_score = fetch(lang, &raw_scores);
            assert_eq!(raw_score, 4);
        }

        for lang in &CYRILLIC_LANGS {
            let score = fetch(lang, &scores);
            assert_eq!(score, 1.0);
        }
    }

    #[test]
    fn test_when_ukrainian_specific_chars_given() {
        let text = LowercaseText::new("Дуже цікаво");
        let RawOutcome {
            count,
            raw_scores,
            scores,
        } = alphabet_calculate_scores(&text, &FilterList::default());

        assert_eq!(count, 10);

        assert_eq!(fetch(&Lang::Ukr, &raw_scores), 10);
        assert_eq!(fetch(&Lang::Rus, &raw_scores), 8);

        assert_eq!(fetch(&Lang::Ukr, &scores), 1.0);
        assert_eq!(fetch(&Lang::Rus, &scores), 0.8);
    }
}
