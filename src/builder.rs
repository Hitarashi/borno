//! Fluent, chainable `StrBuilder` for composing sequential string transformations.

use std::{fmt, ops::Deref};

#[cfg(feature = "case-camel")]
use crate::case::to_camel_case;
#[cfg(feature = "case-capital")]
use crate::case::to_capital_case;
#[cfg(feature = "case-dot")]
use crate::case::to_dot_case;
#[cfg(feature = "case-kebab")]
use crate::case::to_kebab_case;
#[cfg(feature = "case-no")]
use crate::case::to_no_case;
#[cfg(feature = "case-pascal")]
use crate::case::to_pascal_case;
#[cfg(feature = "case-screaming-snake")]
use crate::case::to_screaming_snake_case;
#[cfg(feature = "case-sentence")]
use crate::case::to_sentence_case;
#[cfg(feature = "case-snake")]
use crate::case::to_snake_case;
#[cfg(feature = "case-train")]
use crate::case::to_train_case;
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
use crate::case::{classify, deconstantize, demodulize, foreign_key, headline, humanize, tableize};
#[cfg(feature = "case-title")]
use crate::case::{to_title_case, to_title_case_apa};
#[cfg(feature = "grammar")]
use crate::grammar::{
    pluralize, pluralize_with_count, pluralize_with_count_inclusive, singularize,
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
    suffix, suffix_with_options, surround, unwrap_quotes, wrap,
};
#[cfg(feature = "util-truncate")]
use crate::utils::{TruncateOptions, truncate, words};
#[cfg(feature = "util-text")]
use crate::utils::{
    WordWrapOptions, html_escape, html_unescape, word_wrap, word_wrap_with_options,
};
#[cfg(feature = "util-slice")]
use crate::utils::{
    after, after_last, before, before_last, between, between_first, chop_end, chop_start,
    deduplicate, ends_with_any, remove, replace_first, replace_last, starts_with_any,
};
#[cfg(feature = "util-normalize")]
use crate::utils::{normalize_slashes, squish};

/// Fluent string transformation pipeline.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct StrBuilder {
    value: String,
}

impl StrBuilder {
    /// Creates a new `StrBuilder` from an initial string or slice.
    pub fn new(initial: impl Into<String>) -> Self {
        Self {
            value: initial.into(),
        }
    }

    /// Transforms the inner string into `snake_case`.
    #[cfg(feature = "case-snake")]
    pub fn snake_case(mut self) -> Self {
        self.value = to_snake_case(&self.value).into_owned();
        self
    }

    /// Transforms the inner string into `SCREAMING_SNAKE_CASE`.
    #[cfg(feature = "case-screaming-snake")]
    pub fn screaming_snake_case(mut self) -> Self {
        self.value = to_screaming_snake_case(&self.value).into_owned();
        self
    }

    /// Transforms the inner string into `camelCase`.
    #[cfg(feature = "case-camel")]
    pub fn camel_case(mut self) -> Self {
        self.value = to_camel_case(&self.value).into_owned();
        self
    }

    /// Transforms the inner string into `PascalCase`.
    #[cfg(feature = "case-pascal")]
    pub fn pascal_case(mut self) -> Self {
        self.value = to_pascal_case(&self.value).into_owned();
        self
    }

    /// Transforms the inner string into `kebab-case`.
    #[cfg(feature = "case-kebab")]
    pub fn kebab_case(mut self) -> Self {
        self.value = to_kebab_case(&self.value).into_owned();
        self
    }

    /// Transforms the inner string into `Train-Case`.
    #[cfg(feature = "case-train")]
    pub fn train_case(mut self) -> Self {
        self.value = to_train_case(&self.value).into_owned();
        self
    }

    /// Transforms the inner string into `Title Case`.
    #[cfg(feature = "case-title")]
    pub fn title_case(mut self) -> Self {
        self.value = to_title_case(&self.value).into_owned();
        self
    }

    /// Transforms the inner string into APA Title Case.
    #[cfg(feature = "case-title")]
    pub fn title_case_apa(mut self) -> Self {
        self.value = to_title_case_apa(&self.value).into_owned();
        self
    }

    /// Transforms the inner string into `no case`.
    #[cfg(feature = "case-no")]
    pub fn no_case(mut self) -> Self {
        self.value = to_no_case(&self.value).into_owned();
        self
    }

    /// Transforms the inner string into `Capital Case`.
    #[cfg(feature = "case-capital")]
    pub fn capital_case(mut self) -> Self {
        self.value = to_capital_case(&self.value).into_owned();
        self
    }

    /// Capitalizes only the first Unicode scalar character of the inner string.
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
    pub fn capitalize_first(mut self) -> Self {
        self.value = capitalize_first(&self.value).into_owned();
        self
    }

    /// Lowercases only the first Unicode scalar character of the inner string.
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
    pub fn uncapitalize_first(mut self) -> Self {
        self.value = uncapitalize_first(&self.value).into_owned();
        self
    }

    /// Alias for [`Self::pascal_case`].
    #[cfg(feature = "case-pascal")]
    pub fn studly_case(self) -> Self {
        self.pascal_case()
    }

    /// Alias for [`Self::pascal_case`].
    #[cfg(feature = "case-pascal")]
    pub fn class_case(self) -> Self {
        self.pascal_case()
    }

    /// Inverts character cases throughout the inner string.
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
    pub fn swap_case(mut self) -> Self {
        self.value = swap_case(&self.value);
        self
    }

    /// Transforms the inner string into `Sentence case`.
    #[cfg(feature = "case-sentence")]
    pub fn sentence_case(mut self) -> Self {
        self.value = to_sentence_case(&self.value).into_owned();
        self
    }

    /// Transforms the inner string into `dot.case`.
    #[cfg(feature = "case-dot")]
    pub fn dot_case(mut self) -> Self {
        self.value = to_dot_case(&self.value).into_owned();
        self
    }

    /// Converts a model name into a conventional database table name.
    #[cfg(feature = "conventions")]
    pub fn tableize(mut self) -> Self {
        self.value = tableize(&self.value);
        self
    }

    /// Alias for [`Self::tableize`].
    #[cfg(feature = "conventions")]
    pub fn table_case(self) -> Self {
        self.tableize()
    }

    /// Converts a database table name into a conventional model class name.
    #[cfg(feature = "conventions")]
    pub fn classify(mut self) -> Self {
        self.value = classify(&self.value);
        self
    }

    /// Generates a foreign key column name.
    #[cfg(feature = "conventions")]
    pub fn foreign_key(mut self) -> Self {
        self.value = foreign_key(&self.value);
        self
    }

    /// Converts an identifier into capitalized headline words.
    #[cfg(feature = "conventions")]
    pub fn headline(mut self) -> Self {
        self.value = headline(&self.value);
        self
    }

    /// Converts an identifier into sentence-style human-readable text.
    #[cfg(feature = "conventions")]
    pub fn humanize(mut self) -> Self {
        self.value = humanize(&self.value);
        self
    }

    /// Extracts the type name, removing namespace modules.
    #[cfg(feature = "conventions")]
    pub fn demodulize(mut self) -> Self {
        self.value = demodulize(&self.value).to_string();
        self
    }

    /// Extracts the parent module namespace.
    #[cfg(feature = "conventions")]
    pub fn deconstantize(mut self) -> Self {
        self.value = deconstantize(&self.value).to_string();
        self
    }

    /// Pluralizes the inner string.
    #[cfg(feature = "grammar")]
    pub fn pluralize(mut self) -> Self {
        self.value = pluralize(&self.value).into_owned();
        self
    }

    /// Alias for [`Self::pluralize`].
    #[cfg(feature = "grammar")]
    pub fn to_plural(self) -> Self {
        self.pluralize()
    }

    /// Singularizes the inner string.
    #[cfg(feature = "grammar")]
    pub fn singularize(mut self) -> Self {
        self.value = singularize(&self.value).into_owned();
        self
    }

    /// Alias for [`Self::singularize`].
    #[cfg(feature = "grammar")]
    pub fn to_singular(self) -> Self {
        self.singularize()
    }

    /// Inflects the inner string based on the given count.
    #[cfg(feature = "grammar")]
    pub fn pluralize_with_count(mut self, count: usize) -> Self {
        self.value = pluralize_with_count(&self.value, count).into_owned();
        self
    }

    /// Inflects the inner string based on the given count, prefixing the count.
    #[cfg(feature = "grammar")]
    pub fn pluralize_with_count_inclusive(mut self, count: usize) -> Self {
        self.value = pluralize_with_count_inclusive(&self.value, count);
        self
    }

    /// Converts the inner string to a URL-safe slug.
    #[cfg(feature = "slug")]
    pub fn slugify(mut self) -> Self {
        self.value = slugify(&self.value);
        self
    }

    /// Converts the inner string to a URL-safe slug with options.
    #[cfg(feature = "slug")]
    pub fn slugify_with_options(mut self, options: &SlugOptions) -> Self {
        self.value = slugify_with_options(&self.value, options);
        self
    }

    /// Truncates the inner string to a maximum number of characters with the default ellipsis.
    #[cfg(feature = "util-truncate")]
    pub fn truncate(mut self, max_characters: usize) -> Self {
        self.value = truncate(&self.value, max_characters, &TruncateOptions::default());
        self
    }

    /// Truncates the inner string with custom options.
    #[cfg(feature = "util-truncate")]
    pub fn truncate_with_options(
        mut self,
        max_characters: usize,
        options: &TruncateOptions,
    ) -> Self {
        self.value = truncate(&self.value, max_characters, options);
        self
    }

    /// Limits the inner string to a maximum number of words.
    #[cfg(feature = "util-truncate")]
    pub fn words(mut self, max_words: usize, suffix: &str) -> Self {
        self.value = words(&self.value, max_words, suffix);
        self
    }

    /// Masks a substring of the inner string with the given character.
    #[cfg(feature = "util-mask")]
    pub fn mask(mut self, mask_char: char, start_idx: usize, length: Option<usize>) -> Self {
        self.value = mask(&self.value, mask_char, start_idx, length);
        self
    }

    /// Ensures the inner string starts with the given prefix.
    #[cfg(feature = "util-pad")]
    pub fn start(mut self, prefix: &str) -> Self {
        self.value = start(&self.value, prefix);
        self
    }

    /// Ensures the inner string ends with the given suffix.
    #[cfg(feature = "util-pad")]
    pub fn finish(mut self, suffix: &str) -> Self {
        self.value = finish(&self.value, suffix);
        self
    }

    /// Wraps the inner string with prefix and suffix strings.
    #[cfg(feature = "util-pad")]
    pub fn wrap(mut self, before_val: &str, after_val: &str) -> Self {
        self.value = wrap(&self.value, before_val, after_val);
        self
    }

    /// Ensures the inner string begins with a prefix and terminates with a suffix without duplication.
    #[cfg(feature = "util-pad")]
    pub fn ensure_wrap(mut self, before_val: &str, after_val: &str) -> Self {
        self.value = ensure_wrap(&self.value, before_val, after_val);
        self
    }

    /// Semantic alias for [`wrap`].
    #[cfg(feature = "util-pad")]
    pub fn surround(mut self, before_val: &str, after_val: &str) -> Self {
        self.value = surround(&self.value, before_val, after_val);
        self
    }

    /// Strips matching enclosing quotes (single or double) from the inner string.
    #[cfg(feature = "util-pad")]
    pub fn unwrap_quotes(mut self) -> Self {
        self.value = unwrap_quotes(&self.value).to_string();
        self
    }

    /// Slices the inner string before the first occurrence of a delimiter.
    #[cfg(feature = "util-slice")]
    pub fn before(mut self, delimiter: &str) -> Self {
        self.value = before(&self.value, delimiter).into_owned();
        self
    }

    /// Slices the inner string before the last occurrence of a delimiter.
    #[cfg(feature = "util-slice")]
    pub fn before_last(mut self, delimiter: &str) -> Self {
        self.value = before_last(&self.value, delimiter).into_owned();
        self
    }

    /// Slices the inner string after the first occurrence of a delimiter.
    #[cfg(feature = "util-slice")]
    pub fn after(mut self, delimiter: &str) -> Self {
        self.value = after(&self.value, delimiter).into_owned();
        self
    }

    /// Slices the inner string after the last occurrence of a delimiter.
    #[cfg(feature = "util-slice")]
    pub fn after_last(mut self, delimiter: &str) -> Self {
        self.value = after_last(&self.value, delimiter).into_owned();
        self
    }

    /// Slices the inner string between two delimiters.
    #[cfg(feature = "util-slice")]
    pub fn between(mut self, start_delim: &str, end_delim: &str) -> Self {
        self.value = between(&self.value, start_delim, end_delim).into_owned();
        self
    }

    /// Slices the inner string between the first occurrence of two delimiters.
    #[cfg(feature = "util-slice")]
    pub fn between_first(mut self, start_delim: &str, end_delim: &str) -> Self {
        self.value = between_first(&self.value, start_delim, end_delim).into_owned();
        self
    }

    /// Removes a leading prefix if present.
    #[cfg(feature = "util-slice")]
    pub fn chop_start(mut self, needle: &str) -> Self {
        self.value = chop_start(&self.value, needle).to_string();
        self
    }

    /// Removes a trailing suffix if present.
    #[cfg(feature = "util-slice")]
    pub fn chop_end(mut self, needle: &str) -> Self {
        self.value = chop_end(&self.value, needle).to_string();
        self
    }

    /// Strips an English ordinal suffix from a number string.
    #[cfg(feature = "util-format")]
    pub fn deordinalize(mut self) -> Self {
        self.value = deordinalize(&self.value).to_string();
        self
    }

    /// Strips leading and trailing whitespace and collapses consecutive spaces.
    #[cfg(feature = "util-normalize")]
    pub fn squish(mut self) -> Self {
        self.value = squish(&self.value).into_owned();
        self
    }

    /// Normalizes Windows backslashes into forward slashes.
    #[cfg(feature = "util-normalize")]
    pub fn normalize_slashes(mut self) -> Self {
        self.value = normalize_slashes(&self.value).into_owned();
        self
    }

    /// Escapes HTML characters.
    #[cfg(feature = "util-text")]
    pub fn html_escape(mut self) -> Self {
        self.value = html_escape(&self.value).into_owned();
        self
    }

    /// Unescapes HTML character entities.
    #[cfg(feature = "util-text")]
    pub fn html_unescape(mut self) -> Self {
        self.value = html_unescape(&self.value).into_owned();
        self
    }

    /// Wraps text to line width breaking on words.
    #[cfg(feature = "util-text")]
    pub fn word_wrap(mut self, max_width: usize) -> Self {
        self.value = word_wrap(&self.value, max_width);
        self
    }

    /// Wraps text with custom options.
    #[cfg(feature = "util-text")]
    pub fn word_wrap_with_options(mut self, options: &WordWrapOptions) -> Self {
        self.value = word_wrap_with_options(&self.value, options);
        self
    }

    /// Interpolates named parameters into the inner string.
    #[cfg(feature = "util-interpolate")]
    pub fn interpolate(mut self, params: &[(&str, &str)]) -> Self {
        self.value = interpolate(&self.value, params);
        self
    }

    /// Conditionally executes an operation on the builder.
    pub fn when(self, condition: bool, op: impl FnOnce(Self) -> Self) -> Self {
        if condition { op(self) } else { self }
    }

    /// Executes an operation on the builder if the inner string is not empty.
    pub fn when_not_empty(self, op: impl FnOnce(Self) -> Self) -> Self {
        if !self.value.is_empty() {
            op(self)
        } else {
            self
        }
    }

    /// Executes an operation on the builder if the inner string is empty.
    pub fn when_empty(self, op: impl FnOnce(Self) -> Self) -> Self {
        if self.value.is_empty() {
            op(self)
        } else {
            self
        }
    }

    /// Inspects the builder state without mutating or breaking the chain.
    pub fn tap(self, op: impl FnOnce(&Self)) -> Self {
        op(&self);
        self
    }

    /// Pipes the builder into an arbitrary transformation function.
    pub fn pipe<R>(self, op: impl FnOnce(Self) -> R) -> R {
        op(self)
    }

    /// Returns `true` if the inner string is empty.
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    /// Returns `true` if the inner string is not empty.
    pub fn is_not_empty(&self) -> bool {
        !self.value.is_empty()
    }

    /// Checks if the inner string starts with any of the needles.
    #[cfg(feature = "util-slice")]
    pub fn starts_with_any(&self, needles: &[&str]) -> bool {
        starts_with_any(&self.value, needles)
    }

    /// Checks if the inner string ends with any of the needles.
    #[cfg(feature = "util-slice")]
    pub fn ends_with_any(&self, needles: &[&str]) -> bool {
        ends_with_any(&self.value, needles)
    }

    /// Replaces the first occurrence of a substring.
    #[cfg(feature = "util-slice")]
    pub fn replace_first(mut self, from: &str, to: &str) -> Self {
        self.value = replace_first(&self.value, from, to);
        self
    }

    /// Replaces the last occurrence of a substring.
    #[cfg(feature = "util-slice")]
    pub fn replace_last(mut self, from: &str, to: &str) -> Self {
        self.value = replace_last(&self.value, from, to);
        self
    }

    /// Removes all occurrences of the specified needle.
    #[cfg(feature = "util-slice")]
    pub fn remove(mut self, needle: &str) -> Self {
        self.value = remove(&self.value, needle);
        self
    }

    /// Deduplicates consecutive occurrences of a character.
    #[cfg(feature = "util-slice")]
    pub fn deduplicate(mut self, character: char) -> Self {
        self.value = deduplicate(&self.value, character).into_owned();
        self
    }

    /// Adds a prefix to the inner string, replacing any existing occurrence.
    #[cfg(feature = "util-pad")]
    pub fn prefix(mut self, prefix_val: &str) -> Self {
        self.value = prefix(&self.value, prefix_val);
        self
    }

    /// Adds a prefix to the inner string with configurable options.
    #[cfg(feature = "util-pad")]
    pub fn prefix_with_options(mut self, prefix_val: &str, options: &AffixOptions) -> Self {
        self.value = prefix_with_options(&self.value, prefix_val, options);
        self
    }

    /// Removes a prefix from the inner string with default options.
    #[cfg(feature = "util-pad")]
    pub fn remove_prefix(mut self, prefix_val: &str) -> Self {
        self.value = remove_prefix(&self.value, prefix_val).to_string();
        self
    }

    /// Removes a prefix from the inner string with configurable options.
    #[cfg(feature = "util-pad")]
    pub fn remove_prefix_with_options(mut self, prefix_val: &str, options: &AffixOptions) -> Self {
        self.value = remove_prefix_with_options(&self.value, prefix_val, options).to_string();
        self
    }

    /// Adds a suffix to the inner string, replacing any existing occurrence.
    #[cfg(feature = "util-pad")]
    pub fn suffix(mut self, suffix_val: &str) -> Self {
        self.value = suffix(&self.value, suffix_val);
        self
    }

    /// Adds a suffix to the inner string with configurable options.
    #[cfg(feature = "util-pad")]
    pub fn suffix_with_options(mut self, suffix_val: &str, options: &AffixOptions) -> Self {
        self.value = suffix_with_options(&self.value, suffix_val, options);
        self
    }

    /// Removes a suffix from the inner string with default options.
    #[cfg(feature = "util-pad")]
    pub fn remove_suffix(mut self, suffix_val: &str) -> Self {
        self.value = remove_suffix(&self.value, suffix_val).to_string();
        self
    }

    /// Removes a suffix from the inner string with configurable options.
    #[cfg(feature = "util-pad")]
    pub fn remove_suffix_with_options(mut self, suffix_val: &str, options: &AffixOptions) -> Self {
        self.value = remove_suffix_with_options(&self.value, suffix_val, options).to_string();
        self
    }

    /// Sets or replaces the file extension of the inner string.
    #[cfg(feature = "util-pad")]
    pub fn ext(mut self, extension: &str) -> Self {
        self.value = ext(&self.value, extension);
        self
    }

    /// Strips the file extension from the inner string.
    #[cfg(feature = "util-pad")]
    pub fn remove_extension(mut self) -> Self {
        self.value = remove_extension(&self.value).to_string();
        self
    }

    /// Appends a string slice to the inner string.
    pub fn append(mut self, text: &str) -> Self {
        self.value.push_str(text);
        self
    }

    /// Prepends a string slice to the inner string.
    pub fn prepend(mut self, text: &str) -> Self {
        self.value.insert_str(0, text);
        self
    }

    /// Trims leading and trailing whitespace from the inner string.
    pub fn trim(mut self) -> Self {
        self.value = self.value.trim().to_string();
        self
    }

    /// Trims leading whitespace from the inner string.
    pub fn trim_start(mut self) -> Self {
        self.value = self.value.trim_start().to_string();
        self
    }

    /// Trims trailing whitespace from the inner string.
    pub fn trim_end(mut self) -> Self {
        self.value = self.value.trim_end().to_string();
        self
    }

    /// Converts the inner string to lowercase.
    pub fn lowercase(mut self) -> Self {
        self.value = self.value.to_lowercase();
        self
    }

    /// Converts the inner string to uppercase.
    pub fn uppercase(mut self) -> Self {
        self.value = self.value.to_uppercase();
        self
    }

    /// Consumes the builder and returns the underlying `String`. Alias for `into_string`.
    pub fn build(self) -> String {
        self.into_string()
    }

    /// Consumes the builder and returns the underlying `String`.
    pub fn into_string(self) -> String {
        self.value
    }

    /// Returns a string slice of the current builder value.
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl Deref for StrBuilder {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl AsRef<str> for StrBuilder {
    fn as_ref(&self) -> &str {
        &self.value
    }
}

impl fmt::Display for StrBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<&str> for StrBuilder {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<String> for StrBuilder {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

#[cfg(all(test, feature = "full"))]
mod tests {
    use super::*;

    #[test]
    fn test_builder_chaining() {
        let result = StrBuilder::new("user_account_id")
            .pascal_case()
            .pluralize()
            .into_string();
        assert_eq!(result, "UserAccountIds");

        let singular = StrBuilder::new("boxes").singularize().into_string();
        assert_eq!(singular, "box");

        let count_res = StrBuilder::new("item")
            .pluralize_with_count_inclusive(3)
            .into_string();
        assert_eq!(count_res, "3 items");

        let count_only = StrBuilder::new("item")
            .pluralize_with_count(3)
            .into_string();
        assert_eq!(count_only, "items");

        let slug = StrBuilder::new("Hello World & Rust")
            .slugify()
            .into_string();
        assert_eq!(slug, "hello-world-and-rust");

        let slug_opts = StrBuilder::new("Custom Title")
            .slugify_with_options(&SlugOptions::new().with_delimiter('_'))
            .into_string();
        assert_eq!(slug_opts, "custom_title");

        let masked = StrBuilder::new("123456789")
            .mask('*', 2, Some(4))
            .into_string();
        assert_eq!(masked, "12****789");

        let wrapped_guard = StrBuilder::new("path")
            .start("/")
            .finish("/")
            .wrap("<", ">")
            .into_string();
        assert_eq!(wrapped_guard, "</path/>");

        let truncated = StrBuilder::new("Long text that needs trimming")
            .truncate(10)
            .into_string();
        assert_eq!(truncated, "Long te...");

        let truncated_opts = StrBuilder::new("Long text")
            .truncate_with_options(6, &TruncateOptions::new().with_suffix(".."))
            .into_string();
        assert_eq!(truncated_opts, "Long..");

        let words_res = StrBuilder::new("One two three four")
            .words(2, "..")
            .into_string();
        assert_eq!(words_res, "One two..");

        let chained = StrBuilder::new("user_account_id")
            .camel_case()
            .finish("Updated")
            .into_string();
        assert_eq!(chained, "userAccountIdUpdated");

        let dot = StrBuilder::new("user_account_id").dot_case().into_string();
        assert_eq!(dot, "user.account.id");

        let headline_res = StrBuilder::new("user_account_id").headline().into_string();
        assert_eq!(headline_res, "User Account Id");

        let humanize_res = StrBuilder::new("author_id").humanize().into_string();
        assert_eq!(humanize_res, "Author");

        let demod = StrBuilder::new("crate::services::User")
            .demodulize()
            .into_string();
        assert_eq!(demod, "User");

        let deconst = StrBuilder::new("crate::services::User")
            .deconstantize()
            .into_string();
        assert_eq!(deconst, "crate::services");

        let sliced = StrBuilder::new("a/b/c").before("/").into_string();
        assert_eq!(sliced, "a");

        let sliced_last = StrBuilder::new("a/b/c").before_last("/").into_string();
        assert_eq!(sliced_last, "a/b");

        let after_res = StrBuilder::new("a/b/c").after("/").into_string();
        assert_eq!(after_res, "b/c");

        let after_last_res = StrBuilder::new("a/b/c").after_last("/").into_string();
        assert_eq!(after_last_res, "c");

        let between_res = StrBuilder::new("[token]").between("[", "]").into_string();
        assert_eq!(between_res, "token");

        let between_first_res = StrBuilder::new("[token]")
            .between_first("[", "]")
            .into_string();
        assert_eq!(between_first_res, "token");

        let squished = StrBuilder::new("  hello   world  ").squish().into_string();
        assert_eq!(squished, "hello world");

        let norm = StrBuilder::new("path\\to\\file")
            .normalize_slashes()
            .into_string();
        assert_eq!(norm, "path/to/file");

        let escaped = StrBuilder::new("<b>test</b>").html_escape().into_string();
        assert_eq!(escaped, "&lt;b&gt;test&lt;/b&gt;");

        let unescaped = StrBuilder::new("&lt;b&gt;test&lt;/b&gt;")
            .html_unescape()
            .into_string();
        assert_eq!(unescaped, "<b>test</b>");

        let wrapped = StrBuilder::new("a b c d e").word_wrap(3).into_string();
        assert_eq!(wrapped, "a b\nc d\ne");

        let wrapped_opts = StrBuilder::new("a b c")
            .word_wrap_with_options(&WordWrapOptions::new(3).with_indent(" "))
            .into_string();
        assert_eq!(wrapped_opts, "a b\n c");

        let interp = StrBuilder::new("hello :name")
            .interpolate(&[("name", "world")])
            .into_string();
        assert_eq!(interp, "hello world");

        let trimmed = StrBuilder::new("  hello  ").trim().build();
        assert_eq!(trimmed, "hello");

        let trim_s = StrBuilder::new("  hello").trim_start().into_string();
        assert_eq!(trim_s, "hello");

        let trim_e = StrBuilder::new("hello  ").trim_end().into_string();
        assert_eq!(trim_e, "hello");

        let lower = StrBuilder::new("HELLO").lowercase().into_string();
        assert_eq!(lower, "hello");

        let upper = StrBuilder::new("hello").uppercase().into_string();
        assert_eq!(upper, "HELLO");

        let title_apa = StrBuilder::new("the old man and the sea")
            .title_case_apa()
            .into_string();
        assert_eq!(title_apa, "The Old Man and the Sea");

        let swapped = StrBuilder::new("Hello World!").swap_case().into_string();
        assert_eq!(swapped, "hELLO wORLD!");

        let chopped_s = StrBuilder::new("https://example.com")
            .chop_start("https://")
            .into_string();
        assert_eq!(chopped_s, "example.com");

        let chopped_e = StrBuilder::new("file.tar.gz").chop_end(".gz").into_string();
        assert_eq!(chopped_e, "file.tar");

        let deord = StrBuilder::new("22nd").deordinalize().into_string();
        assert_eq!(deord, "22");

        let ens_wrap = StrBuilder::new("path").ensure_wrap("/", "/").into_string();
        assert_eq!(ens_wrap, "/path/");

        let surr = StrBuilder::new("tag").surround("<", ">").into_string();
        assert_eq!(surr, "<tag>");

        let unwrapped_q = StrBuilder::new("\"quoted\"").unwrap_quotes().into_string();
        assert_eq!(unwrapped_q, "quoted");

        let app = StrBuilder::new("hello").append(" world").into_string();
        assert_eq!(app, "hello world");

        let prep = StrBuilder::new("world").prepend("hello ").into_string();
        assert_eq!(prep, "hello world");

        fn append_exclamation(b: StrBuilder) -> StrBuilder {
            b.append("!")
        }
        let flow_t = StrBuilder::new("item")
            .when(true, append_exclamation)
            .into_string();
        assert_eq!(flow_t, "item!");

        let flow_f = StrBuilder::new("item")
            .when(false, append_exclamation)
            .into_string();
        assert_eq!(flow_f, "item");

        let ne_t = StrBuilder::new("item")
            .when_not_empty(append_exclamation)
            .into_string();
        assert_eq!(ne_t, "item!");

        let ne_f = StrBuilder::new("")
            .when_not_empty(append_exclamation)
            .into_string();
        assert_eq!(ne_f, "");

        let e_t = StrBuilder::new("")
            .when_empty(append_exclamation)
            .into_string();
        assert_eq!(e_t, "!");

        let e_f = StrBuilder::new("item")
            .when_empty(append_exclamation)
            .into_string();
        assert_eq!(e_f, "item");

        let aft = StrBuilder::new("foo/bar/baz").after("/").into_string();
        assert_eq!(aft, "bar/baz");

        let bef = StrBuilder::new("foo/bar/baz").before("/").into_string();
        assert_eq!(bef, "foo");

        let aft_last = StrBuilder::new("foo/bar/baz").after_last("/").into_string();
        assert_eq!(aft_last, "baz");

        let bef_last = StrBuilder::new("foo/bar/baz")
            .before_last("/")
            .into_string();
        assert_eq!(bef_last, "foo/bar");

        let btw = StrBuilder::new("[hello]").between("[", "]").into_string();
        assert_eq!(btw, "hello");

        let btw_f = StrBuilder::new("[a][b]")
            .between_first("[", "]")
            .into_string();
        assert_eq!(btw_f, "a");

        let ch_s = StrBuilder::new("/path").chop_start("/").into_string();
        assert_eq!(ch_s, "path");

        let ch_e = StrBuilder::new("path/").chop_end("/").into_string();
        assert_eq!(ch_e, "path");

        let dmod = StrBuilder::new("App::Models::User")
            .demodulize()
            .into_string();
        assert_eq!(dmod, "User");

        let dconst = StrBuilder::new("App::Models::User")
            .deconstantize()
            .into_string();
        assert_eq!(dconst, "App::Models");

        let deord = StrBuilder::new("1st").deordinalize().into_string();
        assert_eq!(deord, "1");

        let t_apa = StrBuilder::new("the lord of the rings")
            .title_case_apa()
            .into_string();
        assert_eq!(t_apa, "The Lord of the Rings");

        let tr_s = StrBuilder::new("  hello").trim_start().into_string();
        assert_eq!(tr_s, "hello");

        let tr_e = StrBuilder::new("hello  ").trim_end().into_string();
        assert_eq!(tr_e, "hello");

        let lc = StrBuilder::new("HELLO").lowercase().into_string();
        assert_eq!(lc, "hello");

        let uc = StrBuilder::new("hello").uppercase().into_string();
        assert_eq!(uc, "HELLO");

        let sc = StrBuilder::new("Hello World").swap_case().into_string();
        assert_eq!(sc, "hELLO wORLD");

        let ww_opt = StrBuilder::new("one two three")
            .word_wrap_with_options(&WordWrapOptions::new(5))
            .into_string();
        assert_eq!(ww_opt, "one\ntwo\nthree");

        let pl_inc = StrBuilder::new("item")
            .pluralize_with_count_inclusive(3)
            .into_string();
        assert_eq!(pl_inc, "3 items");

        let r_pref_opt = StrBuilder::new("ApiUser")
            .remove_prefix_with_options("Api", &AffixOptions::new())
            .into_string();
        assert_eq!(r_pref_opt, "User");

        let r_suff_opt = StrBuilder::new("UserController")
            .remove_suffix_with_options("Controller", &AffixOptions::new())
            .into_string();
        assert_eq!(r_suff_opt, "User");

        let screaming = StrBuilder::from("user_id").screaming_snake_case();
        assert_eq!(screaming.as_str(), "USER_ID");
        assert_eq!(format!("{screaming}"), "USER_ID");
        assert_eq!(screaming.as_ref(), "USER_ID");
        assert_eq!(&*screaming, "USER_ID");

        let from_str: StrBuilder = "test".into();
        assert_eq!(from_str.into_string(), "test");

        let from_string: StrBuilder = String::from("test").into();
        assert_eq!(from_string.into_string(), "test");

        let snake = StrBuilder::new("UserId").snake_case().into_string();
        assert_eq!(snake, "user_id");

        let kebab = StrBuilder::new("user_id").kebab_case().into_string();
        assert_eq!(kebab, "user-id");

        let train = StrBuilder::new("user_id").train_case().into_string();
        assert_eq!(train, "User-Id");

        let title = StrBuilder::new("user_id").title_case().into_string();
        assert_eq!(title, "User Id");

        let sentence = StrBuilder::new("user_id").sentence_case().into_string();
        assert_eq!(sentence, "User id");

        let no_c = StrBuilder::new("UserProfile").no_case().into_string();
        assert_eq!(no_c, "user profile");

        let cap_c = StrBuilder::new("user_profile").capital_case().into_string();
        assert_eq!(cap_c, "User Profile");

        let cap_f = StrBuilder::new("hello").capitalize_first().into_string();
        assert_eq!(cap_f, "Hello");

        let uncap_f = StrBuilder::new("Hello").uncapitalize_first().into_string();
        assert_eq!(uncap_f, "hello");

        let studly = StrBuilder::new("user_id").studly_case().into_string();
        assert_eq!(studly, "UserId");

        let cls = StrBuilder::new("user_id").class_case().into_string();
        assert_eq!(cls, "UserId");

        let tbl = StrBuilder::new("UserProfile").table_case().into_string();
        assert_eq!(tbl, "user_profiles");

        let to_pl = StrBuilder::new("user").to_plural().into_string();
        assert_eq!(to_pl, "users");

        let to_sing = StrBuilder::new("users").to_singular().into_string();
        assert_eq!(to_sing, "user");

        let classify = StrBuilder::new("users").classify().into_string();
        assert_eq!(classify, "User");

        let tableize = StrBuilder::new("User").tableize().into_string();
        assert_eq!(tableize, "users");

        let fk = StrBuilder::new("User").foreign_key().into_string();
        assert_eq!(fk, "user_id");

        let mut tapped = false;
        let t = StrBuilder::new("v").tap(|_| tapped = true).into_string();
        assert!(tapped);
        assert_eq!(t, "v");

        let piped = StrBuilder::new("hello").pipe(|b| b.into_string().len());
        assert_eq!(piped, 5);

        let chk = StrBuilder::new("api/v1");
        assert!(!chk.is_empty());
        assert!(chk.is_not_empty());
        assert!(chk.starts_with_any(&["api", "web"]));
        assert!(chk.ends_with_any(&["v1", "v2"]));

        let rep = StrBuilder::new("a b a c")
            .replace_first("a", "x")
            .replace_last("a", "y")
            .remove("b ")
            .into_string();
        assert_eq!(rep, "x y c");

        let dedup = StrBuilder::new("a///b").deduplicate('/').into_string();
        assert_eq!(dedup, "a/b");

        let aff = StrBuilder::new("Controller")
            .prefix("User")
            .prefix_with_options("Api", &AffixOptions::new())
            .suffix("Ext")
            .suffix_with_options(".rs", &AffixOptions::new())
            .remove_prefix("Api")
            .remove_prefix_with_options("User", &AffixOptions::new())
            .remove_suffix(".rs")
            .remove_suffix_with_options("Ext", &AffixOptions::new())
            .ext("rs")
            .remove_extension()
            .into_string();
        assert_eq!(aff, "Controller");
    }
}
