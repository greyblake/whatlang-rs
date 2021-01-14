use std::ops::Deref;

#[derive(Debug)]
pub struct LowercaseText {
    inner: String
}

impl LowercaseText {
    pub fn new(original_text: &str) -> Self {
        let inner = original_text.to_lowercase();
        Self { inner }
    }
}

impl Deref for LowercaseText {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[derive(Debug)]
pub struct Text<'a> {
    original: &'a str,
    lowercase: Option<LowercaseText>
}

impl<'a> Text<'a> {
    pub fn new(original_text: &'a str) -> Self {
        Self {
            original: original_text,
            lowercase: None
        }
    }

    pub fn original(&self) -> &str {
        self.original
    }

    pub fn lowercase(&mut self) -> &LowercaseText {
        if self.lowercase.is_none() {
            self.lowercase = Some(LowercaseText::new(self.original));
        }
        self.lowercase.as_ref().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text() {
        let mut text = Text::new("Hello THERE");
        assert_eq!(text.original(), "Hello THERE");
        assert_eq!(text.lowercase().deref(), "hello there");
    }
}
