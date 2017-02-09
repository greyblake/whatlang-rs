use lang::Lang;
use script::Script;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Info {
    pub lang: Lang,
    pub script: Script
}
