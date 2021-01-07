use std::error::Error as StdError;
use std::fmt::{self, Display};

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

impl StdError for Error {}
