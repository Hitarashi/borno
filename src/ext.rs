//! Extension trait providing fluent methods on string slices.

use std::borrow::Cow;

#[cfg(feature = "conventions")]
use crate::case::{
    classify, deconstantize, demodulize, foreign_key, headline, humanize, is_foreign_key, tableize,
};
#[cfg(feature = "case-camel")]
use crate::case::{is_camel_case, to_camel_case};
#[cfg(feature = "case-dot")]
use crate::case::{is_dot_case, to_dot_case};
#[cfg(feature = "case-kebab")]
use crate::case::{is_kebab_case, to_kebab_case};
#[cfg(feature = "case-pascal")]
use crate::case::{is_pascal_case, to_pascal_case};
#[cfg(feature = "case-screaming-snake")]
use crate::case::{is_screaming_snake_case, to_screaming_snake_case};
#[cfg(feature = "case-sentence")]
use crate::case::{is_sentence_case, to_sentence_case};
#[cfg(feature = "case-snake")]
use crate::case::{is_snake_case, to_snake_case};
#[cfg(feature = "case-title")]
use crate::case::{is_title_case, to_title_case};
#[cfg(feature = "case-train")]
use crate::case::{is_train_case, to_train_case};
#[cfg(feature = "grammar")]
use crate::grammar::{
    is_plural, is_singular, pluralize, pluralize_with_count, pluralize_with_count_inclusive,
    singularize,
};
#[cfg(feature = "slug")]
use crate::slug::{SlugOptions, slugify, slugify_with_options};
#[cfg(feature = "util-interpolate")]
use crate::utils::interpolate;
#[cfg(feature = "util-mask")]
use crate::utils::mask;
#[cfg(feature = "util-truncate")]
use crate::utils::{ExcerptOptions, TruncateOptions, excerpt, truncate, words};
#[cfg(feature = "util-text")]
use crate::utils::{
    WordWrapOptions, html_escape, html_unescape, word_wrap, word_wrap_with_options,
};
#[cfg(feature = "util-slice")]
use crate::utils::{after, after_last, before, before_last, between, between_first};
#[cfg(feature = "util-pad")]
use crate::utils::{finish, start, unwrap, wrap};
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

    /// Converts to `Sentence case`.
    #[cfg(feature = "case-sentence")]
    fn to_sentence_case(&self) -> Cow<'_, str>;

    /// Converts to `dot.case`.
    #[cfg(feature = "case-dot")]
    fn to_dot_case(&self) -> Cow<'_, str>;

    /// Converts a model name into a conventional database table name.
    #[cfg(feature = "conventions")]
    fn tableize(&self) -> String;

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

    /// Checks if the slice is valid `Sentence case`.
    #[cfg(feature = "case-sentence")]
    fn is_sentence_case(&self) -> bool;

    /// Checks if the slice is valid `dot.case`.
    #[cfg(feature = "case-dot")]
    fn is_dot_case(&self) -> bool;

    /// Pluralizes a word.
    #[cfg(feature = "grammar")]
    fn to_plural(&self) -> String;

    /// Singularizes a word.
    #[cfg(feature = "grammar")]
    fn to_singular(&self) -> String;

    /// Inflects a word based on a count.
    #[cfg(feature = "grammar")]
    fn pluralize_with_count(&self, count: usize) -> String;

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

    /// Removes the enclosing prefix and suffix if present.
    #[cfg(feature = "util-pad")]
    fn unwrap(&self, before: &str, after: &str) -> &str;

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

    #[cfg(feature = "case-sentence")]
    fn to_sentence_case(&self) -> Cow<'_, str> {
        to_sentence_case(self)
    }

    #[cfg(feature = "case-dot")]
    fn to_dot_case(&self) -> Cow<'_, str> {
        to_dot_case(self)
    }

    #[cfg(feature = "conventions")]
    fn tableize(&self) -> String {
        tableize(self)
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

    #[cfg(feature = "case-sentence")]
    fn is_sentence_case(&self) -> bool {
        is_sentence_case(self)
    }

    #[cfg(feature = "case-dot")]
    fn is_dot_case(&self) -> bool {
        is_dot_case(self)
    }

    #[cfg(feature = "grammar")]
    fn to_plural(&self) -> String {
        pluralize(self)
    }

    #[cfg(feature = "grammar")]
    fn to_singular(&self) -> String {
        singularize(self)
    }

    #[cfg(feature = "grammar")]
    fn pluralize_with_count(&self, count: usize) -> String {
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
    fn unwrap(&self, before_val: &str, after_val: &str) -> &str {
        unwrap(self, before_val, after_val)
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
    }
}
