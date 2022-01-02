use std::error::Error as StdError;
use std::fmt::{self, Display};

#[derive(Debug)]
pub enum ParseError {
    Script(String),
    Lang(String),
    Method(String),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Script(ref val) => {
                write!(f, "Cannot parse str into whatlang::Script: {:?}", val)
            }
            ParseError::Lang(ref val) => {
                write!(f, "Cannot parse str into whatlang::Lang: {:?}", val)
            }
            ParseError::Method(ref val) => {
                write!(f, "Cannot parse str into whatlang::Method: {:?}", val)
            }
        }
    }
}

impl StdError for ParseError {}
