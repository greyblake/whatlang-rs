use lang::Lang;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Options<'a> {
    None,
    Blacklist(&'a [Lang]),
    Whitelist(&'a [Lang]),
}

impl<'a> Default for Options<'a> {
    fn default() -> Self {
        Options::None
    }
}
