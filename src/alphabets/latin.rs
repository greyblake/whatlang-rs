use std::cmp::Reverse;
use std::collections::HashMap;

use once_cell::sync::Lazy;

use super::RawOutcome;
use crate::core::{FilterList, LowercaseText};
use crate::utils::is_stop_char;
use crate::{Lang, Script};

const AFR: &str = "abcdefghijklmnopqrstuvwxyzáèéêëíîïóôúû";
const AKA: &str = "abdefghiklmnoprstuwyɔɛ";
const AZE: &str = "abcdefghijklmnopqrstuvxyzçöüğışə̇";
const CAT: &str = "abcdefghijklmnopqrstuvwxyz·àçèéíïòóúü";
const CES: &str = "abcdefghijklmnopqrstuvwxyzáéóúýčďěňřšťůž";
const DAN: &str = "abcdefghijklmnopqrstuvwxyzåæø";
const DEU: &str = "abcdefghijklmnopqrstuvwxyzßäöü";
const ENG: &str = "abcdefghijklmnopqrstuvwxyz";
const EPO: &str = "abcdefghijklmnoprstuvzĉĝĥĵŝŭ";
const EST: &str = "abcdefghijklmnopqrstuvwxyzäõöü";
const FIN: &str = "abcdefghijklmnopqrstuvwxyzäöšž";
const FRA: &str = "abcdefghijklmnopqrstuvwxyzàâçèéêëîïôùûüÿœ";
const HRV: &str = "abcdefghijklmnopqrstuvwxyzćčđšž";
const HUN: &str = "abcdefghijklmnopqrstuvwxyzáéíóöúüőű";
const IND: &str = "abcdefghijklmnopqrstuvwxyz";
const ITA: &str = "abcdefghijklmnopqrstuvwxyzàèéìòù";
const JAV: &str = "abcdefghijklmnopqrstuvwxyzèé";
const LAT: &str = "abcdefghijklmnopqrstuvwxyz";
const LAV: &str = "abcdefghijklmnopqrstuvwxyzāčēģīķļņōŗšūž";
const LIT: &str = "abcdefghijklmnopqrstuvwxyząčėęįšūųž";
const NLD: &str = "abcdefghijklmnopqrstuvwxyzàèéëïĳ";
const NOB: &str = "abcdefghijklmnopqrstuvwxyzåæø";
const POL: &str = "abcdefghijklmnopqrstuvwxyzóąćęłńśźż";
const POR: &str = "abcdefghijklmnopqrstuvwxyzàáâãçéêíóôõú";
const RON: &str = "abcdefghijklmnopqrstuvwxyzâîăşţ";
const SLK: &str = "abcdefghijklmnopqrstuvwxyzáäéíóôúýčďĺľňŕšťž";
const SLV: &str = "abcdefghijklmnopqrstuvwxyzčšž";
const SNA: &str = "abcdefghijklmnopqrstuvwxyz";
const SPA: &str = "abcdefghijklmnopqrstuvwxyz¡¿áéíñóúü";
const SWE: &str = "abcdefghijklmnopqrstuvwxyzäåö";
const TGL: &str = "abcdefghijklmnopqrstuvwxyzáéíñóú";
const TUK: &str = "abdefghijklmnoprstuwyzäçöüýňşž";
const TUR: &str = "abcdefghijklmnopqrstuvwxyzçöüğış̇";
const UZB: &str = "abcdefghijklmnopqrstuvxyzʻ";
const VIE: &str =
    "abcdefghijklmnopqrstuvwxyzàáâãèéêìíòóôõùúýăđĩũơưạảấầẩẫậắằẳẵặẹẻẽếềểễệỉịọỏốồổỗộớờởỡợụủứừửữựỳỵỷỹ";
const ZUL: &str = "abcdefghijklmnopqrstuvwxyz";

const LATIN_ALPHABETS: &[(Lang, &str)] = &[
    (Lang::Afr, AFR),
    (Lang::Aka, AKA),
    (Lang::Aze, AZE),
    (Lang::Cat, CAT),
    (Lang::Ces, CES),
    (Lang::Dan, DAN),
    (Lang::Deu, DEU),
    (Lang::Eng, ENG),
    (Lang::Epo, EPO),
    (Lang::Est, EST),
    (Lang::Fin, FIN),
    (Lang::Fra, FRA),
    (Lang::Hrv, HRV),
    (Lang::Hun, HUN),
    (Lang::Ind, IND),
    (Lang::Ita, ITA),
    (Lang::Jav, JAV),
    (Lang::Lat, LAT),
    (Lang::Lav, LAV),
    (Lang::Lit, LIT),
    (Lang::Nld, NLD),
    (Lang::Nob, NOB),
    (Lang::Pol, POL),
    (Lang::Por, POR),
    (Lang::Ron, RON),
    (Lang::Slk, SLK),
    (Lang::Slv, SLV),
    (Lang::Sna, SNA),
    (Lang::Spa, SPA),
    (Lang::Swe, SWE),
    (Lang::Tgl, TGL),
    (Lang::Tuk, TUK),
    (Lang::Tur, TUR),
    (Lang::Uzb, UZB),
    (Lang::Vie, VIE),
    (Lang::Zul, ZUL),
];

/// Inverted map binding a character to a set of languages.
pub static ALPHABET_LANG_MAP: Lazy<(Vec<char>, Vec<Vec<Lang>>)> = Lazy::new(|| {
    let mut map = HashMap::new();

    for (lang, alphabet) in LATIN_ALPHABETS {
        for c in alphabet.chars() {
            let entry = map.entry(c).or_insert_with(Vec::new);
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
    let (chars, langs) = &*ALPHABET_LANG_MAP;

    // score of each character.
    let mut char_scores = vec![0; chars.len()];
    let mut max_raw_score = 0;
    // iterate over the text and scores characters.
    for ch in text.chars() {
        if is_stop_char(ch) {
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
            if languages.len() == LATIN_ALPHABETS.len() {
                common_score += char_score;
            } else {
                for &lang in languages {
                    lang_scores[lang as usize] += char_score;
                }
            }
        }
    }

    // remap languages with theirs scores.
    let mut raw_scores: Vec<(Lang, usize)> = Script::Latin
        .langs()
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
        let normalized_score = raw_score as f64 / max_raw_score as f64;
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

    // Old naive implementation, that is not very effective but easy to understand
    fn naive_alphabet_calculate_scores(
        text: &LowercaseText,
        filter_list: &FilterList,
    ) -> RawOutcome {
        let mut raw_scores: Vec<(Lang, i32)> = Script::Latin
            .langs()
            .iter()
            .filter(|&&l| filter_list.is_allowed(l))
            .map(|&l| (l, 0i32))
            .collect();

        let max_raw_score = text.chars().filter(|&ch| !is_stop_char(ch)).count();

        for (lang, score) in &mut raw_scores {
            let alphabet: Vec<char> = LATIN_ALPHABETS
                .iter()
                .find(|(l, _)| l == lang)
                .unwrap()
                .1
                .chars()
                .collect();

            for ch in text.chars() {
                if is_stop_char(ch) {
                    continue;
                };
                if alphabet.contains(&ch) {
                    *score += 1;
                } else {
                    *score -= 1;
                }
            }
        }

        raw_scores.sort_by(|a, b| b.1.cmp(&a.1));

        let raw_scores: Vec<(Lang, usize)> = raw_scores
            .into_iter()
            .map(|(l, s)| {
                let score = if s < 0 { 0usize } else { s as usize };
                (l, score)
            })
            .collect();

        let mut normalized_scores = vec![];

        for &(lang, raw_score) in &raw_scores {
            let normalized_score = raw_score as f64 / max_raw_score as f64;
            normalized_scores.push((lang, normalized_score));
        }

        RawOutcome {
            count: max_raw_score,
            raw_scores,
            scores: normalized_scores,
        }
    }

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

    #[test]
    fn test_works_as_the_old_naive_implementation() {
        let filter = FilterList::All;
        let texts = [
            "Tyrkiske språk eller turkotatariske språk er en språkgruppe bestående av minst 35 språk",
            "René Descartes est un mathématicien, physicien et philosophe français, né le 31 mars 1596 à La Haye-en-Touraine",
            "Die Sonne scheint in das Büro der Grabdenkmalsfirma Heinrich Kroll & Söhne. Es ist April 923, und das Geschäf geht gut.",
        ];

        for text in texts {
            let lowercase_text = LowercaseText::new(text);
            let outcome = alphabet_calculate_scores(&lowercase_text, &filter);
            let naive_outcome = naive_alphabet_calculate_scores(&lowercase_text, &filter);

            // We can just compare outcome against naive_outcome, because ordering maybe different,
            // what is acceptable.
            assert_eq!(
                outcome.count, naive_outcome.count,
                "count failed. Text: {}",
                text
            );
            for (lang, raw_naive_score) in naive_outcome.raw_scores.into_iter() {
                let lookup_raw_score = |lang| {
                    outcome
                        .raw_scores
                        .iter()
                        .find(|(l, _)| *l == lang)
                        .unwrap()
                        .1
                };
                let raw_score = lookup_raw_score(lang);
                assert_eq!(
                    raw_score, raw_naive_score,
                    "raw_score VS raw_naive_score failed. Lang={}, Text: {}",
                    lang, text
                );
            }
            for (lang, naive_score) in naive_outcome.scores.into_iter() {
                let lookup_score =
                    |lang| outcome.scores.iter().find(|(l, _)| *l == lang).unwrap().1;
                let score = lookup_score(lang);
                assert_eq!(
                    score, naive_score,
                    "score VS naive_score failed. Lang={}, Text: {}",
                    lang, text
                );
            }
        }
    }
}
