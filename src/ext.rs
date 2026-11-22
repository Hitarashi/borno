//! Extension trait providing fluent methods on string slices.

use std::borrow::Cow;

#[cfg(feature = "case-capital")]
use crate::case::capital::{is_capital_case, to_capital_case};
#[cfg(all(feature = "conventions", feature = "grammar"))]
use crate::case::is_table_case;
#[cfg(feature = "case-no")]
use crate::case::no::{is_no_case, to_no_case};
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
    feature = "case-no",
    feature = "case-capital"
))]
use crate::case::{capitalize_first, swap_case, uncapitalize_first};
#[cfg(feature = "conventions")]
use crate::case::{
    classify, deconstantize, demodulize, foreign_key, headline, humanize, is_foreign_key, tableize,
    to_table_case,
};
#[cfg(feature = "case-camel")]
use crate::case::{is_camel_case, to_camel_case};
#[cfg(feature = "case-pascal")]
use crate::case::{
    is_class_case, is_pascal_case, is_studly_case, to_class_case, to_pascal_case, to_studly_case,
};
#[cfg(feature = "case-dot")]
use crate::case::{is_dot_case, to_dot_case};
#[cfg(feature = "case-kebab")]
use crate::case::{is_kebab_case, to_kebab_case};
#[cfg(feature = "case-screaming-snake")]
use crate::case::{is_screaming_snake_case, to_screaming_snake_case};
#[cfg(feature = "case-sentence")]
use crate::case::{is_sentence_case, to_sentence_case};
#[cfg(feature = "case-snake")]
use crate::case::{is_snake_case, to_snake_case};
#[cfg(feature = "case-title")]
use crate::case::{is_title_case, is_title_case_apa, to_title_case, to_title_case_apa};
#[cfg(feature = "case-train")]
use crate::case::{is_train_case, to_train_case};
#[cfg(feature = "grammar")]
use crate::grammar::{
    is_plural, is_singular, pluralize, pluralize_with_count, pluralize_with_count_inclusive,
    singularize,
};
#[cfg(feature = "slug")]
use crate::slug::{SlugOptions, slugify, slugify_with_options};
#[cfg(feature = "util-format")]
use crate::utils::deordinalize;
#[cfg(feature = "util-interpolate")]
use crate::utils::interpolate;
#[cfg(feature = "util-mask")]
use crate::utils::mask;
#[cfg(feature = "util-pad")]
use crate::utils::{
    AffixOptions, ensure_wrap, ext, finish, prefix, prefix_with_options, remove_extension,
    remove_prefix, remove_prefix_with_options, remove_suffix, remove_suffix_with_options, start,
    suffix, suffix_with_options, surround, unwrap, unwrap_quotes, wrap,
};
#[cfg(feature = "util-truncate")]
use crate::utils::{ExcerptOptions, TruncateOptions, excerpt, truncate, words};
#[cfg(feature = "util-bytes")]
use crate::utils::{ParseBytesError, parse_bytes};
#[cfg(feature = "util-duration")]
use crate::utils::{ParseDurationError, parse_duration};
#[cfg(feature = "util-text")]
use crate::utils::{
    WordWrapOptions, html_escape, html_unescape, word_wrap, word_wrap_with_options,
};
#[cfg(feature = "util-slice")]
use crate::utils::{
    after, after_last, before, before_last, between, between_first, chop_end, chop_start,
    contains_all, contains_any, deduplicate, ends_with_any, is_empty_or_whitespace, remove,
    replace_first, replace_last, starts_with_any,
};
#[cfg(feature = "util-normalize")]
use crate::utils::{normalize_slashes, squish};

/// Extension trait enabling fluent inflections, casing transformations, and utilities on `&str`.
pub trait StrExt {
    /// Converts to `snake_case`.
    #[cfg(feature = "case-snake")]
    fn to_snake_case(&self) -> Cow<'_, str>;

    /// Converts to `SCREAMING_SNAKE_CASE`.
    #[cfg(feature = "case-screaming-snake")]
    fn to_screaming_snake_case(&self) -> Cow<'_, str>;

    /// Converts to `camelCase`.
    #[cfg(feature = "case-camel")]
    fn to_camel_case(&self) -> Cow<'_, str>;

    /// Converts to `PascalCase`.
    #[cfg(feature = "case-pascal")]
    fn to_pascal_case(&self) -> Cow<'_, str>;

    /// Converts to `kebab-case`.
    #[cfg(feature = "case-kebab")]
    fn to_kebab_case(&self) -> Cow<'_, str>;

    /// Converts to `Train-Case`.
    #[cfg(feature = "case-train")]
    fn to_train_case(&self) -> Cow<'_, str>;

    /// Converts to `Title Case`.
    #[cfg(feature = "case-title")]
    fn to_title_case(&self) -> Cow<'_, str>;

    /// Converts to `Title Case` using APA / Chicago grammar rules.
    #[cfg(feature = "case-title")]
    fn to_title_case_apa(&self) -> Cow<'_, str>;

    /// Converts to `Sentence case`.
    #[cfg(feature = "case-sentence")]
    fn to_sentence_case(&self) -> Cow<'_, str>;

    /// Converts to `dot.case`.
    #[cfg(feature = "case-dot")]
    fn to_dot_case(&self) -> Cow<'_, str>;

    /// Converts to `no case`.
    #[cfg(feature = "case-no")]
    fn to_no_case(&self) -> Cow<'_, str>;

    /// Checks if the slice is in valid `no case`.
    #[cfg(feature = "case-no")]
    fn is_no_case(&self) -> bool;

    /// Converts to `Capital Case`.
    #[cfg(feature = "case-capital")]
    fn to_capital_case(&self) -> Cow<'_, str>;

    /// Checks if the slice is in valid `Capital Case`.
    #[cfg(feature = "case-capital")]
    fn is_capital_case(&self) -> bool;

    /// Capitalizes only the first Unicode scalar character.
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
        feature = "case-no",
        feature = "case-capital"
    ))]
    fn capitalize_first(&self) -> Cow<'_, str>;

    /// Lowercases only the first Unicode scalar character.
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
        feature = "case-no",
        feature = "case-capital"
    ))]
    fn uncapitalize_first(&self) -> Cow<'_, str>;

    /// Alias for [`Self::to_pascal_case`].
    #[cfg(feature = "case-pascal")]
    fn to_studly_case(&self) -> Cow<'_, str>;

    /// Checks if the slice is in PascalCase / StudlyCase.
    #[cfg(feature = "case-pascal")]
    fn is_studly_case(&self) -> bool;

    /// Alias for [`Self::to_pascal_case`].
    #[cfg(feature = "case-pascal")]
    fn to_class_case(&self) -> Cow<'_, str>;

    /// Checks if the slice is in ClassCase.
    #[cfg(feature = "case-pascal")]
    fn is_class_case(&self) -> bool;

    /// Converts a model name into a conventional database table name.
    #[cfg(feature = "conventions")]
    fn tableize(&self) -> String;

    /// Alias for [`Self::tableize`].
    #[cfg(feature = "conventions")]
    fn to_table_case(&self) -> String;

    /// Checks if the slice is a valid table name.
    #[cfg(all(feature = "conventions", feature = "grammar"))]
    fn is_table_case(&self) -> bool;

    /// Converts a database table name into a conventional model class name.
    #[cfg(feature = "conventions")]
    fn classify(&self) -> String;

    /// Generates a foreign key column name.
    #[cfg(feature = "conventions")]
    fn foreign_key(&self) -> String;

    /// Checks if an identifier represents a foreign key ending in `_id`.
    #[cfg(feature = "conventions")]
    fn is_foreign_key(&self) -> bool;

    /// Converts an identifier into capitalized headline words.
    #[cfg(feature = "conventions")]
    fn headline(&self) -> String;

    /// Converts an identifier into sentence-style human-readable text.
    #[cfg(feature = "conventions")]
    fn humanize(&self) -> String;

    /// Extracts the type name, removing namespace modules.
    #[cfg(feature = "conventions")]
    fn demodulize(&self) -> &str;

    /// Extracts the parent module namespace.
    #[cfg(feature = "conventions")]
    fn deconstantize(&self) -> &str;

    /// Checks if the slice is valid `snake_case`.
    #[cfg(feature = "case-snake")]
    fn is_snake_case(&self) -> bool;

    /// Checks if the slice is valid `SCREAMING_SNAKE_CASE`.
    #[cfg(feature = "case-screaming-snake")]
    fn is_screaming_snake_case(&self) -> bool;

    /// Checks if the slice is valid `camelCase`.
    #[cfg(feature = "case-camel")]
    fn is_camel_case(&self) -> bool;

    /// Checks if the slice is valid `PascalCase`.
    #[cfg(feature = "case-pascal")]
    fn is_pascal_case(&self) -> bool;

    /// Checks if the slice is valid `kebab-case`.
    #[cfg(feature = "case-kebab")]
    fn is_kebab_case(&self) -> bool;

    /// Checks if the slice is valid `Train-Case`.
    #[cfg(feature = "case-train")]
    fn is_train_case(&self) -> bool;

    /// Checks if the slice is valid `Title Case`.
    #[cfg(feature = "case-title")]
    fn is_title_case(&self) -> bool;

    /// Checks if the slice is valid `Title Case` according to APA grammar rules.
    #[cfg(feature = "case-title")]
    fn is_title_case_apa(&self) -> bool;

    /// Inverts character cases throughout the string slice.
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
        feature = "case-no",
        feature = "case-capital"
    ))]
    fn swap_case(&self) -> String;

    /// Checks if the slice is valid `Sentence case`.
    #[cfg(feature = "case-sentence")]
    fn is_sentence_case(&self) -> bool;

    /// Checks if the slice is valid `dot.case`.
    #[cfg(feature = "case-dot")]
    fn is_dot_case(&self) -> bool;

    /// Pluralizes a word.
    #[cfg(feature = "grammar")]
    fn to_plural(&self) -> Cow<'_, str>;

    /// Singularizes a word.
    #[cfg(feature = "grammar")]
    fn to_singular(&self) -> Cow<'_, str>;

    /// Inflects a word based on a count.
    #[cfg(feature = "grammar")]
    fn pluralize_with_count(&self, count: usize) -> Cow<'_, str>;

    /// Inflects a word with a count prefix.
    #[cfg(feature = "grammar")]
    fn pluralize_with_count_inclusive(&self, count: usize) -> String;

    /// Checks if a word is plural.
    #[cfg(feature = "grammar")]
    fn is_plural(&self) -> bool;

    /// Checks if a word is singular.
    #[cfg(feature = "grammar")]
    fn is_singular(&self) -> bool;

    /// Converts to a URL-safe slug.
    #[cfg(feature = "slug")]
    fn to_slug(&self) -> String;

    /// Converts to a URL-safe slug with options.
    #[cfg(feature = "slug")]
    fn to_slug_with_options(&self, options: &SlugOptions) -> String;

    /// Truncates to a maximum character length with the default ellipsis.
    #[cfg(feature = "util-truncate")]
    fn truncate(&self, max_characters: usize) -> String;

    /// Truncates to a maximum character length with custom options.
    #[cfg(feature = "util-truncate")]
    fn truncate_with_options(&self, max_characters: usize, options: &TruncateOptions) -> String;

    /// Limits the word count.
    #[cfg(feature = "util-truncate")]
    fn words(&self, max_words: usize, suffix: &str) -> String;

    /// Extracts a surrounding context excerpt.
    #[cfg(feature = "util-truncate")]
    fn excerpt(&self, term: &str, options: &ExcerptOptions) -> String;

    /// Masks a substring with a character.
    #[cfg(feature = "util-mask")]
    fn mask(&self, mask_char: char, start_idx: usize, length: Option<usize>) -> String;

    /// Ensures the string starts with the given prefix.
    #[cfg(feature = "util-pad")]
    fn start(&self, prefix: &str) -> String;

    /// Ensures the string ends with the given suffix.
    #[cfg(feature = "util-pad")]
    fn finish(&self, suffix: &str) -> String;

    /// Wraps the string with prefix and suffix strings.
    #[cfg(feature = "util-pad")]
    fn wrap(&self, before: &str, after: &str) -> String;

    /// Ensures the string begins with a prefix and terminates with a suffix without duplicating them.
    #[cfg(feature = "util-pad")]
    fn ensure_wrap(&self, before: &str, after: &str) -> String;

    /// Semantic alias for [`wrap`].
    #[cfg(feature = "util-pad")]
    fn surround(&self, before: &str, after: &str) -> String;

    /// Removes the enclosing prefix and suffix if present.
    #[cfg(feature = "util-pad")]
    fn unwrap(&self, before: &str, after: &str) -> &str;

    /// Strips matching enclosing quotes (either single or double) from the string.
    #[cfg(feature = "util-pad")]
    fn unwrap_quotes(&self) -> &str;

    /// Adds a prefix to the string slice, replacing any existing occurrence.
    #[cfg(feature = "util-pad")]
    fn prefix(&self, prefix_val: &str) -> String;

    /// Adds a prefix to the string slice with configurable options.
    #[cfg(feature = "util-pad")]
    fn prefix_with_options(&self, prefix: &str, options: &AffixOptions) -> String;

    /// Removes a prefix from the string slice with default options.
    #[cfg(feature = "util-pad")]
    fn remove_prefix(&self, prefix: &str) -> &str;

    /// Removes a prefix from the string slice with configurable options.
    #[cfg(feature = "util-pad")]
    fn remove_prefix_with_options<'a>(&'a self, prefix: &str, options: &AffixOptions) -> &'a str;

    /// Adds a suffix to the string slice, replacing any existing occurrence.
    #[cfg(feature = "util-pad")]
    fn suffix(&self, suffix_val: &str) -> String;

    /// Adds a suffix to the string slice with configurable options.
    #[cfg(feature = "util-pad")]
    fn suffix_with_options(&self, suffix: &str, options: &AffixOptions) -> String;

    /// Removes a suffix from the string slice with default options.
    #[cfg(feature = "util-pad")]
    fn remove_suffix(&self, suffix: &str) -> &str;

    /// Removes a suffix from the string slice with configurable options.
    #[cfg(feature = "util-pad")]
    fn remove_suffix_with_options<'a>(&'a self, suffix: &str, options: &AffixOptions) -> &'a str;

    /// Sets or replaces the file extension.
    #[cfg(feature = "util-pad")]
    fn ext(&self, extension: &str) -> String;

    /// Strips the file extension.
    #[cfg(feature = "util-pad")]
    fn remove_extension(&self) -> &str;

    /// Extracts the substring before the first delimiter occurrence.
    #[cfg(feature = "util-slice")]
    fn before(&self, delimiter: &str) -> Cow<'_, str>;

    /// Extracts the substring before the last delimiter occurrence.
    #[cfg(feature = "util-slice")]
    fn before_last(&self, delimiter: &str) -> Cow<'_, str>;

    /// Extracts the substring after the first delimiter occurrence.
    #[cfg(feature = "util-slice")]
    fn after(&self, delimiter: &str) -> Cow<'_, str>;

    /// Extracts the substring after the last delimiter occurrence.
    #[cfg(feature = "util-slice")]
    fn after_last(&self, delimiter: &str) -> Cow<'_, str>;

    /// Extracts the substring between delimiters.
    #[cfg(feature = "util-slice")]
    fn between(&self, start_delim: &str, end_delim: &str) -> Cow<'_, str>;

    /// Extracts the substring between the first occurrences of delimiters.
    #[cfg(feature = "util-slice")]
    fn between_first(&self, start_delim: &str, end_delim: &str) -> Cow<'_, str>;

    /// Removes a leading prefix from the string slice if present.
    #[cfg(feature = "util-slice")]
    fn chop_start(&self, needle: &str) -> &str;

    /// Removes a trailing suffix from the string slice if present.
    #[cfg(feature = "util-slice")]
    fn chop_end(&self, needle: &str) -> &str;

    /// Checks if the string slice contains all specified needles.
    #[cfg(feature = "util-slice")]
    fn contains_all(&self, needles: &[&str]) -> bool;

    /// Checks if the string slice contains any of the specified needles.
    #[cfg(feature = "util-slice")]
    fn contains_any(&self, needles: &[&str]) -> bool;

    /// Checks if the string slice is empty or contains only whitespace.
    #[cfg(feature = "util-slice")]
    fn is_empty_or_whitespace(&self) -> bool;

    /// Checks if the string slice starts with any of the specified needles.
    #[cfg(feature = "util-slice")]
    fn starts_with_any(&self, needles: &[&str]) -> bool;

    /// Checks if the string slice ends with any of the specified needles.
    #[cfg(feature = "util-slice")]
    fn ends_with_any(&self, needles: &[&str]) -> bool;

    /// Replaces the first occurrence of a substring.
    #[cfg(feature = "util-slice")]
    fn replace_first(&self, from: &str, to: &str) -> String;

    /// Replaces the last occurrence of a substring.
    #[cfg(feature = "util-slice")]
    fn replace_last(&self, from: &str, to: &str) -> String;

    /// Removes all occurrences of the specified needle.
    #[cfg(feature = "util-slice")]
    fn remove(&self, needle: &str) -> String;

    /// Deduplicates consecutive occurrences of a character.
    #[cfg(feature = "util-slice")]
    fn deduplicate(&self, character: char) -> Cow<'_, str>;

    /// Strips an English ordinal suffix from a number string.
    #[cfg(feature = "util-format")]
    fn deordinalize(&self) -> &str;

    /// Parses the string slice as a human-readable byte expression.
    #[cfg(feature = "util-bytes")]
    fn parse_bytes(&self) -> Result<u64, ParseBytesError>;

    /// Parses the string slice as a human-readable duration expression.
    #[cfg(feature = "util-duration")]
    fn parse_duration(&self) -> Result<std::time::Duration, ParseDurationError>;

    /// Collapses consecutive whitespace and trims the ends.
    #[cfg(feature = "util-normalize")]
    fn squish(&self) -> Cow<'_, str>;

    /// Normalizes Windows backslashes into forward slashes.
    #[cfg(feature = "util-normalize")]
    fn normalize_slashes(&self) -> Cow<'_, str>;

    /// Escapes HTML special characters.
    #[cfg(feature = "util-text")]
    fn html_escape(&self) -> Cow<'_, str>;

    /// Unescapes HTML character entities.
    #[cfg(feature = "util-text")]
    fn html_unescape(&self) -> Cow<'_, str>;

    /// Wraps text to a maximum width, breaking on words.
    #[cfg(feature = "util-text")]
    fn word_wrap(&self, max_width: usize) -> String;

    /// Wraps text with custom options.
    #[cfg(feature = "util-text")]
    fn word_wrap_with_options(&self, options: &WordWrapOptions) -> String;

    /// Interpolates named parameters from a slice.
    #[cfg(feature = "util-interpolate")]
    fn interpolate(&self, params: &[(&str, &str)]) -> String;
}

impl StrExt for str {
    #[cfg(feature = "case-snake")]
    fn to_snake_case(&self) -> Cow<'_, str> {
        to_snake_case(self)
    }

    #[cfg(feature = "case-screaming-snake")]
    fn to_screaming_snake_case(&self) -> Cow<'_, str> {
        to_screaming_snake_case(self)
    }

    #[cfg(feature = "case-camel")]
    fn to_camel_case(&self) -> Cow<'_, str> {
        to_camel_case(self)
    }

    #[cfg(feature = "case-pascal")]
    fn to_pascal_case(&self) -> Cow<'_, str> {
        to_pascal_case(self)
    }

    #[cfg(feature = "case-kebab")]
    fn to_kebab_case(&self) -> Cow<'_, str> {
        to_kebab_case(self)
    }

    #[cfg(feature = "case-train")]
    fn to_train_case(&self) -> Cow<'_, str> {
        to_train_case(self)
    }

    #[cfg(feature = "case-title")]
    fn to_title_case(&self) -> Cow<'_, str> {
        to_title_case(self)
    }

    #[cfg(feature = "case-title")]
    fn to_title_case_apa(&self) -> Cow<'_, str> {
        to_title_case_apa(self)
    }

    #[cfg(feature = "case-sentence")]
    fn to_sentence_case(&self) -> Cow<'_, str> {
        to_sentence_case(self)
    }

    #[cfg(feature = "case-dot")]
    fn to_dot_case(&self) -> Cow<'_, str> {
        to_dot_case(self)
    }

    #[cfg(feature = "case-no")]
    fn to_no_case(&self) -> Cow<'_, str> {
        to_no_case(self)
    }

    #[cfg(feature = "case-no")]
    fn is_no_case(&self) -> bool {
        is_no_case(self)
    }

    #[cfg(feature = "case-capital")]
    fn to_capital_case(&self) -> Cow<'_, str> {
        to_capital_case(self)
    }

    #[cfg(feature = "case-capital")]
    fn is_capital_case(&self) -> bool {
        is_capital_case(self)
    }

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
        feature = "case-no",
        feature = "case-capital"
    ))]
    fn capitalize_first(&self) -> Cow<'_, str> {
        capitalize_first(self)
    }

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
        feature = "case-no",
        feature = "case-capital"
    ))]
    fn uncapitalize_first(&self) -> Cow<'_, str> {
        uncapitalize_first(self)
    }

    #[cfg(feature = "case-pascal")]
    fn to_studly_case(&self) -> Cow<'_, str> {
        to_studly_case(self)
    }

    #[cfg(feature = "case-pascal")]
    fn is_studly_case(&self) -> bool {
        is_studly_case(self)
    }

    #[cfg(feature = "case-pascal")]
    fn to_class_case(&self) -> Cow<'_, str> {
        to_class_case(self)
    }

    #[cfg(feature = "case-pascal")]
    fn is_class_case(&self) -> bool {
        is_class_case(self)
    }

    #[cfg(feature = "conventions")]
    fn tableize(&self) -> String {
        tableize(self)
    }

    #[cfg(feature = "conventions")]
    fn to_table_case(&self) -> String {
        to_table_case(self)
    }

    #[cfg(all(feature = "conventions", feature = "grammar"))]
    fn is_table_case(&self) -> bool {
        is_table_case(self)
    }

    #[cfg(feature = "conventions")]
    fn classify(&self) -> String {
        classify(self)
    }

    #[cfg(feature = "conventions")]
    fn foreign_key(&self) -> String {
        foreign_key(self)
    }

    #[cfg(feature = "conventions")]
    fn is_foreign_key(&self) -> bool {
        is_foreign_key(self)
    }

    #[cfg(feature = "conventions")]
    fn headline(&self) -> String {
        headline(self)
    }

    #[cfg(feature = "conventions")]
    fn humanize(&self) -> String {
        humanize(self)
    }

    #[cfg(feature = "conventions")]
    fn demodulize(&self) -> &str {
        demodulize(self)
    }

    #[cfg(feature = "conventions")]
    fn deconstantize(&self) -> &str {
        deconstantize(self)
    }

    #[cfg(feature = "case-snake")]
    fn is_snake_case(&self) -> bool {
        is_snake_case(self)
    }

    #[cfg(feature = "case-screaming-snake")]
    fn is_screaming_snake_case(&self) -> bool {
        is_screaming_snake_case(self)
    }

    #[cfg(feature = "case-camel")]
    fn is_camel_case(&self) -> bool {
        is_camel_case(self)
    }

    #[cfg(feature = "case-pascal")]
    fn is_pascal_case(&self) -> bool {
        is_pascal_case(self)
    }

    #[cfg(feature = "case-kebab")]
    fn is_kebab_case(&self) -> bool {
        is_kebab_case(self)
    }

    #[cfg(feature = "case-train")]
    fn is_train_case(&self) -> bool {
        is_train_case(self)
    }

    #[cfg(feature = "case-title")]
    fn is_title_case(&self) -> bool {
        is_title_case(self)
    }

    #[cfg(feature = "case-title")]
    fn is_title_case_apa(&self) -> bool {
        is_title_case_apa(self)
    }

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
        feature = "case-no",
        feature = "case-capital"
    ))]
    fn swap_case(&self) -> String {
        swap_case(self)
    }

    #[cfg(feature = "case-sentence")]
    fn is_sentence_case(&self) -> bool {
        is_sentence_case(self)
    }

    #[cfg(feature = "case-dot")]
    fn is_dot_case(&self) -> bool {
        is_dot_case(self)
    }

    #[cfg(feature = "grammar")]
    fn to_plural(&self) -> Cow<'_, str> {
        pluralize(self)
    }

    #[cfg(feature = "grammar")]
    fn to_singular(&self) -> Cow<'_, str> {
        singularize(self)
    }

    #[cfg(feature = "grammar")]
    fn pluralize_with_count(&self, count: usize) -> Cow<'_, str> {
        pluralize_with_count(self, count)
    }

    #[cfg(feature = "grammar")]
    fn pluralize_with_count_inclusive(&self, count: usize) -> String {
        pluralize_with_count_inclusive(self, count)
    }

    #[cfg(feature = "grammar")]
    fn is_plural(&self) -> bool {
        is_plural(self)
    }

    #[cfg(feature = "grammar")]
    fn is_singular(&self) -> bool {
        is_singular(self)
    }

    #[cfg(feature = "slug")]
    fn to_slug(&self) -> String {
        slugify(self)
    }

    #[cfg(feature = "slug")]
    fn to_slug_with_options(&self, options: &SlugOptions) -> String {
        slugify_with_options(self, options)
    }

    #[cfg(feature = "util-truncate")]
    fn truncate(&self, max_characters: usize) -> String {
        truncate(self, max_characters, &TruncateOptions::default())
    }

    #[cfg(feature = "util-truncate")]
    fn truncate_with_options(&self, max_characters: usize, options: &TruncateOptions) -> String {
        truncate(self, max_characters, options)
    }

    #[cfg(feature = "util-truncate")]
    fn words(&self, max_words: usize, suffix: &str) -> String {
        words(self, max_words, suffix)
    }

    #[cfg(feature = "util-truncate")]
    fn excerpt(&self, term: &str, options: &ExcerptOptions) -> String {
        excerpt(self, term, options)
    }

    #[cfg(feature = "util-mask")]
    fn mask(&self, mask_char: char, start_idx: usize, length: Option<usize>) -> String {
        mask(self, mask_char, start_idx, length)
    }

    #[cfg(feature = "util-pad")]
    fn start(&self, prefix: &str) -> String {
        start(self, prefix)
    }

    #[cfg(feature = "util-pad")]
    fn finish(&self, suffix: &str) -> String {
        finish(self, suffix)
    }

    #[cfg(feature = "util-pad")]
    fn wrap(&self, before_val: &str, after_val: &str) -> String {
        wrap(self, before_val, after_val)
    }

    #[cfg(feature = "util-pad")]
    fn ensure_wrap(&self, before_val: &str, after_val: &str) -> String {
        ensure_wrap(self, before_val, after_val)
    }

    #[cfg(feature = "util-pad")]
    fn surround(&self, before_val: &str, after_val: &str) -> String {
        surround(self, before_val, after_val)
    }

    #[cfg(feature = "util-pad")]
    fn unwrap(&self, before_val: &str, after_val: &str) -> &str {
        unwrap(self, before_val, after_val)
    }

    #[cfg(feature = "util-pad")]
    fn unwrap_quotes(&self) -> &str {
        unwrap_quotes(self)
    }

    #[cfg(feature = "util-pad")]
    fn prefix(&self, prefix_val: &str) -> String {
        prefix(self, prefix_val)
    }

    #[cfg(feature = "util-pad")]
    fn prefix_with_options(&self, prefix: &str, options: &AffixOptions) -> String {
        prefix_with_options(self, prefix, options)
    }

    #[cfg(feature = "util-pad")]
    fn remove_prefix(&self, prefix: &str) -> &str {
        remove_prefix(self, prefix)
    }

    #[cfg(feature = "util-pad")]
    fn remove_prefix_with_options<'a>(&'a self, prefix: &str, options: &AffixOptions) -> &'a str {
        remove_prefix_with_options(self, prefix, options)
    }

    #[cfg(feature = "util-pad")]
    fn suffix(&self, suffix_val: &str) -> String {
        suffix(self, suffix_val)
    }

    #[cfg(feature = "util-pad")]
    fn suffix_with_options(&self, suffix: &str, options: &AffixOptions) -> String {
        suffix_with_options(self, suffix, options)
    }

    #[cfg(feature = "util-pad")]
    fn remove_suffix(&self, suffix: &str) -> &str {
        remove_suffix(self, suffix)
    }

    #[cfg(feature = "util-pad")]
    fn remove_suffix_with_options<'a>(&'a self, suffix: &str, options: &AffixOptions) -> &'a str {
        remove_suffix_with_options(self, suffix, options)
    }

    #[cfg(feature = "util-pad")]
    fn ext(&self, extension: &str) -> String {
        ext(self, extension)
    }

    #[cfg(feature = "util-pad")]
    fn remove_extension(&self) -> &str {
        remove_extension(self)
    }

    #[cfg(feature = "util-slice")]
    fn before(&self, delimiter: &str) -> Cow<'_, str> {
        before(self, delimiter)
    }

    #[cfg(feature = "util-slice")]
    fn before_last(&self, delimiter: &str) -> Cow<'_, str> {
        before_last(self, delimiter)
    }

    #[cfg(feature = "util-slice")]
    fn after(&self, delimiter: &str) -> Cow<'_, str> {
        after(self, delimiter)
    }

    #[cfg(feature = "util-slice")]
    fn after_last(&self, delimiter: &str) -> Cow<'_, str> {
        after_last(self, delimiter)
    }

    #[cfg(feature = "util-slice")]
    fn between(&self, start_delim: &str, end_delim: &str) -> Cow<'_, str> {
        between(self, start_delim, end_delim)
    }

    #[cfg(feature = "util-slice")]
    fn between_first(&self, start_delim: &str, end_delim: &str) -> Cow<'_, str> {
        between_first(self, start_delim, end_delim)
    }

    #[cfg(feature = "util-slice")]
    fn chop_start(&self, needle: &str) -> &str {
        chop_start(self, needle)
    }

    #[cfg(feature = "util-slice")]
    fn chop_end(&self, needle: &str) -> &str {
        chop_end(self, needle)
    }

    #[cfg(feature = "util-slice")]
    fn contains_all(&self, needles: &[&str]) -> bool {
        contains_all(self, needles)
    }

    #[cfg(feature = "util-slice")]
    fn contains_any(&self, needles: &[&str]) -> bool {
        contains_any(self, needles)
    }

    #[cfg(feature = "util-slice")]
    fn is_empty_or_whitespace(&self) -> bool {
        is_empty_or_whitespace(self)
    }

    #[cfg(feature = "util-slice")]
    fn starts_with_any(&self, needles: &[&str]) -> bool {
        starts_with_any(self, needles)
    }

    #[cfg(feature = "util-slice")]
    fn ends_with_any(&self, needles: &[&str]) -> bool {
        ends_with_any(self, needles)
    }

    #[cfg(feature = "util-slice")]
    fn replace_first(&self, from: &str, to: &str) -> String {
        replace_first(self, from, to)
    }

    #[cfg(feature = "util-slice")]
    fn replace_last(&self, from: &str, to: &str) -> String {
        replace_last(self, from, to)
    }

    #[cfg(feature = "util-slice")]
    fn remove(&self, needle: &str) -> String {
        remove(self, needle)
    }

    #[cfg(feature = "util-slice")]
    fn deduplicate(&self, character: char) -> Cow<'_, str> {
        deduplicate(self, character)
    }

    #[cfg(feature = "util-format")]
    fn deordinalize(&self) -> &str {
        deordinalize(self)
    }

    #[cfg(feature = "util-bytes")]
    fn parse_bytes(&self) -> Result<u64, ParseBytesError> {
        parse_bytes(self)
    }

    #[cfg(feature = "util-duration")]
    fn parse_duration(&self) -> Result<std::time::Duration, ParseDurationError> {
        parse_duration(self)
    }

    #[cfg(feature = "util-normalize")]
    fn squish(&self) -> Cow<'_, str> {
        squish(self)
    }

    #[cfg(feature = "util-normalize")]
    fn normalize_slashes(&self) -> Cow<'_, str> {
        normalize_slashes(self)
    }

    #[cfg(feature = "util-text")]
    fn html_escape(&self) -> Cow<'_, str> {
        html_escape(self)
    }

    #[cfg(feature = "util-text")]
    fn html_unescape(&self) -> Cow<'_, str> {
        html_unescape(self)
    }

    #[cfg(feature = "util-text")]
    fn word_wrap(&self, max_width: usize) -> String {
        word_wrap(self, max_width)
    }

    #[cfg(feature = "util-text")]
    fn word_wrap_with_options(&self, options: &WordWrapOptions) -> String {
        word_wrap_with_options(self, options)
    }

    #[cfg(feature = "util-interpolate")]
    fn interpolate(&self, params: &[(&str, &str)]) -> String {
        interpolate(self, params)
    }
}

#[cfg(all(test, feature = "full"))]
mod tests {
    use super::*;

    #[test]
    fn test_str_ext_fluent_methods() {
        assert_eq!("user_profile".to_camel_case(), "userProfile");
        assert_eq!("user_profile".to_dot_case(), "user.profile");
        assert_eq!("user_profile".tableize(), "user_profiles");
        assert_eq!("user_profiles".classify(), "UserProfile");
        assert_eq!("user".to_plural(), "users");
        assert_eq!("users".to_singular(), "user");
        assert_eq!("Hello World".to_slug(), "hello-world");
        assert_eq!("123456".mask('*', 2, Some(2)), "12**56");
        assert_eq!("endpoint".finish("/"), "endpoint/");
        assert_eq!("/endpoint".start("/"), "/endpoint");
        assert!("user_id".is_foreign_key());
        assert_eq!(
            "EmailNotificationSent".headline(),
            "Email Notification Sent"
        );
        assert_eq!("employee_salary".humanize(), "Employee salary");
        assert_eq!("crate::models::User".demodulize(), "User");
        assert_eq!("crate::models::User".deconstantize(), "crate::models");
        assert_eq!("user_id".before("_"), "user");
        assert_eq!("user_id".after("_"), "id");
        assert_eq!("a/b/c".before_last("/"), "a/b");
        assert_eq!("a/b/c".after_last("/"), "c");
        assert_eq!("[id]".between("[", "]"), "id");
        assert_eq!("[id] [name]".between_first("[", "]"), "id");
        assert_eq!("  foo   bar  ".squish(), "foo bar");
        assert_eq!("foo\\bar".normalize_slashes(), "foo/bar");
        assert_eq!("<p>".html_escape(), "&lt;p&gt;");
        assert_eq!("&lt;p&gt;".html_unescape(), "<p>");
        assert_eq!("a b c d e".word_wrap(3), "a b\nc d\ne");
        let wrap_opts = WordWrapOptions::new(3).with_indent(" ");
        assert_eq!("a b c".word_wrap_with_options(&wrap_opts), "a b\n c");
        assert_eq!(
            "hello :name".interpolate(&[("name", "world")]),
            "hello world"
        );
        assert!("user.name".is_dot_case());
        assert_eq!(
            "the old man and the sea".to_title_case_apa(),
            "The Old Man and the Sea"
        );
        assert!("The Old Man and the Sea".is_title_case_apa());
        assert_eq!("Hello World!".swap_case(), "hELLO wORLD!");
        assert_eq!("https://example.com".chop_start("https://"), "example.com");
        assert_eq!("filename.tar.gz".chop_end(".gz"), "filename.tar");
        assert!("apple banana".contains_all(&["apple", "banana"]));
        assert!("apple banana".contains_any(&["banana", "cherry"]));
        assert!("   ".is_empty_or_whitespace());
        assert_eq!("22nd".deordinalize(), "22");
        assert_eq!("path".ensure_wrap("/", "/"), "/path/");
        assert_eq!("tag".surround("<", ">"), "<tag>");
        assert_eq!("\"hello\"".unwrap_quotes(), "hello");
        assert_eq!("10MB".parse_bytes().unwrap(), 10_485_760);
        assert_eq!(
            "15m".parse_duration().unwrap(),
            std::time::Duration::from_secs(900)
        );
        assert_eq!("foo_bar".to_no_case(), "foo bar");
        assert!("foo bar".is_no_case());
        assert_eq!("foo_bar".to_capital_case(), "Foo Bar");
        assert!("Foo Bar".is_capital_case());
        assert_eq!("hello".capitalize_first(), "Hello");
        assert_eq!("Hello".uncapitalize_first(), "hello");
        assert_eq!("user_id".to_studly_case(), "UserId");
        assert!("UserId".is_studly_case());
        assert_eq!("user_id".to_class_case(), "UserId");
        assert!("UserId".is_class_case());
        assert_eq!("user_role".to_table_case(), "user_roles");
        assert!("user_roles".is_table_case());
        assert_eq!("path".prefix("/"), "/path");
        assert_eq!(
            "path".prefix_with_options("/", &AffixOptions::new()),
            "/path"
        );
        assert_eq!("/path".remove_prefix("/"), "path");
        assert_eq!(
            "/path".remove_prefix_with_options("/", &AffixOptions::new()),
            "path"
        );
        assert_eq!("file".suffix(".rs"), "file.rs");
        assert_eq!(
            "file".suffix_with_options(".rs", &AffixOptions::new()),
            "file.rs"
        );
        assert_eq!("file.rs".remove_suffix(".rs"), "file");
        assert_eq!(
            "file.rs".remove_suffix_with_options(".rs", &AffixOptions::new()),
            "file"
        );
        assert_eq!("photo".ext("png"), "photo.png");
        assert_eq!("photo.png".remove_extension(), "photo");
        assert!("hello world".starts_with_any(&["hi", "hello"]));
        assert!("hello world".ends_with_any(&["world", "earth"]));
        assert_eq!("foo foo bar".replace_first("foo", "qux"), "qux foo bar");
        assert_eq!("foo foo bar".replace_last("foo", "qux"), "foo qux bar");
        assert_eq!("foo bar foo".remove("foo "), "bar foo");
        assert_eq!("a///b".deduplicate('/'), "a/b");
    }
}
