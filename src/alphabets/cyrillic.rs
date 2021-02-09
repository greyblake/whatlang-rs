use super::RawOutcome;
use crate::core::LowercaseText;
use crate::utils::is_stop_char;
use crate::Lang;

const BUL: &str = "абвгдежзийклмнопрстуфхцчшщъьюя";
const RUS: &str = "абвгдежзийклмнопрстуфхцчшщъыьэюяё";
const UKR: &str = "абвгдежзийклмнопрстуфхцчшщьюяєіїґ";
const BEL: &str = "абвгдежзйклмнопрстуфхцчшыьэюяёіў";
const SRP: &str = "абвгдежзиклмнопрстуфхцчшђјљњћџ";
const MKD: &str = "абвгдежзиклмнопрстуфхцчшѓѕјљњќџ";

pub fn alphabet_calculate_scores(text: &LowercaseText) -> RawOutcome {
    let mut raw_scores = vec![
        (Lang::Bul, 0i32),
        (Lang::Rus, 0i32),
        (Lang::Ukr, 0i32),
        (Lang::Bel, 0i32),
        (Lang::Srp, 0i32),
        (Lang::Mkd, 0i32),
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

fn get_lang_chars(lang: Lang) -> Vec<char> {
    let alphabet = match lang {
        Lang::Bul => BUL,
        Lang::Rus => RUS,
        Lang::Ukr => UKR,
        Lang::Bel => BEL,
        Lang::Srp => SRP,
        Lang::Mkd => MKD,

        _ => panic!(format!("No alphabet for {}", lang)),
    };
    alphabet.chars().collect()
}
