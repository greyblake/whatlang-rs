mod allow_list;
mod detect;
mod info;
mod lang_scores;
mod method;
mod options;
mod query;
mod text;

pub use allow_list::AllowList;
pub use info::Info;
pub use lang_scores::LangScores;
pub use method::Method;
pub use options::Options;
pub use query::{InternalQuery, Query};
pub use text::{LowercaseText, Text};

pub use detect::{detect, detect_lang, detect_with_options};
