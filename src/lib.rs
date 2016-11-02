use std::collections::HashMap;

mod lang;
mod trigrams;

use lang::*;
use trigrams::*;

const MAX_DIST : u32 = 300;

pub fn detect_lang(text : String) -> Lang {
    let mut lang_distances : Vec<(Lang, u32)> = vec![];
    let trigrams = get_trigrams_with_positions(text);

    for &(ref lang, lang_trigrams) in LANGS {
        let dist = calculate_distance(lang_trigrams, &trigrams);
        lang_distances.push(((*lang).clone(), dist));
    }

    lang_distances.sort_by_key(|key| key.1 );
    (lang_distances.iter().nth(0).unwrap().0).clone()
}

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
        let spa_text = "Además de todo lo anteriormente dicho, también encontramos...".to_string();
        let por_text = "A princípio, o interesse do Corinthians na contratação...".to_string();

        assert_eq!(detect_lang(eng_text), Lang::Eng);
        assert_eq!(detect_lang(spa_text), Lang::Spa);
        assert_eq!(detect_lang(por_text), Lang::Por);
    }
}
