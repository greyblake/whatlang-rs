use once_cell::sync::Lazy;

use super::common::{build_inverted_map, generic_alphabet_calculate_scores};
use super::RawOutcome;
use crate::core::{FilterList, LowercaseText};
use crate::{Lang, Script};

const AFR: &str = "abcdefghijklmnopqrstuvwxyzáèéêëíîïóôúû";
const AKA: &str = "abdefghiklmnoprstuwyɔɛ";
const AZE: &str = "abcdefghijklmnopqrstuvxyzçöüğışə̇";
const CAT: &str = "abcdefghijklmnopqrstuvwxyz·àçèéíïòóúü";
const CES: &str = "abcdefghijklmnopqrstuvwxyzáéíóúýčďěňřšťůž";
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
pub static ALPHABET_LANG_MAP: Lazy<(Vec<char>, Vec<Vec<Lang>>)> =
    Lazy::new(|| build_inverted_map(LATIN_ALPHABETS));

pub fn alphabet_calculate_scores(text: &LowercaseText, filter_list: &FilterList) -> RawOutcome {
    generic_alphabet_calculate_scores(Script::Latin, &ALPHABET_LANG_MAP, text, filter_list)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::is_stop_char;
    use crate::Script;

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
