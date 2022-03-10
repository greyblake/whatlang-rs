use super::RawOutcome;
use crate::core::{FilterList, LowercaseText};
use crate::utils::is_stop_char;
use crate::{Lang, Script};

use once_cell::sync::Lazy;
use std::collections::HashMap;

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
pub static ALPHABET_LANG_MAP: Lazy<Vec<(char, Vec<Lang>)>> = Lazy::new(|| {
    let mut map = HashMap::new();

    for (lang, alphabet) in LATIN_ALPHABETS {
        for c in alphabet.chars() {
            let entry = map.entry(c).or_insert_with(Vec::new);
            entry.push(*lang);
        }
    }

    let mut char_lang = Vec::new();

    for (ch, languages) in map {
        char_lang.push((ch, languages));
    }

    char_lang
});

pub fn alphabet_calculate_scores(text: &LowercaseText, filter_list: &FilterList) -> RawOutcome {
    let char_lang = &*ALPHABET_LANG_MAP;
    let max_raw_score = text.chars().filter(|&ch| !is_stop_char(ch)).count();

    let mut raw_scores: Vec<(Lang, usize)> = Script::Latin
        .langs()
        .iter()
        .filter(|&&l| filter_list.is_allowed(l))
        .map(|&l| (l, 0))
        .collect();

    for ch in text.chars() {
        if is_stop_char(ch) {
            continue;
        }

        if let Some((_, languages)) = char_lang.iter().find(|(c, _)| *c == ch) {
            for lang in languages {
                if let Some((_, score)) = raw_scores.iter_mut().find(|(l, _)| l == lang) {
                    *score += 2;
                }
            }
        }
    }

    raw_scores.sort_unstable_by(|a, b| b.1.cmp(&a.1));

    let mut normalized_scores = vec![];

    for &(lang, raw_score) in &raw_scores {
        let normalized_score =
            raw_score.saturating_sub(max_raw_score) as f64 / max_raw_score as f64;
        normalized_scores.push((lang, normalized_score));
    }

    RawOutcome {
        count: max_raw_score,
        raw_scores,
        scores: normalized_scores,
    }
}
