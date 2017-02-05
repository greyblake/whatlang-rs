use lang::Lang;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Query<'a> {
    pub text: &'a str,
    pub blacklist: Option<&'a [Lang]>,
    pub whitelist: Option<&'a [Lang]>
}

impl<'a> Query<'a> {
    pub fn new(text: &'a str) -> Query<'a> {
        Query {
            text: text,
            blacklist: None,
            whitelist: None
        }
    }

    pub fn blacklist(mut self, blacklist: &'a [Lang]) -> Self {
        self.blacklist = Some(blacklist);
        self
    }

    pub fn whitelist(mut self, whitelist: &'a [Lang]) -> Self {
        self.whitelist = Some(whitelist);
        self
    }
}


#[cfg(test)]
mod tests {
    use super::Query;
    use super::Lang;

    #[test]
    fn test_new() {
        let text = "Example".to_string();
        let query = Query::new(&text);
        assert_eq!(query.text, &text);
        assert_eq!(query.blacklist, None);
        assert_eq!(query.whitelist, None);
    }

    #[test]
    fn test_new_with_blacklist() {
        let list = [Lang::Eng, Lang::Fra];
        let query = Query::new("words").blacklist(&list);
        assert_eq!(query.text, "words");
        assert_eq!(query.blacklist.unwrap(), &list);
        assert_eq!(query.whitelist, None);
    }

    #[test]
    fn test_new_with_whitelist() {
        let list = [Lang::Epo, Lang::Ukr];
        let query = Query::new("vortoj").whitelist(&list);
        assert_eq!(query.text, "vortoj");
        assert_eq!(query.whitelist.unwrap(), &list);
        assert_eq!(query.blacklist, None);
    }
}
