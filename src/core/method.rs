use crate::error::ParseError;
use std::fmt;
use std::str::FromStr;

#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Method {
    Trigram,
    Alphabet,
    #[default]
    Combined,
}

impl FromStr for Method {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().trim() {
            "trigram" => Ok(Method::Trigram),
            "alphabet" => Ok(Method::Alphabet),
            "combined" => Ok(Method::Combined),
            _ => Err(ParseError::Method(s.to_string())),
        }
    }
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            Method::Trigram => "Trigram",
            Method::Alphabet => "Alphabet",
            Method::Combined => "Combined",
        };
        write!(f, "{}", name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!("trigram".parse::<Method>().unwrap(), Method::Trigram);
        assert_eq!("ALPHABET".parse::<Method>().unwrap(), Method::Alphabet);

        let result = "foobar".parse::<Method>();
        assert!(result.is_err());
    }
}
