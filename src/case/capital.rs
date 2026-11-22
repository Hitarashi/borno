//! Capital Case (`Capital Case`) conversions.

use std::borrow::Cow;

use super::{is_delimited_title, join_words_title, split_words};

/// Checks if the input slice is already in valid `Capital Case` without allocating.
///
/// A valid `Capital Case` string is non-empty, contains words starting with uppercase
/// characters followed by lowercase characters or digits, separated by single spaces.
#[inline]
pub fn is_capital_case(input: &str) -> bool {
    is_delimited_title(input, ' ')
}

/// Converts a string slice to `Capital Case`.
///
/// Returns `Cow::Borrowed` with zero allocations if the input already conforms to `Capital Case`.
pub fn to_capital_case(input: &str) -> Cow<'_, str> {
    if is_capital_case(input) {
        return Cow::Borrowed(input);
    }

    let words = split_words(input);
    if words.is_empty() {
        return Cow::Borrowed("");
    }

    Cow::Owned(join_words_title(&words, ' ', input.len()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_capital_case() {
        assert!(is_capital_case("Hello World"));
        assert!(is_capital_case("User Account Id"));
        assert!(!is_capital_case("hello world"));
        assert!(!is_capital_case("helloWorld"));
        assert!(!is_capital_case("Hello_World"));
        assert!(!is_capital_case(""));
        assert!(!is_capital_case(" "));
    }

    #[test]
    fn test_to_capital_case_borrowed() {
        let text = "Hello World";
        let result = to_capital_case(text);
        assert!(matches!(result, Cow::Borrowed(_)));
        assert_eq!(result, "Hello World");
    }

    #[test]
    fn test_to_capital_case_conversions() {
        assert_eq!(to_capital_case("helloWorld"), "Hello World");
        assert_eq!(to_capital_case("hello_world"), "Hello World");
        assert_eq!(to_capital_case("HELLO_WORLD"), "Hello World");
        assert_eq!(to_capital_case("hello-world"), "Hello World");
        assert_eq!(to_capital_case(""), "");
    }
}
