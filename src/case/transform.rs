//! Core casing transformation primitives and byte streaming algorithms.

use std::borrow::Cow;

use super::scanner::{for_each_word, for_each_word_ascii, split_words};

#[inline]
pub(crate) fn append_cased(buffer: &mut String, character: char, uppercase: bool) {
    if character.is_ascii() {
        if uppercase {
            buffer.push(character.to_ascii_uppercase());
        } else {
            buffer.push(character.to_ascii_lowercase());
        }
    } else if uppercase {
        for upper_char in character.to_uppercase() {
            buffer.push(upper_char);
        }
    } else {
        for lower_char in character.to_lowercase() {
            buffer.push(lower_char);
        }
    }
}

#[inline]
fn transform_ascii_words<F>(input: &str, capacity_extra: usize, mut format_word: F) -> Cow<'_, str>
where
    F: FnMut(&mut Vec<u8>, &[u8], usize),
{
    let bytes = input.as_bytes();
    let mut buf = Vec::with_capacity(bytes.len() + capacity_extra);
    let mut word_index = 0;
    for_each_word_ascii(bytes, |start, end| {
        format_word(&mut buf, &bytes[start..end], word_index);
        word_index += 1;
    });
    if buf.is_empty() {
        Cow::Borrowed("")
    } else {
        // SAFETY: format_word only emits valid ASCII bytes (a-z, A-Z, 0-9, and ASCII delimiters)
        // into buf, which constitutes valid UTF-8.
        Cow::Owned(unsafe { String::from_utf8_unchecked(buf) })
    }
}

/// Transforms an input string slice into single-character delimited cased words without intermediate vector allocations.
#[inline]
pub(crate) fn transform_cased(input: &str, separator: char, uppercase: bool) -> Cow<'_, str> {
    if input.is_empty() {
        return Cow::Borrowed("");
    }

    if input.is_ascii() && separator.is_ascii() {
        let sep_byte = separator as u8;
        return transform_ascii_words(input, 4, |buf, word, word_index| {
            if word_index > 0 {
                buf.push(sep_byte);
            }
            if uppercase {
                for &b in word {
                    buf.push(b.to_ascii_uppercase());
                }
            } else {
                for &b in word {
                    buf.push(b.to_ascii_lowercase());
                }
            }
        });
    }

    let mut result = String::with_capacity(input.len() + 4);
    let mut word_index = 0;
    for_each_word(input, |start, end| {
        if word_index > 0 {
            result.push(separator);
        }
        for current_char in input[start..end].chars() {
            append_cased(&mut result, current_char, uppercase);
        }
        word_index += 1;
    });

    if result.is_empty() {
        Cow::Borrowed("")
    } else {
        Cow::Owned(result)
    }
}

/// Transforms an input string slice into camelCase or PascalCase without intermediate vector allocations.
#[inline]
pub(crate) fn transform_camel_or_pascal(input: &str, capitalize_first: bool) -> Cow<'_, str> {
    if input.is_empty() {
        return Cow::Borrowed("");
    }

    if input.is_ascii() {
        return transform_ascii_words(input, 0, |buf, word, word_index| {
            let capitalize = (word_index == 0 && capitalize_first) || word_index > 0;
            if capitalize {
                buf.push(word[0].to_ascii_uppercase());
            } else {
                buf.push(word[0].to_ascii_lowercase());
            }
            for &b in &word[1..] {
                buf.push(b.to_ascii_lowercase());
            }
        });
    }

    let mut result = String::with_capacity(input.len());
    let mut word_index = 0;
    for_each_word(input, |start, end| {
        let capitalize = (word_index == 0 && capitalize_first) || word_index > 0;
        let word = &input[start..end];
        for (char_idx, c) in word.chars().enumerate() {
            append_cased(&mut result, c, char_idx == 0 && capitalize);
        }
        word_index += 1;
    });

    if result.is_empty() {
        Cow::Borrowed("")
    } else {
        Cow::Owned(result)
    }
}

/// Transforms an input string slice into capitalized title-separated words without intermediate vector allocations.
#[inline]
pub(crate) fn transform_title(input: &str, separator: char) -> Cow<'_, str> {
    if input.is_empty() {
        return Cow::Borrowed("");
    }

    if input.is_ascii() && separator.is_ascii() {
        let sep_byte = separator as u8;
        return transform_ascii_words(input, 4, |buf, word, word_index| {
            if word_index > 0 {
                buf.push(sep_byte);
            }
            buf.push(word[0].to_ascii_uppercase());
            for &b in &word[1..] {
                buf.push(b.to_ascii_lowercase());
            }
        });
    }

    let mut result = String::with_capacity(input.len() + 4);
    let mut word_index = 0;
    for_each_word(input, |start, end| {
        if word_index > 0 {
            result.push(separator);
        }
        for (char_idx, c) in input[start..end].chars().enumerate() {
            append_cased(&mut result, c, char_idx == 0);
        }
        word_index += 1;
    });

    if result.is_empty() {
        Cow::Borrowed("")
    } else {
        Cow::Owned(result)
    }
}

/// Transforms an input string slice into Sentence case.
#[inline]
pub(crate) fn transform_sentence_case(input: &str) -> Cow<'_, str> {
    let words = split_words(input);
    if words.is_empty() {
        return Cow::Borrowed("");
    }

    let total_separator_bytes = words.len().saturating_sub(1);
    let estimated_capacity = input.len() + total_separator_bytes;
    let mut result = String::with_capacity(estimated_capacity);

    for (index, word) in words.iter().enumerate() {
        if index > 0 {
            result.push(' ');
        }
        for (char_idx, current_char) in word.chars().enumerate() {
            if index == 0 && char_idx == 0 {
                for upper_char in current_char.to_uppercase() {
                    result.push(upper_char);
                }
            } else {
                for lower_char in current_char.to_lowercase() {
                    result.push(lower_char);
                }
            }
        }
    }

    Cow::Owned(result)
}
