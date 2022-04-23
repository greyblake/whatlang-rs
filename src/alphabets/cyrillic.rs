use super::RawOutcome;
use crate::alphabets::generic;
use crate::core::{FilterList, LowercaseText};
use crate::Script;

pub fn alphabet_calculate_scores(text: &LowercaseText, filter_list: &FilterList) -> RawOutcome {
    let all_langs = Script::Cyrillic.langs();
    generic::alphabet_calculate_scores_generic(text, filter_list, all_langs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Lang;

    fn fetch<T: Copy>(lang: &Lang, scores: &[(Lang, T)]) -> T {
        scores.iter().find(|(l, _)| l == lang).unwrap().1
    }

    #[test]
    fn test_when_latin_is_given() {
        let cyrillic_langs: &[Lang] = Script::Cyrillic.langs();

        let text = LowercaseText::new("Foobar, hoh");
        let RawOutcome {
            count,
            raw_scores,
            scores,
        } = alphabet_calculate_scores(&text, &FilterList::default());

        assert_eq!(count, 0);
        assert_eq!(raw_scores.len(), cyrillic_langs.len());
        assert_eq!(scores.len(), cyrillic_langs.len());

        for lang in cyrillic_langs {
            let raw_score = fetch(lang, &raw_scores);
            assert_eq!(raw_score, 0);
        }

        for lang in cyrillic_langs {
            let score = fetch(lang, &scores);
            assert_eq!(score, 0.0);
        }
    }

    #[test]
    fn test_when_common_cyrllic_is_given() {
        let cyrillic_langs: &[Lang] = Script::Cyrillic.langs();

        let text = LowercaseText::new("абвг ww");
        let RawOutcome {
            count,
            raw_scores,
            scores,
        } = alphabet_calculate_scores(&text, &FilterList::default());

        assert_eq!(count, 4);

        for lang in cyrillic_langs {
            let raw_score = fetch(lang, &raw_scores);
            assert_eq!(raw_score, 4);
        }

        for lang in cyrillic_langs {
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
