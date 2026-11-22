//! Data size and byte count parsing and formatting.

use std::fmt;

/// Standard units for data size and byte representations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BytesUnit {
    /// Bytes (B)
    B,
    /// Kilobytes (KB, 1,000 bytes) or Kibibytes (KiB, 1,024 bytes)
    KB,
    /// Megabytes (MB, 1,000,000 bytes) or Mebibytes (MiB, 1,048,576 bytes)
    MB,
    /// Gigabytes (GB, 10^9 bytes) or Gibibytes (GiB, 2^30 bytes)
    GB,
    /// Terabytes (TB, 10^12 bytes) or Tebibytes (TiB, 2^40 bytes)
    TB,
    /// Petabytes (PB, 10^15 bytes) or Pebibytes (PiB, 2^50 bytes)
    PB,
    /// Kibibytes (KiB, 1,024 bytes)
    KiB,
    /// Mebibytes (MiB, 1,048,576 bytes)
    MiB,
    /// Gibibytes (GiB, 1,073,741,824 bytes)
    GiB,
    /// Tebibytes (TiB, 1,099,511,627,776 bytes)
    TiB,
    /// Pebibytes (PiB, 1,125,899,906,842,624 bytes)
    PiB,
}

impl BytesUnit {
    /// Returns the standard symbol for the unit.
    #[must_use]
    pub const fn symbol(self) -> &'static str {
        match self {
            Self::B => "B",
            Self::KB => "KB",
            Self::MB => "MB",
            Self::GB => "GB",
            Self::TB => "TB",
            Self::PB => "PB",
            Self::KiB => "KiB",
            Self::MiB => "MiB",
            Self::GiB => "GiB",
            Self::TiB => "TiB",
            Self::PiB => "PiB",
        }
    }
}

/// Base standard used for byte calculations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BytesStandard {
    /// International System of Units (SI) metric standard with base 1,000.
    Decimal,
    /// International Electrotechnical Commission (IEC) binary standard with base 1,024.
    #[default]
    Binary,
}

impl BytesStandard {
    /// Returns the multiplier base factor for this standard.
    #[must_use]
    pub const fn base(self) -> u64 {
        match self {
            Self::Decimal => 1000,
            Self::Binary => 1024,
        }
    }
}

/// Options configuring human-readable byte formatting.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BytesOptions {
    /// Maximum number of decimal places to include (default: `2`).
    pub decimal_places: usize,
    /// Whether to always display the exact number of decimal places (default: `false`).
    pub fixed_decimals: bool,
    /// Optional thousands separator (default: `""`).
    pub thousands_separator: String,
    /// Separator between the formatted number and the unit symbol (default: `" "`).
    pub unit_separator: String,
    /// Calculation standard for units (default: [`BytesStandard::Binary`]).
    pub standard: BytesStandard,
    /// Explicit target unit. If `None`, the unit is automatically detected.
    pub unit: Option<BytesUnit>,
}

impl Default for BytesOptions {
    fn default() -> Self {
        Self {
            decimal_places: 2,
            fixed_decimals: false,
            thousands_separator: String::new(),
            unit_separator: " ".to_string(),
            standard: BytesStandard::Binary,
            unit: None,
        }
    }
}

impl BytesOptions {
    /// Creates a default `BytesOptions` configuration.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the maximum decimal places.
    #[must_use]
    pub fn with_decimal_places(mut self, decimal_places: usize) -> Self {
        self.decimal_places = decimal_places;
        self
    }

    /// Sets whether decimal places are fixed with trailing zeros.
    #[must_use]
    pub fn with_fixed_decimals(mut self, fixed_decimals: bool) -> Self {
        self.fixed_decimals = fixed_decimals;
        self
    }

    /// Sets the thousands separator string.
    #[must_use]
    pub fn with_thousands_separator(mut self, separator: impl Into<String>) -> Self {
        self.thousands_separator = separator.into();
        self
    }

    /// Sets the separator string between number and unit.
    #[must_use]
    pub fn with_unit_separator(mut self, separator: impl Into<String>) -> Self {
        self.unit_separator = separator.into();
        self
    }

    /// Sets the unit calculation standard.
    #[must_use]
    pub fn with_standard(mut self, standard: BytesStandard) -> Self {
        self.standard = standard;
        self
    }

    /// Sets an explicit target unit.
    #[must_use]
    pub fn with_unit(mut self, unit: BytesUnit) -> Self {
        self.unit = Some(unit);
        self
    }
}

/// Errors that can occur when parsing byte string expressions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseBytesError {
    /// The input string was empty or contained only whitespace.
    EmptyInput,
    /// The input string had an invalid format.
    InvalidFormat(String),
    /// The numeric component could not be parsed.
    InvalidNumber(String),
    /// The unit string was unrecognized.
    UnknownUnit(String),
    /// The computed byte count exceeded 64-bit unsigned integer capacity.
    Overflow,
}

impl fmt::Display for ParseBytesError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyInput => write!(formatter, "byte string input is empty"),
            Self::InvalidFormat(message) => write!(formatter, "invalid byte format: {message}"),
            Self::InvalidNumber(number) => write!(formatter, "invalid byte number: {number}"),
            Self::UnknownUnit(unit) => write!(formatter, "unknown byte unit: {unit}"),
            Self::Overflow => write!(
                formatter,
                "byte calculation overflowed 64-bit integer limit"
            ),
        }
    }
}

impl std::error::Error for ParseBytesError {}

/// Parses a human-readable byte string into the total count of bytes.
///
/// Supports both SI decimal (`KB`, `MB`, `GB`, `TB`, `PB`) and IEC binary (`KiB`, `MiB`, `GiB`, `TiB`, `PiB`)
/// notations, as well as abbreviations and case-insensitive expressions.
///
/// # Examples
/// ```
/// use borno::utils::parse_bytes;
///
/// assert_eq!(parse_bytes("1024").unwrap(), 1024);
/// assert_eq!(parse_bytes("1 KB").unwrap(), 1024);
/// assert_eq!(parse_bytes("1 KiB").unwrap(), 1024);
/// assert_eq!(parse_bytes("10MB").unwrap(), 10_485_760);
/// assert_eq!(parse_bytes("1.5 GB").unwrap(), 1_610_612_736);
/// ```
pub fn parse_bytes(input: &str) -> Result<u64, ParseBytesError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(ParseBytesError::EmptyInput);
    }

    if trimmed.starts_with('-') {
        return Err(ParseBytesError::InvalidFormat(
            "negative byte counts are not supported".to_string(),
        ));
    }

    let without_plus = trimmed.strip_prefix('+').unwrap_or(trimmed).trim_start();
    if without_plus.is_empty() {
        return Err(ParseBytesError::InvalidFormat(
            "missing number in byte expression".to_string(),
        ));
    }

    let mut number_end = 0;
    let mut saw_dot = false;
    for (index, character) in without_plus.char_indices() {
        if character.is_ascii_digit() {
            number_end = index + character.len_utf8();
        } else if character == '.' && !saw_dot {
            saw_dot = true;
            number_end = index + character.len_utf8();
        } else {
            break;
        }
    }

    if number_end == 0 {
        return Err(ParseBytesError::InvalidFormat(format!(
            "failed to find valid numeric prefix in '{input}'"
        )));
    }

    let number_slice = &without_plus[..number_end];
    let unit_slice = without_plus[number_end..].trim();

    let numeric_value: f64 =
        number_slice
            .parse()
            .map_err(|parse_error: std::num::ParseFloatError| {
                ParseBytesError::InvalidNumber(parse_error.to_string())
            })?;

    if numeric_value.is_nan() || numeric_value.is_infinite() {
        return Err(ParseBytesError::InvalidNumber(
            "numeric value cannot be NaN or infinite".to_string(),
        ));
    }

    let multiplier: f64 = resolve_unit_multiplier(unit_slice)?;
    let total_bytes = numeric_value * multiplier;

    if total_bytes < 0.0 || total_bytes > u64::MAX as f64 {
        return Err(ParseBytesError::Overflow);
    }

    Ok(total_bytes.round() as u64)
}

fn resolve_unit_multiplier(unit_str: &str) -> Result<f64, ParseBytesError> {
    let normalized = unit_str.to_ascii_lowercase();
    match normalized.as_str() {
        "" | "b" | "byte" | "bytes" => Ok(1.0),
        "k" | "kb" | "kbyte" | "kbytes" | "kilobyte" | "kilobytes" => Ok(1024.0),
        "kib" | "kibibyte" | "kibibytes" => Ok(1024.0),
        "m" | "mb" | "mbyte" | "mbytes" | "megabyte" | "megabytes" => Ok(1024.0 * 1024.0),
        "mib" | "mebibyte" | "mebibytes" => Ok(1024.0 * 1024.0),
        "g" | "gb" | "gbyte" | "gbytes" | "gigabyte" | "gigabytes" => Ok(1024.0 * 1024.0 * 1024.0),
        "gib" | "gibibyte" | "gibibytes" => Ok(1024.0 * 1024.0 * 1024.0),
        "t" | "tb" | "tbyte" | "tbytes" | "terabyte" | "terabytes" => {
            Ok(1024.0 * 1024.0 * 1024.0 * 1024.0)
        }
        "tib" | "tebibyte" | "tebibytes" => Ok(1024.0 * 1024.0 * 1024.0 * 1024.0),
        "p" | "pb" | "pbyte" | "pbytes" | "petabyte" | "petabytes" => {
            Ok(1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0)
        }
        "pib" | "pebibyte" | "pebibytes" => Ok(1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0),
        _ => Err(ParseBytesError::UnknownUnit(unit_str.to_string())),
    }
}

/// Formats a byte quantity into a human-readable string with default options.
///
/// Uses the binary standard (`1024` base), automatic unit selection, and up to 2 decimal places.
///
/// # Examples
/// ```
/// use borno::utils::format_bytes;
///
/// assert_eq!(format_bytes(0), "0 B");
/// assert_eq!(format_bytes(1024), "1 KB");
/// assert_eq!(format_bytes(1_572_864), "1.5 MB");
/// assert_eq!(format_bytes(1_073_741_824), "1 GB");
/// ```
#[must_use]
pub fn format_bytes(bytes: u64) -> String {
    format_bytes_with_options(bytes, &BytesOptions::default())
}

/// Formats a byte quantity into a human-readable string using custom options.
///
/// # Examples
/// ```
/// use borno::utils::{BytesOptions, BytesStandard, BytesUnit, format_bytes_with_options};
///
/// let options = BytesOptions::new()
///     .with_decimal_places(3)
///     .with_fixed_decimals(true)
///     .with_unit_separator("");
///
/// assert_eq!(format_bytes_with_options(1024, &options), "1.000KB");
///
/// let decimal_options = BytesOptions::new()
///     .with_standard(BytesStandard::Decimal)
///     .with_unit(BytesUnit::KB);
/// assert_eq!(format_bytes_with_options(1500, &decimal_options), "1.5 KB");
/// ```
#[must_use]
pub fn format_bytes_with_options(bytes: u64, options: &BytesOptions) -> String {
    let base = options.standard.base() as f64;
    let (value, unit_symbol) = if let Some(target_unit) = options.unit {
        let factor = unit_factor(target_unit, options.standard);
        (bytes as f64 / factor, target_unit.symbol())
    } else if bytes == 0 {
        (0.0, "B")
    } else {
        let units = match options.standard {
            BytesStandard::Decimal => ["B", "KB", "MB", "GB", "TB", "PB"],
            BytesStandard::Binary => ["B", "KB", "MB", "GB", "TB", "PB"],
        };

        let mut current_value = bytes as f64;
        let mut unit_index = 0;
        while current_value >= base && unit_index + 1 < units.len() {
            current_value /= base;
            unit_index += 1;
        }
        (current_value, units[unit_index])
    };

    let formatted_number = format_number(
        value,
        options.decimal_places,
        options.fixed_decimals,
        &options.thousands_separator,
    );

    format!("{formatted_number}{}{unit_symbol}", options.unit_separator)
}

fn unit_factor(unit: BytesUnit, standard: BytesStandard) -> f64 {
    let base = match unit {
        BytesUnit::KiB | BytesUnit::MiB | BytesUnit::GiB | BytesUnit::TiB | BytesUnit::PiB => {
            1024.0
        }
        _ => standard.base() as f64,
    };

    match unit {
        BytesUnit::B => 1.0,
        BytesUnit::KB | BytesUnit::KiB => base,
        BytesUnit::MB | BytesUnit::MiB => base * base,
        BytesUnit::GB | BytesUnit::GiB => base * base * base,
        BytesUnit::TB | BytesUnit::TiB => base * base * base * base,
        BytesUnit::PB | BytesUnit::PiB => base * base * base * base * base,
    }
}

fn format_number(
    value: f64,
    decimal_places: usize,
    fixed_decimals: bool,
    thousands_separator: &str,
) -> String {
    let rounded = format!("{value:.precision$}", precision = decimal_places);
    let (integer_part, fractional_part) = match rounded.split_once('.') {
        Some((int_part, frac_part)) => (int_part, Some(frac_part)),
        None => (rounded.as_str(), None),
    };

    let separated_integer = if thousands_separator.is_empty() {
        integer_part.to_string()
    } else {
        insert_thousands_separator(integer_part, thousands_separator)
    };

    match fractional_part {
        None => separated_integer,
        Some(fraction) => {
            if fixed_decimals {
                format!("{separated_integer}.{fraction}")
            } else {
                let trimmed_fraction = fraction.trim_end_matches('0');
                if trimmed_fraction.is_empty() {
                    separated_integer
                } else {
                    format!("{separated_integer}.{trimmed_fraction}")
                }
            }
        }
    }
}

fn insert_thousands_separator(digits: &str, separator: &str) -> String {
    let mut result = String::with_capacity(digits.len() + (digits.len() / 3) * separator.len());
    let remainder = digits.len() % 3;
    let initial_chunk_len = if remainder == 0 { 3 } else { remainder };

    result.push_str(&digits[..initial_chunk_len]);
    let mut current_offset = initial_chunk_len;
    while current_offset < digits.len() {
        result.push_str(separator);
        result.push_str(&digits[current_offset..current_offset + 3]);
        current_offset += 3;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_bytes_valid_inputs() {
        assert_eq!(parse_bytes("0").unwrap(), 0);
        assert_eq!(parse_bytes("1024").unwrap(), 1024);
        assert_eq!(parse_bytes("1 B").unwrap(), 1);
        assert_eq!(parse_bytes("10 bytes").unwrap(), 10);
        assert_eq!(parse_bytes("1KB").unwrap(), 1024);
        assert_eq!(parse_bytes("1.5 KB").unwrap(), 1536);
        assert_eq!(parse_bytes("10MB").unwrap(), 10_485_760);
        assert_eq!(parse_bytes("2.5 GB").unwrap(), 2_684_354_560);
        assert_eq!(parse_bytes("1 TB").unwrap(), 1_099_511_627_776);
        assert_eq!(parse_bytes("1 PB").unwrap(), 1_125_899_906_842_624);
        assert_eq!(parse_bytes("+500 kb").unwrap(), 512_000);
        assert_eq!(parse_bytes("1 KiB").unwrap(), 1024);
        assert_eq!(parse_bytes("1 MiB").unwrap(), 1_048_576);
    }

    #[test]
    fn test_parse_bytes_error_handling() {
        assert_eq!(parse_bytes("").unwrap_err(), ParseBytesError::EmptyInput);
        assert_eq!(parse_bytes("   ").unwrap_err(), ParseBytesError::EmptyInput);
        assert!(matches!(
            parse_bytes("-10MB").unwrap_err(),
            ParseBytesError::InvalidFormat(_)
        ));
        assert!(matches!(
            parse_bytes("+").unwrap_err(),
            ParseBytesError::InvalidFormat(_)
        ));
        assert!(matches!(
            parse_bytes("invalid").unwrap_err(),
            ParseBytesError::InvalidFormat(_)
        ));
        assert!(matches!(
            parse_bytes("100 XYZ").unwrap_err(),
            ParseBytesError::UnknownUnit(_)
        ));
        assert_eq!(
            parse_bytes("999999999999999999999 PB").unwrap_err(),
            ParseBytesError::Overflow
        );
    }

    #[test]
    fn test_format_bytes_defaults() {
        assert_eq!(format_bytes(0), "0 B");
        assert_eq!(format_bytes(500), "500 B");
        assert_eq!(format_bytes(1024), "1 KB");
        assert_eq!(format_bytes(1536), "1.5 KB");
        assert_eq!(format_bytes(10_485_760), "10 MB");
        assert_eq!(format_bytes(1_073_741_824), "1 GB");
        assert_eq!(format_bytes(1_099_511_627_776), "1 TB");
        assert_eq!(format_bytes(1_125_899_906_842_624), "1 PB");
    }

    #[test]
    fn test_format_bytes_options() {
        let fixed = BytesOptions::new()
            .with_decimal_places(2)
            .with_fixed_decimals(true);
        assert_eq!(format_bytes_with_options(1024, &fixed), "1.00 KB");

        let separator = BytesOptions::new().with_thousands_separator(",");
        assert_eq!(
            format_bytes_with_options(1_000_000, &separator),
            "976.56 KB"
        );

        let forced_unit = BytesOptions::new().with_unit(BytesUnit::MB);
        assert_eq!(
            format_bytes_with_options(1024 * 1024 * 5, &forced_unit),
            "5 MB"
        );

        let decimal_std = BytesOptions::new().with_standard(BytesStandard::Decimal);
        assert_eq!(format_bytes_with_options(1000, &decimal_std), "1 KB");

        let custom_sep = BytesOptions::new().with_unit_separator("-");
        assert_eq!(format_bytes_with_options(1024, &custom_sep), "1-KB");

        let zero_decimals = BytesOptions::new().with_decimal_places(0);
        assert_eq!(format_bytes_with_options(1024, &zero_decimals), "1 KB");

        let millions_bytes = BytesOptions::new()
            .with_unit(BytesUnit::B)
            .with_thousands_separator(",");
        assert_eq!(
            format_bytes_with_options(10_000_000, &millions_bytes),
            "10,000,000 B"
        );
    }

    #[test]
    fn test_bytes_all_edge_cases() {
        let units = [
            BytesUnit::B,
            BytesUnit::KB,
            BytesUnit::MB,
            BytesUnit::GB,
            BytesUnit::TB,
            BytesUnit::PB,
            BytesUnit::KiB,
            BytesUnit::MiB,
            BytesUnit::GiB,
            BytesUnit::TiB,
            BytesUnit::PiB,
        ];
        for unit in units {
            let symbol = unit.symbol();
            assert!(!symbol.is_empty());
            let options = BytesOptions::new().with_unit(unit);
            let formatted = format_bytes_with_options(1024 * 1024, &options);
            assert!(formatted.ends_with(symbol));
        }

        assert_eq!(BytesStandard::Decimal.base(), 1000);
        assert_eq!(BytesStandard::Binary.base(), 1024);

        let errors = [
            ParseBytesError::EmptyInput,
            ParseBytesError::InvalidFormat("bad".to_string()),
            ParseBytesError::InvalidNumber("NaN".to_string()),
            ParseBytesError::UnknownUnit("foo".to_string()),
            ParseBytesError::Overflow,
        ];
        for error in &errors {
            let displayed = format!("{error}");
            assert!(!displayed.is_empty());
        }

        assert_eq!(parse_bytes("1 TB").unwrap(), 1_099_511_627_776);
        assert_eq!(parse_bytes("1 TiB").unwrap(), 1_099_511_627_776);
        assert_eq!(parse_bytes("1 PB").unwrap(), 1_125_899_906_842_624);
        assert_eq!(parse_bytes("1 PiB").unwrap(), 1_125_899_906_842_624);
        assert_eq!(parse_bytes("1 GiB").unwrap(), 1_073_741_824);
        assert_eq!(parse_bytes("1 gibibyte").unwrap(), 1_073_741_824);
        assert_eq!(parse_bytes("1 gibibytes").unwrap(), 1_073_741_824);

        assert!(parse_bytes("1.2.3 MB").is_err());
        assert!(parse_bytes(". MB").is_err());
        assert!(parse_bytes("NaN MB").is_err());
        assert!(parse_bytes("inf MB").is_err());
        let huge_bytes = format!("{} MB", "9".repeat(320));
        assert!(parse_bytes(&huge_bytes).is_err());
    }
}
