//! Substring slicing and boundary extraction utilities.

use std::borrow::Cow;

/// Returns the portion of a string before the first occurrence of a delimiter.
///
/// If the delimiter is empty or not found in the input, the entire input is returned.
///
/// # Examples
/// ```
/// use borno::before;
///
/// assert_eq!(before("user_profile_id", "_"), "user");
/// assert_eq!(before("user.account.v1", "."), "user");
/// assert_eq!(before("user", "_"), "user");
/// assert_eq!(before("user", ""), "user");
/// ```
#[inline]
pub fn before<'a>(input: &'a str, delimiter: &str) -> Cow<'a, str> {
    if delimiter.is_empty() {
        return Cow::Borrowed(input);
    }
    match input.find(delimiter) {
        Some(index) => Cow::Borrowed(&input[..index]),
        None => Cow::Borrowed(input),
    }
}

/// Returns the portion of a string before the last occurrence of a delimiter.
///
/// If the delimiter is empty or not found in the input, the entire input is returned.
///
/// # Examples
/// ```
/// use borno::before_last;
///
/// assert_eq!(before_last("user_profile_id", "_"), "user_profile");
/// assert_eq!(before_last("v1.users.show", "."), "v1.users");
/// assert_eq!(before_last("user", "_"), "user");
/// assert_eq!(before_last("user", ""), "user");
/// ```
#[inline]
pub fn before_last<'a>(input: &'a str, delimiter: &str) -> Cow<'a, str> {
    if delimiter.is_empty() {
        return Cow::Borrowed(input);
    }
    match input.rfind(delimiter) {
        Some(index) => Cow::Borrowed(&input[..index]),
        None => Cow::Borrowed(input),
    }
}

/// Returns the portion of a string after the first occurrence of a delimiter.
///
/// If the delimiter is empty, the entire input is returned.
/// If the delimiter is not found in the input, an empty string is returned.
///
/// # Examples
/// ```
/// use borno::after;
///
/// assert_eq!(after("user_profile_id", "_"), "profile_id");
/// assert_eq!(after("v1.users.show", "."), "users.show");
/// assert_eq!(after("user", "_"), "");
/// assert_eq!(after("user", ""), "user");
/// ```
#[inline]
pub fn after<'a>(input: &'a str, delimiter: &str) -> Cow<'a, str> {
    if delimiter.is_empty() {
        return Cow::Borrowed(input);
    }
    match input.find(delimiter) {
        Some(index) => Cow::Borrowed(&input[index + delimiter.len()..]),
        None => Cow::Borrowed(""),
    }
}

/// Returns the portion of a string after the last occurrence of a delimiter.
///
/// If the delimiter is empty, the entire input is returned.
/// If the delimiter is not found in the input, an empty string is returned.
///
/// # Examples
/// ```
/// use borno::after_last;
///
/// assert_eq!(after_last("user_profile_id", "_"), "id");
/// assert_eq!(after_last("v1.users.show", "."), "show");
/// assert_eq!(after_last("user", "_"), "");
/// assert_eq!(after_last("user", ""), "user");
/// ```
#[inline]
pub fn after_last<'a>(input: &'a str, delimiter: &str) -> Cow<'a, str> {
    if delimiter.is_empty() {
        return Cow::Borrowed(input);
    }
    match input.rfind(delimiter) {
        Some(index) => Cow::Borrowed(&input[index + delimiter.len()..]),
        None => Cow::Borrowed(""),
    }
}

/// Returns the portion of a string between the first occurrence of two delimiters.
///
/// Returns an empty string if either delimiter is empty, missing, or if the end
/// delimiter does not occur after the start delimiter.
///
/// # Examples
/// ```
/// use borno::between;
///
/// assert_eq!(between("[user_id]", "[", "]"), "user_id");
/// assert_eq!(between("name: 'Alice';", "'", "'"), "Alice");
/// assert_eq!(between("no markers here", "[", "]"), "");
/// ```
#[inline]
pub fn between<'a>(input: &'a str, start_delimiter: &str, end_delimiter: &str) -> Cow<'a, str> {
    if start_delimiter.is_empty() || end_delimiter.is_empty() {
        return Cow::Borrowed("");
    }

    let Some(start_index) = input.find(start_delimiter) else {
        return Cow::Borrowed("");
    };

    let remainder_start = start_index + start_delimiter.len();
    let remainder = &input[remainder_start..];

    let Some(end_index) = remainder.find(end_delimiter) else {
        return Cow::Borrowed("");
    };

    Cow::Borrowed(&remainder[..end_index])
}

/// Alias for [`between`].
#[inline]
pub fn between_first<'a>(
    input: &'a str,
    start_delimiter: &str,
    end_delimiter: &str,
) -> Cow<'a, str> {
    between(input, start_delimiter, end_delimiter)
}

/// Removes a prefix from the start of a string slice if present.
///
/// If `needle` is empty or not found at the start, the original slice is returned unchanged.
/// This operation is completely zero-allocation.
///
/// # Examples
/// ```
/// use borno::chop_start;
///
/// assert_eq!(chop_start("https://example.com", "https://"), "example.com");
/// assert_eq!(chop_start("example.com", "https://"), "example.com");
/// assert_eq!(chop_start("hello", ""), "hello");
/// ```
#[inline]
#[must_use]
pub fn chop_start<'a>(input: &'a str, needle: &str) -> &'a str {
    if needle.is_empty() {
        input
    } else {
        input.strip_prefix(needle).unwrap_or(input)
    }
}

/// Removes a suffix from the end of a string slice if present.
///
/// If `needle` is empty or not found at the end, the original slice is returned unchanged.
/// This operation is completely zero-allocation.
///
/// # Examples
/// ```
/// use borno::chop_end;
///
/// assert_eq!(chop_end("filename.tar.gz", ".gz"), "filename.tar");
/// assert_eq!(chop_end("filename.tar", ".gz"), "filename.tar");
/// assert_eq!(chop_end("hello", ""), "hello");
/// ```
#[inline]
#[must_use]
pub fn chop_end<'a>(input: &'a str, needle: &str) -> &'a str {
    if needle.is_empty() {
        input
    } else {
        input.strip_suffix(needle).unwrap_or(input)
    }
}

/// Checks whether a haystack contains all of the specified needles.
///
/// Returns `true` if `needles` is empty.
///
/// # Examples
/// ```
/// use borno::contains_all;
///
/// assert!(contains_all("the quick brown fox", &["quick", "fox"]));
/// assert!(!contains_all("the quick brown fox", &["quick", "dog"]));
/// assert!(contains_all("anything", &[]));
/// ```
#[inline]
#[must_use]
pub fn contains_all(haystack: &str, needles: &[&str]) -> bool {
    needles.iter().all(|needle| haystack.contains(needle))
}

/// Checks whether a haystack contains any of the specified needles.
///
/// Returns `false` if `needles` is empty.
///
/// # Examples
/// ```
/// use borno::contains_any;
///
/// assert!(contains_any("the quick brown fox", &["cat", "dog", "fox"]));
/// assert!(!contains_any("the quick brown fox", &["cat", "dog"]));
/// assert!(!contains_any("anything", &[]));
/// ```
#[inline]
#[must_use]
pub fn contains_any(haystack: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| haystack.contains(needle))
}

/// Checks whether a string slice is empty or contains only Unicode whitespace scalar values.
///
/// Operates with zero heap allocations.
///
/// # Examples
/// ```
/// use borno::is_empty_or_whitespace;
///
/// assert!(is_empty_or_whitespace(""));
/// assert!(is_empty_or_whitespace("   \t\n\r  "));
/// assert!(!is_empty_or_whitespace("  hello  "));
/// ```
#[inline]
#[must_use]
pub fn is_empty_or_whitespace(input: &str) -> bool {
    input.trim().is_empty()
}

/// Checks whether a string slice starts with any of the specified needles.
///
/// Returns `false` if `needles` is empty.
///
/// # Examples
/// ```
/// use borno::starts_with_any;
///
/// assert!(starts_with_any("https://example.com", &["https://", "sftp://"]));
/// assert!(!starts_with_any("ssh://example.com", &["https://", "sftp://"]));
/// assert!(!starts_with_any("example.com", &[]));
/// ```
#[inline]
#[must_use]
pub fn starts_with_any(input: &str, needles: &[&str]) -> bool {
    needles.iter().any(|&needle| input.starts_with(needle))
}

/// Checks whether a string slice ends with any of the specified needles.
///
/// Returns `false` if `needles` is empty.
///
/// # Examples
/// ```
/// use borno::ends_with_any;
///
/// assert!(ends_with_any("archive.tar.gz", &[".tar", ".gz", ".zip"]));
/// assert!(!ends_with_any("document.pdf", &[".tar", ".gz", ".zip"]));
/// assert!(!ends_with_any("document.pdf", &[]));
/// ```
#[inline]
#[must_use]
pub fn ends_with_any(input: &str, needles: &[&str]) -> bool {
    needles.iter().any(|&needle| input.ends_with(needle))
}

/// Replaces the first occurrence of a substring.
///
/// If `from` is not found or is empty, returns the original string.
///
/// # Examples
/// ```
/// use borno::replace_first;
///
/// assert_eq!(replace_first("foo_bar_foo", "foo", "qux"), "qux_bar_foo");
/// assert_eq!(replace_first("hello world", "missing", "qux"), "hello world");
/// ```
#[must_use]
pub fn replace_first(input: &str, from: &str, to: &str) -> String {
    if from.is_empty() {
        return input.to_string();
    }
    if let Some(idx) = input.find(from) {
        let mut result = String::with_capacity(input.len().saturating_sub(from.len()) + to.len());
        result.push_str(&input[..idx]);
        result.push_str(to);
        result.push_str(&input[idx + from.len()..]);
        result
    } else {
        input.to_string()
    }
}

/// Replaces the last occurrence of a substring.
///
/// If `from` is not found or is empty, returns the original string.
///
/// # Examples
/// ```
/// use borno::replace_last;
///
/// assert_eq!(replace_last("foo_bar_foo", "foo", "qux"), "foo_bar_qux");
/// assert_eq!(replace_last("hello world", "missing", "qux"), "hello world");
/// ```
#[must_use]
pub fn replace_last(input: &str, from: &str, to: &str) -> String {
    if from.is_empty() {
        return input.to_string();
    }
    if let Some(idx) = input.rfind(from) {
        let mut result = String::with_capacity(input.len().saturating_sub(from.len()) + to.len());
        result.push_str(&input[..idx]);
        result.push_str(to);
        result.push_str(&input[idx + from.len()..]);
        result
    } else {
        input.to_string()
    }
}

/// Removes all occurrences of the specified needle from a string slice.
///
/// # Examples
/// ```
/// use borno::remove;
///
/// assert_eq!(remove("foo_bar_baz", "_"), "foobarbaz");
/// assert_eq!(remove("hello world", "o"), "hell wrld");
/// assert_eq!(remove("test", ""), "test");
/// ```
#[must_use]
pub fn remove(input: &str, needle: &str) -> String {
    if needle.is_empty() {
        return input.to_string();
    }
    input.replace(needle, "")
}

/// Deduplicates consecutive occurrences of a character in a string slice.
///
/// Returns `Cow::Borrowed` with zero allocations if the input has no consecutive duplicates.
///
/// # Examples
/// ```
/// use borno::deduplicate;
///
/// assert_eq!(deduplicate("hello///world//test", '/'), "hello/world/test");
/// assert_eq!(deduplicate("already/clean", '/'), "already/clean");
/// ```
#[must_use]
pub fn deduplicate(input: &str, character: char) -> Cow<'_, str> {
    let mut chars = input.chars();
    let mut has_duplicate = false;
    if let Some(mut prev) = chars.next() {
        for next in chars {
            if prev == character && next == character {
                has_duplicate = true;
                break;
            }
            prev = next;
        }
    }

    if !has_duplicate {
        return Cow::Borrowed(input);
    }

    let mut result = String::with_capacity(input.len());
    let mut previous_was_char = false;
    for c in input.chars() {
        if c == character {
            if !previous_was_char {
                result.push(c);
                previous_was_char = true;
            }
        } else {
            result.push(c);
            previous_was_char = false;
        }
    }
    Cow::Owned(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_before_and_before_last() {
        assert_eq!(before("this_is_a_test", "_"), "this");
        assert_eq!(before("singleword", "_"), "singleword");
        assert_eq!(before("singleword", ""), "singleword");
        assert_eq!(before("", "_"), "");
        assert_eq!(before("_leading", "_"), "");

        assert_eq!(before_last("this_is_a_test", "_"), "this_is_a");
        assert_eq!(before_last("singleword", "_"), "singleword");
        assert_eq!(before_last("singleword", ""), "singleword");
        assert_eq!(before_last("", "_"), "");
        assert_eq!(before_last("trailing_", "_"), "trailing");
    }

    #[test]
    fn test_after_and_after_last() {
        assert_eq!(after("this_is_a_test", "_"), "is_a_test");
        assert_eq!(after("singleword", "_"), "");
        assert_eq!(after("singleword", ""), "singleword");
        assert_eq!(after("", "_"), "");
        assert_eq!(after("trailing_", "_"), "");

        assert_eq!(after_last("this_is_a_test", "_"), "test");
        assert_eq!(after_last("singleword", "_"), "");
        assert_eq!(after_last("singleword", ""), "singleword");
        assert_eq!(after_last("", "_"), "");
        assert_eq!(after_last("_leading", "_"), "leading");
    }

    #[test]
    fn test_between() {
        assert_eq!(between("prefix[content]suffix", "[", "]"), "content");
        assert_eq!(between("name='Alice' and age='30'", "'", "'"), "Alice");
        assert_eq!(between("missing end [content", "[", "]"), "");
        assert_eq!(between("missing start content]", "[", "]"), "");
        assert_eq!(between("no delimiters", "[", "]"), "");
        assert_eq!(between("content", "", "]"), "");
        assert_eq!(between("content", "[", ""), "");
        assert_eq!(between_first("prefix[content]suffix", "[", "]"), "content");
    }

    #[test]
    fn test_chop_and_predicates() {
        assert_eq!(chop_start("https://example.com", "https://"), "example.com");
        assert_eq!(chop_start("example.com", "https://"), "example.com");
        assert_eq!(chop_start("example.com", ""), "example.com");

        assert_eq!(chop_end("filename.tar.gz", ".gz"), "filename.tar");
        assert_eq!(chop_end("filename.tar", ".gz"), "filename.tar");
        assert_eq!(chop_end("filename.tar", ""), "filename.tar");

        assert!(contains_all("apple banana cherry", &["apple", "cherry"]));
        assert!(!contains_all("apple banana cherry", &["apple", "date"]));
        assert!(contains_all("anything", &[]));

        assert!(contains_any("apple banana cherry", &["date", "banana"]));
        assert!(!contains_any("apple banana cherry", &["date", "fig"]));
        assert!(!contains_any("anything", &[]));

        assert!(is_empty_or_whitespace(""));
        assert!(is_empty_or_whitespace("   \r\n\t "));
        assert!(!is_empty_or_whitespace("   a   "));
    }

    #[test]
    fn test_slice_advanced_utilities() {
        assert!(starts_with_any(
            "https://example.com",
            &["https://", "sftp://"]
        ));
        assert!(!starts_with_any(
            "ssh://example.com",
            &["https://", "sftp://"]
        ));
        assert!(!starts_with_any("https://example.com", &[]));

        assert!(ends_with_any("archive.tar.gz", &[".tar", ".gz", ".zip"]));
        assert!(!ends_with_any("document.pdf", &[".tar", ".gz", ".zip"]));
        assert!(!ends_with_any("archive.tar.gz", &[]));

        assert_eq!(replace_first("foo_bar_foo", "foo", "qux"), "qux_bar_foo");
        assert_eq!(
            replace_first("foo_bar_foo", "missing", "qux"),
            "foo_bar_foo"
        );
        assert_eq!(replace_first("foo_bar_foo", "", "qux"), "foo_bar_foo");

        assert_eq!(replace_last("foo_bar_foo", "foo", "qux"), "foo_bar_qux");
        assert_eq!(replace_last("foo_bar_foo", "missing", "qux"), "foo_bar_foo");
        assert_eq!(replace_last("foo_bar_foo", "", "qux"), "foo_bar_foo");

        assert_eq!(remove("foo_bar_baz", "_"), "foobarbaz");
        assert_eq!(remove("foo_bar_baz", ""), "foo_bar_baz");

        assert_eq!(deduplicate("hello///world//test", '/'), "hello/world/test");
        assert_eq!(deduplicate("already/clean", '/'), "already/clean");
        assert!(matches!(
            deduplicate("already/clean", '/'),
            Cow::Borrowed(_)
        ));
    }
}
