//! Dot case (`dot.case`) conversions and predicates.

use std::borrow::Cow;

use super::{is_delimited_ascii, transform_cased};

/// Checks if the input slice is already in valid `dot.case` without allocating.
///
/// A valid `dot.case` string is non-empty, contains only lowercase ASCII letters,
/// digits, and single dot (`.`) separators, with no leading or trailing dots.
#[inline]
pub fn is_dot_case(input: &str) -> bool {
    is_delimited_ascii(input, b'.', false)
}

/// Converts a string slice to `dot.case`.
///
/// Returns `Cow::Borrowed` with zero allocations if the input already conforms to `dot.case`.
pub fn to_dot_case(input: &str) -> Cow<'_, str> {
    if is_dot_case(input) {
        return Cow::Borrowed(input);
    }

    transform_cased(input, '.', false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_dot_case() {
        assert!(is_dot_case("user.id"));
        assert!(is_dot_case("foo.bar.baz"));
        assert!(is_dot_case("v1.api.endpoint"));

        assert!(!is_dot_case(""));
        assert!(!is_dot_case(".user.id"));
        assert!(!is_dot_case("user.id."));
        assert!(!is_dot_case("user..id"));
        assert!(!is_dot_case("userId"));
        assert!(!is_dot_case("user_id"));
        assert!(!is_dot_case("USER.ID"));
    }

    #[test]
    fn test_to_dot_case_borrowed() {
        let input = "user.profile.id";
        let result = to_dot_case(input);
        assert!(matches!(result, Cow::Borrowed(_)));
        assert_eq!(result, "user.profile.id");
    }

    #[test]
    fn test_to_dot_case_conversions() {
        assert_eq!(to_dot_case("userId"), "user.id");
        assert_eq!(to_dot_case("UserId"), "user.id");
        assert_eq!(to_dot_case("user_id"), "user.id");
        assert_eq!(to_dot_case("user-id"), "user.id");
        assert_eq!(to_dot_case("HTTPResponse"), "http.response");
        assert_eq!(to_dot_case("..hello..world.."), "hello.world");
        assert_eq!(to_dot_case(""), "");
    }
}
