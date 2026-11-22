//! Zero-allocation casing validation predicates.

/// Validates whether a string slice matches single-separator delimited ASCII casing.
#[inline]
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
pub(crate) fn is_camel_or_pascal(input: &str, require_uppercase_first: bool) -> bool {
    if input.is_empty() {
        return false;
    }

    if input.is_ascii() {
        let bytes = input.as_bytes();
        let first = bytes[0];
        let first_valid = if require_uppercase_first {
            first.is_ascii_uppercase()
        } else {
            first.is_ascii_lowercase()
        };
        if !first_valid {
            return false;
        }

        let mut prev = first;
        for &b in &bytes[1..] {
            if !b.is_ascii_alphanumeric() {
                return false;
            }
            if prev.is_ascii_uppercase() && b.is_ascii_uppercase() {
                return false;
            }
            prev = b;
        }
        return true;
    }

    let mut characters = input.chars();
    let first_char = characters.next().unwrap();

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

/// Validates whether a string slice conforms to sentence case without allocating.
#[inline]
pub(crate) fn is_sentence_case_internal(input: &str) -> bool {
    let mut characters = input.chars();
    let Some(first_char) = characters.next() else {
        return false;
    };

    if !first_char.is_uppercase() || input.ends_with(' ') {
        return false;
    }

    let mut previous_was_space = false;
    for current_char in characters {
        if current_char == ' ' {
            if previous_was_space {
                return false;
            }
            previous_was_space = true;
        } else if current_char.is_lowercase() || current_char.is_numeric() {
            previous_was_space = false;
        } else {
            return false;
        }
    }

    !previous_was_space
}
