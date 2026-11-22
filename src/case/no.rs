//! No Case (`no case`) conversions.

use std::borrow::Cow;

use super::{is_delimited_ascii, join_words_cased, split_words};

/// Checks if the input slice is already in valid `no case` without allocating.
///
/// A valid `no case` string is non-empty, contains only lowercase ASCII letters,
/// digits, and single space separators, with no leading or trailing spaces.
#[inline]
pub fn is_no_case(input: &str) -> bool {
    is_delimited_ascii(input, b' ', false)
}

/// Converts a string slice to `no case`.
///
/// Returns `Cow::Borrowed` with zero allocations if the input already conforms to `no case`.
pub fn to_no_case(input: &str) -> Cow<'_, str> {
    if is_no_case(input) {
        return Cow::Borrowed(input);
    }

    let words = split_words(input);
    if words.is_empty() {
        return Cow::Borrowed("");
    }

    Cow::Owned(join_words_cased(&words, ' ', false, input.len()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_no_case() {
        assert!(is_no_case("hello world"));
        assert!(is_no_case("user account id"));
        assert!(!is_no_case("helloWorld"));
        assert!(!is_no_case("hello_world"));
        assert!(!is_no_case("Hello World"));
        assert!(!is_no_case(""));
        assert!(!is_no_case(" "));
        assert!(!is_no_case(" hello"));
        assert!(!is_no_case("hello "));
    }

    #[test]
    fn test_to_no_case_borrowed() {
        let text = "hello world";
        let result = to_no_case(text);
        assert!(matches!(result, Cow::Borrowed(_)));
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_to_no_case_conversions() {
        assert_eq!(to_no_case("helloWorld"), "hello world");
        assert_eq!(to_no_case("HelloWorld"), "hello world");
        assert_eq!(to_no_case("hello_world"), "hello world");
        assert_eq!(to_no_case("HELLO_WORLD"), "hello world");
        assert_eq!(to_no_case("hello-world"), "hello world");
        assert_eq!(to_no_case(""), "");
    }
}
