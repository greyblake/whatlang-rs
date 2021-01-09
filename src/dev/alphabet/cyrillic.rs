use crate::Lang;

const BUL: &'static str = "АаБбВвГгДдЕеЖжЗзИиЙйКкЛлМмНнОоПпРрСсТтУуФфХхЦцЧчШшЩщЪъЬьЮюЯя";
const RUS: &'static str = "АаБбВвГгДдЕеЁёЖжЗзИиЙйКкЛлМмНнОоПпРрСсТтУуФфХхЦцЧчШшЩщЪъЫыЬьЭэЮюЯя";
const UKR: &'static str = "АаБбВвГгҐґДдЕеЄєЖжЗзИиІіЇїЙйКкЛлМмНнОоПпРрСсТтУуФфХхЦцЧчШшЩщЬьЮюЯя";
const BEL: &'static str = "АаБбВвГгДдЕеЁёЖжЗзІіЙйКкЛлМмНнОоПпРрСсТтУуЎўФфХхЦцЧчШшЫыЬьЭэЮюЯя";
const SRP: &'static str = "АаБбВвГгДдЂђЕеЖжЗзИиЈјКкЛлЉљМмНнЊњОоПпРрСсТтЋћУуФфХхЦцЧчЏџШш";
const MKD: &'static str = "АаБбВвГгДдЃѓЕеЖжЗзЅѕИиЈјКкЛлЉљМмНнЊњОоПпРрСсТтЌќУуФфХхЦцЧчЏџШш";

pub fn is_stop_char(ch: char) -> bool {
    matches!(ch, '\u{0000}'..='\u{0040}' | '\u{005B}'..='\u{0060}' | '\u{007B}'..='\u{007E}')
}

fn get_lang_chars(lang: Lang) -> Vec<char> {
    let alphabet = match lang {
        Lang::Rus => RUS,
        Lang::Ukr => UKR,
        Lang::Bul => BUL,
        Lang::Bel => BEL,
        Lang::Srp => SRP,
        Lang::Mkd => MKD,
        _ => panic!(format!("No alphabet for {}", lang))
    };
    alphabet.chars().collect()
}

pub fn alphabet_calculate_scores(text: &str) -> Vec<(Lang, f64)> {
    let mut lang_scores: Vec<(Lang, u32)> = vec![
        (Lang::Rus, 0),
        (Lang::Ukr, 0),
        (Lang::Bul, 0),
        (Lang::Bel, 0),
        (Lang::Mkd, 0),
        (Lang::Srp, 0),
    ];

    let char_count = text.chars().filter(|&ch| !is_stop_char(ch)).count();

    for (lang, score) in &mut lang_scores {

        let alphabet = get_lang_chars(*lang);

        for ch in text.chars() {
            if is_stop_char(ch) { continue };
            if alphabet.contains(&ch) {
                *score += 1;
            } else {
                *score -= 1;
            }
        }
    }

    lang_scores.sort_by(|a, b| b.1.cmp(&a.1));

    let lang_confidences: Vec<(Lang, f64)> = lang_scores
        .iter()
        .map(|&(lang, score)| (lang, score as f64 / char_count as f64))
        .collect();

    lang_confidences
}

pub fn combo_detect(text: &str) -> Option<Lang> {
    use crate::calculate_scores_with_options;
    use crate::Options;

    let alphabet_scores = alphabet_calculate_scores(text);

    let whitelist = vec![Lang::Rus, Lang::Ukr, Lang::Bul, Lang::Bel, Lang::Srp, Lang::Mkd];
    let options = Options::new().set_whitelist(whitelist);
    let trigram_scores = calculate_scores_with_options(text, &options);

    let mut all_langs: Vec<Lang> = alphabet_scores.iter().map(|x| x.0).collect();
    trigram_scores.iter().for_each(|(lang, _)| {
        if !all_langs.contains(lang) {
            all_langs.push(*lang);
        }
    });

    let mut scores = vec![];

    for lang in all_langs {
        let a: f64 = alphabet_scores.iter().find(|(l, _)| l == &lang).map(|x| x.1).unwrap_or(0.0);
        let t: f64 = trigram_scores.iter().find(|(l, _)| l == &lang).map(|x| x.1).unwrap_or(0.0);
        let score = a * t;
        scores.push((lang, score));
    }

    scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Less));
    Some(scores[0].0)
}
