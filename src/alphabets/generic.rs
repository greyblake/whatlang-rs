use crate::alphabets::RawOutcome;
use crate::core::{FilterList, LowercaseText};
use crate::Lang;
use std::cmp::Reverse;
use std::collections::HashSet;

const BUL: &str = "абвгдежзийклмнопрстуфхцчшщъьюя";
const RUS: &str = "абвгдежзийклмнопрстуфхцчшщъыьэюяё";
const UKR: &str = "абвгдежзийклмнопрстуфхцчшщьюяєіїґ";
const BEL: &str = "абвгдежзйклмнопрстуфхцчшыьэюяёіў";
const SRP: &str = "абвгдежзиклмнопрстуфхцчшђјљњћџ";
const MKD: &str = "абвгдежзиклмнопрстуфхцчшѓѕјљњќџ";

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

pub fn get_lang_chars(lang: Lang) -> Vec<char> {
    let alphabet = match lang {
        Lang::Bul => BUL,
        Lang::Rus => RUS,
        Lang::Ukr => UKR,
        Lang::Bel => BEL,
        Lang::Srp => SRP,
        Lang::Mkd => MKD,
        Lang::Afr => AFR,
        Lang::Aka => AKA,
        Lang::Aze => AZE,
        Lang::Cat => CAT,
        Lang::Ces => CES,
        Lang::Dan => DAN,
        Lang::Deu => DEU,
        Lang::Eng => ENG,
        Lang::Epo => EPO,
        Lang::Est => EST,
        Lang::Fin => FIN,
        Lang::Fra => FRA,
        Lang::Hrv => HRV,
        Lang::Hun => HUN,
        Lang::Ind => IND,
        Lang::Ita => ITA,
        Lang::Jav => JAV,
        Lang::Lat => LAT,
        Lang::Lav => LAV,
        Lang::Lit => LIT,
        Lang::Nld => NLD,
        Lang::Nob => NOB,
        Lang::Pol => POL,
        Lang::Por => POR,
        Lang::Ron => RON,
        Lang::Slk => SLK,
        Lang::Slv => SLV,
        Lang::Sna => SNA,
        Lang::Spa => SPA,
        Lang::Swe => SWE,
        Lang::Tgl => TGL,
        Lang::Tuk => TUK,
        Lang::Tur => TUR,
        Lang::Uzb => UZB,
        Lang::Vie => VIE,
        Lang::Zul => ZUL,
        _ => panic!("No alphabet for {}", lang),
    };
    alphabet.chars().collect()
}

pub fn get_all_chars_in_langs(langs: &[Lang]) -> HashSet<char> {
    langs
        .iter()
        .flat_map(|&lang| get_lang_chars(lang))
        .collect()
}

pub fn is_relevant_for_langs(ch: &char, chars: &HashSet<char>) -> bool {
    chars.contains(ch)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_get_lang_chars() {
        assert_eq!(get_lang_chars(Lang::Bul).len(), 30);
        assert_eq!(get_lang_chars(Lang::Rus).len(), 33);
        //     TODO: finish tests
    }

    #[test]
    fn test_get_all_chars_in_langs() {
        assert_eq!(
            get_all_chars_in_langs(&vec![Lang::Bul, Lang::Rus]).len(),
            33
        );
        assert_eq!(
            get_all_chars_in_langs(&vec![Lang::Bul, Lang::Rus, Lang::Ukr]).len(),
            37
        );
        // TODO: finish tests
    }

    #[test]
    fn test_is_relevant_for_langs() {
        let chars: HashSet<char> = vec!['a', 'b'].into_iter().collect();
        assert_eq!(is_relevant_for_langs(&'a', &chars), true);
        assert_eq!(is_relevant_for_langs(&'c', &chars), false);
    }
}

pub fn alphabet_calculate_scores_generic(
    text: &LowercaseText,
    filter_list: &FilterList,
    all_langs: &[Lang],
) -> RawOutcome {
    let all_chars_in_langs = get_all_chars_in_langs(all_langs);

    let mut raw_scores: Vec<(Lang, i32)> = all_langs
        .iter()
        .filter(|&&l| filter_list.is_allowed(l))
        .map(|&l| (l, 0i32))
        .collect();

    let max_raw_score = text
        .chars()
        .filter(|&ch| is_relevant_for_langs(&ch, &all_chars_in_langs))
        .count();

    for (lang, score) in &mut raw_scores {
        let alphabet = get_lang_chars(*lang);

        for ch in text.chars() {
            // if !is_relevant(ch) {
            if !is_relevant_for_langs(&ch, &all_chars_in_langs) {
                continue;
            } else if alphabet.contains(&ch) {
                *score += 1;
            } else {
                *score -= 1;
            }
        }
    }

    raw_scores.sort_unstable_by_key(|(_, score)| Reverse(*score));

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
