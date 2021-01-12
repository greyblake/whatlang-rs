
// Input (query):
// * text = Str
// * list = Whitelist | Blacklist | None
// * method = Trigram | Alphabet | Combined
//
// Output (info)
// * script = Script
// * Lang = lang
//


use crate::{Lang, Script};
use crate::detect_script;

#[derive(Debug, Clone, Copy)]
enum Method {
    Trigram,
    Alphabet,
    Combined
}

// TODO: rename
// * FilterList
// * FilterRule
// * GuardList
// * AccessList
// * AllowList


struct Query<'a, 'b> {
    text: &'a str,
    allow_list: &'b AllowList,
    method: Method
}

struct Options {
    allow_list: AllowList,
    method: Method
}

struct Output {
    lang: Lang,
    script: Script,
}

fn detect_with_options(text: &str, options: &Options) {
    let query = Query {
        text,
        allow_list: &options.allow_list,
        method: options.method
    };
}

use crate::scripts::grouping::{ScriptLangGroup, MultiLangScript};

fn detect_by_query(query: &Query) -> Option<Output> {
    let script = detect_script(query.text)?;

    match script.to_lang_group() {
        ScriptLangGroup::One(lang) => Some(Output { script, lang }),
        ScriptLangGroup::Multi(multi_lang_script) => {
            detect_by_query_based_on_script(query, multi_lang_script)
        }
    }


}

fn detect_by_query_based_on_script(query: &Query, multi_lang_script: MultiLangScript) -> Option<Output> {
    let iquery = InternalQuery {
        text: query.text,
        allow_list: query.allow_list,
        multi_lang_script,
    };
    match query.method {
        Method::Alphabet => alphabet_detect(&iquery),
        Method::Trigram => trigram_detect(&iquery),
        Method::Combined => combined_detect(&iquery),
    }
}

// TODO: find a better name?
// A query after script detection
struct InternalQuery<'a, 'b> {
    text: &'a str,
    multi_lang_script: MultiLangScript,
    allow_list: &'b AllowList
}

fn alphabet_detect(iquery: &InternalQuery) -> Option<Output> {
    None
}

fn trigram_detect(iquery: &InternalQuery) -> Option<Output> {
    None
}
fn combined_detect(iquery: &InternalQuery) -> Option<Output> {
    None
}
