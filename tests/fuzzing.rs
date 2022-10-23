#[cfg(feature = "arbitrary")]
#[test]
fn test_fuzzing() {
    use ::arbitrary::{Arbitrary, Unstructured};
    use whatlang::Detector;

    fn prop(u: &mut Unstructured) -> ::arbitrary::Result<()> {
        let detector = Detector::arbitrary(u)?;
        let input = String::arbitrary(u)?;
        detector.detect(&input);
        Ok(())
    }

    arbtest::builder().run(prop)
}
