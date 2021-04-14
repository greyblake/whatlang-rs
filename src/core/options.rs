use super::{FilterList, Method};

#[derive(Debug, Clone)]
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

    #[cfg(feature = "dev")]
    pub fn with_filter_list(filter_list: FilterList) -> Self {
        Self::new().set_filter_list(filter_list)
    }

    #[cfg(feature = "dev")]
    pub fn with_method(method: Method) -> Self {
        Self::new().set_method(method)
    }

    pub fn set_filter_list(mut self, filter_list: FilterList) -> Self {
        self.filter_list = filter_list;
        self
    }

    #[cfg(feature = "dev")]
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
