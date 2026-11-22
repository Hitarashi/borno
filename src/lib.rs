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

pub mod builder;
pub mod case;
pub mod ext;
pub mod grammar;
pub mod slug;
pub mod utils;

pub use builder::StrBuilder;
pub use case::{
    classify, deconstantize, demodulize, foreign_key, foreign_key_with_suffix, headline, humanize,
    is_camel_case, is_dot_case, is_foreign_key, is_foreign_key_with_suffix, is_kebab_case,
    is_pascal_case, is_screaming_snake_case, is_sentence_case, is_snake_case, is_title_case,
    is_train_case, tableize, to_camel_case, to_dot_case, to_kebab_case, to_pascal_case,
    to_screaming_snake_case, to_sentence_case, to_snake_case, to_title_case, to_train_case,
};
pub use ext::StrExt;
pub use grammar::{
    Inflector, add_irregular, add_uncountable, is_plural, is_singular, pluralize,
    pluralize_with_count, pluralize_with_count_inclusive, singularize,
};
#[cfg(feature = "regex")]
pub use grammar::{add_plural_rule, add_singular_rule};
pub use slug::{SlugOptions, slugify, slugify_with_options};
pub use utils::{
    Alignment, ExcerptOptions, SentenceOptions, TruncateOptions, WordWrapOptions, after,
    after_last, before, before_last, between, between_first, excerpt, finish, html_escape,
    html_unescape, interpolate, interpolate_pairs, justify_columns, mask, mask_email,
    normalize_slashes, ordinal, pad_both, pad_end, pad_start, sentence, sentence_with_options,
    squish, start, truncate, unwrap, word_wrap, word_wrap_with_options, words, wrap,
};

/// Prelude module for convenient glob-importing common types and traits.
pub mod prelude {
    pub use super::{
        builder::StrBuilder,
        ext::StrExt,
        grammar::Inflector,
        slug::SlugOptions,
        utils::{Alignment, ExcerptOptions, SentenceOptions, TruncateOptions, WordWrapOptions},
    };
}
