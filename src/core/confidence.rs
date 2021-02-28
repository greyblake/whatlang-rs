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

    debug_assert!(highest_score <= 1.0);
    debug_assert!(highest_score >= 0.0);
    debug_assert!(second_score <= 1.0);
    debug_assert!(second_score >= 0.0);

    // Hyperbola function. Everything that is above the function has confidence = 1.0
    // If rate is below, confidence is calculated proportionally.
    // Constants are used based on experiments.
    let confident_rate = (3.0 / count as f64) + 0.015;
    let rate = (highest_score - second_score) / second_score;

    if rate > confident_rate {
        1.0
    } else {
        rate / confident_rate
    }
}
