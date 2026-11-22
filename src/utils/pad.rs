//! String padding and boundary guards.

/// Computes remaining padding count and allocates a string with appropriate capacity.
/// Returns None if no padding is required.
fn prepare_padded_buffer(
    input: &str,
    target_len: usize,
    pad_char: char,
) -> Option<(usize, String)> {
    let current_len = input.chars().count();
    if current_len >= target_len {
        return None;
    }
    let pad_count = target_len - current_len;
    let buffer = String::with_capacity(input.len() + pad_count * pad_char.len_utf8());
    Some((pad_count, buffer))
}

fn push_repeated(buffer: &mut String, pad_char: char, count: usize) {
    buffer.extend(std::iter::repeat_n(pad_char, count));
}

/// Left-pads a string until it reaches the specified character length.
///
/// # Examples
/// ```
/// use borno::utils::pad_start;
///
/// assert_eq!(pad_start("42", 5, '0'), "00042");
/// assert_eq!(pad_start("hello", 3, ' '), "hello");
/// ```
pub fn pad_start(input: &str, target_len: usize, pad_char: char) -> String {
    let Some((pad_count, mut result)) = prepare_padded_buffer(input, target_len, pad_char) else {
        return input.to_string();
    };
    push_repeated(&mut result, pad_char, pad_count);
    result.push_str(input);
    result
}

/// Right-pads a string until it reaches the specified character length.
///
/// # Examples
/// ```
/// use borno::utils::pad_end;
///
/// assert_eq!(pad_end("42", 5, '0'), "42000");
/// assert_eq!(pad_end("hello", 3, ' '), "hello");
/// ```
pub fn pad_end(input: &str, target_len: usize, pad_char: char) -> String {
    let Some((pad_count, mut result)) = prepare_padded_buffer(input, target_len, pad_char) else {
        return input.to_string();
    };
    result.push_str(input);
    push_repeated(&mut result, pad_char, pad_count);
    result
}

/// Pads both sides of a string evenly until it reaches the specified character length.
///
/// # Examples
/// ```
/// use borno::utils::pad_both;
///
/// assert_eq!(pad_both("title", 9, '-'), "--title--");
/// assert_eq!(pad_both("title", 10, '-'), "--title---");
/// ```
pub fn pad_both(input: &str, target_len: usize, pad_char: char) -> String {
    let Some((total_pad, mut result)) = prepare_padded_buffer(input, target_len, pad_char) else {
        return input.to_string();
    };
    let left_pad = total_pad / 2;
    let right_pad = total_pad - left_pad;
    push_repeated(&mut result, pad_char, left_pad);
    result.push_str(input);
    push_repeated(&mut result, pad_char, right_pad);
    result
}

use std::borrow::Cow;

/// Deep affix engine providing boundary guardrails, idempotent prefixes/suffixes, and wrapping.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Affix;

impl Affix {
    /// Idempotently ensures an input string begins with the specified prefix without duplicating it.
    #[inline]
    pub fn ensure_prefix<'a>(input: &'a str, prefix: &str) -> Cow<'a, str> {
        ensure_prefix(input, prefix)
    }

    /// Idempotently ensures an input string ends with the specified suffix without duplicating it.
    #[inline]
    pub fn ensure_suffix<'a>(input: &'a str, suffix: &str) -> Cow<'a, str> {
        ensure_suffix(input, suffix)
    }

    /// Idempotently ensures an input string starts with before and ends with after.
    #[inline]
    pub fn ensure_wrapped<'a>(input: &'a str, before: &str, after: &str) -> Cow<'a, str> {
        ensure_wrapped(input, before, after)
    }

    /// Strips an exact prefix from an input slice if present.
    #[inline]
    pub fn strip_prefix<'a>(input: &'a str, prefix: &str) -> &'a str {
        strip_prefix(input, prefix)
    }

    /// Strips an exact suffix from an input slice if present.
    #[inline]
    pub fn strip_suffix<'a>(input: &'a str, suffix: &str) -> &'a str {
        strip_suffix(input, suffix)
    }

    /// Wraps a string with prefix and suffix.
    #[inline]
    pub fn wrap(input: &str, before: &str, after: &str) -> String {
        wrap(input, before, after)
    }

    /// Strips enclosing prefix and suffix delimiters from a string slice.
    #[inline]
    pub fn unwrap<'a>(input: &'a str, before: &str, after: &str) -> &'a str {
        unwrap(input, before, after)
    }

    /// Strips matching enclosing quotes (`"` or `'`).
    #[inline]
    pub fn unwrap_quotes(input: &str) -> &str {
        unwrap_quotes(input)
    }

    /// Left-pads a string slice.
    #[inline]
    pub fn pad_start(input: &str, target_len: usize, pad_char: char) -> String {
        pad_start(input, target_len, pad_char)
    }

    /// Right-pads a string slice.
    #[inline]
    pub fn pad_end(input: &str, target_len: usize, pad_char: char) -> String {
        pad_end(input, target_len, pad_char)
    }

    /// Center-pads a string slice.
    #[inline]
    pub fn pad_both(input: &str, target_len: usize, pad_char: char) -> String {
        pad_both(input, target_len, pad_char)
    }
}

/// Ensures an input string slice starts with the given prefix without duplicating it.
///
/// Returns `Cow::Borrowed` if the input already begins with `prefix`.
#[must_use]
pub fn ensure_prefix<'a>(input: &'a str, prefix: &str) -> Cow<'a, str> {
    if prefix.is_empty() || input.starts_with(prefix) {
        Cow::Borrowed(input)
    } else {
        let mut result = String::with_capacity(prefix.len() + input.len());
        result.push_str(prefix);
        result.push_str(input);
        Cow::Owned(result)
    }
}

/// Ensures an input string slice ends with the given suffix without duplicating it.
///
/// Returns `Cow::Borrowed` if the input already ends with `suffix`.
#[must_use]
pub fn ensure_suffix<'a>(input: &'a str, suffix: &str) -> Cow<'a, str> {
    if suffix.is_empty() || input.ends_with(suffix) {
        Cow::Borrowed(input)
    } else {
        let mut result = String::with_capacity(input.len() + suffix.len());
        result.push_str(input);
        result.push_str(suffix);
        Cow::Owned(result)
    }
}

/// Ensures a string slice starts with a prefix and ends with a suffix without duplicating them.
///
/// Returns `Cow::Borrowed` with zero allocations if already correctly wrapped.
#[must_use]
pub fn ensure_wrapped<'a>(input: &'a str, before: &str, after: &str) -> Cow<'a, str> {
    let has_before = before.is_empty() || input.starts_with(before);
    let has_after = after.is_empty() || input.ends_with(after);

    if has_before && has_after {
        return Cow::Borrowed(input);
    }

    let extra_before = if has_before { 0 } else { before.len() };
    let extra_after = if has_after { 0 } else { after.len() };
    let mut result = String::with_capacity(input.len() + extra_before + extra_after);

    if !has_before {
        result.push_str(before);
    }
    result.push_str(input);
    if !has_after {
        result.push_str(after);
    }
    Cow::Owned(result)
}

/// Strips an exact prefix from an input slice if present.
#[inline]
#[must_use]
pub fn strip_prefix<'a>(input: &'a str, prefix: &str) -> &'a str {
    input.strip_prefix(prefix).unwrap_or(input)
}

/// Strips an exact suffix from an input slice if present.
#[inline]
#[must_use]
pub fn strip_suffix<'a>(input: &'a str, suffix: &str) -> &'a str {
    input.strip_suffix(suffix).unwrap_or(input)
}

/// Ensures a string starts with the given prefix.
///
/// # Examples
/// ```
/// use borno::utils::start;
///
/// assert_eq!(start("path/to/resource", "/"), "/path/to/resource");
/// assert_eq!(start("/path/to/resource", "/"), "/path/to/resource");
/// ```
pub fn start(input: &str, prefix: &str) -> String {
    ensure_prefix(input, prefix).into_owned()
}

/// Ensures a string ends with the given suffix.
///
/// # Examples
/// ```
/// use borno::utils::finish;
///
/// assert_eq!(finish("https://example.com", "/"), "https://example.com/");
/// assert_eq!(finish("https://example.com/", "/"), "https://example.com/");
/// ```
pub fn finish(input: &str, suffix: &str) -> String {
    ensure_suffix(input, suffix).into_owned()
}

/// Wraps a string with prefix and suffix strings.
///
/// # Examples
/// ```
/// use borno::utils::wrap;
///
/// assert_eq!(wrap("value", "\"", "\""), "\"value\"");
/// assert_eq!(wrap("tag", "<", ">"), "<tag>");
/// ```
pub fn wrap(input: &str, before: &str, after: &str) -> String {
    let mut result = String::with_capacity(before.len() + input.len() + after.len());
    result.push_str(before);
    result.push_str(input);
    result.push_str(after);
    result
}

/// Removes enclosing prefix and suffix from a string if both are present.
///
/// # Examples
/// ```
/// use borno::utils::unwrap;
///
/// assert_eq!(unwrap("\"value\"", "\"", "\""), "value");
/// assert_eq!(unwrap("<tag>", "<", ">"), "tag");
/// assert_eq!(unwrap("unwrapped", "<", ">"), "unwrapped");
/// ```
pub fn unwrap<'a>(input: &'a str, before: &str, after: &str) -> &'a str {
    if input.starts_with(before)
        && input.ends_with(after)
        && input.len() >= before.len() + after.len()
    {
        &input[before.len()..input.len() - after.len()]
    } else {
        input
    }
}

/// Ensures a string starts with a prefix and ends with a suffix without duplicating them.
///
/// # Examples
/// ```
/// use borno::utils::ensure_wrap;
///
/// assert_eq!(ensure_wrap("value", "/", "/"), "/value/");
/// assert_eq!(ensure_wrap("/value", "/", "/"), "/value/");
/// assert_eq!(ensure_wrap("/value/", "/", "/"), "/value/");
/// ```
#[must_use]
pub fn ensure_wrap(input: &str, before: &str, after: &str) -> String {
    ensure_wrapped(input, before, after).into_owned()
}

/// Semantic alias for [`wrap`].
#[inline]
#[must_use]
pub fn surround(input: &str, before: &str, after: &str) -> String {
    wrap(input, before, after)
}

/// Options for affix (prefix and suffix) removal and addition.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AffixOptions {
    /// Words that should prevent stripping if matched at the boundary.
    pub similar_words: Vec<String>,
    /// Whether matching should ignore case (default: true).
    pub ignore_case: bool,
}

impl Default for AffixOptions {
    fn default() -> Self {
        Self {
            similar_words: Vec::new(),
            ignore_case: true,
        }
    }
}

impl AffixOptions {
    /// Creates a new default `AffixOptions`.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets words that protect against suffix or prefix stripping.
    #[must_use]
    pub fn with_similar_words<I, S>(mut self, words: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.similar_words = words.into_iter().map(|s| s.as_ref().to_string()).collect();
        self
    }

    /// Configures case sensitivity.
    #[must_use]
    pub fn with_ignore_case(mut self, ignore_case: bool) -> Self {
        self.ignore_case = ignore_case;
        self
    }
}

fn ends_with_affix(input: &str, affix: &str, ignore_case: bool) -> bool {
    if affix.is_empty() || input.len() < affix.len() {
        return false;
    }
    let tail = &input[input.len() - affix.len()..];
    if ignore_case {
        tail.eq_ignore_ascii_case(affix)
    } else {
        tail == affix
    }
}

fn strip_affix_end<'a>(input: &'a str, affix: &str, ignore_case: bool) -> &'a str {
    if affix.is_empty() || input.len() < affix.len() {
        return input;
    }
    let tail = &input[input.len() - affix.len()..];
    let matches = if ignore_case {
        tail.eq_ignore_ascii_case(affix)
    } else {
        tail == affix
    };
    if matches {
        let remainder = &input[..input.len() - affix.len()];
        if remainder.ends_with('_') || remainder.ends_with('-') {
            return &remainder[..remainder.len() - 1];
        }
        return remainder;
    }
    input
}

fn starts_with_affix(input: &str, affix: &str, ignore_case: bool) -> bool {
    if affix.is_empty() || input.len() < affix.len() {
        return false;
    }
    let head = &input[..affix.len()];
    if ignore_case {
        head.eq_ignore_ascii_case(affix)
    } else {
        head == affix
    }
}

fn strip_affix_start<'a>(input: &'a str, affix: &str, ignore_case: bool) -> &'a str {
    if affix.is_empty() || input.len() < affix.len() {
        return input;
    }
    let head = &input[..affix.len()];
    let matches = if ignore_case {
        head.eq_ignore_ascii_case(affix)
    } else {
        head == affix
    };
    if matches {
        let remainder = &input[affix.len()..];
        if remainder.starts_with('_') || remainder.starts_with('-') {
            return &remainder[1..];
        }
        return remainder;
    }
    input
}

/// Removes a suffix from a string slice with default options.
#[must_use]
pub fn remove_suffix<'a>(input: &'a str, suffix: &str) -> &'a str {
    remove_suffix_with_options(input, suffix, &AffixOptions::new())
}

/// Removes a suffix from a string slice with configurable options.
#[must_use]
pub fn remove_suffix_with_options<'a>(
    input: &'a str,
    suffix: &str,
    options: &AffixOptions,
) -> &'a str {
    if suffix.is_empty() || input.is_empty() {
        return input;
    }
    for similar in &options.similar_words {
        if ends_with_affix(input, similar, options.ignore_case) {
            return input;
        }
    }
    strip_affix_end(input, suffix, options.ignore_case)
}

/// Adds a suffix to a string slice, replacing any existing occurrence.
#[must_use]
pub fn suffix(input: &str, suffix_val: &str) -> String {
    suffix_with_options(input, suffix_val, &AffixOptions::new())
}

/// Adds a suffix to a string slice with configurable options.
#[must_use]
pub fn suffix_with_options(input: &str, suffix_val: &str, options: &AffixOptions) -> String {
    let stripped = remove_suffix_with_options(input, suffix_val, options);
    let mut result = String::with_capacity(stripped.len() + suffix_val.len());
    result.push_str(stripped);
    result.push_str(suffix_val);
    result
}

/// Removes a prefix from a string slice with default options.
#[must_use]
pub fn remove_prefix<'a>(input: &'a str, prefix: &str) -> &'a str {
    remove_prefix_with_options(input, prefix, &AffixOptions::new())
}

/// Removes a prefix from a string slice with configurable options.
#[must_use]
pub fn remove_prefix_with_options<'a>(
    input: &'a str,
    prefix: &str,
    options: &AffixOptions,
) -> &'a str {
    if prefix.is_empty() || input.is_empty() {
        return input;
    }
    for similar in &options.similar_words {
        if starts_with_affix(input, similar, options.ignore_case) {
            return input;
        }
    }
    strip_affix_start(input, prefix, options.ignore_case)
}

/// Adds a prefix to a string slice, replacing any existing occurrence.
#[must_use]
pub fn prefix(input: &str, prefix_val: &str) -> String {
    prefix_with_options(input, prefix_val, &AffixOptions::new())
}

/// Adds a prefix to a string slice with configurable options.
#[must_use]
pub fn prefix_with_options(input: &str, prefix_val: &str, options: &AffixOptions) -> String {
    let stripped = remove_prefix_with_options(input, prefix_val, options);
    let mut result = String::with_capacity(prefix_val.len() + stripped.len());
    result.push_str(prefix_val);
    result.push_str(stripped);
    result
}

/// Strips the file extension from a path or filename.
#[must_use]
pub fn remove_extension(input: &str) -> &str {
    let filename_start = input.rfind(['/', '\\']).map_or(0, |idx| idx + 1);
    let filename = &input[filename_start..];
    if let Some(dot_idx) = filename.rfind('.') {
        if dot_idx > 0 {
            return &input[..filename_start + dot_idx];
        }
    }
    input
}

/// Sets or replaces the file extension of a path or filename.
#[must_use]
pub fn ext(input: &str, extension: &str) -> String {
    let base = remove_extension(input);
    let clean_ext = extension.trim_start_matches('.');
    if clean_ext.is_empty() {
        base.to_string()
    } else {
        let mut result = String::with_capacity(base.len() + clean_ext.len() + 1);
        result.push_str(base);
        result.push('.');
        result.push_str(clean_ext);
        result
    }
}

/// Strips matching enclosing double quotes (`"`) or single quotes (`'`) from a string.
///
/// If the string is not enclosed in matching quotes, it is returned unchanged.
/// Operates with zero heap allocations.
///
/// # Examples
/// ```
/// use borno::utils::unwrap_quotes;
///
/// assert_eq!(unwrap_quotes("\"hello\""), "hello");
/// assert_eq!(unwrap_quotes("'world'"), "world");
/// assert_eq!(unwrap_quotes("\"mismatched'"), "\"mismatched'");
/// assert_eq!(unwrap_quotes("unquoted"), "unquoted");
/// ```
#[must_use]
pub fn unwrap_quotes(input: &str) -> &str {
    if input.len() >= 2
        && ((input.starts_with('"') && input.ends_with('"'))
            || (input.starts_with('\'') && input.ends_with('\'')))
    {
        return &input[1..input.len() - 1];
    }
    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_padding() {
        assert_eq!(pad_start("42", 5, '0'), "00042");
        assert_eq!(pad_end("42", 5, '0'), "42000");
        assert_eq!(pad_both("title", 9, '-'), "--title--");
        assert_eq!(pad_both("title", 10, '-'), "--title---");
    }

    #[test]
    fn test_start_finish() {
        assert_eq!(start("path/to/resource", "/"), "/path/to/resource");
        assert_eq!(start("/path/to/resource", "/"), "/path/to/resource");
        assert_eq!(finish("https://example.com", "/"), "https://example.com/");
        assert_eq!(finish("https://example.com/", "/"), "https://example.com/");
    }

    #[test]
    fn test_wrap_unwrap() {
        assert_eq!(wrap("value", "\"", "\""), "\"value\"");
        assert_eq!(surround("value", "<", ">"), "<value>");
        assert_eq!(unwrap("\"value\"", "\"", "\""), "value");
        assert_eq!(unwrap("<tag>", "<", ">"), "tag");
        assert_eq!(unwrap("tag", "<", ">"), "tag");

        assert_eq!(ensure_wrap("value", "/", "/"), "/value/");
        assert_eq!(ensure_wrap("/value", "/", "/"), "/value/");
        assert_eq!(ensure_wrap("value/", "/", "/"), "/value/");
        assert_eq!(ensure_wrap("/value/", "/", "/"), "/value/");

        assert_eq!(unwrap_quotes("\"quoted\""), "quoted");
        assert_eq!(unwrap_quotes("'quoted'"), "quoted");
        assert_eq!(unwrap_quotes("\"mismatched'"), "\"mismatched'");
        assert_eq!(unwrap_quotes("unquoted"), "unquoted");
        assert_eq!(unwrap_quotes("\""), "\"");
        assert_eq!(unwrap_quotes(""), "");
    }

    #[test]
    fn test_affix_and_extension() {
        assert_eq!(remove_suffix("usersController", "controller"), "users");
        assert_eq!(remove_suffix("user_controller", "controller"), "user");
        assert_eq!(remove_suffix("user-controller", "controller"), "user");
        assert_eq!(remove_suffix("usercontroller", "controller"), "user");
        assert_eq!(remove_suffix("a", "controller"), "a");
        assert_eq!(remove_prefix("a", "app"), "a");
        assert_eq!(
            remove_suffix_with_options(
                "a",
                "controller",
                &AffixOptions::new().with_ignore_case(false)
            ),
            "a"
        );
        assert_eq!(
            remove_prefix_with_options("a", "app", &AffixOptions::new().with_ignore_case(false)),
            "a"
        );
        assert_eq!(remove_suffix("", "controller"), "");
        assert_eq!(remove_suffix("user", ""), "user");

        let options = AffixOptions::new()
            .with_similar_words(["email", "emailer"])
            .with_ignore_case(true);
        assert_eq!(
            remove_suffix_with_options("verify_email", "mail", &options),
            "verify_email"
        );
        assert_eq!(
            remove_suffix_with_options("verify_mail", "mail", &options),
            "verify"
        );
        assert_eq!(
            suffix_with_options("verify_email", "Mail", &options),
            "verify_emailMail"
        );

        assert_eq!(suffix("user", "Controller"), "userController");
        assert_eq!(suffix("userController", "Controller"), "userController");
        assert_eq!(suffix("user_controller", "Controller"), "userController");

        assert_eq!(remove_prefix("app_user", "app"), "user");
        assert_eq!(remove_prefix("app-user", "app"), "user");
        assert_eq!(remove_prefix("AppUser", "app"), "User");
        assert_eq!(remove_prefix("user", "app"), "user");
        assert_eq!(remove_prefix("", "app"), "");
        assert_eq!(remove_prefix("user", ""), "user");

        let pref_opts = AffixOptions::new().with_similar_words(["email"]);
        assert_eq!(
            remove_prefix_with_options("email_address", "mail", &pref_opts),
            "email_address"
        );
        assert_eq!(
            prefix_with_options("address", "api_", &pref_opts),
            "api_address"
        );

        assert_eq!(prefix("user", "admin_"), "admin_user");
        assert_eq!(prefix("admin_user", "admin_"), "admin_user");

        assert_eq!(remove_extension("index.html"), "index");
        assert_eq!(remove_extension("archive.tar.gz"), "archive.tar");
        assert_eq!(remove_extension("path/to/file.ts"), "path/to/file");
        assert_eq!(remove_extension("path.dir/file"), "path.dir/file");
        assert_eq!(remove_extension("no_ext"), "no_ext");
        assert_eq!(remove_extension(""), "");

        assert_eq!(ext("index.html", "ts"), "index.ts");
        assert_eq!(ext("index", ".ts"), "index.ts");
        assert_eq!(ext("index", ""), "index");
        assert_eq!(ext("path/to/file.d.ts", "js"), "path/to/file.d.js");
    }
}
