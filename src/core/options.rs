use super::{FilterList, Method};

// TODO: write doc
pub struct Options {
    pub(crate) filter_list: FilterList,
    pub(crate) method: Method,
}

impl Options {
    pub fn new() -> Self {
        Self {
            filter_list: FilterList::All,
            method: Method::Combined,
        }
    }

    pub fn with_filter_list(filter_list: FilterList) -> Self {
        Self::new().set_filter_list(filter_list)
    }

    pub fn with_method(method: Method) -> Self {
        Self::new().set_method(method)
    }

    pub fn set_filter_list(mut self, filter_list: FilterList) -> Self {
        self.filter_list = filter_list;
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
