//! Title Case (`Title Case`) conversions.

use std::borrow::Cow;

use super::{append_cased, is_delimited_title, join_words_title, split_words};

/// Checks if the input slice is already in valid `Title Case` without allocating.
///
/// A valid `Title Case` string consists of non-empty words separated by single spaces,
/// where each word begins with an uppercase character followed by lowercase characters or digits.
#[inline]
pub fn is_title_case(input: &str) -> bool {
    is_delimited_title(input, ' ')
}

/// Converts a string slice to `Title Case`.
///
/// Returns `Cow::Borrowed` with zero allocations if the input already conforms to `Title Case`.
pub fn to_title_case(input: &str) -> Cow<'_, str> {
    if is_title_case(input) {
        return Cow::Borrowed(input);
    }

    let words = split_words(input);
    if words.is_empty() {
        return Cow::Borrowed("");
    }

    Cow::Owned(join_words_title(&words, ' ', input.len()))
}

/// List of minor English grammatical words (articles, coordinating conjunctions, and short prepositions)
/// that remain in lowercase in APA/Chicago title case style unless at the start or end of the title.
const APA_MINOR_WORDS: [&str; 22] = [
    "a", "an", "the", "and", "but", "for", "nor", "or", "so", "yet", "as", "at", "by", "in", "of",
    "off", "on", "per", "to", "up", "via", "with",
];

/// Checks if a lowercase word is a minor grammatical word in APA style.
#[inline]
fn is_apa_minor_word(word: &str) -> bool {
    APA_MINOR_WORDS
        .iter()
        .any(|&minor| minor.eq_ignore_ascii_case(word))
}

/// Checks if the input slice conforms to APA Title Case conventions without allocating.
pub fn is_title_case_apa(input: &str) -> bool {
    let words = split_words(input);
    if words.is_empty() {
        return false;
    }

    let mut current_offset = 0;
    for (index, &word) in words.iter().enumerate() {
        if index > 0 {
            if current_offset >= input.len() || input.as_bytes()[current_offset] != b' ' {
                return false;
            }
            current_offset += 1;
        }

        let is_boundary = index == 0 || index == words.len() - 1;
        let should_be_lowercase = !is_boundary && is_apa_minor_word(word);

        for (char_idx, character) in word.chars().enumerate() {
            if char_idx == 0 {
                if should_be_lowercase {
                    if !character.is_lowercase() {
                        return false;
                    }
                } else if !character.is_uppercase() {
                    return false;
                }
            } else if !character.is_lowercase() && !character.is_ascii_digit() {
                return false;
            }
            current_offset += character.len_utf8();
        }
    }

    current_offset == input.len()
}

/// Converts a string slice to APA Title Case.
///
/// Follows APA/Chicago grammatical rules: articles (*a, an, the*), coordinating conjunctions
/// (*and, but, for, nor, or, so, yet*), and short prepositions (*in, on, of, to, with, etc.*)
/// remain lowercase unless they appear as the first or last word.
///
/// Returns `Cow::Borrowed` with zero heap allocations if the input already conforms.
///
/// # Examples
/// ```
/// use borno::case::to_title_case_apa;
///
/// assert_eq!(to_title_case_apa("the old man and the sea"), "The Old Man and the Sea");
/// assert_eq!(to_title_case_apa("war and peace"), "War and Peace");
/// assert_eq!(to_title_case_apa("step by step"), "Step by Step");
/// ```
pub fn to_title_case_apa(input: &str) -> Cow<'_, str> {
    if is_title_case_apa(input) {
        return Cow::Borrowed(input);
    }

    let words = split_words(input);
    if words.is_empty() {
        return Cow::Borrowed("");
    }

    let total_separator_bytes = words.len().saturating_sub(1);
    let mut result = String::with_capacity(input.len() + total_separator_bytes);

    for (index, &word) in words.iter().enumerate() {
        if index > 0 {
            result.push(' ');
        }

        let is_boundary = index == 0 || index == words.len() - 1;
        let should_be_lowercase = !is_boundary && is_apa_minor_word(word);

        for (char_idx, character) in word.chars().enumerate() {
            let capitalize = !should_be_lowercase && char_idx == 0;
            append_cased(&mut result, character, capitalize);
        }
    }

    Cow::Owned(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_title_case() {
        assert!(is_title_case("User Profile"));
        assert!(is_title_case("Foo Bar Baz"));
        assert!(is_title_case("Http Response"));

        assert!(!is_title_case(""));
        assert!(!is_title_case("user profile"));
        assert!(!is_title_case("User profile"));
        assert!(!is_title_case(" User Profile"));
        assert!(!is_title_case("User Profile "));
        assert!(!is_title_case("User  Profile"));
        assert!(!is_title_case("USER PROFILE"));
    }

    #[test]
    fn test_to_title_case_borrowed() {
        let input = "User Profile";
        let result = to_title_case(input);
        assert!(matches!(result, Cow::Borrowed(_)));
        assert_eq!(result, "User Profile");
    }

    #[test]
    fn test_to_title_case_conversions() {
        assert_eq!(to_title_case("user_profile"), "User Profile");
        assert_eq!(to_title_case("userProfile"), "User Profile");
        assert_eq!(to_title_case("HTTPResponse"), "Http Response");
        assert_eq!(to_title_case(""), "");
    }

    #[test]
    fn test_title_case_apa() {
        assert_eq!(
            to_title_case_apa("the old man and the sea"),
            "The Old Man and the Sea"
        );
        assert_eq!(to_title_case_apa("war and peace"), "War and Peace");
        assert_eq!(to_title_case_apa("step by step"), "Step by Step");
        assert_eq!(to_title_case_apa(""), "");

        let already_apa = "The Old Man and the Sea";
        let borrowed = to_title_case_apa(already_apa);
        assert!(matches!(borrowed, Cow::Borrowed(_)));

        assert!(is_title_case_apa("War and Peace"));
        assert!(!is_title_case_apa("war and peace"));
        assert!(!is_title_case_apa(""));
    }
}
