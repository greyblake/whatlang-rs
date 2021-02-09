use super::RawOutcome;
use crate::core::LowercaseText;
use crate::utils::is_stop_char;
use crate::Lang;

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
const NNO: &str = "abcdefghijklmnopqrstuvwxyzåæø";
const NOB: &str = "abcdefghijklmnopqrstuvwxyzåæø";
const POL: &str = "abcdefghijklmnopqrstuvwxyzóąćęłńśźż";
const POR: &str = "abcdefghijklmnopqrstuvwxyzàáâãçéêíóôõú";
const RON: &str = "abcdefghijklmnopqrstuvwxyzâîăşţ";
const SLK: &str = "abcdefghijklmnopqrstuvwxyzáäéíóôúýčďĺľňŕšťž";
const SLV: &str = "abcdefghijklmnopqrstuvwxyzčšž";
const SNA: &str = "abcdefghijklmnopqrstuvwxyz";
const SPA: &str = "abcdefghijklmnopqrstuvwxyz¡¿áéíñóúü";
const SWE: &str = "abcdefghijklmnopqrstuvwxyzäåö";
const TUK: &str = "abdefghijklmnoprstuwyzäçöüýňşž";
const TUR: &str = "abcdefghijklmnopqrstuvwxyzçöüğış̇";
const UZB: &str = "abcdefghijklmnopqrstuvxyzʻ";
const VIE: &str =
    "abcdefghijklmnopqrstuvwxyzàáâãèéêìíòóôõùúýăđĩũơưạảấầẩẫậắằẳẵặẹẻẽếềểễệỉịọỏốồổỗộớờởỡợụủứừửữựỳỵỷỹ";
const YOR: &str = "abcdefghijklmnoprstuvwyzàáèéìíòóùúńɔɛ̀́ṣẹọ";
const ZUL: &str = "abcdefghijklmnopqrstuvwxyz";

fn get_lang_chars(lang: Lang) -> Vec<char> {
    let alphabet = match lang {
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
        Lang::Nno => NNO,
        Lang::Nob => NOB,
        Lang::Pol => POL,
        Lang::Por => POR,
        Lang::Ron => RON,
        Lang::Slk => SLK,
        Lang::Slv => SLV,
        Lang::Sna => SNA,
        Lang::Spa => SPA,
        Lang::Swe => SWE,
        Lang::Tuk => TUK,
        Lang::Tur => TUR,
        Lang::Uzb => UZB,
        Lang::Vie => VIE,
        Lang::Yor => YOR,
        Lang::Zul => ZUL,

        _ => panic!(format!("No alphabet for {}", lang)),
    };
    alphabet.chars().collect()
}

pub fn alphabet_calculate_scores(text: &LowercaseText) -> RawOutcome {
    let mut raw_scores = vec![
        (Lang::Afr, 0i32),
        (Lang::Aka, 0i32),
        (Lang::Aze, 0i32),
        (Lang::Cat, 0i32),
        (Lang::Ces, 0i32),
        (Lang::Dan, 0i32),
        (Lang::Deu, 0i32),
        (Lang::Eng, 0i32),
        (Lang::Epo, 0i32),
        (Lang::Est, 0i32),
        (Lang::Fin, 0i32),
        (Lang::Fra, 0i32),
        (Lang::Hrv, 0i32),
        (Lang::Hun, 0i32),
        (Lang::Ind, 0i32),
        (Lang::Ita, 0i32),
        (Lang::Jav, 0i32),
        (Lang::Lat, 0i32),
        (Lang::Lav, 0i32),
        (Lang::Lit, 0i32),
        (Lang::Nld, 0i32),
        (Lang::Nno, 0i32),
        (Lang::Nob, 0i32),
        (Lang::Pol, 0i32),
        (Lang::Por, 0i32),
        (Lang::Ron, 0i32),
        (Lang::Slk, 0i32),
        (Lang::Slv, 0i32),
        (Lang::Sna, 0i32),
        (Lang::Spa, 0i32),
        (Lang::Swe, 0i32),
        (Lang::Tuk, 0i32),
        (Lang::Tur, 0i32),
        (Lang::Uzb, 0i32),
        (Lang::Vie, 0i32),
        (Lang::Yor, 0i32),
        (Lang::Zul, 0i32),
    ];

    let max_raw_score = text.chars().filter(|&ch| !is_stop_char(ch)).count();

    for (lang, score) in &mut raw_scores {
        let alphabet = get_lang_chars(*lang);

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
        raw_scores: raw_scores,
        scores: normalized_scores,
    }
}
