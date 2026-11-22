//! Casing transformation engine, predicates, and framework naming conventions.

#[cfg(feature = "case-camel")]
pub mod camel;
#[cfg(feature = "conventions")]
pub mod conventions;
#[cfg(feature = "case-dot")]
pub mod dot;
#[cfg(feature = "case-kebab")]
pub mod kebab;
#[cfg(feature = "case-pascal")]
pub mod pascal;
#[cfg(feature = "case-sentence")]
pub mod sentence;
#[cfg(any(feature = "case-snake", feature = "case-screaming-snake"))]
pub mod snake;
#[cfg(feature = "case-title")]
pub mod title;
#[cfg(feature = "case-train")]
pub mod train;

#[cfg(feature = "case-camel")]
pub use camel::{is_camel_case, to_camel_case};
#[cfg(feature = "conventions")]
pub use conventions::{
    classify, deconstantize, demodulize, foreign_key, foreign_key_with_suffix, headline, humanize,
    is_foreign_key, is_foreign_key_with_suffix, tableize,
};
#[cfg(feature = "case-dot")]
pub use dot::{is_dot_case, to_dot_case};
#[cfg(feature = "case-kebab")]
pub use kebab::{is_kebab_case, to_kebab_case};
#[cfg(feature = "case-pascal")]
pub use pascal::{is_pascal_case, to_pascal_case};
#[cfg(feature = "case-sentence")]
pub use sentence::{is_sentence_case, to_sentence_case};
#[cfg(feature = "case-screaming-snake")]
pub use snake::{is_screaming_snake_case, to_screaming_snake_case};
#[cfg(feature = "case-snake")]
pub use snake::{is_snake_case, to_snake_case};
#[cfg(feature = "case-title")]
pub use title::{is_title_case, to_title_case};
#[cfg(feature = "case-train")]
pub use train::{is_train_case, to_train_case};

/// Checks if a character acts as a word separator.
#[inline]
#[cfg(any(
    feature = "case-camel",
    feature = "case-dot",
    feature = "case-kebab",
    feature = "case-pascal",
    feature = "case-sentence",
    feature = "case-snake",
    feature = "case-screaming-snake",
    feature = "case-title",
    feature = "case-train",
    feature = "conventions",
    feature = "grammar"
))]
pub(crate) fn is_separator(character: char) -> bool {
    character.is_whitespace()
        || character == '_'
        || character == '-'
        || character == '.'
        || character == '/'
        || character == '\\'
        || character == ':'
}

/// Splits a string slice into word slices by detecting separators,
/// camelCase transitions, and consecutive uppercase acronym boundaries.
#[cfg(any(
    feature = "case-camel",
    feature = "case-dot",
    feature = "case-kebab",
    feature = "case-pascal",
    feature = "case-sentence",
    feature = "case-snake",
    feature = "case-screaming-snake",
    feature = "case-title",
    feature = "case-train",
    feature = "conventions",
    feature = "grammar"
))]
pub(crate) fn split_words(input: &str) -> Vec<&str> {
    if input.is_empty() {
        return Vec::new();
    }

    let character_indices: Vec<(usize, char)> = input.char_indices().collect();
    let total_characters = character_indices.len();
    let mut words = Vec::new();
    let mut word_start: Option<usize> = None;

    for index in 0..total_characters {
        let (current_offset, current_char) = character_indices[index];

        if is_separator(current_char) {
            if let Some(start_offset) = word_start.take() {
                words.push(&input[start_offset..current_offset]);
            }
            continue;
        }

        match word_start {
            None => {
                word_start = Some(current_offset);
            }
            Some(start_offset) => {
                let (_, previous_char) = character_indices[index - 1];

                let is_camel_boundary = previous_char.is_lowercase() && current_char.is_uppercase();

                let is_acronym_boundary = if previous_char.is_uppercase()
                    && current_char.is_uppercase()
                    && index + 1 < total_characters
                {
                    let (_, next_char) = character_indices[index + 1];
                    next_char.is_lowercase()
                } else {
                    false
                };

                let is_number_boundary = (previous_char.is_alphabetic()
                    && current_char.is_ascii_digit())
                    || (previous_char.is_ascii_digit() && current_char.is_alphabetic());

                if is_camel_boundary || is_acronym_boundary || is_number_boundary {
                    words.push(&input[start_offset..current_offset]);
                    word_start = Some(current_offset);
                }
            }
        }
    }

    if let Some(start_offset) = word_start {
        words.push(&input[start_offset..input.len()]);
    }

    words
}

/// Validates whether a string slice matches single-separator delimited ASCII casing.
#[inline]
#[cfg(any(
    feature = "case-snake",
    feature = "case-screaming-snake",
    feature = "case-kebab",
    feature = "case-dot"
))]
pub(crate) fn is_delimited_ascii(input: &str, separator: u8, require_uppercase: bool) -> bool {
    if input.is_empty() {
        return false;
    }
    let bytes = input.as_bytes();
    if bytes[0] == separator || bytes[bytes.len() - 1] == separator {
        return false;
    }

    let mut previous_was_separator = false;
    for &byte in bytes {
        if byte == separator {
            if previous_was_separator {
                return false;
            }
            previous_was_separator = true;
        } else {
            let valid = if require_uppercase {
                byte.is_ascii_uppercase() || byte.is_ascii_digit()
            } else {
                byte.is_ascii_lowercase() || byte.is_ascii_digit()
            };
            if !valid {
                return false;
            }
            previous_was_separator = false;
        }
    }
    true
}

/// Validates whether a string slice matches delimited title casing.
#[inline]
#[cfg(any(feature = "case-title", feature = "case-train"))]
pub(crate) fn is_delimited_title(input: &str, separator: char) -> bool {
    if input.is_empty() || input.starts_with(separator) || input.ends_with(separator) {
        return false;
    }

    let mut new_word = true;
    for current_char in input.chars() {
        if current_char == separator {
            if new_word {
                return false;
            }
            new_word = true;
        } else if new_word {
            if !current_char.is_uppercase() {
                return false;
            }
            new_word = false;
        } else if !current_char.is_lowercase() && !current_char.is_numeric() {
            return false;
        }
    }
    !new_word
}

/// Validates whether a string slice conforms to camelCase or PascalCase.
#[inline]
#[cfg(any(feature = "case-camel", feature = "case-pascal"))]
pub(crate) fn is_camel_or_pascal(input: &str, require_uppercase_first: bool) -> bool {
    let mut characters = input.chars();
    let Some(first_char) = characters.next() else {
        return false;
    };

    let first_valid = if require_uppercase_first {
        first_char.is_uppercase()
    } else {
        first_char.is_lowercase()
    };
    if !first_valid {
        return false;
    }

    let mut previous_char = first_char;
    for current_char in characters {
        if !current_char.is_alphanumeric() {
            return false;
        }
        if previous_char.is_uppercase() && current_char.is_uppercase() {
            return false;
        }
        previous_char = current_char;
    }
    true
}

#[inline]
#[cfg(any(
    feature = "case-camel",
    feature = "case-pascal",
    feature = "case-dot",
    feature = "case-kebab",
    feature = "case-snake",
    feature = "case-screaming-snake",
    feature = "case-title",
    feature = "case-train"
))]
fn append_cased(buffer: &mut String, character: char, uppercase: bool) {
    if uppercase {
        for upper_char in character.to_uppercase() {
            buffer.push(upper_char);
        }
    } else {
        for lower_char in character.to_lowercase() {
            buffer.push(lower_char);
        }
    }
}

/// Joins word slices with a single separator character in either all-lowercase or all-uppercase.
#[cfg(any(
    feature = "case-dot",
    feature = "case-kebab",
    feature = "case-snake",
    feature = "case-screaming-snake"
))]
pub(crate) fn join_words_cased(
    words: &[&str],
    separator: char,
    uppercase: bool,
    original_len: usize,
) -> String {
    let total_separator_bytes = words.len().saturating_sub(1) * separator.len_utf8();
    let mut result = String::with_capacity(original_len + total_separator_bytes);
    for (index, word) in words.iter().enumerate() {
        if index > 0 {
            result.push(separator);
        }
        for current_char in word.chars() {
            append_cased(&mut result, current_char, uppercase);
        }
    }
    result
}

/// Joins word slices where each word is capitalized, separated by a delimiter.
#[cfg(any(feature = "case-title", feature = "case-train"))]
pub(crate) fn join_words_title(words: &[&str], separator: char, original_len: usize) -> String {
    let total_separator_bytes = words.len().saturating_sub(1) * separator.len_utf8();
    let mut result = String::with_capacity(original_len + total_separator_bytes);
    for (index, word) in words.iter().enumerate() {
        if index > 0 {
            result.push(separator);
        }
        for (char_idx, current_char) in word.chars().enumerate() {
            append_cased(&mut result, current_char, char_idx == 0);
        }
    }
    result
}

/// Joins word slices into camelCase or PascalCase.
#[cfg(any(feature = "case-camel", feature = "case-pascal"))]
pub(crate) fn join_words_camel_or_pascal(
    words: &[&str],
    capitalize_first: bool,
    original_len: usize,
) -> String {
    let mut result = String::with_capacity(original_len);
    for (index, word) in words.iter().enumerate() {
        for (char_idx, current_char) in word.chars().enumerate() {
            let should_capitalize =
                (index == 0 && capitalize_first && char_idx == 0) || (index > 0 && char_idx == 0);
            append_cased(&mut result, current_char, should_capitalize);
        }
    }
    result
}
