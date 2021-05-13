use std::cell::{Ref, RefCell};
use std::ops::Deref;

#[derive(Debug)]
pub struct LowercaseText {
    inner: String,
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
    lowercase: RefCell<Option<LowercaseText>>,
}

impl<'a> Text<'a> {
    pub fn new(original_text: &'a str) -> Self {
        Self {
            original: original_text,
            lowercase: RefCell::new(None),
        }
    }

    pub fn lowercase(&self) -> Ref<'_, LowercaseText> {
        if self.lowercase.borrow().is_none() {
            let lowercase_text = LowercaseText::new(self.original);
            self.lowercase.replace(Some(lowercase_text));
        }

        let ref_opt_lowercase = self.lowercase.borrow();
        Ref::map(ref_opt_lowercase, |r| r.as_ref().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text() {
        let text = Text::new("Hello THERE");
        assert_eq!(text.lowercase().deref().deref(), "hello there");
    }
}
