use super::{AllowList, Method};

pub struct Options {
    pub(crate) allow_list: AllowList,
    pub(crate) method: Method
}

impl Default for Options {
    fn default() -> Self {
        Self { allow_list: AllowList::All, method: Method::Combined }
    }
}
