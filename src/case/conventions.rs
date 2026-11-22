//! Schema, identifier, and relational convention helpers.

use crate::{
    case::{is_snake_case, scanner::split_words, to_pascal_case, to_snake_case},
    grammar::{pluralize, singularize},
};

/// Converts a model name into a conventional database table name.
///
/// # Examples
/// ```
/// use borno::case::tableize;
///
/// assert_eq!(tableize("UserProfile"), "user_profiles");
/// assert_eq!(tableize("Person"), "people");
/// assert_eq!(tableize("Category"), "categories");
/// ```
pub fn tableize(model_name: &str) -> String {
    let snake = to_snake_case(model_name);
    pluralize(&snake).into_owned()
}

/// Converts a database table name into a conventional model class name.
///
/// # Examples
/// ```
/// use borno::case::classify;
///
/// assert_eq!(classify("user_profiles"), "UserProfile");
/// assert_eq!(classify("people"), "Person");
/// assert_eq!(classify("categories"), "Category");
/// ```
pub fn classify(table_name: &str) -> String {
    let singular = singularize(table_name);
    to_pascal_case(&singular).into_owned()
}

/// Generates a foreign key column name from a model name with the default `_id` suffix.
///
/// # Examples
/// ```
/// use borno::case::foreign_key;
///
/// assert_eq!(foreign_key("UserProfile"), "user_profile_id");
/// assert_eq!(foreign_key("Account"), "account_id");
/// ```
pub fn foreign_key(model_name: &str) -> String {
    foreign_key_with_suffix(model_name, "_id")
}

/// Generates a foreign key column name with a custom suffix.
pub fn foreign_key_with_suffix(model_name: &str, suffix: &str) -> String {
    let snake = to_snake_case(model_name);
    let mut result = String::with_capacity(snake.len() + suffix.len());
    result.push_str(&snake);
    result.push_str(suffix);
    result
}

/// Checks whether an identifier represents a foreign key ending in `_id`.
///
/// # Examples
/// ```
/// use borno::is_foreign_key;
///
/// assert!(is_foreign_key("user_id"));
/// assert!(is_foreign_key("user_account_id"));
/// assert!(!is_foreign_key("user"));
/// assert!(!is_foreign_key("_id"));
/// ```
pub fn is_foreign_key(input: &str) -> bool {
    is_foreign_key_with_suffix(input, "_id")
}

/// Checks whether an identifier represents a foreign key ending with the specified suffix.
pub fn is_foreign_key_with_suffix(input: &str, suffix: &str) -> bool {
    if suffix.is_empty() || !input.ends_with(suffix) || input.len() <= suffix.len() {
        return false;
    }
    let stem = &input[..input.len() - suffix.len()];
    is_snake_case(stem)
}

/// Transforms an identifier into capitalized, space-separated headline words.
///
/// # Examples
/// ```
/// use borno::headline;
///
/// assert_eq!(headline("EmailNotificationSent"), "Email Notification Sent");
/// assert_eq!(headline("user_profile_id"), "User Profile Id");
/// ```
pub fn headline(input: &str) -> String {
    let words = split_words(input);
    if words.is_empty() {
        return String::new();
    }

    let mut result = String::with_capacity(input.len() + words.len());
    let mut is_first = true;

    for word in words {
        if !is_first {
            result.push(' ');
        }
        is_first = false;

        let mut characters = word.chars();
        let first_char = characters.next().unwrap();
        for upper_char in first_char.to_uppercase() {
            result.push(upper_char);
        }
        for lower_char in characters.flat_map(char::to_lowercase) {
            result.push(lower_char);
        }
    }

    result
}

/// Transforms an identifier into human-readable sentence-style text.
///
/// Strips trailing `_id` foreign key suffixes, capitalizes the first word, and lowercases subsequent words.
///
/// # Examples
/// ```
/// use borno::humanize;
///
/// assert_eq!(humanize("employee_salary"), "Employee salary");
/// assert_eq!(humanize("author_id"), "Author");
/// assert_eq!(humanize("user_profile"), "User profile");
/// ```
pub fn humanize(input: &str) -> String {
    let base = input.strip_suffix("_id").unwrap_or(input);
    let words = split_words(base);
    if words.is_empty() {
        return String::new();
    }

    let mut result = String::with_capacity(input.len() + words.len());
    let mut is_first_word = true;

    for word in words {
        if !is_first_word {
            result.push(' ');
        }

        if is_first_word {
            let mut characters = word.chars();
            let first_char = characters.next().unwrap();
            for upper_char in first_char.to_uppercase() {
                result.push(upper_char);
            }
            for lower_char in characters.flat_map(char::to_lowercase) {
                result.push(lower_char);
            }
            is_first_word = false;
        } else {
            for lower_char in word.chars().flat_map(char::to_lowercase) {
                result.push(lower_char);
            }
        }
    }

    result
}

/// Removes module namespaces from a fully qualified type path.
///
/// Delegates to [`crate::utils::slice::demodulize`].
///
/// # Examples
/// ```
/// use borno::demodulize;
///
/// assert_eq!(demodulize("crate::services::UserService"), "UserService");
/// assert_eq!(demodulize("User"), "User");
/// ```
#[inline]
pub fn demodulize(input: &str) -> &str {
    crate::utils::slice::demodulize(input)
}

/// Removes the rightmost component from a namespace or module path.
///
/// Delegates to [`crate::utils::slice::deconstantize`].
///
/// # Examples
/// ```
/// use borno::deconstantize;
///
/// assert_eq!(deconstantize("crate::services::UserService"), "crate::services");
/// assert_eq!(deconstantize("User"), "");
/// ```
#[inline]
pub fn deconstantize(input: &str) -> &str {
    crate::utils::slice::deconstantize(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tableize() {
        assert_eq!(tableize("UserProfile"), "user_profiles");
        assert_eq!(tableize("Person"), "people");
        assert_eq!(tableize("Category"), "categories");
        assert_eq!(tableize("AdminUser"), "admin_users");
    }

    #[test]
    fn test_classify() {
        assert_eq!(classify("user_profiles"), "UserProfile");
        assert_eq!(classify("people"), "Person");
        assert_eq!(classify("categories"), "Category");
        assert_eq!(classify("admin_users"), "AdminUser");
    }

    #[test]
    fn test_foreign_key() {
        assert_eq!(foreign_key("UserProfile"), "user_profile_id");
        assert_eq!(foreign_key("Account"), "account_id");
        assert_eq!(
            foreign_key_with_suffix("UserProfile", "_pk"),
            "user_profile_pk"
        );
    }

    #[test]
    fn test_is_foreign_key() {
        assert!(is_foreign_key("user_id"));
        assert!(is_foreign_key("user_account_id"));
        assert!(!is_foreign_key("user"));
        assert!(!is_foreign_key("_id"));
        assert!(!is_foreign_key(""));
        assert!(!is_foreign_key("userId"));

        assert!(is_foreign_key_with_suffix("user_fk", "_fk"));
        assert!(!is_foreign_key_with_suffix("user_fk", ""));
    }

    #[test]
    fn test_headline() {
        assert_eq!(headline("EmailNotificationSent"), "Email Notification Sent");
        assert_eq!(headline("user_profile_id"), "User Profile Id");
        assert_eq!(headline(""), "");
    }

    #[test]
    fn test_humanize() {
        assert_eq!(humanize("employee_salary"), "Employee salary");
        assert_eq!(humanize("author_id"), "Author");
        assert_eq!(humanize("user_profile"), "User profile");
        assert_eq!(humanize(""), "");
    }

    #[test]
    fn test_demodulize_and_deconstantize() {
        assert_eq!(demodulize("crate::service::User"), "User");
        assert_eq!(demodulize("User"), "User");
        assert_eq!(demodulize(""), "");

        assert_eq!(deconstantize("crate::service::User"), "crate::service");
        assert_eq!(deconstantize("User"), "");
        assert_eq!(deconstantize(""), "");
    }
}
