use crate::core::{Info, Method, Options, Query};
use crate::scripts::{
    detect_script,
    grouping::{MultiLangScript, ScriptLangGroup},
};
use crate::Lang;
use crate::{alphabets, combined, trigrams};

pub fn detect_lang(text: &str) -> Option<Lang> {
    detect(text).map(|output| output.lang())
}

pub fn detect(text: &str) -> Option<Info> {
    let opts = Options::default();
    detect_with_options(text, &opts)
}

pub fn detect_with_options(text: &str, options: &Options) -> Option<Info> {
    let query = Query {
        text,
        filter_list: &options.filter_list,
        method: options.method,
    };
    detect_by_query(&query)
}

pub fn detect_by_query(query: &Query) -> Option<Info> {
    let script = detect_script(query.text)?;

    match script.to_lang_group() {
        ScriptLangGroup::One(lang) => Some(Info::new(script, lang, 1.0)),
        ScriptLangGroup::Multi(multi_lang_script) => {
            detect_by_query_based_on_script(query, multi_lang_script)
        }
        ScriptLangGroup::Mandarin => {
            // Sometimes Mandarin can be Japanese.
            // See https://github.com/greyblake/whatlang-rs/pull/45
            let lang = if query.filter_list.is_allowed(Lang::Cmn) {
                Lang::Cmn
            } else {
                Lang::Jpn
            };
            Some(Info::new(script, lang, 1.0))
        }
    }
}

fn detect_by_query_based_on_script(
    query: &Query,
    multi_lang_script: MultiLangScript,
) -> Option<Info> {
    let mut iquery = query.to_internal(multi_lang_script);
    match query.method {
        Method::Alphabet => alphabets::detect(&mut iquery),
        Method::Trigram => trigrams::detect(&mut iquery),
        Method::Combined => combined::detect(&mut iquery),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::FilterList;
    use crate::scripts::Script;

    #[test]
    fn test_detect_spanish() {
        let text = "Además de todo lo anteriormente dicho, también encontramos...";
        let output = detect(text);
        assert_eq!(output.is_some(), true);

        let info = output.unwrap();
        assert_eq!(info.lang(), Lang::Spa);
        assert_eq!(info.script(), Script::Latin);
    }

    #[test]
    fn test_detect_lang_ukrainian() {
        let text = "Та нічого, все нормально. А в тебе як?";
        assert_eq!(detect_lang(text), Some(Lang::Ukr));
    }

    #[test]
    fn test_detect_with_options_with_filter_list_except() {
        let text = "I am begging pardon";

        // without filter list
        let output = detect_with_options(text, &Options::default());
        assert_eq!(output.is_some(), true);
        let info = output.unwrap();
        assert_eq!(info.lang(), Lang::Jav);

        // with filter list
        let filter_list =
            FilterList::deny(vec![Lang::Jav, Lang::Nld, Lang::Uzb, Lang::Swe, Lang::Nob]);
        let options = Options::new().set_filter_list(filter_list);
        let output = detect_with_options(text, &options);
        assert_eq!(output.is_some(), true);
        let info = output.unwrap();
        assert_eq!(info.lang(), Lang::Eng);
    }

    // TODO:  see https://github.com/greyblake/whatlang-rs/issues/78
    // #[test]
    // fn test_detect_with_options_with_filter_list_except_none() {
    //     let text = "האקדמיה ללשון העברית";

    //     // All languages with Hebrew script are filtered out, so result must be None
    //     let filter_list = FilterList::except(vec![Lang::Heb, Lang::Yid]);
    //     let options = Options::new().set_filter_list(filter_list);
    //     let output = detect_with_options(text, &options);
    //     assert_eq!(output, None);
    // }

    #[test]
    fn test_detect_with_options_with_filter_list_only() {
        let filter_list = FilterList::allow(vec![Lang::Epo, Lang::Ukr]);
        let options = Options::new().set_filter_list(filter_list);

        let text = "Mi ne scias!";
        let output = detect_with_options(text, &options);
        assert_eq!(output.is_some(), true);
        let info = output.unwrap();
        assert_eq!(info.lang(), Lang::Epo);
    }

    #[test]
    fn test_detect_with_options_with_whitelist_mandarin_japanese() {
        let text = "水";

        let jpn_opts = Options::new().set_filter_list(FilterList::allow(vec![Lang::Jpn]));
        let info = detect_with_options(text, &jpn_opts).unwrap();
        assert_eq!(info.lang(), Lang::Jpn);

        let cmn_opts = Options::new().set_filter_list(FilterList::allow(vec![Lang::Cmn]));
        let info = detect_with_options(text, &cmn_opts).unwrap();
        assert_eq!(info.lang(), Lang::Cmn);
    }

    #[test]
    fn test_detect_with_options_with_blacklist_mandarin_japanese() {
        let text = "水";

        let jpn_opts = Options::new().set_filter_list(FilterList::deny(vec![Lang::Jpn]));
        let info = detect_with_options(text, &jpn_opts).unwrap();
        assert_eq!(info.lang(), Lang::Cmn);

        let cmn_opts = Options::new().set_filter_list(FilterList::deny(vec![Lang::Cmn]));
        let info = detect_with_options(text, &cmn_opts).unwrap();
        assert_eq!(info.lang(), Lang::Jpn);
    }
}
