use crate::Lang;

#[derive(Debug)]
pub enum AllowList {
    All,
    Only(Vec<Lang>),
    Except(Vec<Lang>),
}

impl AllowList {
    fn all() -> Self {
        Self::All
    }

    fn only(whitelist: Vec<Lang>) -> Self {
        Self::Only(whitelist)
    }

    fn except(blacklist: Vec<Lang>) -> Self {
        Self::Except(blacklist)
    }

    fn is_allowed(&self, lang: Lang) -> bool {
        match self {
            Self::All=> true,
            Self::Only(ref whitelist) => whitelist.contains(&lang),
            Self::Except(ref blacklist) => !blacklist.contains(&lang),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all() {
        let list = AllowList::all();
        assert!(list.is_allowed(Lang::Epo));
    }

    #[test]
    fn test_only() {
        let list = AllowList::only(vec![Lang::Rus, Lang::Ukr]);

        assert!(!list.is_allowed(Lang::Epo));
        assert!(!list.is_allowed(Lang::Eng));

        assert!(list.is_allowed(Lang::Rus));
        assert!(list.is_allowed(Lang::Ukr));
    }

    #[test]
    fn test_except() {
        let list = AllowList::except(vec![Lang::Rus, Lang::Ukr]);

        assert!(list.is_allowed(Lang::Epo));
        assert!(list.is_allowed(Lang::Eng));

        assert!(!list.is_allowed(Lang::Rus));
        assert!(!list.is_allowed(Lang::Ukr));
    }
}
