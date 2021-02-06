use alloc::string::String;
use core::fmt::{self, Display};

#[derive(Debug)]
pub enum Error {
    ParseScript(String),
    ParseLang(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ParseScript(ref val) => {
                write!(f, "Cannot parse str into whatlang::Script: {:?}", val)
            }
            Error::ParseLang(ref val) => {
                write!(f, "Cannot parse str into whatlang::Lang: {:?}", val)
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}
