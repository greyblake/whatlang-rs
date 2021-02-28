use super::RawOutcome;
use super::{cyrillic, latin};
use crate::core::{calculate_confidence, Info, InternalQuery, LowercaseText};
use crate::Lang;

pub fn detect(iquery: &mut InternalQuery) -> Option<Info> {
    let raw_outcome = raw_detect(iquery);
    let RawOutcome { count, scores, .. } = raw_outcome;

    let mut normalized_scores_iter = scores.into_iter();
    let opt_lang_score1 = normalized_scores_iter.next();
    let opt_lang_score2 = normalized_scores_iter.next();

    opt_lang_score1.map(|(lang1, score1)| {
        let script = iquery.multi_lang_script.to_script();
        let confidence = if let Some((_, score2)) = opt_lang_score2 {
            calculate_confidence(score1, score2, count)
        } else {
            1.0
        };
        Info::new(script, lang1, confidence)
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
        scores,
    }
}
