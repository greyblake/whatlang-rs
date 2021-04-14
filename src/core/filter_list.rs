use crate::Lang;

#[derive(Debug, Clone)]
pub enum FilterList {
    All,
    Allow(Vec<Lang>),
    Deny(Vec<Lang>),
}

impl FilterList {
    #[cfg(feature = "dev")]
    pub fn all() -> Self {
        Self::All
    }

    pub fn allow(allowlist: Vec<Lang>) -> Self {
        Self::Allow(allowlist)
    }

    pub fn deny(blacklist: Vec<Lang>) -> Self {
        Self::Deny(blacklist)
    }

    pub fn is_allowed(&self, lang: Lang) -> bool {
        match self {
            Self::All => true,
            Self::Allow(ref allowlist) => allowlist.contains(&lang),
            Self::Deny(ref blacklist) => !blacklist.contains(&lang),
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
    #[cfg(feature = "dev")]
    fn test_all() {
        let list = FilterList::all();
        assert!(list.is_allowed(Lang::Epo));
    }

    #[test]
    fn test_only() {
        let list = FilterList::allow(vec![Lang::Rus, Lang::Ukr]);

        assert!(!list.is_allowed(Lang::Epo));
        assert!(!list.is_allowed(Lang::Eng));

        assert!(list.is_allowed(Lang::Rus));
        assert!(list.is_allowed(Lang::Ukr));
    }

    #[test]
    fn test_except() {
        let list = FilterList::deny(vec![Lang::Rus, Lang::Ukr]);

        assert!(list.is_allowed(Lang::Epo));
        assert!(list.is_allowed(Lang::Eng));

        assert!(!list.is_allowed(Lang::Rus));
        assert!(!list.is_allowed(Lang::Ukr));
    }
}
