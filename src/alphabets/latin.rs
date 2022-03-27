use super::RawOutcome;
use crate::core::{FilterList, LowercaseText};
use crate::utils::is_stop_char;
use crate::{alphabets, Lang, Script};

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

fn calculate_lang_score(lang: &Lang, text: &LowercaseText) -> usize {
    // TODO: merge with calculate_lang_score from cyrillic.rs
    let alphabet = get_lang_chars(*lang);
    let score: i32 = text
        .chars()
        .map(|ch| {
            if !is_stop_char(ch) {
                0
            } else if alphabet.contains(&ch) {
                1
            } else {
                -1
            }
        })
        .sum();

    if score < 0 {
        0usize
    } else {
        score as usize
    }
}

pub fn alphabet_calculate_scores(text: &LowercaseText, filter_list: &FilterList) -> RawOutcome {
    let max_raw_score = text.chars().filter(|&ch| !is_stop_char(ch)).count();

    let raw_scores: Vec<(Lang, usize)> = Script::Latin
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
