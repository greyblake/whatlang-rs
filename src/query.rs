use lang::Lang;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Query<'txt> {
    pub text: &'txt String,
    pub blacklist: Option<Vec<Lang>>
}

impl<'txt> Query<'txt> {
    pub fn new(text: &'txt String) -> Query<'txt> {
        Query { text: text, blacklist: None }
    }

    pub fn blacklist(mut self, blacklist: Vec<Lang>) -> Self {
        self.blacklist = Some(blacklist);
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
        let text = "Example".to_string();
        let query = Query::new(&text).blacklist(vec![Lang::Eng, Lang::Fra]);
        assert_eq!(query.text, &text);
        assert_eq!(query.blacklist.unwrap(), vec![Lang::Eng, Lang::Fra]);
    }
}
