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
        allow_list: &options.allow_list,
        method: options.method,
    };
    detect_by_query(&query)
}

pub fn detect_by_query(query: &Query) -> Option<Info> {
    let script = detect_script(query.text)?;

    match script.to_lang_group() {
        ScriptLangGroup::One(lang) => Some(Info::new(script, lang)),
        ScriptLangGroup::Multi(multi_lang_script) => {
            detect_by_query_based_on_script(query, multi_lang_script)
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
