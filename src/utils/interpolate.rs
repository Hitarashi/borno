//! Template and placeholder interpolation utilities.

use std::collections::HashMap;

/// Interpolates named parameters into a template string slice using a slice of key-value pairs.
///
/// Supports:
/// - Double braces: `{{ key }}` and `{{key}}`
/// - Single braces: `{ key }}` and `{key}`
/// - Escaped braces: `\{{ key }}` and `\{ key }` (output as unescaped literals)
/// - Colon prefix: `:key`
///
/// # Examples
/// ```
/// use borno::interpolate;
///
/// let template = "Hello {{ name }}, welcome to :site!";
/// let params = [("name", "Alice"), ("site", "Example")];
/// assert_eq!(interpolate(template, &params), "Hello Alice, welcome to Example!");
///
/// let escaped = "Use \\{{ variable }} in your template";
/// assert_eq!(interpolate(escaped, &params), "Use {{ variable }} in your template");
/// ```
pub fn interpolate(template: &str, params: &[(&str, &str)]) -> String {
    interpolate_pairs(template, params.iter().copied())
}

/// Interpolates named parameters into a template string from an arbitrary key-value iterator.
///
/// # Examples
/// ```
/// use std::collections::HashMap;
/// use borno::interpolate_pairs;
///
/// let mut map = HashMap::new();
/// map.insert("field", "email");
/// map.insert("rule", "required");
///
/// let result = interpolate_pairs("The :field is :rule", &map);
/// assert_eq!(result, "The email is required");
/// ```
pub fn interpolate_pairs<K, V, I>(template: &str, params: I) -> String
where
    I: IntoIterator<Item = (K, V)>,
    K: AsRef<str>,
    V: AsRef<str>,
{
    if template.is_empty() {
        return String::new();
    }

    let param_map: HashMap<String, String> = params
        .into_iter()
        .map(|(k, v)| (k.as_ref().to_string(), v.as_ref().to_string()))
        .collect();

    if param_map.is_empty() {
        // Still need to unescape any escaped braces
        return unescape_braces(template);
    }

    let mut result = String::with_capacity(template.len());
    let chars: Vec<(usize, char)> = template.char_indices().collect();
    let total_chars = chars.len();
    let mut index = 0;

    while index < total_chars {
        let (byte_offset, current_char) = chars[index];

        // Case 1: Escaped double brace \{{ or single brace \{
        if current_char == '\\' && index + 1 < total_chars && chars[index + 1].1 == '{' {
            if index + 2 < total_chars && chars[index + 2].1 == '{' {
                result.push_str("{{");
                index += 3;
            } else {
                result.push('{');
                index += 2;
            }
            continue;
        }

        // Case 2: Double brace {{ key }}
        if current_char == '{' && index + 1 < total_chars && chars[index + 1].1 == '{' {
            let search_start = byte_offset + 2;
            if let Some(close_offset) = template[search_start..].find("}}") {
                let key_raw = &template[search_start..search_start + close_offset];
                let key = key_raw.trim();
                if let Some(value) = param_map.get(key) {
                    result.push_str(value);
                    let target_byte = search_start + close_offset + 2;
                    while index < total_chars && chars[index].0 < target_byte {
                        index += 1;
                    }
                    continue;
                }
            }
        }

        // Case 3: Single brace { key }
        if current_char == '{' && (index + 1 >= total_chars || chars[index + 1].1 != '{') {
            let search_start = byte_offset + 1;
            if let Some(close_offset) = template[search_start..].find('}') {
                let candidate = &template[search_start..search_start + close_offset];
                // Ensure no nested braces
                if !candidate.contains('{') {
                    let key = candidate.trim();
                    if let Some(value) = param_map.get(key) {
                        result.push_str(value);
                        let target_byte = search_start + close_offset + 1;
                        while index < total_chars && chars[index].0 < target_byte {
                            index += 1;
                        }
                        continue;
                    }
                }
            }
        }

        // Case 4: Colon placeholder :key
        if current_char == ':' && index + 1 < total_chars && is_ident_start(chars[index + 1].1) {
            let key_start = chars[index + 1].0;
            let mut key_end = key_start;
            let mut scan_index = index + 1;
            while scan_index < total_chars && is_ident_char(chars[scan_index].1) {
                key_end = chars[scan_index].0 + chars[scan_index].1.len_utf8();
                scan_index += 1;
            }
            let key = &template[key_start..key_end];
            if let Some(value) = param_map.get(key) {
                result.push_str(value);
                index = scan_index;
                continue;
            }
        }

        result.push(current_char);
        index += 1;
    }

    result
}

fn unescape_braces(template: &str) -> String {
    let mut result = String::with_capacity(template.len());
    let chars: Vec<(usize, char)> = template.char_indices().collect();
    let total_chars = chars.len();
    let mut index = 0;

    while index < total_chars {
        let (_, current_char) = chars[index];
        if current_char == '\\' && index + 1 < total_chars && chars[index + 1].1 == '{' {
            if index + 2 < total_chars && chars[index + 2].1 == '{' {
                result.push_str("{{");
                index += 3;
            } else {
                result.push('{');
                index += 2;
            }
            continue;
        }
        result.push(current_char);
        index += 1;
    }

    result
}

#[inline]
fn is_ident_start(character: char) -> bool {
    character.is_ascii_alphabetic() || character == '_'
}

#[inline]
fn is_ident_char(character: char) -> bool {
    character.is_ascii_alphanumeric() || character == '_'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpolate_mustache() {
        let params = [("name", "World"), ("item", "Rust")];
        assert_eq!(
            interpolate("Hello {{ name }} in {{item}}!", &params),
            "Hello World in Rust!"
        );
        assert_eq!(interpolate("Hello { name }!", &params), "Hello World!");
        assert_eq!(
            interpolate("Unmatched {{ missing }} and { unmapped }", &params),
            "Unmatched {{ missing }} and { unmapped }"
        );
        assert_eq!(
            interpolate("Nested {bad{brace}} test", &params),
            "Nested {bad{brace}} test"
        );
        assert_eq!(
            interpolate("Unclosed {{ token", &params),
            "Unclosed {{ token"
        );
        assert_eq!(interpolate("Unclosed { token", &params), "Unclosed { token");
    }

    #[test]
    fn test_interpolate_colon() {
        let params = [("field", "email"), ("rule", "valid_email"), ("_tag", "v1")];
        assert_eq!(
            interpolate(
                "The :field failed validation for :rule. Tag: :_tag",
                &params
            ),
            "The email failed validation for valid_email. Tag: v1"
        );
        assert_eq!(interpolate(":field:rule", &params), "emailvalid_email");
        assert_eq!(
            interpolate("No param :unknown here", &params),
            "No param :unknown here"
        );
    }

    #[test]
    fn test_interpolate_escaping_and_edges() {
        let params = [("key", "value")];
        assert_eq!(
            interpolate(r"Literal \{{ key }} and \{ key }", &params),
            "Literal {{ key }} and { key }"
        );
        assert_eq!(interpolate("", &params), "");
        assert_eq!(
            interpolate(r"Empty params \{{ test }} and \{ single }", &[]),
            "Empty params {{ test }} and { single }"
        );
        assert_eq!(
            interpolate("Just plain text with no tokens", &params),
            "Just plain text with no tokens"
        );
    }
}
