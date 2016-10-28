use std::collections::HashMap;

mod lang;
mod trigrams;

use lang::Lang;
use lang::LangProfile;
use lang::LANGS;
use trigrams::count_trigrams;

pub fn detect_lang(text : String) -> Lang {
    let mut lang_distances : Vec<(Lang, u32)> = vec![];
    let trigrams = count_trigrams(text);

    for &(ref lang, lang_trigrams) in LANGS {
        let dist = calculate_distance(lang_trigrams, &trigrams);
        lang_distances.push(((*lang).clone(), dist));
    }

    lang_distances.sort_by_key(|key| key.1 );
    (lang_distances.iter().nth(0).unwrap().0).clone()
}

const MAX_DIST : u32 = 300;

fn calculate_distance(lang_trigrams: LangProfile,  text_trigrams: &HashMap<String, u32>) -> u32 {
    let mut total_dist = 0u32;

    for (i, &trigram) in lang_trigrams.iter().enumerate() {
        let dist = match text_trigrams.get(trigram) {
            Some(&n) => (n as i32 - i as i32).abs() as u32,
            None => MAX_DIST
        };
        total_dist += dist;
    }
    total_dist
}

#[cfg(test)]
mod tests {
    use lang::Lang;
    use super::detect_lang;

    #[test]
    fn test_detect_lang() {
        let eng_text = "English does not suit well for the role of international language".to_string();
        let spa_text = "Además de todo lo anteriormente dicho, también encontramos dentro de estos recintos la aduana, lugar donde los pasajeros que salen o entran del país son controlados.".to_string();
        let por_text = "A princípio, o interesse do Corinthians na contratação de Ronaldo foi tratado como algo impossível no Parque São Jorge".to_string();

        assert_eq!(detect_lang(eng_text), Lang::ENG);
        assert_eq!(detect_lang(spa_text), Lang::SPA);
        assert_eq!(detect_lang(por_text), Lang::POR);
    }
}
