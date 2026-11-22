//! Text layout, wrapping, justification, and HTML escaping utilities.

use std::borrow::Cow;

use super::pad::{pad_both, pad_end, pad_start};

/// Column alignment mode for justification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Alignment {
    /// Left-aligned column content.
    #[default]
    Left,
    /// Horizontally centered column content.
    Center,
    /// Right-aligned column content.
    Right,
}

/// Configuration options for word wrapping.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WordWrapOptions {
    /// Maximum line character width before wrapping.
    pub max_width: usize,
    /// Newline string sequence (defaults to `"\n"`).
    pub newline: String,
    /// Indentation string prefixed to wrapped lines (defaults to `""`).
    pub indent: String,
}

impl Default for WordWrapOptions {
    fn default() -> Self {
        Self {
            max_width: 80,
            newline: "\n".to_string(),
            indent: String::new(),
        }
    }
}

impl WordWrapOptions {
    /// Creates a new word wrap configuration with the specified maximum width.
    pub fn new(max_width: usize) -> Self {
        Self {
            max_width,
            newline: "\n".to_string(),
            indent: String::new(),
        }
    }

    /// Sets the newline separator string.
    pub fn with_newline(mut self, newline: impl Into<String>) -> Self {
        self.newline = newline.into();
        self
    }

    /// Sets the indentation prefix for wrapped continuation lines.
    pub fn with_indent(mut self, indent: impl Into<String>) -> Self {
        self.indent = indent.into();
        self
    }
}

/// Wraps text to multiple lines breaking at word boundaries when exceeding `max_width`.
///
/// Preserves existing explicit newline boundaries.
///
/// # Examples
/// ```
/// use borno::word_wrap;
///
/// let text = "The quick brown fox jumps over the lazy dog";
/// assert_eq!(
///     word_wrap(text, 20),
///     "The quick brown fox\njumps over the lazy\ndog"
/// );
/// ```
pub fn word_wrap(input: &str, max_width: usize) -> String {
    word_wrap_with_options(input, &WordWrapOptions::new(max_width))
}

/// Wraps text with custom options for maximum width, newline, and indentation.
pub fn word_wrap_with_options(input: &str, options: &WordWrapOptions) -> String {
    if input.is_empty() || options.max_width == 0 {
        return input.to_string();
    }

    let mut output = String::with_capacity(input.len());
    let mut is_first_line = true;

    for raw_line in input.split('\n') {
        if !is_first_line {
            output.push_str(&options.newline);
        }
        is_first_line = false;

        let words: Vec<&str> = raw_line.split_whitespace().collect();
        if words.is_empty() {
            continue;
        }

        let mut current_line_len = 0;
        let mut first_word_on_line = true;

        for word in words {
            let word_len = word.chars().count();

            if first_word_on_line {
                output.push_str(word);
                current_line_len = word_len;
                first_word_on_line = false;
            } else if current_line_len + 1 + word_len <= options.max_width {
                output.push(' ');
                output.push_str(word);
                current_line_len += 1 + word_len;
            } else {
                output.push_str(&options.newline);
                output.push_str(&options.indent);
                output.push_str(word);
                current_line_len = options.indent.chars().count() + word_len;
            }
        }
    }

    output
}

/// Justifies a list of column strings to a uniform character width using the specified alignment.
///
/// # Examples
/// ```
/// use borno::{Alignment, justify_columns};
///
/// let columns = ["Status", "OK", "Pending"];
/// let left = justify_columns(&columns, 10, Alignment::Left);
/// assert_eq!(left, vec!["Status    ", "OK        ", "Pending   "]);
///
/// let right = justify_columns(&columns, 10, Alignment::Right);
/// assert_eq!(right, vec!["    Status", "        OK", "   Pending"]);
/// ```
pub fn justify_columns(columns: &[&str], width: usize, alignment: Alignment) -> Vec<String> {
    columns
        .iter()
        .map(|&col| {
            let char_count = col.chars().count();
            if char_count >= width {
                col.to_string()
            } else {
                match alignment {
                    Alignment::Left => pad_end(col, width, ' '),
                    Alignment::Center => pad_both(col, width, ' '),
                    Alignment::Right => pad_start(col, width, ' '),
                }
            }
        })
        .collect()
}

/// Escapes HTML special characters (`&`, `<`, `>`, `"`, `'`) with their character entity equivalents.
///
/// Returns `Cow::Borrowed` with zero allocations if the input contains no HTML sensitive characters.
///
/// # Examples
/// ```
/// use borno::html_escape;
///
/// assert_eq!(html_escape("<p>Hello & \"World\"</p>"), "&lt;p&gt;Hello &amp; &quot;World&quot;&lt;/p&gt;");
/// assert_eq!(html_escape("plain text"), "plain text");
/// ```
pub fn html_escape(input: &str) -> Cow<'_, str> {
    if !input.contains(['&', '<', '>', '"', '\'']) {
        return Cow::Borrowed(input);
    }

    let mut result = String::with_capacity(input.len() + 16);
    for character in input.chars() {
        match character {
            '&' => result.push_str("&amp;"),
            '<' => result.push_str("&lt;"),
            '>' => result.push_str("&gt;"),
            '"' => result.push_str("&quot;"),
            '\'' => result.push_str("&#39;"),
            other => result.push(other),
        }
    }

    Cow::Owned(result)
}

/// Unescapes HTML character entities back to their original characters.
///
/// Returns `Cow::Borrowed` with zero allocations if the input contains no entity delimiter (`&`).
///
/// # Examples
/// ```
/// use borno::html_unescape;
///
/// assert_eq!(html_unescape("&lt;div&gt;&amp;copy;&lt;/div&gt;"), "<div>&copy;</div>");
/// assert_eq!(html_unescape("already clean"), "already clean");
/// ```
pub fn html_unescape(input: &str) -> Cow<'_, str> {
    if !input.contains('&') {
        return Cow::Borrowed(input);
    }

    let unescaped = input
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&#039;", "'")
        .replace("&apos;", "'");

    Cow::Owned(unescaped)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_wrap() {
        assert_eq!(WordWrapOptions::default().max_width, 80);
        let text = "The quick brown fox jumps over the lazy dog";
        assert_eq!(
            word_wrap(text, 15),
            "The quick brown\nfox jumps over\nthe lazy dog"
        );
        assert_eq!(word_wrap("", 10), "");
        assert_eq!(word_wrap("hello", 0), "hello");

        let multiline = "Line one is here\nLine two is also here";
        assert_eq!(
            word_wrap(multiline, 12),
            "Line one is\nhere\nLine two is\nalso here"
        );

        let empty_line = "First\n\nSecond";
        assert_eq!(word_wrap(empty_line, 10), "First\n\nSecond");
    }

    #[test]
    fn test_word_wrap_custom_options() {
        let text = "First second third fourth";
        let options = WordWrapOptions::new(15)
            .with_newline("\r\n")
            .with_indent("  ");
        assert_eq!(
            word_wrap_with_options(text, &options),
            "First second\r\n  third fourth"
        );
    }

    #[test]
    fn test_justify_columns() {
        let cols = ["a", "bb", "ccc", "verylongcolumn"];
        let left = justify_columns(&cols, 5, Alignment::Left);
        assert_eq!(left, vec!["a    ", "bb   ", "ccc  ", "verylongcolumn"]);

        let right = justify_columns(&cols, 5, Alignment::Right);
        assert_eq!(right, vec!["    a", "   bb", "  ccc", "verylongcolumn"]);

        let center = justify_columns(&cols, 5, Alignment::Center);
        assert_eq!(center, vec!["  a  ", " bb  ", " ccc ", "verylongcolumn"]);
    }

    #[test]
    fn test_html_escape_and_unescape() {
        let raw = "<script>alert('Hello & \"World\"');</script>";
        let escaped = html_escape(raw);
        assert_eq!(
            escaped,
            "&lt;script&gt;alert(&#39;Hello &amp; &quot;World&quot;&#39;);&lt;/script&gt;"
        );
        assert_eq!(html_unescape(&escaped), raw);

        let plain = "just text";
        assert!(matches!(html_escape(plain), Cow::Borrowed(_)));
        assert!(matches!(html_unescape(plain), Cow::Borrowed(_)));

        assert_eq!(html_unescape("test &apos; test &#039;"), "test ' test '");
    }
}
