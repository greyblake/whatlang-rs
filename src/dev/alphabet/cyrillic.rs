use super::NormalizedOutcome;
use crate::utils::is_stop_char;
use crate::Lang;

const BUL: &'static str = "АаБбВвГгДдЕеЖжЗзИиЙйКкЛлМмНнОоПпРрСсТтУуФфХхЦцЧчШшЩщЪъЬьЮюЯя";
const RUS: &'static str = "АаБбВвГгДдЕеЁёЖжЗзИиЙйКкЛлМмНнОоПпРрСсТтУуФфХхЦцЧчШшЩщЪъЫыЬьЭэЮюЯя";
const UKR: &'static str = "АаБбВвГгҐґДдЕеЄєЖжЗзИиІіЇїЙйКкЛлМмНнОоПпРрСсТтУуФфХхЦцЧчШшЩщЬьЮюЯя";
const BEL: &'static str = "АаБбВвГгДдЕеЁёЖжЗзІіЙйКкЛлМмНнОоПпРрСсТтУуЎўФфХхЦцЧчШшЫыЬьЭэЮюЯя";
const SRP: &'static str = "АаБбВвГгДдЂђЕеЖжЗзИиЈјКкЛлЉљМмНнЊњОоПпРрСсТтЋћУуФфХхЦцЧчЏџШш";
const MKD: &'static str = "АаБбВвГгДдЃѓЕеЖжЗзЅѕИиЈјКкЛлЉљМмНнЊњОоПпРрСсТтЌќУуФфХхЦцЧчЏџШш";

type RawScores = [(Lang, usize); 6];
type NormalizedScores = [(Lang, f64); 6];

#[derive(Debug)]
pub struct Outcome {
    max_raw_score: usize,
    raw_scores: RawScores,
    normalized_scores: NormalizedScores,
}

impl NormalizedOutcome for Outcome {
    fn normalized_scores(&self) -> &[(Lang, f64)] {
        &self.normalized_scores
    }
}

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
    let mut raw_scores: RawScores = [
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

    let mut normalized_scores: NormalizedScores = [
        (Lang::Rus, 0.0),
        (Lang::Ukr, 0.0),
        (Lang::Bul, 0.0),
        (Lang::Bel, 0.0),
        (Lang::Mkd, 0.0),
        (Lang::Srp, 0.0),
    ];

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

// pub fn combo_detect(text: &str) -> Option<Lang> {
//     use crate::calculate_scores_with_options;
//     use crate::Options;
//
//     let alphabet_scores = alphabet_calculate_scores(text);
//
//     let whitelist = vec![Lang::Rus, Lang::Ukr, Lang::Bul, Lang::Bel, Lang::Srp, Lang::Mkd];
//     let options = Options::new().set_whitelist(whitelist);
//     let trigram_scores = calculate_scores_with_options(text, &options);
//
//     let mut all_langs: Vec<Lang> = alphabet_scores.iter().map(|x| x.0).collect();
//     trigram_scores.iter().for_each(|(lang, _)| {
//         if !all_langs.contains(lang) {
//             all_langs.push(*lang);
//         }
//     });
//
//     let mut scores = vec![];
//
//     for lang in all_langs {
//         let a: f64 = alphabet_scores.iter().find(|(l, _)| l == &lang).map(|x| x.1).unwrap_or(0.0);
//         let t: f64 = trigram_scores.iter().find(|(l, _)| l == &lang).map(|x| x.1).unwrap_or(0.0);
//         let score = a * t;
//         scores.push((lang, score));
//     }
//
//     scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Less));
//     Some(scores[0].0)
// }
