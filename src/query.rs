use lang::Lang;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Query<'txt> {
    pub text: &'txt String,
    pub blacklist: Option<Vec<Lang>>,
    pub whitelist: Option<Vec<Lang>>
}

impl<'txt> Query<'txt> {
    pub fn new(text: &'txt String) -> Query<'txt> {
        Query {
            text: text,
            blacklist: None,
            whitelist: None
        }
    }

    pub fn blacklist(mut self, blacklist: Vec<Lang>) -> Self {
        self.blacklist = Some(blacklist);
        self
    }

    pub fn whitelist(mut self, whitelist: Vec<Lang>) -> Self {
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
    }

    #[test]
    fn test_new_with_blacklist() {
        let text = String::from("Example");
        let query = Query::new(&text).blacklist(vec![Lang::Eng, Lang::Fra]);
        assert_eq!(query.text, &text);
        assert_eq!(query.blacklist.unwrap(), vec![Lang::Eng, Lang::Fra]);
    }

    #[test]
    fn test_new_with_whitelist() {
        let text = String::from("Ekzemplo");
        let query = Query::new(&text).whitelist(vec![Lang::Epo, Lang::Ukr]);
        assert_eq!(query.text, &text);
        assert_eq!(query.whitelist.unwrap(), vec![Lang::Epo, Lang::Ukr]);
    }
}
