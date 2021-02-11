// Calculate confidence that the language with the highest score is the correct one.
// highest_score - within 0.0..1.0
// second_score - within 0.0..1.0
// count - number of chars or trigrams
pub fn calculate_confidence(highest_score: f64, second_score: f64, count: usize) -> f64 {
    if highest_score == 0.0 {
        return 0.0;
    }
    if second_score == 0.0 {
        return highest_score;
    }

    // if highest_score > 1.0 {
    //     println!("{}", highest_score);
    // }
    // assert!(highest_score <= 1.0);
    // assert!(second_score <= 1.0);

    // Hyperbola function. Everything that is above the function has confidence = 1.0
    // If rate is below, confidence is calculated proportionally.
    // Numbers 12.0 and 0.05 are obtained experimentally, so the function represents common sense.
    let rate = (highest_score - second_score) / second_score;

    // TODO: Play with the coefficients to get better results
    let confident_rate = (1.0 / count as f64) + 0.01;

    if rate > confident_rate {
        1.0
    } else {
        rate / confident_rate
    }
}
