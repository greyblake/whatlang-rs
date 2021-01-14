mod text;
mod method;
mod allow_list;
mod query;
mod options;
mod output;
mod detect;
mod lang_scores;

pub use method::Method;
pub use text::{Text, LowercaseText};
pub use options::Options;
pub use allow_list::AllowList;
pub use output::Output;
pub use query::{Query, InternalQuery};
pub use lang_scores::LangScores;

pub use detect::{detect_with_options, detect, detect_lang};
