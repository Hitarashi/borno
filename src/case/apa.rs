//! APA and Chicago style grammatical title casing.

use std::borrow::Cow;

use super::{scanner::split_words, transform::append_cased};

/// Minor English grammatical words that remain in lowercase in APA/Chicago title case style
/// unless they appear as the first or last word.
pub(crate) const APA_MINOR_WORDS: [&str; 22] = [
    "a", "an", "the", "and", "but", "for", "nor", "or", "so", "yet", "as", "at", "by", "in", "of",
    "off", "on", "per", "to", "up", "via", "with",
];

#[inline]
pub(crate) fn is_apa_minor_word(word: &str) -> bool {
    APA_MINOR_WORDS
        .iter()
        .any(|&minor| minor.eq_ignore_ascii_case(word))
}

/// Checks if an input string slice conforms to APA Title Case conventions without allocating.
#[must_use]
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
#[must_use]
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
