use lang::Lang;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum List {
    White(Vec<Lang>),
    Black(Vec<Lang>)
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Options {
    pub(crate) list: Option<List>
}

impl Options {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_whitelist(mut self, whitelist: Vec<Lang>) -> Self {
        self.list = Some(List::White(whitelist));
        self
    }

    pub fn set_blacklist(mut self, blacklist: Vec<Lang>) -> Self {
        self.list = Some(List::Black(blacklist));
        self
    }
}
