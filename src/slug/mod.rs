//! URL slug generation with Unicode transliteration and configurable delimiter options.

use std::collections::HashMap;

mod transliterate;

use transliterate::{transliterate_char, transliterate_char_lower};

/// Configuration options for URL slug generation.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SlugOptions {
    /// Delimiter separating words (default: `'-'`).
    pub delimiter: char,
    /// Whether to convert output to lowercase (default: `true`).
    pub lowercase: bool,
    /// Whether to preserve non-ASCII alphanumeric characters (default: `false`).
    pub allow_unicode: bool,
    /// Whether to expand common symbols like `&` to "and" and `@` to "at" (default: `true`).
    pub symbol_expansion: bool,
    /// Custom character replacement map.
    pub custom_replacements: HashMap<char, String>,
}

impl Default for SlugOptions {
    fn default() -> Self {
        Self {
            delimiter: '-',
            lowercase: true,
            allow_unicode: false,
            symbol_expansion: true,
            custom_replacements: HashMap::new(),
        }
    }
}

impl SlugOptions {
    /// Creates a default `SlugOptions` instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the word separator delimiter.
    pub fn with_delimiter(mut self, delimiter: char) -> Self {
        self.delimiter = delimiter;
        self
    }

    /// Configures lowercase conversion.
    pub fn with_lowercase(mut self, lowercase: bool) -> Self {
        self.lowercase = lowercase;
        self
    }

    /// Configures whether non-ASCII alphanumeric characters are preserved.
    pub fn with_allow_unicode(mut self, allow: bool) -> Self {
        self.allow_unicode = allow;
        self
    }

    /// Configures whether symbols like `&` and `@` are expanded into words.
    pub fn with_symbol_expansion(mut self, expand: bool) -> Self {
        self.symbol_expansion = expand;
        self
    }

    /// Adds a custom character replacement rule.
    pub fn with_replacement(mut self, from: char, to: impl Into<String>) -> Self {
        self.custom_replacements.insert(from, to.into());
        self
    }
}

fn slugify_raw(input: &str) -> String {
    if input.is_empty() {
        return String::new();
    }

    let mut buf = Vec::with_capacity(input.len());
    let mut previous_was_delimiter = true;

    for current_char in input.chars() {
        if current_char.is_ascii() {
            let b = current_char as u8;
            match b {
                b'a'..=b'z' | b'0'..=b'9' => {
                    previous_was_delimiter = false;
                    buf.push(b);
                }
                b'A'..=b'Z' => {
                    previous_was_delimiter = false;
                    buf.push(b + 32);
                }
                _ => {
                    if !previous_was_delimiter {
                        buf.push(b'-');
                        previous_was_delimiter = true;
                    }
                }
            }
        } else if let Some(transliterated) = transliterate_char_lower(current_char) {
            for &tb in transliterated.as_bytes() {
                buf.push(tb);
            }
            previous_was_delimiter = false;
        } else if !previous_was_delimiter {
            buf.push(b'-');
            previous_was_delimiter = true;
        }
    }

    if buf.last() == Some(&b'-') {
        buf.pop();
    }

    // SAFETY: buf contains only ASCII bytes (a-z, 0-9, and '-') which are valid UTF-8.
    unsafe { String::from_utf8_unchecked(buf) }
}

/// Generates a URL-safe slug from a string slice with default options.
///
/// # Examples
/// ```
/// use borno::slug::slugify;
///
/// assert_eq!(slugify("Hello World & Deep Dive!"), "hello-world-and-deep-dive");
/// assert_eq!(slugify("Café Déjà Vu"), "cafe-deja-vu");
/// ```
pub fn slugify(input: &str) -> String {
    if input.is_empty() {
        return String::new();
    }

    let mut buf = Vec::with_capacity(input.len() + 16);
    let mut previous_was_delimiter = true;

    for current_char in input.chars() {
        if current_char.is_ascii() {
            let b = current_char as u8;
            match b {
                b'a'..=b'z' | b'0'..=b'9' => {
                    previous_was_delimiter = false;
                    buf.push(b);
                }
                b'A'..=b'Z' => {
                    previous_was_delimiter = false;
                    buf.push(b + 32);
                }
                b'&' => {
                    if !previous_was_delimiter {
                        buf.push(b'-');
                    }
                    buf.extend_from_slice(b"and-");
                    previous_was_delimiter = true;
                }
                b'@' => {
                    if !previous_was_delimiter {
                        buf.push(b'-');
                    }
                    buf.extend_from_slice(b"at-");
                    previous_was_delimiter = true;
                }
                _ => {
                    if !previous_was_delimiter {
                        buf.push(b'-');
                        previous_was_delimiter = true;
                    }
                }
            }
        } else if let Some(transliterated) = transliterate_char_lower(current_char) {
            for &tb in transliterated.as_bytes() {
                buf.push(tb);
            }
            previous_was_delimiter = false;
        } else if !previous_was_delimiter {
            buf.push(b'-');
            previous_was_delimiter = true;
        }
    }

    if buf.last() == Some(&b'-') {
        buf.pop();
    }

    // SAFETY: buf contains only ASCII bytes (a-z, 0-9, and '-') which are valid UTF-8.
    unsafe { String::from_utf8_unchecked(buf) }
}

/// Generates a URL-safe slug with customized options.
pub fn slugify_with_options(input: &str, options: &SlugOptions) -> String {
    if input.is_empty() {
        return String::new();
    }

    if options.delimiter == '-'
        && options.lowercase
        && !options.allow_unicode
        && options.custom_replacements.is_empty()
    {
        if options.symbol_expansion {
            return slugify(input);
        }
        return slugify_raw(input);
    }

    let has_custom = !options.custom_replacements.is_empty();

    if !options.allow_unicode && !has_custom && options.delimiter.is_ascii() {
        let delim_byte = options.delimiter as u8;
        let mut buf = Vec::with_capacity(input.len() + 16);
        let mut previous_was_delimiter = true;

        for current_char in input.chars() {
            if current_char.is_ascii() {
                let b = current_char as u8;
                match b {
                    b'a'..=b'z' | b'0'..=b'9' => {
                        previous_was_delimiter = false;
                        buf.push(b);
                    }
                    b'A'..=b'Z' => {
                        previous_was_delimiter = false;
                        buf.push(if options.lowercase { b + 32 } else { b });
                    }
                    b'&' if options.symbol_expansion => {
                        if !previous_was_delimiter {
                            buf.push(delim_byte);
                        }
                        buf.extend_from_slice(b"and");
                        buf.push(delim_byte);
                        previous_was_delimiter = true;
                    }
                    b'@' if options.symbol_expansion => {
                        if !previous_was_delimiter {
                            buf.push(delim_byte);
                        }
                        buf.extend_from_slice(b"at");
                        buf.push(delim_byte);
                        previous_was_delimiter = true;
                    }
                    _ => {
                        if !previous_was_delimiter {
                            buf.push(delim_byte);
                            previous_was_delimiter = true;
                        }
                    }
                }
            } else if let Some(transliterated) = if options.lowercase {
                transliterate_char_lower(current_char)
            } else {
                transliterate_char(current_char)
            } {
                for &tb in transliterated.as_bytes() {
                    buf.push(tb);
                }
                previous_was_delimiter = false;
            } else if !previous_was_delimiter {
                buf.push(delim_byte);
                previous_was_delimiter = true;
            }
        }

        if buf.last() == Some(&delim_byte) {
            buf.pop();
        }

        // SAFETY: buf contains only ASCII bytes which are valid UTF-8.
        return unsafe { String::from_utf8_unchecked(buf) };
    }

    let mut result = String::with_capacity(input.len());
    let mut previous_was_delimiter = true;

    #[inline(always)]
    fn emit_char(
        c: char,
        result: &mut String,
        previous_was_delimiter: &mut bool,
        delimiter: char,
        lowercase: bool,
    ) {
        if c == delimiter {
            if !*previous_was_delimiter {
                result.push(delimiter);
                *previous_was_delimiter = true;
            }
        } else {
            *previous_was_delimiter = false;
            if lowercase {
                if c.is_ascii() {
                    result.push(c.to_ascii_lowercase());
                } else {
                    for lower_char in c.to_lowercase() {
                        result.push(lower_char);
                    }
                }
            } else {
                result.push(c);
            }
        }
    }

    #[inline]
    fn emit_str(
        s: &str,
        result: &mut String,
        previous_was_delimiter: &mut bool,
        delimiter: char,
        lowercase: bool,
    ) {
        for c in s.chars() {
            emit_char(c, result, previous_was_delimiter, delimiter, lowercase);
        }
    }

    for current_char in input.chars() {
        if has_custom {
            if let Some(replacement) = options.custom_replacements.get(&current_char) {
                emit_char(
                    options.delimiter,
                    &mut result,
                    &mut previous_was_delimiter,
                    options.delimiter,
                    options.lowercase,
                );
                emit_str(
                    replacement,
                    &mut result,
                    &mut previous_was_delimiter,
                    options.delimiter,
                    options.lowercase,
                );
                emit_char(
                    options.delimiter,
                    &mut result,
                    &mut previous_was_delimiter,
                    options.delimiter,
                    options.lowercase,
                );
                continue;
            }
        }

        if current_char.is_ascii() {
            if current_char.is_ascii_alphanumeric() {
                emit_char(
                    current_char,
                    &mut result,
                    &mut previous_was_delimiter,
                    options.delimiter,
                    options.lowercase,
                );
            } else if options.symbol_expansion && current_char == '&' {
                emit_char(
                    options.delimiter,
                    &mut result,
                    &mut previous_was_delimiter,
                    options.delimiter,
                    options.lowercase,
                );
                emit_str(
                    "and",
                    &mut result,
                    &mut previous_was_delimiter,
                    options.delimiter,
                    options.lowercase,
                );
                emit_char(
                    options.delimiter,
                    &mut result,
                    &mut previous_was_delimiter,
                    options.delimiter,
                    options.lowercase,
                );
            } else if options.symbol_expansion && current_char == '@' {
                emit_char(
                    options.delimiter,
                    &mut result,
                    &mut previous_was_delimiter,
                    options.delimiter,
                    options.lowercase,
                );
                emit_str(
                    "at",
                    &mut result,
                    &mut previous_was_delimiter,
                    options.delimiter,
                    options.lowercase,
                );
                emit_char(
                    options.delimiter,
                    &mut result,
                    &mut previous_was_delimiter,
                    options.delimiter,
                    options.lowercase,
                );
            } else {
                emit_char(
                    options.delimiter,
                    &mut result,
                    &mut previous_was_delimiter,
                    options.delimiter,
                    options.lowercase,
                );
            }
        } else if let Some(transliterated) = transliterate_char(current_char) {
            emit_str(
                transliterated,
                &mut result,
                &mut previous_was_delimiter,
                options.delimiter,
                options.lowercase,
            );
        } else if options.allow_unicode && current_char.is_alphanumeric() {
            emit_char(
                current_char,
                &mut result,
                &mut previous_was_delimiter,
                options.delimiter,
                options.lowercase,
            );
        } else {
            emit_char(
                options.delimiter,
                &mut result,
                &mut previous_was_delimiter,
                options.delimiter,
                options.lowercase,
            );
        }
    }

    if result.ends_with(options.delimiter) {
        result.pop();
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slugify_defaults() {
        assert_eq!(
            slugify("Hello World & Deep Dive!"),
            "hello-world-and-deep-dive"
        );
        assert_eq!(slugify("Café Déjà Vu"), "cafe-deja-vu");
        assert_eq!(slugify("user@example.com"), "user-at-example-com");
        assert_eq!(slugify("___hello___world___"), "hello-world");
        assert_eq!(slugify(""), "");
    }

    #[test]
    fn test_slugify_custom_options() {
        let options = SlugOptions::new()
            .with_delimiter('_')
            .with_lowercase(false)
            .with_replacement('%', "percent");

        assert_eq!(slugify_with_options("50% Off", &options), "50_percent_Off");
    }

    #[test]
    fn test_slugify_unicode_retention() {
        let options = SlugOptions::new().with_allow_unicode(true);
        assert_eq!(
            slugify_with_options("Hello 東京 World", &options),
            "hello-東京-world"
        );
    }
}
