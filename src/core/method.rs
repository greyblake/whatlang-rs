use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    Trigram,
    Alphabet,
    Combined,
}

impl FromStr for Method {
    // TODO: implement normal error
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().trim() {
            "trigram" => Ok(Method::Trigram),
            "alphabet" => Ok(Method::Alphabet),
            "combined" => Ok(Method::Combined),
            _ => Err(format!("Cannot obtain Method from {:?}", s)),
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
