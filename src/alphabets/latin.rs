use std::cmp::Reverse;
use std::collections::HashMap;

use once_cell::sync::Lazy;

use super::RawOutcome;
use crate::alphabets::generic::{get_all_chars_in_langs, get_lang_chars, is_relevant_for_langs};
use crate::core::{FilterList, LowercaseText};
use crate::{Lang, Script};

/// Inverted map binding a character to a set of languages.
pub static ALPHABET_LANG_MAP: Lazy<(Vec<char>, Vec<Vec<Lang>>)> = Lazy::new(|| {
    let all_langs = Script::Latin.langs();

    let mut map = HashMap::new();
    for lang in all_langs {
        let alphabet = get_lang_chars(&lang);
        for c in alphabet.iter() {
            let entry = map.entry(*c).or_insert_with(Vec::new);
            entry.push(*lang);
        }
    }
    let mut char_lang: Vec<_> = map.into_iter().collect();
    char_lang.sort_unstable_by_key(|(c, _)| *c);

    let mut chars = Vec::with_capacity(char_lang.len());
    let mut langs = Vec::with_capacity(char_lang.len());
    for (ch, languages) in char_lang {
        chars.push(ch);
        langs.push(languages);
    }

    (chars, langs)
});

pub fn alphabet_calculate_scores(text: &LowercaseText, filter_list: &FilterList) -> RawOutcome {
    let all_langs = Script::Latin.langs();
    let all_chars_in_langs = get_all_chars_in_langs(all_langs);

    let (chars, langs) = &*ALPHABET_LANG_MAP;

    // score of each character.
    let mut char_scores = vec![0; chars.len()];
    let mut max_raw_score = 0;
    // iterate over the text and scores characters.
    for ch in text.chars() {
        if !is_relevant_for_langs(&ch, &all_chars_in_langs) {
            continue;
        }

        max_raw_score += 1;

        if let Ok(position) = chars.binary_search(&ch) {
            // add 2 and remove max_raw_score at the end,
            // to keep the score interval of -max_raw_score..max_raw_score
            char_scores[position] += 2;
        }
    }

    // score of each lang.
    let mut lang_scores = vec![0; Lang::all().len()];
    let mut common_score: usize = 0;
    // iterate over scored characters to compute language's scores.
    for (position, char_score) in char_scores.into_iter().enumerate() {
        if char_score > 0 {
            let languages = &langs[position];
            // if current character is common to all Languages, increment a common score
            // instead of iterating over all Languages scores.
            if languages.len() == all_langs.len() {
                common_score += char_score;
            } else {
                for &lang in languages {
                    lang_scores[lang as usize] += char_score;
                }
            }
        }
    }

    // remap languages with theirs scores.
    let mut raw_scores: Vec<(Lang, usize)> = all_langs
        .iter()
        .filter(|&&l| filter_list.is_allowed(l))
        .map(|&l| {
            let score = (lang_scores[l as usize] + common_score).saturating_sub(max_raw_score);
            (l, score)
        })
        .collect();

    raw_scores.sort_unstable_by_key(|(_, score)| Reverse(*score));

    let mut normalized_scores = vec![];

    for &(lang, raw_score) in raw_scores.iter() {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alphabet_calculate_scores_against_harmaja_hauras() {
        let text =
            LowercaseText::new("Ja kulkee kylmä hetki pariimme, Olet hauras kuin jää, Ja kulke");
        let filter = FilterList::All;

        let outcome = alphabet_calculate_scores(&text, &filter);
        assert_eq!(outcome.count, 50);
        assert_eq!(outcome.raw_scores.len(), 36);
        assert_eq!(outcome.scores.len(), 36);

        let raw_scores_for = |lang: Lang| {
            outcome
                .raw_scores
                .iter()
                .find(|(l, _)| *l == lang)
                .unwrap()
                .1
        };
        let scores_for = |lang: Lang| outcome.scores.iter().find(|(l, _)| *l == lang).unwrap().1;

        assert_eq!(raw_scores_for(Lang::Fin), 50);
        assert_eq!(raw_scores_for(Lang::Deu), 50);
        assert_eq!(raw_scores_for(Lang::Epo), 42);

        assert_eq!(scores_for(Lang::Fin), 1.0);
        assert_eq!(scores_for(Lang::Deu), 1.0);
        assert_eq!(scores_for(Lang::Epo), 0.84);
    }
}
