use super::RawOutcome;
use crate::core::{FilterList, LowercaseText};
use crate::{Lang, Script};

const BUL: &str = "абвгдежзийклмнопрстуфхцчшщъьюя";
const RUS: &str = "абвгдежзийклмнопрстуфхцчшщъыьэюяё";
const UKR: &str = "абвгдежзийклмнопрстуфхцчшщьюяєіїґ";
const BEL: &str = "абвгдежзйклмнопрстуфхцчшыьэюяёіў";
const SRP: &str = "абвгдежзиклмнопрстуфхцчшђјљњћџ";
const MKD: &str = "абвгдежзиклмнопрстуфхцчшѓѕјљњќџ";

const ALL: &str = "абвгдежзийклмнопрстуфхцчшщъыьэюяёєіїґўђјљњћџѓѕќ";

pub fn alphabet_calculate_scores(text: &LowercaseText, filter_list: &FilterList) -> RawOutcome {
    let mut raw_scores: Vec<(Lang, i32)> = Script::Cyrillic
        .langs()
        .iter()
        .filter(|&&l| filter_list.is_allowed(l))
        .map(|&l| (l, 0i32))
        .collect();

    let max_raw_score = text.chars().filter(|&ch| is_relevant(ch)).count();

    for (lang, score) in &mut raw_scores {
        let alphabet = get_lang_chars(*lang);

        for ch in text.chars() {
            if !is_relevant(ch) {
                continue;
            } else if alphabet.contains(&ch) {
                *score += 1;
            } else {
                *score -= 1;
            }
        }
    }

    raw_scores.sort_unstable_by(|a, b| b.1.cmp(&a.1));

    let raw_scores: Vec<(Lang, usize)> = raw_scores
        .into_iter()
        .map(|(l, s)| {
            let score = if s < 0 { 0usize } else { s as usize };
            (l, score)
        })
        .collect();

    let mut normalized_scores = vec![];

    for &(lang, raw_score) in &raw_scores {
        // avoid devision by zero
        let normalized_score = if raw_score == 0 {
            0.0
        } else {
            raw_score as f64 / max_raw_score as f64
        };
        normalized_scores.push((lang, normalized_score));
    }

    RawOutcome {
        count: max_raw_score,
        raw_scores,
        scores: normalized_scores,
    }
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
