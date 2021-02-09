use super::{AllowList, Method, Text};
use crate::scripts::grouping::MultiLangScript;

pub struct Query<'a, 'b> {
    pub(crate) text: &'a str,
    pub(crate) allow_list: &'b AllowList,
    pub(crate) method: Method,
}

// TODO: find a better name?
// A query after script detection
pub struct InternalQuery<'a, 'b> {
    pub(crate) text: Text<'a>,
    pub(crate) allow_list: &'b AllowList,
    pub(crate) multi_lang_script: MultiLangScript,
}

impl<'a, 'b> Query<'a, 'b> {
    pub(crate) fn to_internal(&self, multi_lang_script: MultiLangScript) -> InternalQuery<'a, 'b> {
        InternalQuery {
            text: Text::new(self.text),
            allow_list: self.allow_list,
            multi_lang_script,
        }
    }
}
