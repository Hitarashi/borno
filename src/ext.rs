//! Extension trait providing fluent methods on string slices.

use std::borrow::Cow;

use crate::{
    case::{
        classify, deconstantize, demodulize, foreign_key, headline, humanize, is_camel_case,
        is_dot_case, is_foreign_key, is_kebab_case, is_pascal_case, is_screaming_snake_case,
        is_sentence_case, is_snake_case, is_title_case, is_train_case, tableize, to_camel_case,
        to_dot_case, to_kebab_case, to_pascal_case, to_screaming_snake_case, to_sentence_case,
        to_snake_case, to_title_case, to_train_case,
    },
    grammar::{
        is_plural, is_singular, pluralize, pluralize_with_count, pluralize_with_count_inclusive,
        singularize,
    },
    slug::{SlugOptions, slugify, slugify_with_options},
    utils::{
        ExcerptOptions, TruncateOptions, WordWrapOptions, after, after_last, before, before_last,
        between, between_first, excerpt, finish, html_escape, html_unescape, interpolate, mask,
        normalize_slashes, squish, start, truncate, unwrap, word_wrap, word_wrap_with_options,
        words, wrap,
    },
};

/// Extension trait enabling fluent inflections, casing transformations, and utilities on `&str`.
pub trait StrExt {
    /// Converts to `snake_case`.
    fn to_snake_case(&self) -> Cow<'_, str>;

    /// Converts to `SCREAMING_SNAKE_CASE`.
    fn to_screaming_snake_case(&self) -> Cow<'_, str>;

    /// Converts to `camelCase`.
    fn to_camel_case(&self) -> Cow<'_, str>;

    /// Converts to `PascalCase`.
    fn to_pascal_case(&self) -> Cow<'_, str>;

    /// Converts to `kebab-case`.
    fn to_kebab_case(&self) -> Cow<'_, str>;

    /// Converts to `Train-Case`.
    fn to_train_case(&self) -> Cow<'_, str>;

    /// Converts to `Title Case`.
    fn to_title_case(&self) -> Cow<'_, str>;

    /// Converts to `Sentence case`.
    fn to_sentence_case(&self) -> Cow<'_, str>;

    /// Converts to `dot.case`.
    fn to_dot_case(&self) -> Cow<'_, str>;

    /// Converts model name to database table name.
    fn tableize(&self) -> String;

    /// Converts table name to model class name.
    fn classify(&self) -> String;

    /// Generates foreign key column name.
    fn foreign_key(&self) -> String;

    /// Checks if identifier represents a foreign key ending in `_id`.
    fn is_foreign_key(&self) -> bool;

    /// Converts identifier into capitalized headline words.
    fn headline(&self) -> String;

    /// Converts identifier into sentence-style human readable text.
    fn humanize(&self) -> String;

    /// Extracts type name removing namespace modules.
    fn demodulize(&self) -> &str;

    /// Extracts parent module namespace.
    fn deconstantize(&self) -> &str;

    /// Checks if slice is valid `snake_case`.
    fn is_snake_case(&self) -> bool;

    /// Checks if slice is valid `SCREAMING_SNAKE_CASE`.
    fn is_screaming_snake_case(&self) -> bool;

    /// Checks if slice is valid `camelCase`.
    fn is_camel_case(&self) -> bool;

    /// Checks if slice is valid `PascalCase`.
    fn is_pascal_case(&self) -> bool;

    /// Checks if slice is valid `kebab-case`.
    fn is_kebab_case(&self) -> bool;

    /// Checks if slice is valid `Train-Case`.
    fn is_train_case(&self) -> bool;

    /// Checks if slice is valid `Title Case`.
    fn is_title_case(&self) -> bool;

    /// Checks if slice is valid `Sentence case`.
    fn is_sentence_case(&self) -> bool;

    /// Checks if slice is valid `dot.case`.
    fn is_dot_case(&self) -> bool;

    /// Pluralizes word.
    fn to_plural(&self) -> String;

    /// Singularizes word.
    fn to_singular(&self) -> String;

    /// Inflects based on count.
    fn pluralize_with_count(&self, count: usize) -> String;

    /// Inflects with count prefix.
    fn pluralize_with_count_inclusive(&self, count: usize) -> String;

    /// Checks if word is plural.
    fn is_plural(&self) -> bool;

    /// Checks if word is singular.
    fn is_singular(&self) -> bool;

    /// Converts to URL-safe slug.
    fn to_slug(&self) -> String;

    /// Converts to URL-safe slug with options.
    fn to_slug_with_options(&self, options: &SlugOptions) -> String;

    /// Truncates to maximum characters with default ellipsis.
    fn truncate(&self, max_characters: usize) -> String;

    /// Truncates to maximum characters with custom options.
    fn truncate_with_options(&self, max_characters: usize, options: &TruncateOptions) -> String;

    /// Limits word count.
    fn words(&self, max_words: usize, suffix: &str) -> String;

    /// Extracts surrounding context excerpt.
    fn excerpt(&self, term: &str, options: &ExcerptOptions) -> String;

    /// Masks substring with character.
    fn mask(&self, mask_char: char, start: usize, length: Option<usize>) -> String;

    /// Ensures prefix.
    fn start(&self, prefix: &str) -> String;

    /// Ensures suffix.
    fn finish(&self, suffix: &str) -> String;

    /// Wraps with prefix and suffix.
    fn wrap(&self, before: &str, after: &str) -> String;

    /// Removes enclosing prefix and suffix if present.
    fn unwrap<'a>(&'a self, before: &str, after: &str) -> &'a str;

    /// Extracts substring before first delimiter occurrence.
    fn before<'a>(&'a self, delimiter: &str) -> Cow<'a, str>;

    /// Extracts substring before last delimiter occurrence.
    fn before_last<'a>(&'a self, delimiter: &str) -> Cow<'a, str>;

    /// Extracts substring after first delimiter occurrence.
    fn after<'a>(&'a self, delimiter: &str) -> Cow<'a, str>;

    /// Extracts substring after last delimiter occurrence.
    fn after_last<'a>(&'a self, delimiter: &str) -> Cow<'a, str>;

    /// Extracts substring between delimiters.
    fn between<'a>(&'a self, start_delim: &str, end_delim: &str) -> Cow<'a, str>;

    /// Extracts substring between first occurrences of delimiters.
    fn between_first<'a>(&'a self, start_delim: &str, end_delim: &str) -> Cow<'a, str>;

    /// Collapses consecutive whitespace and trims ends.
    fn squish(&self) -> Cow<'_, str>;

    /// Normalizes Windows backslashes into forward slashes.
    fn normalize_slashes(&self) -> Cow<'_, str>;

    /// Escapes HTML special characters.
    fn html_escape(&self) -> Cow<'_, str>;

    /// Unescapes HTML character entities.
    fn html_unescape(&self) -> Cow<'_, str>;

    /// Wraps text to maximum width breaking on words.
    fn word_wrap(&self, max_width: usize) -> String;

    /// Wraps text with custom options.
    fn word_wrap_with_options(&self, options: &WordWrapOptions) -> String;

    /// Interpolates named parameters from slice.
    fn interpolate(&self, params: &[(&str, &str)]) -> String;
}

impl StrExt for str {
    fn to_snake_case(&self) -> Cow<'_, str> {
        to_snake_case(self)
    }

    fn to_screaming_snake_case(&self) -> Cow<'_, str> {
        to_screaming_snake_case(self)
    }

    fn to_camel_case(&self) -> Cow<'_, str> {
        to_camel_case(self)
    }

    fn to_pascal_case(&self) -> Cow<'_, str> {
        to_pascal_case(self)
    }

    fn to_kebab_case(&self) -> Cow<'_, str> {
        to_kebab_case(self)
    }

    fn to_train_case(&self) -> Cow<'_, str> {
        to_train_case(self)
    }

    fn to_title_case(&self) -> Cow<'_, str> {
        to_title_case(self)
    }

    fn to_sentence_case(&self) -> Cow<'_, str> {
        to_sentence_case(self)
    }

    fn to_dot_case(&self) -> Cow<'_, str> {
        to_dot_case(self)
    }

    fn tableize(&self) -> String {
        tableize(self)
    }

    fn classify(&self) -> String {
        classify(self)
    }

    fn foreign_key(&self) -> String {
        foreign_key(self)
    }

    fn is_foreign_key(&self) -> bool {
        is_foreign_key(self)
    }

    fn headline(&self) -> String {
        headline(self)
    }

    fn humanize(&self) -> String {
        humanize(self)
    }

    fn demodulize(&self) -> &str {
        demodulize(self)
    }

    fn deconstantize(&self) -> &str {
        deconstantize(self)
    }

    fn is_snake_case(&self) -> bool {
        is_snake_case(self)
    }

    fn is_screaming_snake_case(&self) -> bool {
        is_screaming_snake_case(self)
    }

    fn is_camel_case(&self) -> bool {
        is_camel_case(self)
    }

    fn is_pascal_case(&self) -> bool {
        is_pascal_case(self)
    }

    fn is_kebab_case(&self) -> bool {
        is_kebab_case(self)
    }

    fn is_train_case(&self) -> bool {
        is_train_case(self)
    }

    fn is_title_case(&self) -> bool {
        is_title_case(self)
    }

    fn is_sentence_case(&self) -> bool {
        is_sentence_case(self)
    }

    fn is_dot_case(&self) -> bool {
        is_dot_case(self)
    }

    fn to_plural(&self) -> String {
        pluralize(self)
    }

    fn to_singular(&self) -> String {
        singularize(self)
    }

    fn pluralize_with_count(&self, count: usize) -> String {
        pluralize_with_count(self, count)
    }

    fn pluralize_with_count_inclusive(&self, count: usize) -> String {
        pluralize_with_count_inclusive(self, count)
    }

    fn is_plural(&self) -> bool {
        is_plural(self)
    }

    fn is_singular(&self) -> bool {
        is_singular(self)
    }

    fn to_slug(&self) -> String {
        slugify(self)
    }

    fn to_slug_with_options(&self, options: &SlugOptions) -> String {
        slugify_with_options(self, options)
    }

    fn truncate(&self, max_characters: usize) -> String {
        truncate(self, max_characters, &TruncateOptions::default())
    }

    fn truncate_with_options(&self, max_characters: usize, options: &TruncateOptions) -> String {
        truncate(self, max_characters, options)
    }

    fn words(&self, max_words: usize, suffix: &str) -> String {
        words(self, max_words, suffix)
    }

    fn excerpt(&self, term: &str, options: &ExcerptOptions) -> String {
        excerpt(self, term, options)
    }

    fn mask(&self, mask_char: char, start_idx: usize, length: Option<usize>) -> String {
        mask(self, mask_char, start_idx, length)
    }

    fn start(&self, prefix: &str) -> String {
        start(self, prefix)
    }

    fn finish(&self, suffix: &str) -> String {
        finish(self, suffix)
    }

    fn wrap(&self, before_val: &str, after_val: &str) -> String {
        wrap(self, before_val, after_val)
    }

    fn unwrap<'a>(&'a self, before_val: &str, after_val: &str) -> &'a str {
        unwrap(self, before_val, after_val)
    }

    fn before<'a>(&'a self, delimiter: &str) -> Cow<'a, str> {
        before(self, delimiter)
    }

    fn before_last<'a>(&'a self, delimiter: &str) -> Cow<'a, str> {
        before_last(self, delimiter)
    }

    fn after<'a>(&'a self, delimiter: &str) -> Cow<'a, str> {
        after(self, delimiter)
    }

    fn after_last<'a>(&'a self, delimiter: &str) -> Cow<'a, str> {
        after_last(self, delimiter)
    }

    fn between<'a>(&'a self, start_delim: &str, end_delim: &str) -> Cow<'a, str> {
        between(self, start_delim, end_delim)
    }

    fn between_first<'a>(&'a self, start_delim: &str, end_delim: &str) -> Cow<'a, str> {
        between_first(self, start_delim, end_delim)
    }

    fn squish(&self) -> Cow<'_, str> {
        squish(self)
    }

    fn normalize_slashes(&self) -> Cow<'_, str> {
        normalize_slashes(self)
    }

    fn html_escape(&self) -> Cow<'_, str> {
        html_escape(self)
    }

    fn html_unescape(&self) -> Cow<'_, str> {
        html_unescape(self)
    }

    fn word_wrap(&self, max_width: usize) -> String {
        word_wrap(self, max_width)
    }

    fn word_wrap_with_options(&self, options: &WordWrapOptions) -> String {
        word_wrap_with_options(self, options)
    }

    fn interpolate(&self, params: &[(&str, &str)]) -> String {
        interpolate(self, params)
    }
}

#[cfg(test)]
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
