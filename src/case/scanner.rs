//! Word boundary scanning and compound identifier token extraction.

/// Scans ASCII byte slices for word boundaries with zero allocations.
#[inline]
pub(crate) fn for_each_word_ascii<F>(bytes: &[u8], mut callback: F)
where
    F: FnMut(usize, usize),
{
    #[inline]
    fn is_sep(b: u8) -> bool {
        b.is_ascii_whitespace() || matches!(b, b'_' | b'-' | b'.' | b'/' | b'\\' | b':')
    }

    let len = bytes.len();
    let mut start = usize::MAX;
    let mut i = 0;

    while i < len {
        let b = bytes[i];
        if is_sep(b) {
            if start != usize::MAX {
                callback(start, i);
                start = usize::MAX;
            }
            i += 1;
            continue;
        }

        if start != usize::MAX {
            let prev = bytes[i - 1];
            let is_boundary = if prev.is_ascii_lowercase() {
                b.is_ascii_uppercase() || b.is_ascii_digit()
            } else if prev.is_ascii_uppercase() {
                b.is_ascii_digit()
                    || (b.is_ascii_uppercase() && i + 1 < len && bytes[i + 1].is_ascii_lowercase())
            } else if prev.is_ascii_digit() {
                b.is_ascii_alphabetic()
            } else {
                false
            };

            if is_boundary {
                callback(start, i);
                start = i;
            }
        } else {
            start = i;
        }
        i += 1;
    }

    if start != usize::MAX {
        callback(start, len);
    }
}

/// Splits a string slice into word slices by detecting separators,
/// camelCase transitions, and consecutive uppercase acronym boundaries,
/// invoking a callback for each word with zero intermediate heap allocations.
#[inline]
pub(crate) fn for_each_word<F>(input: &str, mut callback: F)
where
    F: FnMut(usize, usize),
{
    #[inline]
    fn is_separator(character: char) -> bool {
        character.is_whitespace() || matches!(character, '_' | '-' | '.' | '/' | '\\' | ':')
    }

    if input.is_empty() {
        return;
    }

    if input.is_ascii() {
        let bytes = input.as_bytes();
        for_each_word_ascii(bytes, callback);
        return;
    }

    let mut char_indices = input.char_indices().peekable();
    let mut word_start: usize = 0;
    let mut in_word = false;
    let mut previous_char: Option<char> = None;

    while let Some((current_offset, current_char)) = char_indices.next() {
        if is_separator(current_char) {
            if in_word {
                callback(word_start, current_offset);
                in_word = false;
            }
            previous_char = None;
            continue;
        }

        if let Some(prev) = previous_char {
            let is_camel_boundary = prev.is_lowercase() && current_char.is_uppercase();

            let is_acronym_boundary = if prev.is_uppercase() && current_char.is_uppercase() {
                char_indices
                    .peek()
                    .is_some_and(|&(_, next_char)| next_char.is_lowercase())
            } else {
                false
            };

            let is_number_boundary = (prev.is_alphabetic() && current_char.is_ascii_digit())
                || (prev.is_ascii_digit() && current_char.is_alphabetic());

            if is_camel_boundary || is_acronym_boundary || is_number_boundary {
                callback(word_start, current_offset);
                word_start = current_offset;
            }
        } else {
            word_start = current_offset;
            in_word = true;
        }
        previous_char = Some(current_char);
    }

    if in_word {
        callback(word_start, input.len());
    }
}

/// Splits a string slice into word slices by detecting separators,
/// camelCase transitions, and consecutive uppercase acronym boundaries.
#[must_use]
pub(crate) fn split_words(input: &str) -> Vec<&str> {
    let mut words = Vec::new();
    for_each_word(input, |start, end| words.push(&input[start..end]));
    words
}

/// Scans backwards to find the last word boundary of a compound identifier with zero allocations.
///
/// Returns `Some((prefix, last_token))` if a delimiter or camelCase/PascalCase boundary was found.
/// Returns `None` if the input is a single monolithic word or empty.
#[must_use]
pub(crate) fn split_compound_tail(input: &str) -> Option<(&str, &str)> {
    if input.is_empty() {
        return None;
    }

    // 1. Delimited compounds (e.g. "user_profile", "user-profile", "user profile")
    if let Some(sep_idx) = input.rfind(['_', '-', ' ', '.', '/', '\\', ':']) {
        let prefix = &input[..=sep_idx];
        let tail = &input[sep_idx + 1..];
        if !tail.is_empty() {
            return Some((prefix, tail));
        }
    }

    // 2. CamelCase / PascalCase compounds (e.g. "userProfile", "UserProfile", "HTTPResponse")
    let bytes = input.as_bytes();
    let len = bytes.len();
    if len <= 1 {
        return None;
    }

    // ASCII fast-path
    if input.is_ascii() {
        let mut idx = len - 1;
        let mut seen_lower_in_tail = bytes[idx].is_ascii_lowercase();

        while idx > 0 {
            let curr = bytes[idx];
            let prev = bytes[idx - 1];

            // Boundary: lower -> UPPER (e.g. userProfile -> 'P')
            if prev.is_ascii_lowercase() && curr.is_ascii_uppercase() {
                return Some((&input[..idx], &input[idx..]));
            }

            // Boundary: acronym -> Word (e.g. HTTPResponse -> 'R' followed by lower 'e')
            if prev.is_ascii_uppercase() && curr.is_ascii_uppercase() && seen_lower_in_tail {
                return Some((&input[..idx], &input[idx..]));
            }

            // Boundary: digit <-> letter
            if (prev.is_ascii_digit() && curr.is_ascii_alphabetic())
                || (prev.is_ascii_alphabetic() && curr.is_ascii_digit())
            {
                return Some((&input[..idx], &input[idx..]));
            }

            if curr.is_ascii_lowercase() {
                seen_lower_in_tail = true;
            }
            idx -= 1;
        }

        return None;
    }

    // Unicode fallback
    let char_indices: Vec<(usize, char)> = input.char_indices().collect();
    if char_indices.len() <= 1 {
        return None;
    }

    let mut i = char_indices.len() - 1;
    let mut seen_lower = char_indices[i].1.is_lowercase();

    while i > 0 {
        let (curr_idx, curr_char) = char_indices[i];
        let (_, prev_char) = char_indices[i - 1];

        if prev_char.is_lowercase() && curr_char.is_uppercase() {
            return Some((&input[..curr_idx], &input[curr_idx..]));
        }

        if prev_char.is_uppercase() && curr_char.is_uppercase() && seen_lower {
            return Some((&input[..curr_idx], &input[curr_idx..]));
        }

        if (prev_char.is_numeric() && curr_char.is_alphabetic())
            || (prev_char.is_alphabetic() && curr_char.is_numeric())
        {
            return Some((&input[..curr_idx], &input[curr_idx..]));
        }

        if curr_char.is_lowercase() {
            seen_lower = true;
        }
        i -= 1;
    }

    None
}
