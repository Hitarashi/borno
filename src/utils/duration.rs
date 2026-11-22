//! Human-readable duration parsing and formatting.

use std::{fmt, time::Duration};

/// Errors that can occur when parsing human-readable duration strings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseDurationError {
    /// The input string was empty or contained only whitespace.
    EmptyInput,
    /// The duration expression had an invalid structure.
    InvalidFormat(String),
    /// The numeric component could not be parsed.
    InvalidNumber(String),
    /// An unknown or unsupported time unit was encountered.
    UnknownUnit(String),
    /// The computed duration exceeded allowable time representation limits.
    Overflow,
}

impl fmt::Display for ParseDurationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyInput => write!(formatter, "duration string input is empty"),
            Self::InvalidFormat(message) => write!(formatter, "invalid duration format: {message}"),
            Self::InvalidNumber(number) => write!(formatter, "invalid duration number: {number}"),
            Self::UnknownUnit(unit) => write!(formatter, "unknown duration unit: {unit}"),
            Self::Overflow => write!(formatter, "duration calculation overflowed limits"),
        }
    }
}

impl std::error::Error for ParseDurationError {}

/// Parses a human-readable duration string into a standard [`Duration`].
///
/// Supports standard abbreviations, full English unit names, and compound expressions
/// (e.g. `"1h 30m"`, `"2 hours, 15 minutes"`).
///
/// Supported units:
/// - Milliseconds: `ms`, `msec`, `msecs`, `millisecond`, `milliseconds`
/// - Seconds: `s`, `sec`, `secs`, `second`, `seconds`
/// - Minutes: `m`, `min`, `mins`, `minute`, `minutes`
/// - Hours: `h`, `hr`, `hrs`, `hour`, `hours`
/// - Days: `d`, `day`, `days`
/// - Weeks: `w`, `wk`, `wks`, `week`, `weeks`
///
/// # Examples
/// ```
/// use std::time::Duration;
/// use borno::utils::parse_duration;
///
/// assert_eq!(parse_duration("500ms").unwrap(), Duration::from_millis(500));
/// assert_eq!(parse_duration("15m").unwrap(), Duration::from_secs(900));
/// assert_eq!(parse_duration("2 hours").unwrap(), Duration::from_secs(7200));
/// assert_eq!(parse_duration("1h 30m").unwrap(), Duration::from_secs(5400));
/// assert_eq!(parse_duration("7 days").unwrap(), Duration::from_secs(604800));
/// ```
pub fn parse_duration(input: &str) -> Result<Duration, ParseDurationError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(ParseDurationError::EmptyInput);
    }

    if trimmed.starts_with('-') {
        return Err(ParseDurationError::InvalidFormat(
            "negative durations are not supported".to_string(),
        ));
    }

    // Tokenize into (number, unit) pairs
    let mut total_duration = Duration::ZERO;
    let chars: Vec<(usize, char)> = trimmed.char_indices().collect();
    let mut char_index = 0;
    let mut matched_any_tokens = false;

    while char_index < chars.len() {
        // Skip whitespace, commas, and 'and'
        while char_index < chars.len()
            && (chars[char_index].1.is_whitespace() || chars[char_index].1 == ',')
        {
            char_index += 1;
        }

        if char_index >= chars.len() {
            break;
        }

        let current_offset = chars[char_index].0;
        let remaining_slice = &trimmed[current_offset..];
        if let Some(without_and) = remaining_slice.strip_prefix("and ") {
            let next_offset = current_offset + (remaining_slice.len() - without_and.len());
            char_index = chars
                .iter()
                .position(|&(idx, _)| idx >= next_offset)
                .unwrap_or(chars.len());
            continue;
        }

        // Parse number
        let number_start = char_index;
        let mut saw_dot = false;
        while char_index < chars.len() {
            let character = chars[char_index].1;
            if character.is_ascii_digit() {
                char_index += 1;
            } else if character == '.' && !saw_dot {
                saw_dot = true;
                char_index += 1;
            } else {
                break;
            }
        }

        if number_start == char_index {
            return Err(ParseDurationError::InvalidFormat(format!(
                "expected numeric value at '{remaining_slice}'"
            )));
        }

        let number_byte_start = chars[number_start].0;
        let number_byte_end = if char_index < chars.len() {
            chars[char_index].0
        } else {
            trimmed.len()
        };

        let number_str = &trimmed[number_byte_start..number_byte_end];
        let numeric_value: f64 =
            number_str
                .parse()
                .map_err(|parse_err: std::num::ParseFloatError| {
                    ParseDurationError::InvalidNumber(parse_err.to_string())
                })?;

        // Skip whitespace between number and unit
        while char_index < chars.len() && chars[char_index].1.is_whitespace() {
            char_index += 1;
        }

        // Parse unit (alphabetic characters)
        let unit_start = char_index;
        while char_index < chars.len() && chars[char_index].1.is_alphabetic() {
            char_index += 1;
        }

        let unit_byte_start = if unit_start < chars.len() {
            chars[unit_start].0
        } else {
            trimmed.len()
        };
        let unit_byte_end = if char_index < chars.len() {
            chars[char_index].0
        } else {
            trimmed.len()
        };

        let unit_str = &trimmed[unit_byte_start..unit_byte_end];
        let unit_duration = resolve_unit_duration(numeric_value, unit_str)?;

        total_duration = total_duration
            .checked_add(unit_duration)
            .ok_or(ParseDurationError::Overflow)?;
        matched_any_tokens = true;
    }

    if !matched_any_tokens {
        return Err(ParseDurationError::InvalidFormat(
            "no valid duration tokens found".to_string(),
        ));
    }

    Ok(total_duration)
}

fn resolve_unit_duration(value: f64, unit_str: &str) -> Result<Duration, ParseDurationError> {
    if value < 0.0 || value.is_nan() || value.is_infinite() {
        return Err(ParseDurationError::InvalidNumber(
            "invalid duration quantity".to_string(),
        ));
    }

    let normalized = unit_str.to_ascii_lowercase();
    let secs_per_unit: f64 = match normalized.as_str() {
        "ms" | "msec" | "msecs" | "millisecond" | "milliseconds" => 0.001,
        "s" | "sec" | "secs" | "second" | "seconds" => 1.0,
        "m" | "min" | "mins" | "minute" | "minutes" => 60.0,
        "h" | "hr" | "hrs" | "hour" | "hours" => 3600.0,
        "d" | "day" | "days" => 86400.0,
        "w" | "wk" | "wks" | "week" | "weeks" => 604800.0,
        _ => return Err(ParseDurationError::UnknownUnit(unit_str.to_string())),
    };

    let total_secs = value * secs_per_unit;
    Duration::try_from_secs_f64(total_secs).map_err(|_| ParseDurationError::Overflow)
}

/// Formats a [`Duration`] into a concise human-readable string.
///
/// # Examples
/// ```
/// use std::time::Duration;
/// use borno::utils::format_duration;
///
/// assert_eq!(format_duration(Duration::ZERO), "0s");
/// assert_eq!(format_duration(Duration::from_millis(500)), "500ms");
/// assert_eq!(format_duration(Duration::from_secs(65)), "1m 5s");
/// assert_eq!(format_duration(Duration::from_secs(3661)), "1h 1m 1s");
/// assert_eq!(format_duration(Duration::from_secs(86400)), "1d");
/// ```
#[must_use]
pub fn format_duration(duration: Duration) -> String {
    let total_millis = duration.as_millis();
    if total_millis == 0 {
        return "0s".to_string();
    }

    let days = total_millis / (24 * 60 * 60 * 1000);
    let mut remainder = total_millis % (24 * 60 * 60 * 1000);

    let hours = remainder / (60 * 60 * 1000);
    remainder %= 60 * 60 * 1000;

    let minutes = remainder / (60 * 1000);
    remainder %= 60 * 1000;

    let seconds = remainder / 1000;
    let millis = remainder % 1000;

    let mut parts = Vec::new();
    if days > 0 {
        parts.push(format!("{days}d"));
    }
    if hours > 0 {
        parts.push(format!("{hours}h"));
    }
    if minutes > 0 {
        parts.push(format!("{minutes}m"));
    }
    if seconds > 0 {
        parts.push(format!("{seconds}s"));
    }
    if millis > 0 {
        parts.push(format!("{millis}ms"));
    }

    parts.join(" ")
}

/// Formats a [`Duration`] into natural, verbose English words.
///
/// # Examples
/// ```
/// use std::time::Duration;
/// use borno::utils::format_duration_verbose;
///
/// assert_eq!(format_duration_verbose(Duration::ZERO), "0 seconds");
/// assert_eq!(format_duration_verbose(Duration::from_secs(1)), "1 second");
/// assert_eq!(format_duration_verbose(Duration::from_secs(2)), "2 seconds");
/// assert_eq!(format_duration_verbose(Duration::from_secs(3661)), "1 hour, 1 minute, 1 second");
/// ```
#[must_use]
pub fn format_duration_verbose(duration: Duration) -> String {
    let total_millis = duration.as_millis();
    if total_millis == 0 {
        return "0 seconds".to_string();
    }

    let days = total_millis / (24 * 60 * 60 * 1000);
    let mut remainder = total_millis % (24 * 60 * 60 * 1000);

    let hours = remainder / (60 * 60 * 1000);
    remainder %= 60 * 60 * 1000;

    let minutes = remainder / (60 * 1000);
    remainder %= 60 * 1000;

    let seconds = remainder / 1000;
    let millis = remainder % 1000;

    let mut parts = Vec::new();
    if days > 0 {
        let label = if days == 1 { "day" } else { "days" };
        parts.push(format!("{days} {label}"));
    }
    if hours > 0 {
        let label = if hours == 1 { "hour" } else { "hours" };
        parts.push(format!("{hours} {label}"));
    }
    if minutes > 0 {
        let label = if minutes == 1 { "minute" } else { "minutes" };
        parts.push(format!("{minutes} {label}"));
    }
    if seconds > 0 {
        let label = if seconds == 1 { "second" } else { "seconds" };
        parts.push(format!("{seconds} {label}"));
    }
    if millis > 0 {
        let label = if millis == 1 {
            "millisecond"
        } else {
            "milliseconds"
        };
        parts.push(format!("{millis} {label}"));
    }

    parts.join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_duration_units() {
        assert_eq!(parse_duration("500ms").unwrap(), Duration::from_millis(500));
        assert_eq!(parse_duration("10s").unwrap(), Duration::from_secs(10));
        assert_eq!(parse_duration("15m").unwrap(), Duration::from_secs(900));
        assert_eq!(parse_duration("2h").unwrap(), Duration::from_secs(7200));
        assert_eq!(parse_duration("1d").unwrap(), Duration::from_secs(86400));
        assert_eq!(parse_duration("1w").unwrap(), Duration::from_secs(604800));
    }

    #[test]
    fn test_parse_duration_compound_and_words() {
        assert_eq!(
            parse_duration("1 hour, 30 minutes and 15 seconds").unwrap(),
            Duration::from_secs(3600 + 1800 + 15)
        );
        assert_eq!(parse_duration("1h 30m").unwrap(), Duration::from_secs(5400));
        assert_eq!(
            parse_duration("2 days 4 hours").unwrap(),
            Duration::from_secs(2 * 86400 + 4 * 3600)
        );
    }

    #[test]
    fn test_parse_duration_errors() {
        assert_eq!(
            parse_duration("").unwrap_err(),
            ParseDurationError::EmptyInput
        );
        assert_eq!(
            parse_duration("   ").unwrap_err(),
            ParseDurationError::EmptyInput
        );
        assert!(matches!(
            parse_duration("-5s").unwrap_err(),
            ParseDurationError::InvalidFormat(_)
        ));
        assert!(matches!(
            parse_duration("hours").unwrap_err(),
            ParseDurationError::InvalidFormat(_)
        ));
        assert!(matches!(
            parse_duration("100 lightyears").unwrap_err(),
            ParseDurationError::UnknownUnit(_)
        ));
    }

    #[test]
    fn test_format_duration_compact() {
        assert_eq!(format_duration(Duration::ZERO), "0s");
        assert_eq!(format_duration(Duration::from_millis(500)), "500ms");
        assert_eq!(format_duration(Duration::from_secs(45)), "45s");
        assert_eq!(format_duration(Duration::from_secs(125)), "2m 5s");
        assert_eq!(format_duration(Duration::from_secs(3665)), "1h 1m 5s");
        assert_eq!(
            format_duration(Duration::from_secs(86400 * 2 + 3600)),
            "2d 1h"
        );
    }

    #[test]
    fn test_format_duration_verbose() {
        assert_eq!(format_duration_verbose(Duration::ZERO), "0 seconds");
        assert_eq!(format_duration_verbose(Duration::from_secs(1)), "1 second");
        assert_eq!(format_duration_verbose(Duration::from_secs(2)), "2 seconds");
        assert_eq!(
            format_duration_verbose(Duration::from_secs(3661)),
            "1 hour, 1 minute, 1 second"
        );
        assert_eq!(
            format_duration_verbose(Duration::from_millis(1)),
            "1 millisecond"
        );
        assert_eq!(
            format_duration_verbose(Duration::from_millis(500)),
            "500 milliseconds"
        );
        assert_eq!(
            format_duration_verbose(Duration::from_millis(
                86_400_000 + 3_600_000 + 60_000 + 1_000 + 1
            )),
            "1 day, 1 hour, 1 minute, 1 second, 1 millisecond"
        );
        assert_eq!(
            format_duration_verbose(Duration::from_millis(
                2 * 86_400_000 + 2 * 3_600_000 + 2 * 60_000 + 2 * 1_000 + 2
            )),
            "2 days, 2 hours, 2 minutes, 2 seconds, 2 milliseconds"
        );
    }

    #[test]
    fn test_duration_all_edge_cases() {
        let errors = [
            ParseDurationError::EmptyInput,
            ParseDurationError::InvalidFormat("bad".to_string()),
            ParseDurationError::InvalidNumber("NaN".to_string()),
            ParseDurationError::UnknownUnit("foo".to_string()),
            ParseDurationError::Overflow,
        ];
        for error in &errors {
            let displayed = format!("{error}");
            assert!(!displayed.is_empty());
        }

        let half_hour = parse_duration("0.5 hours").unwrap();
        assert_eq!(half_hour, Duration::from_secs(1800));

        let compound_frac = parse_duration("1.5h, 30.5s").unwrap();
        assert_eq!(
            compound_frac,
            Duration::from_secs(5400) + Duration::from_millis(30500)
        );

        assert_eq!(
            parse_duration("1 hour, ").unwrap(),
            Duration::from_secs(3600)
        );
        assert!(parse_duration("1.2.3 s").is_err());
        assert!(parse_duration(". s").is_err());
        assert!(parse_duration("-5 s").is_err());
        assert!(parse_duration("NaN s").is_err());
        assert!(parse_duration("inf s").is_err());
        let huge_dur = format!("{} s", "9".repeat(320));
        assert!(parse_duration(&huge_dur).is_err());
        assert!(parse_duration(", , ,").is_err());
        let dur_overflow = format!("{} s, {} s", u64::MAX / 2 + 1000, u64::MAX / 2 + 1000);
        assert!(parse_duration(&dur_overflow).is_err());
        assert!(parse_duration("100000000000000000000000 days").is_err());
        assert!(parse_duration("100").is_err());
    }
}
