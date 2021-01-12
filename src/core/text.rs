pub struct Text<'a> {
    original: &'a str,
    lowercased: Option<String>
}

impl<'a> Text<'a> {
    pub fn new(original_text: &'a str) -> Self {
        Self {
            original: original_text,
            lowercased: None
        }
    }

    pub fn original(&self) -> &str {
        self.original
    }

    pub fn lowercased(&mut self) -> &str {
        if self.lowercased.is_none() {
            let lowercased_text = self.original.to_lowercase();
            self.lowercased = Some(lowercased_text);
        }
        self.lowercased.as_ref().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text() {
        let mut text = Text::new("Hello THERE");
        assert_eq!(text.original(), "Hello THERE");
        assert_eq!(text.lowercased(), "hello there");
        assert_eq!(text.lowercased(), "hello there");
    }
}
