
use crate::utils::is_stop_char;
use crate::Lang;
use super::Outcome;

const BUL: &'static str = "АаБбВвГгДдЕеЖжЗзИиЙйКкЛлМмНнОоПпРрСсТтУуФфХхЦцЧчШшЩщЪъЬьЮюЯя";
const RUS: &'static str = "АаБбВвГгДдЕеЁёЖжЗзИиЙйКкЛлМмНнОоПпРрСсТтУуФфХхЦцЧчШшЩщЪъЫыЬьЭэЮюЯя";
const UKR: &'static str = "АаБбВвГгҐґДдЕеЄєЖжЗзИиІіЇїЙйКкЛлМмНнОоПпРрСсТтУуФфХхЦцЧчШшЩщЬьЮюЯя";
const BEL: &'static str = "АаБбВвГгДдЕеЁёЖжЗзІіЙйКкЛлМмНнОоПпРрСсТтУуЎўФфХхЦцЧчШшЫыЬьЭэЮюЯя";
const SRP: &'static str = "АаБбВвГгДдЂђЕеЖжЗзИиЈјКкЛлЉљМмНнЊњОоПпРрСсТтЋћУуФфХхЦцЧчЏџШш";
const MKD: &'static str = "АаБбВвГгДдЃѓЕеЖжЗзЅѕИиЈјКкЛлЉљМмНнЊњОоПпРрСсТтЌќУуФфХхЦцЧчЏџШш";


fn get_lang_chars(lang: Lang) -> Vec<char> {
    let alphabet = match lang {
        Lang::Rus => RUS,
        Lang::Ukr => UKR,
        Lang::Bul => BUL,
        Lang::Bel => BEL,
        Lang::Srp => SRP,
        Lang::Mkd => MKD,
        _ => panic!(format!("No alphabet for {}", lang)),
    };
    alphabet.chars().collect()
}

pub fn alphabet_calculate_scores(text: &str) -> Outcome {
    let mut raw_scores = vec![
        (Lang::Rus, 0),
        (Lang::Ukr, 0),
        (Lang::Bul, 0),
        (Lang::Bel, 0),
        (Lang::Mkd, 0),
        (Lang::Srp, 0),
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

    let mut normalized_scores = vec![];

    for (index, &(lang, raw_score)) in raw_scores.iter().enumerate() {
        let normalized_score = raw_score as f64 / max_raw_score as f64;
        normalized_scores[index] = (lang, normalized_score);
    }

    Outcome {
        max_raw_score,
        raw_scores,
        normalized_scores,
    }
}
