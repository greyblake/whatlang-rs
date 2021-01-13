use crate::core::{Options, Method, Query, InternalQuery, Output};
use crate::scripts::{detect_script, grouping::{ScriptLangGroup, MultiLangScript}};
use crate::{trigrams};

pub fn detect_with_options(text: &str, options: &Options) -> Option<Output> {
    let query = Query {
        text,
        allow_list: &options.allow_list,
        method: options.method
    };
    detect_by_query(&query)
}

pub fn detect_by_query(query: &Query) -> Option<Output> {
    let script = detect_script(query.text)?;

    match script.to_lang_group() {
        ScriptLangGroup::One(lang) => Some(Output::new(script, lang)),
        ScriptLangGroup::Multi(multi_lang_script) => {
            detect_by_query_based_on_script(query, multi_lang_script)
        }
    }
}

fn detect_by_query_based_on_script(query: &Query, multi_lang_script: MultiLangScript) -> Option<Output> {
    let iquery = query.to_internal(multi_lang_script);
    match query.method {
        Method::Alphabet => alphabet_detect(&iquery),
        Method::Trigram => trigrams::alt::detect(&iquery),
        Method::Combined => combined_detect(&iquery),
    }
}

fn alphabet_detect(iquery: &InternalQuery) -> Option<Output> {
    None
}

fn combined_detect(iquery: &InternalQuery) -> Option<Output> {
    None
}
