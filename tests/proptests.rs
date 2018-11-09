extern crate whatlang;

#[macro_use] extern crate proptest;
use proptest::prelude::*;

use whatlang::{detect, Lang, Script};

proptest! {
    #[test]
    fn proptest_detect_does_not_crash(text in "\\PC*") {
        detect(&text);
    }
}
