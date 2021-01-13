pub mod detection;
mod profiles;
pub mod utils;
pub mod alt;

pub use profiles::*;

#[derive(Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Clone, Copy)]
pub struct Trigram(pub(crate) char, pub(crate) char, pub(crate) char);

// Maximum distance(difference) for a trigram in a language profile and text profile.
pub const MAX_TRIGRAM_DISTANCE: u32 = 300;

// 300 trigrams where each has MAX_TOTAL_DISTANCE=300, gives us 90_000.
pub const MAX_TOTAL_DISTANCE: u32 = MAX_TRIGRAM_DISTANCE * MAX_TRIGRAM_DISTANCE;

// Double MAX_TRIGRAM_DISTANCE
pub const TEXT_TRIGRAMS_SIZE: usize = 600;
