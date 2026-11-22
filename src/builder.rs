//! Fluent, chainable `StrBuilder` for composing sequential string transformations.

use std::{fmt, ops::Deref};

use crate::{
    case::{
        classify, deconstantize, demodulize, foreign_key, headline, humanize, tableize,
        to_camel_case, to_dot_case, to_kebab_case, to_pascal_case, to_screaming_snake_case,
        to_sentence_case, to_snake_case, to_title_case, to_train_case,
    },
    grammar::{pluralize, pluralize_with_count, singularize},
    slug::{SlugOptions, slugify, slugify_with_options},
    utils::{
        TruncateOptions, WordWrapOptions, after, after_last, before, before_last, between,
        between_first, finish, html_escape, html_unescape, interpolate, mask, normalize_slashes,
        squish, start, truncate, word_wrap, word_wrap_with_options, words, wrap,
    },
};

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
    pub fn snake_case(mut self) -> Self {
        self.value = to_snake_case(&self.value).into_owned();
        self
    }

    /// Transforms the inner string into `SCREAMING_SNAKE_CASE`.
    pub fn screaming_snake_case(mut self) -> Self {
        self.value = to_screaming_snake_case(&self.value).into_owned();
        self
    }

    /// Transforms the inner string into `camelCase`.
    pub fn camel_case(mut self) -> Self {
        self.value = to_camel_case(&self.value).into_owned();
        self
    }

    /// Transforms the inner string into `PascalCase`.
    pub fn pascal_case(mut self) -> Self {
        self.value = to_pascal_case(&self.value).into_owned();
        self
    }

    /// Transforms the inner string into `kebab-case`.
    pub fn kebab_case(mut self) -> Self {
        self.value = to_kebab_case(&self.value).into_owned();
        self
    }

    /// Transforms the inner string into `Train-Case`.
    pub fn train_case(mut self) -> Self {
        self.value = to_train_case(&self.value).into_owned();
        self
    }

    /// Transforms the inner string into `Title Case`.
    pub fn title_case(mut self) -> Self {
        self.value = to_title_case(&self.value).into_owned();
        self
    }

    /// Transforms the inner string into `Sentence case`.
    pub fn sentence_case(mut self) -> Self {
        self.value = to_sentence_case(&self.value).into_owned();
        self
    }

    /// Transforms the inner string into `dot.case`.
    pub fn dot_case(mut self) -> Self {
        self.value = to_dot_case(&self.value).into_owned();
        self
    }

    /// Converts a model name into a conventional table name.
    pub fn tableize(mut self) -> Self {
        self.value = tableize(&self.value);
        self
    }

    /// Converts a table name into a conventional model class name.
    pub fn classify(mut self) -> Self {
        self.value = classify(&self.value);
        self
    }

    /// Generates a foreign key column name.
    pub fn foreign_key(mut self) -> Self {
        self.value = foreign_key(&self.value);
        self
    }

    /// Converts identifier into capitalized headline words.
    pub fn headline(mut self) -> Self {
        self.value = headline(&self.value);
        self
    }

    /// Converts identifier into sentence-style human readable text.
    pub fn humanize(mut self) -> Self {
        self.value = humanize(&self.value);
        self
    }

    /// Extracts type name removing namespace modules.
    pub fn demodulize(mut self) -> Self {
        self.value = demodulize(&self.value).to_string();
        self
    }

    /// Extracts parent module namespace.
    pub fn deconstantize(mut self) -> Self {
        self.value = deconstantize(&self.value).to_string();
        self
    }

    /// Pluralizes the inner string.
    pub fn pluralize(mut self) -> Self {
        self.value = pluralize(&self.value);
        self
    }

    /// Singularizes the inner string.
    pub fn singularize(mut self) -> Self {
        self.value = singularize(&self.value);
        self
    }

    /// Inflects based on the given count.
    pub fn pluralize_with_count(mut self, count: usize) -> Self {
        self.value = pluralize_with_count(&self.value, count);
        self
    }

    /// Converts the inner string to a URL-safe slug.
    pub fn slugify(mut self) -> Self {
        self.value = slugify(&self.value);
        self
    }

    /// Converts the inner string to a URL-safe slug with options.
    pub fn slugify_with_options(mut self, options: &SlugOptions) -> Self {
        self.value = slugify_with_options(&self.value, options);
        self
    }

    /// Truncates the inner string to maximum characters with default ellipsis.
    pub fn truncate(mut self, max_characters: usize) -> Self {
        self.value = truncate(&self.value, max_characters, &TruncateOptions::default());
        self
    }

    /// Truncates the inner string with custom options.
    pub fn truncate_with_options(
        mut self,
        max_characters: usize,
        options: &TruncateOptions,
    ) -> Self {
        self.value = truncate(&self.value, max_characters, options);
        self
    }

    /// Limits the inner string to a maximum number of words.
    pub fn words(mut self, max_words: usize, suffix: &str) -> Self {
        self.value = words(&self.value, max_words, suffix);
        self
    }

    /// Masks a substring of the inner string with the given character.
    pub fn mask(mut self, mask_char: char, start: usize, length: Option<usize>) -> Self {
        self.value = mask(&self.value, mask_char, start, length);
        self
    }

    /// Ensures the inner string starts with the given prefix.
    pub fn start(mut self, prefix: &str) -> Self {
        self.value = start(&self.value, prefix);
        self
    }

    /// Ensures the inner string ends with the given suffix.
    pub fn finish(mut self, suffix: &str) -> Self {
        self.value = finish(&self.value, suffix);
        self
    }

    /// Wraps the inner string with prefix and suffix strings.
    pub fn wrap(mut self, before_val: &str, after_val: &str) -> Self {
        self.value = wrap(&self.value, before_val, after_val);
        self
    }

    /// Slices before the first occurrence of a delimiter.
    pub fn before(mut self, delimiter: &str) -> Self {
        self.value = before(&self.value, delimiter).into_owned();
        self
    }

    /// Slices before the last occurrence of a delimiter.
    pub fn before_last(mut self, delimiter: &str) -> Self {
        self.value = before_last(&self.value, delimiter).into_owned();
        self
    }

    /// Slices after the first occurrence of a delimiter.
    pub fn after(mut self, delimiter: &str) -> Self {
        self.value = after(&self.value, delimiter).into_owned();
        self
    }

    /// Slices after the last occurrence of a delimiter.
    pub fn after_last(mut self, delimiter: &str) -> Self {
        self.value = after_last(&self.value, delimiter).into_owned();
        self
    }

    /// Slices between two delimiters.
    pub fn between(mut self, start_delim: &str, end_delim: &str) -> Self {
        self.value = between(&self.value, start_delim, end_delim).into_owned();
        self
    }

    /// Slices between the first occurrence of two delimiters.
    pub fn between_first(mut self, start_delim: &str, end_delim: &str) -> Self {
        self.value = between_first(&self.value, start_delim, end_delim).into_owned();
        self
    }

    /// Strips leading and trailing whitespace and collapses consecutive spaces.
    pub fn squish(mut self) -> Self {
        self.value = squish(&self.value).into_owned();
        self
    }

    /// Normalizes Windows backslashes into forward slashes.
    pub fn normalize_slashes(mut self) -> Self {
        self.value = normalize_slashes(&self.value).into_owned();
        self
    }

    /// Escapes HTML characters.
    pub fn html_escape(mut self) -> Self {
        self.value = html_escape(&self.value).into_owned();
        self
    }

    /// Unescapes HTML character entities.
    pub fn html_unescape(mut self) -> Self {
        self.value = html_unescape(&self.value).into_owned();
        self
    }

    /// Wraps text to line width breaking on words.
    pub fn word_wrap(mut self, max_width: usize) -> Self {
        self.value = word_wrap(&self.value, max_width);
        self
    }

    /// Wraps text with custom options.
    pub fn word_wrap_with_options(mut self, options: &WordWrapOptions) -> Self {
        self.value = word_wrap_with_options(&self.value, options);
        self
    }

    /// Interpolates named parameters into the inner string.
    pub fn interpolate(mut self, params: &[(&str, &str)]) -> Self {
        self.value = interpolate(&self.value, params);
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
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.value)
    }
}

impl From<String> for StrBuilder {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for StrBuilder {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_chaining() {
        let result = StrBuilder::new("UserProfile")
            .snake_case()
            .pluralize()
            .into_string();
        assert_eq!(result, "user_profiles");

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
    }
}
