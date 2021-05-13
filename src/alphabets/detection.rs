use super::RawOutcome;
use super::{cyrillic, latin};
use crate::core::{calculate_confidence, FilterList, Info, InternalQuery, LowercaseText};
use crate::Lang;

pub fn detect(iquery: &InternalQuery) -> Option<Info> {
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

pub fn raw_detect(iquery: &InternalQuery) -> RawOutcome {
    use crate::scripts::grouping::MultiLangScript as MLS;

    let text: &LowercaseText = &iquery.text.lowercase();
    let filter_list: &FilterList = &iquery.filter_list;
    match iquery.multi_lang_script {
        MLS::Cyrillic => cyrillic::alphabet_calculate_scores(text, filter_list),
        MLS::Latin => latin::alphabet_calculate_scores(text, filter_list),

        // TODO: implement alphabets for Arabic script
        MLS::Arabic => build_mock(vec![Lang::Ara, Lang::Urd, Lang::Pes], filter_list),

        // TODO: implement alphabets for Devanagari script
        MLS::Devanagari => build_mock(vec![Lang::Hin, Lang::Mar, Lang::Nep], filter_list),

        // TODO: implement alphabets for Hebrew script
        MLS::Hebrew => build_mock(vec![Lang::Heb, Lang::Yid], filter_list),
    }
}

fn build_mock(langs: Vec<Lang>, filter_list: &FilterList) -> RawOutcome {
    let filtered_langs = langs
        .into_iter()
        .filter(|lang| filter_list.is_allowed(*lang));
    let raw_scores = filtered_langs.clone().map(|l| (l, 1)).collect();
    let scores = filtered_langs.map(|l| (l, 1.0)).collect();
    RawOutcome {
        count: 1,
        raw_scores,
        scores,
    }
}
