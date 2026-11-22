//! General string utilities.

pub mod format;
pub mod interpolate;
pub mod mask;
pub mod normalize;
pub mod pad;
pub mod slice;
pub mod text;
pub mod truncate;

pub use format::{SentenceOptions, ordinal, sentence, sentence_with_options};
pub use interpolate::{interpolate, interpolate_pairs};
pub use mask::{mask, mask_email};
pub use normalize::{normalize_slashes, squish};
pub use pad::{finish, pad_both, pad_end, pad_start, start, unwrap, wrap};
pub use slice::{after, after_last, before, before_last, between, between_first};
pub use text::{
    Alignment, WordWrapOptions, html_escape, html_unescape, justify_columns, word_wrap,
    word_wrap_with_options,
};
pub use truncate::{ExcerptOptions, TruncateOptions, excerpt, truncate, words};
