use super::RawOutcome;
use super::common::{build_inverted_map, generic_alphabet_calculate_scores};
use crate::core::{FilterList, LowercaseText};
use crate::{Lang, Script};
use std::sync::LazyLock;

const BUL: &str = "абвгдежзийклмнопрстуфхцчшщъьюя";
const RUS: &str = "абвгдежзийклмнопрстуфхцчшщъыьэюяё";
const UKR: &str = "абвгдежзийклмнопрстуфхцчшщьюяєіїґ";
const BEL: &str = "абвгдежзйклмнопрстуфхцчшыьэюяёіў";
const SRP: &str = "абвгдежзиклмнопрстуфхцчшђјљњћџ";
const MKD: &str = "абвгдежзиклмнопрстуфхцчшѓѕјљњќџ";
const KIR: &str = "абвгдеёжзийклмнңоөпрстуүфхцчшщъыьэюя";

const CYRILLIC_ALPHABETS: &[(Lang, &str)] = &[
    (Lang::Bul, BUL),
    (Lang::Rus, RUS),
    (Lang::Ukr, UKR),
    (Lang::Bel, BEL),
    (Lang::Srp, SRP),
    (Lang::Mkd, MKD),
    (Lang::Kir, KIR),
];

/// Inverted map binding a character to a set of languages.
static CYRILLIC_ALPHABET_LANG_MAP: LazyLock<(Vec<char>, Vec<Vec<Lang>>)> =
    LazyLock::new(|| build_inverted_map(CYRILLIC_ALPHABETS));

pub fn alphabet_calculate_scores(text: &LowercaseText, filter_list: &FilterList) -> RawOutcome {
    generic_alphabet_calculate_scores(
        Script::Cyrillic,
        &CYRILLIC_ALPHABET_LANG_MAP,
        text,
        filter_list,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fetch<T: Copy>(lang: &Lang, scores: &[(Lang, T)]) -> T {
        scores.iter().find(|(l, _)| l == lang).unwrap().1
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
