extern crate whatlang;
#[macro_use] extern crate proptest;

use whatlang::detect;

proptest! {
    #[test]
    fn proptest_detect_does_not_crash(text in "\\PC*") {
        detect(&text);
    }
}
