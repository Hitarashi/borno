//! Number ordinalization and list sentence formatting.

/// Returns the English ordinal suffix (`"st"`, `"nd"`, `"rd"`, or `"th"`) for an integer.
///
/// This function operates with zero heap allocations.
///
/// # Examples
/// ```
/// use borno::utils::ordinal_suffix;
///
/// assert_eq!(ordinal_suffix(1), "st");
/// assert_eq!(ordinal_suffix(2), "nd");
/// assert_eq!(ordinal_suffix(3), "rd");
/// assert_eq!(ordinal_suffix(4), "th");
/// assert_eq!(ordinal_suffix(11), "th");
/// assert_eq!(ordinal_suffix(21), "st");
/// ```
#[inline]
#[must_use]
pub const fn ordinal_suffix(number: i64) -> &'static str {
    let absolute = number.unsigned_abs();
    let mod_hundred = absolute % 100;
    if mod_hundred >= 11 && mod_hundred <= 13 {
        "th"
    } else {
        match absolute % 10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        }
    }
}

/// Converts an integer into its English ordinal representation.
///
/// # Examples
/// ```
/// use borno::utils::ordinal;
///
/// assert_eq!(ordinal(1), "1st");
/// assert_eq!(ordinal(2), "2nd");
/// assert_eq!(ordinal(3), "3rd");
/// assert_eq!(ordinal(4), "4th");
/// assert_eq!(ordinal(11), "11th");
/// assert_eq!(ordinal(12), "12th");
/// assert_eq!(ordinal(13), "13th");
/// assert_eq!(ordinal(21), "21st");
/// assert_eq!(ordinal(102), "102nd");
/// ```
#[must_use]
pub fn ordinal(number: i64) -> String {
    let suffix = ordinal_suffix(number);
    format!("{number}{suffix}")
}

/// Strips an English ordinal suffix (`st`, `nd`, `rd`, `th`) from a string.
///
/// Only removes the suffix if it is preceded by an ASCII digit. Strings that look
/// like decimals (containing `.`) or regular words (such as `"south"`, `"band"`, `"nest"`)
/// are returned unchanged. Operates with zero heap allocations.
///
/// # Examples
/// ```
/// use borno::utils::deordinalize;
///
/// assert_eq!(deordinalize("1st"), "1");
/// assert_eq!(deordinalize("2nd"), "2");
/// assert_eq!(deordinalize("3rd"), "3");
/// assert_eq!(deordinalize("4th"), "4");
/// assert_eq!(deordinalize("102nd"), "102");
/// assert_eq!(deordinalize("south"), "south");
/// assert_eq!(deordinalize("3.14th"), "3.14th");
/// ```
#[must_use]
pub fn deordinalize(input: &str) -> &str {
    if input.contains('.') {
        return input;
    }

    const SUFFIXES: [&str; 4] = ["st", "nd", "rd", "th"];
    for suffix in SUFFIXES {
        if let Some(prefix) = input.strip_suffix(suffix) {
            if prefix
                .bytes()
                .last()
                .is_some_and(|byte| byte.is_ascii_digit())
            {
                return prefix;
            }
        }
    }

    input
}

/// Options for formatting arrays of strings into grammatical sentences.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SentenceOptions {
    /// Separator between list items (default: `", "`).
    pub separator: String,
    /// Separator between exactly two items (default: `" and "`).
    pub pair_separator: String,
    /// Separator preceding the final item in lists of three or more (default: `", and "`).
    pub last_separator: String,
}

impl Default for SentenceOptions {
    fn default() -> Self {
        Self {
            separator: ", ".to_string(),
            pair_separator: " and ".to_string(),
            last_separator: ", and ".to_string(),
        }
    }
}

impl SentenceOptions {
    /// Creates a default `SentenceOptions` instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the standard item separator.
    pub fn with_separator(mut self, separator: impl Into<String>) -> Self {
        self.separator = separator.into();
        self
    }

    /// Sets the pair separator.
    pub fn with_pair_separator(mut self, pair_separator: impl Into<String>) -> Self {
        self.pair_separator = pair_separator.into();
        self
    }

    /// Sets the last separator before the final item.
    pub fn with_last_separator(mut self, last_separator: impl Into<String>) -> Self {
        self.last_separator = last_separator.into();
        self
    }
}

/// Converts a slice of string items into an Oxford comma sentence.
///
/// # Examples
/// ```
/// use borno::utils::sentence;
///
/// assert_eq!(sentence(&[]), "");
/// assert_eq!(sentence(&["apples"]), "apples");
/// assert_eq!(sentence(&["apples", "oranges"]), "apples and oranges");
/// assert_eq!(sentence(&["apples", "oranges", "bananas"]), "apples, oranges, and bananas");
/// ```
pub fn sentence(items: &[&str]) -> String {
    sentence_with_options(items, &SentenceOptions::default())
}

/// Converts a slice of string items into a sentence using custom separators.
pub fn sentence_with_options(items: &[&str], options: &SentenceOptions) -> String {
    match items.len() {
        0 => String::new(),
        1 => items[0].to_string(),
        2 => format!("{}{}{}", items[0], options.pair_separator, items[1]),
        _ => {
            let prefix_items = &items[..items.len() - 1];
            let last_item = items[items.len() - 1];
            let joined_prefix = prefix_items.join(&options.separator);
            format!("{}{}{}", joined_prefix, options.last_separator, last_item)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ordinal() {
        assert_eq!(ordinal(0), "0th");
        assert_eq!(ordinal(1), "1st");
        assert_eq!(ordinal(2), "2nd");
        assert_eq!(ordinal(3), "3rd");
        assert_eq!(ordinal(4), "4th");
        assert_eq!(ordinal(11), "11th");
        assert_eq!(ordinal(12), "12th");
        assert_eq!(ordinal(13), "13th");
        assert_eq!(ordinal(14), "14th");
        assert_eq!(ordinal(21), "21st");
        assert_eq!(ordinal(22), "22nd");
        assert_eq!(ordinal(23), "23rd");
        assert_eq!(ordinal(24), "24th");
        assert_eq!(ordinal(101), "101st");
        assert_eq!(ordinal(-1), "-1st");

        assert_eq!(ordinal_suffix(1), "st");
        assert_eq!(ordinal_suffix(2), "nd");
        assert_eq!(ordinal_suffix(3), "rd");
        assert_eq!(ordinal_suffix(4), "th");
        assert_eq!(ordinal_suffix(11), "th");
        assert_eq!(ordinal_suffix(21), "st");
    }

    #[test]
    fn test_deordinalize() {
        assert_eq!(deordinalize("1st"), "1");
        assert_eq!(deordinalize("2nd"), "2");
        assert_eq!(deordinalize("3rd"), "3");
        assert_eq!(deordinalize("4th"), "4");
        assert_eq!(deordinalize("12000th"), "12000");
        assert_eq!(deordinalize("-1st"), "-1");
        assert_eq!(deordinalize("south"), "south");
        assert_eq!(deordinalize("band"), "band");
        assert_eq!(deordinalize("hard"), "hard");
        assert_eq!(deordinalize("nest"), "nest");
        assert_eq!(deordinalize("ststst"), "ststst");
        assert_eq!(deordinalize("1stst"), "1stst");
        assert_eq!(deordinalize("0.1"), "0.1");
        assert_eq!(deordinalize("3.14th"), "3.14th");
        assert_eq!(deordinalize(""), "");
    }

    #[test]
    fn test_sentence() {
        assert_eq!(sentence(&[]), "");
        assert_eq!(sentence(&["apples"]), "apples");
        assert_eq!(sentence(&["apples", "oranges"]), "apples and oranges");
        assert_eq!(
            sentence(&["apples", "oranges", "bananas"]),
            "apples, oranges, and bananas"
        );
        assert_eq!(sentence(&["a", "b", "c", "d"]), "a, b, c, and d");
    }
}
