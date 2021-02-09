use super::{AllowList, Method};

// TODO: write doc
pub struct Options {
    pub(crate) allow_list: AllowList,
    pub(crate) method: Method,
}

impl Options {
    pub fn new() -> Self {
        Self {
            allow_list: AllowList::All,
            method: Method::Combined,
        }
    }

    pub fn with_allow_list(allow_list: AllowList) -> Self {
        Self::new().set_allow_list(allow_list)
    }

    pub fn with_method(method: Method) -> Self {
        Self::new().set_method(method)
    }

    pub fn set_allow_list(mut self, allow_list: AllowList) -> Self {
        self.allow_list = allow_list;
        self
    }

    pub fn set_method(mut self, method: Method) -> Self {
        self.method = method;
        self
    }
}

impl Default for Options {
    fn default() -> Self {
        Self::new()
    }
}
