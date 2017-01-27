#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Query<'txt> {
    pub text: &'txt String
}

impl<'txt> Query<'txt> {
    pub fn new(text: &'txt String) -> Query<'txt> {
        Query { text: text }
    }
}
