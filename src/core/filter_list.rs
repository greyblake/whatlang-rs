use crate::Lang;

#[derive(Debug)]
pub enum FilterList {
    All,
    Only(Vec<Lang>),
    Except(Vec<Lang>),
}

impl FilterList {
    pub fn all() -> Self {
        Self::All
    }

    pub fn only(whitelist: Vec<Lang>) -> Self {
        Self::Only(whitelist)
    }

    pub fn except(blacklist: Vec<Lang>) -> Self {
        Self::Except(blacklist)
    }

    pub fn is_allowed(&self, lang: Lang) -> bool {
        match self {
            Self::All => true,
            Self::Only(ref whitelist) => whitelist.contains(&lang),
            Self::Except(ref blacklist) => !blacklist.contains(&lang),
        }
    }
}

impl Default for FilterList {
    fn default() -> Self {
        FilterList::All
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all() {
        let list = FilterList::all();
        assert!(list.is_allowed(Lang::Epo));
    }

    #[test]
    fn test_only() {
        let list = FilterList::only(vec![Lang::Rus, Lang::Ukr]);

        assert!(!list.is_allowed(Lang::Epo));
        assert!(!list.is_allowed(Lang::Eng));

        assert!(list.is_allowed(Lang::Rus));
        assert!(list.is_allowed(Lang::Ukr));
    }

    #[test]
    fn test_except() {
        let list = FilterList::except(vec![Lang::Rus, Lang::Ukr]);

        assert!(list.is_allowed(Lang::Epo));
        assert!(list.is_allowed(Lang::Eng));

        assert!(!list.is_allowed(Lang::Rus));
        assert!(!list.is_allowed(Lang::Ukr));
    }
}
