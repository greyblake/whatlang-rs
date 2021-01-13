mod cyrillic;
mod latin;

use crate::Lang;
use crate::core::{InternalQuery, Output, LangScores};

pub fn detect(iquery: &mut InternalQuery) -> Option<Output> {
    let lang_scores = raw_detect(iquery);
    lang_scores.scores.first().map( |&(lang, _)| {
        let script = iquery.multi_lang_script.to_script();
        Output::new(script, lang)
    })
}

pub fn raw_detect(iquery: &mut InternalQuery) -> LangScores {
    let text: &str = iquery.text.lowercased();

    use crate::scripts::grouping::MultiLangScript as MLS;
    let scores = match iquery.multi_lang_script {
        // TODO: pass lowercased text as a newtype
        MLS::Cyrillic => cyrillic::alphabet_calculate_scores(text),
        MLS::Latin => latin::alphabet_calculate_scores(text),
        MLS::Arabic => {
            // TODO: implement alphabets for Arabic script
            vec![
                (Lang::Ara, 1.0),
                (Lang::Urd, 1.0),
                (Lang::Pes, 1.0),
            ]
        },
        MLS::Devanagari => {
            // TODO: implement alphabets for Devanagari script
            vec![
                (Lang::Hin, 1.0),
                (Lang::Mar, 1.0),
                (Lang::Nep, 1.0),
            ]
        },
        MLS::Hebrew => {
            // TODO: implement alphabets for Hebrew script
            vec![
                (Lang::Heb, 1.0),
                (Lang::Yid, 1.0),
            ]
        },
    };
    LangScores::new(scores)
}
