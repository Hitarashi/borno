//! Whitespace and file path normalization utilities.

use std::borrow::Cow;

/// Strips all leading and trailing whitespace and collapses all internal consecutive
/// whitespace characters (including tabs, newlines, and carriage returns) into single spaces.
///
/// Returns `Cow::Borrowed` with zero allocations if the input is already squished.
///
/// # Examples
/// ```
/// use borno::squish;
///
/// assert_eq!(squish("   Hello    world!   "), "Hello world!");
/// assert_eq!(squish("line 1\n\nline 2\t\t\tend"), "line 1 line 2 end");
/// assert_eq!(squish("already clean"), "already clean");
/// ```
pub fn squish(input: &str) -> Cow<'_, str> {
    if input.is_empty() {
        return Cow::Borrowed("");
    }

    if is_already_squished(input) {
        return Cow::Borrowed(input);
    }

    let mut result = String::with_capacity(input.len());
    let mut in_whitespace = false;

    for character in input.trim().chars() {
        if character.is_whitespace() {
            if !in_whitespace {
                result.push(' ');
                in_whitespace = true;
            }
        } else {
            result.push(character);
            in_whitespace = false;
        }
    }

    Cow::Owned(result)
}

fn is_already_squished(input: &str) -> bool {
    let mut characters = input.chars();
    let first_char = characters.next().unwrap();

    if first_char.is_whitespace() {
        return false;
    }

    let mut previous_char = first_char;
    let mut last_char = first_char;

    for current_char in characters {
        if current_char.is_whitespace() && (current_char != ' ' || previous_char == ' ') {
            return false;
        }
        previous_char = current_char;
        last_char = current_char;
    }

    !last_char.is_whitespace()
}

/// Normalizes Windows backslashes (`\`) into standard forward slashes (`/`).
///
/// Preserves Windows extended-length paths (beginning with `\\?\`).
/// Returns `Cow::Borrowed` with zero allocations if the input contains no backslashes.
///
/// # Examples
/// ```
/// use borno::normalize_slashes;
///
/// assert_eq!(normalize_slashes("foo\\bar\\baz"), "foo/bar/baz");
/// assert_eq!(normalize_slashes("foo/bar/baz"), "foo/bar/baz");
/// assert_eq!(normalize_slashes("\\\\?\\C:\\very\\long\\path"), "\\\\?\\C:\\very\\long\\path");
/// ```
pub fn normalize_slashes(path: &str) -> Cow<'_, str> {
    if path.starts_with(r"\\?\") {
        return Cow::Borrowed(path);
    }

    if !path.contains('\\') {
        return Cow::Borrowed(path);
    }

    Cow::Owned(path.replace('\\', "/"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_squish_conversions() {
        assert_eq!(squish("   hello   world   "), "hello world");
        assert_eq!(squish("hello\t\tworld\n\nagain"), "hello world again");
        assert_eq!(squish("   "), "");
        assert_eq!(squish(""), "");
        assert_eq!(squish("single"), "single");
    }

    #[test]
    fn test_squish_zero_allocation() {
        let input = "hello world of text";
        let result = squish(input);
        assert!(matches!(result, Cow::Borrowed(_)));
        assert_eq!(result, "hello world of text");
    }

    #[test]
    fn test_normalize_slashes() {
        assert_eq!(normalize_slashes("foo\\bar\\baz"), "foo/bar/baz");
        assert_eq!(
            normalize_slashes("already/normalized/path"),
            "already/normalized/path"
        );
        assert_eq!(
            normalize_slashes(r"\\?\C:\Windows\System32"),
            r"\\?\C:\Windows\System32"
        );
        assert_eq!(normalize_slashes(""), "");
    }

    #[test]
    fn test_normalize_slashes_zero_allocation() {
        let input = "clean/unix/path";
        let result = normalize_slashes(input);
        assert!(matches!(result, Cow::Borrowed(_)));
        assert_eq!(result, "clean/unix/path");
    }
}
