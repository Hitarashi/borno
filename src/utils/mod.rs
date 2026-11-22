//! General string utilities.

#[cfg(feature = "util-bytes")]
pub mod bytes;
#[cfg(feature = "util-duration")]
pub mod duration;
#[cfg(feature = "util-format")]
pub mod format;
#[cfg(feature = "util-interpolate")]
pub mod interpolate;
#[cfg(feature = "util-mask")]
pub mod mask;
#[cfg(feature = "util-normalize")]
pub mod normalize;
#[cfg(feature = "util-pad")]
pub mod pad;
#[cfg(feature = "util-slice")]
pub mod slice;
#[cfg(feature = "util-text")]
pub mod text;
#[cfg(feature = "util-truncate")]
pub mod truncate;

#[cfg(feature = "util-bytes")]
pub use bytes::{
    BytesOptions, BytesStandard, BytesUnit, ParseBytesError, format_bytes,
    format_bytes_with_options, parse_bytes,
};
#[cfg(feature = "util-duration")]
pub use duration::{ParseDurationError, format_duration, format_duration_verbose, parse_duration};
#[cfg(feature = "util-format")]
pub use format::{
    SentenceOptions, deordinalize, ordinal, ordinal_suffix, sentence, sentence_with_options,
};
#[cfg(feature = "util-interpolate")]
pub use interpolate::{interpolate, interpolate_pairs};
#[cfg(feature = "util-mask")]
pub use mask::{mask, mask_email};
#[cfg(feature = "util-normalize")]
pub use normalize::{normalize_slashes, squish};
#[cfg(feature = "util-pad")]
pub use pad::{
    Affix, AffixOptions, ensure_prefix, ensure_suffix, ensure_wrap, ensure_wrapped, ext, finish,
    pad_both, pad_end, pad_start, prefix, prefix_with_options, remove_extension, remove_prefix,
    remove_prefix_with_options, remove_suffix, remove_suffix_with_options, start, strip_prefix,
    strip_suffix, suffix, suffix_with_options, surround, unwrap, unwrap_quotes, wrap,
};
#[cfg(feature = "util-slice")]
pub use slice::{
    after, after_last, before, before_last, between, between_first, chop_end, chop_start,
    contains_all, contains_any, deconstantize, deduplicate, demodulize, ends_with_any,
    is_empty_or_whitespace, remove, replace_first, replace_last, starts_with_any,
};
#[cfg(feature = "util-text")]
pub use text::{
    Alignment, WordWrapOptions, html_escape, html_unescape, justify_columns, word_wrap,
    word_wrap_with_options,
};
#[cfg(feature = "util-truncate")]
pub use truncate::{ExcerptOptions, TruncateOptions, excerpt, truncate, words};
