use lang::Lang;

pub struct Options<'a> {
    pub blacklist: Option<&'a [Lang]>,
    pub whitelist: Option<&'a [Lang]>
}

pub const DEFAULT: Options<'static> = Options { blacklist: None, whitelist: None };
