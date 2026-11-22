//! Fast, allocation-conscious English grammatical inflections, casing conventions, URL slugification, and string utilities.
//!
//! # Architecture & Capabilities
//! - **Zero-Allocation Casing**: Fast casing conversions (`to_snake_case`, `to_camel_case`,
//!   `to_kebab_case`, `to_pascal_case`, `to_dot_case`, etc.) returning [`std::borrow::Cow<'_, str>`] that avoid
//!   heap allocations when the input already conforms to the target casing.
//! - **Acronym Recognition**: Automatically groups uppercase acronym sequences (e.g., `HTTPResponse` $\to$ `http_response`).
//! - **English Grammatical Inflections**: Pluralization and singularization with regular suffix rules,
//!   irregular noun dictionaries, uncountable noun lookups, count-based matching, and letter-casing preservation.
//! - **Custom Rule Engine**: Extendable [`Inflector`] engine supporting custom uncountable words,
//!   domain-specific irregulars, and dynamic regex patterns.
//! - **Schema & Naming Conventions**: High-level helpers (`tableize`, `classify`, `foreign_key`, `is_foreign_key`,
//!   `headline`, `humanize`, `demodulize`, `deconstantize`) for schema modeling, code generation, and scaffolding.
//! - **URL Slugification**: Fast ASCII transliteration for Latin diacritics/accents, symbol expansions, and configurable delimiters.
//! - **String & Slicing Utilities**: Boundary extraction (`before`, `after`, `between`), sensitive data masking (`mask`, `mask_email`),
//!   truncation with whole-word preservation (`truncate`, `words`), context excerpting (`excerpt`), whitespace squishing (`squish`),
//!   slash normalization (`normalize_slashes`), HTML escaping (`html_escape`, `html_unescape`), word wrapping (`word_wrap`),
//!   column justification (`justify_columns`), and parameter interpolation (`interpolate`).
//! - **Ergonomic Tri-Surface API**: Direct top-level functions, fluent [`StrBuilder`] pipeline chaining, and the [`StrExt`] extension trait.

#[cfg(feature = "builder")]
pub mod builder;
#[cfg(any(
    feature = "case-snake",
    feature = "case-screaming-snake",
    feature = "case-camel",
    feature = "case-pascal",
    feature = "case-kebab",
    feature = "case-train",
    feature = "case-title",
    feature = "case-sentence",
    feature = "case-dot",
    feature = "conventions"
))]
pub mod case;

#[cfg(all(
    feature = "grammar",
    not(any(
        feature = "case-snake",
        feature = "case-screaming-snake",
        feature = "case-camel",
        feature = "case-pascal",
        feature = "case-kebab",
        feature = "case-train",
        feature = "case-title",
        feature = "case-sentence",
        feature = "case-dot",
        feature = "conventions"
    ))
))]
pub(crate) mod case;
#[cfg(feature = "ext")]
pub mod ext;
#[cfg(feature = "grammar")]
pub mod grammar;
#[cfg(feature = "slug")]
pub mod slug;
#[cfg(any(
    feature = "util-slice",
    feature = "util-normalize",
    feature = "util-interpolate",
    feature = "util-text",
    feature = "util-truncate",
    feature = "util-mask",
    feature = "util-pad",
    feature = "util-format",
    feature = "util-bytes",
    feature = "util-duration"
))]
pub mod utils;

#[cfg(feature = "builder")]
pub use builder::StrBuilder;
#[cfg(any(
    feature = "case-snake",
    feature = "case-screaming-snake",
    feature = "case-camel",
    feature = "case-pascal",
    feature = "case-kebab",
    feature = "case-train",
    feature = "case-title",
    feature = "case-sentence",
    feature = "case-dot"
))]
pub use case::swap_case;
#[cfg(feature = "conventions")]
pub use case::{
    classify, deconstantize, demodulize, foreign_key, foreign_key_with_suffix, headline, humanize,
    is_foreign_key, is_foreign_key_with_suffix, tableize,
};
#[cfg(feature = "case-camel")]
pub use case::{is_camel_case, to_camel_case};
#[cfg(feature = "case-dot")]
pub use case::{is_dot_case, to_dot_case};
#[cfg(feature = "case-kebab")]
pub use case::{is_kebab_case, to_kebab_case};
#[cfg(feature = "case-pascal")]
pub use case::{is_pascal_case, to_pascal_case};
#[cfg(feature = "case-screaming-snake")]
pub use case::{is_screaming_snake_case, to_screaming_snake_case};
#[cfg(feature = "case-sentence")]
pub use case::{is_sentence_case, to_sentence_case};
#[cfg(feature = "case-snake")]
pub use case::{is_snake_case, to_snake_case};
#[cfg(feature = "case-title")]
pub use case::{is_title_case, is_title_case_apa, to_title_case, to_title_case_apa};
#[cfg(feature = "case-train")]
pub use case::{is_train_case, to_train_case};
#[cfg(feature = "ext")]
pub use ext::StrExt;
#[cfg(feature = "grammar")]
pub use grammar::{
    Inflector, add_irregular, add_uncountable, is_plural, is_singular, pluralize,
    pluralize_with_count, pluralize_with_count_inclusive, singularize,
};
#[cfg(all(feature = "grammar", feature = "regex"))]
pub use grammar::{add_plural_rule, add_singular_rule};
#[cfg(feature = "slug")]
pub use slug::{SlugOptions, slugify, slugify_with_options};
#[cfg(feature = "util-text")]
pub use utils::{
    Alignment, WordWrapOptions, html_escape, html_unescape, justify_columns, word_wrap,
    word_wrap_with_options,
};
#[cfg(feature = "util-bytes")]
pub use utils::{
    BytesOptions, BytesStandard, BytesUnit, ParseBytesError, format_bytes,
    format_bytes_with_options, parse_bytes,
};
#[cfg(feature = "util-truncate")]
pub use utils::{ExcerptOptions, TruncateOptions, excerpt, truncate, words};
#[cfg(feature = "util-duration")]
pub use utils::{ParseDurationError, format_duration, format_duration_verbose, parse_duration};
#[cfg(feature = "util-format")]
pub use utils::{
    SentenceOptions, deordinalize, ordinal, ordinal_suffix, sentence, sentence_with_options,
};
#[cfg(feature = "util-slice")]
pub use utils::{
    after, after_last, before, before_last, between, between_first, chop_end, chop_start,
    contains_all, contains_any, is_empty_or_whitespace,
};
#[cfg(feature = "util-pad")]
pub use utils::{
    ensure_wrap, finish, pad_both, pad_end, pad_start, start, surround, unwrap, unwrap_quotes, wrap,
};
#[cfg(feature = "util-interpolate")]
pub use utils::{interpolate, interpolate_pairs};
#[cfg(feature = "util-mask")]
pub use utils::{mask, mask_email};
#[cfg(feature = "util-normalize")]
pub use utils::{normalize_slashes, squish};

/// Prelude module for convenient glob-importing common types and traits.
pub mod prelude {
    #[cfg(feature = "builder")]
    pub use super::builder::StrBuilder;
    #[cfg(feature = "ext")]
    pub use super::ext::StrExt;
    #[cfg(feature = "grammar")]
    pub use super::grammar::Inflector;
    #[cfg(feature = "slug")]
    pub use super::slug::SlugOptions;
    #[cfg(feature = "util-format")]
    pub use super::utils::SentenceOptions;
    #[cfg(feature = "util-text")]
    pub use super::utils::{Alignment, WordWrapOptions};
    #[cfg(feature = "util-bytes")]
    pub use super::utils::{BytesOptions, BytesStandard, BytesUnit};
    #[cfg(feature = "util-truncate")]
    pub use super::utils::{ExcerptOptions, TruncateOptions};
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_crate_loads() {
        assert_eq!(2 + 2, 4);
    }
}
