use crate::Lang;
use super::{cyrillic, latin};
use crate::core::{InternalQuery, Output, LowercaseText};
use super::RawOutcome;

pub fn detect(iquery: &mut InternalQuery) -> Option<Output> {
    let raw_outcome = raw_detect(iquery);
    raw_outcome.scores.first().map( |&(lang, _)| {
        let script = iquery.multi_lang_script.to_script();
        Output::new(script, lang)
    })
}

pub fn raw_detect(iquery: &mut InternalQuery) -> RawOutcome {
    use crate::scripts::grouping::MultiLangScript as MLS;

    let text: &LowercaseText = iquery.text.lowercase();
    match iquery.multi_lang_script {
        MLS::Cyrillic => cyrillic::alphabet_calculate_scores(text),
        MLS::Latin => latin::alphabet_calculate_scores(text),

        // TODO: implement alphabets for Arabic script
        MLS::Arabic => build_mock(vec![Lang::Ara, Lang::Urd, Lang::Pes]),

        // TODO: implement alphabets for Devanagari script
        MLS::Devanagari => build_mock(vec![Lang::Hin, Lang::Mar, Lang::Nep]),

        // TODO: implement alphabets for Hebrew script
        MLS::Hebrew => build_mock(vec![Lang::Heb, Lang::Yid]),
    }
}

fn build_mock(langs: Vec<Lang>) -> RawOutcome {
    let raw_scores = langs.iter().map(|&l| (l, 1)).collect();
    let scores = langs.iter().map(|&l| (l, 1.0)).collect();
    RawOutcome {
        count: 1,
        raw_scores,
        scores
    }
}
