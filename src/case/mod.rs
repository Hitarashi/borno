//! Casing transformation engine, predicates, and framework naming conventions.

pub mod apa;
#[cfg(feature = "conventions")]
pub mod conventions;
pub mod predicates;
pub mod scanner;
pub mod style;
pub mod transform;

use std::borrow::Cow;

pub use apa::{is_title_case_apa, to_title_case_apa};
#[cfg(feature = "conventions")]
pub use conventions::{
    classify, deconstantize, demodulize, foreign_key, foreign_key_with_suffix, headline, humanize,
    is_foreign_key, is_foreign_key_with_suffix, tableize,
};
// Internal re-exports for crate modules
#[cfg(feature = "grammar")]
pub(crate) use scanner::split_compound_tail;
pub use style::{Case, ParseCaseError};
use transform::append_cased;

/// Converts an input string slice to the specified casing style.
#[inline]
pub fn to_case<'a>(input: &'a str, target_case: Case) -> Cow<'a, str> {
    target_case.transform(input)
}

/// Checks whether an input string conforms to the specified casing style.
#[inline]
#[must_use]
pub fn is_case(input: &str, target_case: Case) -> bool {
    target_case.is_case(input)
}

/// Detects the casing style of an input string slice, if any.
#[inline]
#[must_use]
pub fn detect_case(input: &str) -> Option<Case> {
    Case::detect(input)
}

/// Alias for [`tableize`] (generates pluralized, snake_cased database table name).
#[inline]
#[cfg(feature = "conventions")]
pub fn to_table_case(input: &str) -> String {
    conventions::tableize(input)
}

/// Checks if input is a valid table name (pluralized snake_case).
#[inline]
#[cfg(all(feature = "conventions", feature = "grammar"))]
pub fn is_table_case(input: &str) -> bool {
    use crate::grammar::is_plural;
    is_snake_case(input) && is_plural(input)
}

// -----------------------------------------------------------------------------
// Direct Top-Level Convenience Functions
// -----------------------------------------------------------------------------

/// Converts an input string slice to `snake_case`.
#[inline]
#[cfg(feature = "case-snake")]
pub fn to_snake_case(input: &str) -> Cow<'_, str> {
    Case::Snake.transform(input)
}

/// Checks if an input string slice is already valid `snake_case`.
#[inline]
#[cfg(feature = "case-snake")]
#[must_use]
pub fn is_snake_case(input: &str) -> bool {
    Case::Snake.is_case(input)
}

/// Converts an input string slice to `SCREAMING_SNAKE_CASE`.
#[inline]
#[cfg(feature = "case-screaming-snake")]
pub fn to_screaming_snake_case(input: &str) -> Cow<'_, str> {
    Case::ScreamingSnake.transform(input)
}

/// Checks if an input string slice is already valid `SCREAMING_SNAKE_CASE`.
#[inline]
#[cfg(feature = "case-screaming-snake")]
#[must_use]
pub fn is_screaming_snake_case(input: &str) -> bool {
    Case::ScreamingSnake.is_case(input)
}

/// Converts an input string slice to `camelCase`.
#[inline]
#[cfg(feature = "case-camel")]
pub fn to_camel_case(input: &str) -> Cow<'_, str> {
    Case::Camel.transform(input)
}

/// Checks if an input string slice is already valid `camelCase`.
#[inline]
#[cfg(feature = "case-camel")]
#[must_use]
pub fn is_camel_case(input: &str) -> bool {
    Case::Camel.is_case(input)
}

/// Converts an input string slice to `PascalCase`.
#[inline]
#[cfg(feature = "case-pascal")]
pub fn to_pascal_case(input: &str) -> Cow<'_, str> {
    Case::Pascal.transform(input)
}

/// Checks if an input string slice is already valid `PascalCase`.
#[inline]
#[cfg(feature = "case-pascal")]
#[must_use]
pub fn is_pascal_case(input: &str) -> bool {
    Case::Pascal.is_case(input)
}

/// Alias for [`to_pascal_case`].
#[inline]
#[cfg(feature = "case-pascal")]
pub fn to_studly_case(input: &str) -> Cow<'_, str> {
    to_pascal_case(input)
}

/// Checks if an input string slice is in PascalCase / StudlyCase.
#[inline]
#[cfg(feature = "case-pascal")]
#[must_use]
pub fn is_studly_case(input: &str) -> bool {
    is_pascal_case(input)
}

/// Alias for [`to_pascal_case`].
#[inline]
#[cfg(feature = "case-pascal")]
pub fn to_class_case(input: &str) -> Cow<'_, str> {
    to_pascal_case(input)
}

/// Checks if an input string slice is in ClassCase.
#[inline]
#[cfg(feature = "case-pascal")]
#[must_use]
pub fn is_class_case(input: &str) -> bool {
    is_pascal_case(input)
}

/// Converts an input string slice to `kebab-case`.
#[inline]
#[cfg(feature = "case-kebab")]
pub fn to_kebab_case(input: &str) -> Cow<'_, str> {
    Case::Kebab.transform(input)
}

/// Checks if an input string slice is already valid `kebab-case`.
#[inline]
#[cfg(feature = "case-kebab")]
#[must_use]
pub fn is_kebab_case(input: &str) -> bool {
    Case::Kebab.is_case(input)
}

/// Converts an input string slice to `Train-Case` / `Header-Case`.
#[inline]
#[cfg(feature = "case-train")]
pub fn to_train_case(input: &str) -> Cow<'_, str> {
    Case::Train.transform(input)
}

/// Checks if an input string slice is already valid `Train-Case`.
#[inline]
#[cfg(feature = "case-train")]
#[must_use]
pub fn is_train_case(input: &str) -> bool {
    Case::Train.is_case(input)
}

/// Converts an input string slice to `Title Case`.
#[inline]
#[cfg(feature = "case-title")]
pub fn to_title_case(input: &str) -> Cow<'_, str> {
    Case::Title.transform(input)
}

/// Checks if an input string slice is already valid `Title Case`.
#[inline]
#[cfg(feature = "case-title")]
#[must_use]
pub fn is_title_case(input: &str) -> bool {
    Case::Title.is_case(input)
}

/// Converts an input string slice to `Sentence case`.
#[inline]
#[cfg(feature = "case-sentence")]
pub fn to_sentence_case(input: &str) -> Cow<'_, str> {
    Case::Sentence.transform(input)
}

/// Checks if an input string slice is already valid `Sentence case`.
#[inline]
#[cfg(feature = "case-sentence")]
#[must_use]
pub fn is_sentence_case(input: &str) -> bool {
    Case::Sentence.is_case(input)
}

/// Converts an input string slice to `dot.case`.
#[inline]
#[cfg(feature = "case-dot")]
pub fn to_dot_case(input: &str) -> Cow<'_, str> {
    Case::Dot.transform(input)
}

/// Checks if an input string slice is already valid `dot.case`.
#[inline]
#[cfg(feature = "case-dot")]
#[must_use]
pub fn is_dot_case(input: &str) -> bool {
    Case::Dot.is_case(input)
}

/// Converts an input string slice to `no case`.
#[inline]
#[cfg(feature = "case-no")]
pub fn to_no_case(input: &str) -> Cow<'_, str> {
    Case::No.transform(input)
}

/// Checks if an input string slice is already valid `no case`.
#[inline]
#[cfg(feature = "case-no")]
#[must_use]
pub fn is_no_case(input: &str) -> bool {
    Case::No.is_case(input)
}

/// Converts an input string slice to `Capital Case`.
#[inline]
#[cfg(feature = "case-capital")]
pub fn to_capital_case(input: &str) -> Cow<'_, str> {
    Case::Capital.transform(input)
}

/// Checks if an input string slice is already valid `Capital Case`.
#[inline]
#[cfg(feature = "case-capital")]
#[must_use]
pub fn is_capital_case(input: &str) -> bool {
    Case::Capital.is_case(input)
}

/// Capitalizes only the first Unicode scalar character of a string slice without altering the rest.
pub fn capitalize_first(input: &str) -> Cow<'_, str> {
    let mut chars = input.chars();
    let Some(first_char) = chars.next() else {
        return Cow::Borrowed("");
    };
    if first_char.is_uppercase() {
        return Cow::Borrowed(input);
    }
    let mut result = String::with_capacity(input.len());
    for upper in first_char.to_uppercase() {
        result.push(upper);
    }
    result.push_str(chars.as_str());
    Cow::Owned(result)
}

/// Lowercases only the first Unicode scalar character of a string slice without altering the rest.
pub fn uncapitalize_first(input: &str) -> Cow<'_, str> {
    let mut chars = input.chars();
    let Some(first_char) = chars.next() else {
        return Cow::Borrowed("");
    };
    if first_char.is_lowercase() {
        return Cow::Borrowed(input);
    }
    let mut result = String::with_capacity(input.len());
    for lower in first_char.to_lowercase() {
        result.push(lower);
    }
    result.push_str(chars.as_str());
    Cow::Owned(result)
}

/// Alias for [`capitalize_first`].
#[inline]
pub fn ucfirst(input: &str) -> Cow<'_, str> {
    capitalize_first(input)
}

/// Alias for [`uncapitalize_first`].
#[inline]
pub fn lcfirst(input: &str) -> Cow<'_, str> {
    uncapitalize_first(input)
}

/// Inverts the case of each character in a string slice.
#[must_use]
pub fn swap_case(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    for character in input.chars() {
        if character.is_uppercase() {
            append_cased(&mut result, character, false);
        } else if character.is_lowercase() {
            append_cased(&mut result, character, true);
        } else {
            result.push(character);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_enum_transform_and_predicates() {
        assert_eq!(Case::Snake.transform("userProfileId"), "user_profile_id");
        assert!(Case::Snake.is_case("user_profile_id"));
        assert!(!Case::Snake.is_case("userProfileId"));

        assert_eq!(Case::Camel.transform("user_profile_id"), "userProfileId");
        assert!(Case::Camel.is_case("userProfileId"));

        assert_eq!(Case::Pascal.transform("user_profile_id"), "UserProfileId");
        assert!(Case::Pascal.is_case("UserProfileId"));

        assert_eq!(Case::Kebab.transform("user_profile_id"), "user-profile-id");
        assert!(Case::Kebab.is_case("user-profile-id"));

        assert_eq!(Case::Train.transform("user_profile_id"), "User-Profile-Id");
        assert!(Case::Train.is_case("User-Profile-Id"));

        assert_eq!(Case::Title.transform("user_profile_id"), "User Profile Id");
        assert!(Case::Title.is_case("User Profile Id"));

        assert_eq!(
            Case::Sentence.transform("user_profile_id"),
            "User profile id"
        );
        assert!(Case::Sentence.is_case("User profile id"));

        assert_eq!(Case::Dot.transform("user_profile_id"), "user.profile.id");
        assert!(Case::Dot.is_case("user.profile.id"));

        assert_eq!(Case::No.transform("user_profile_id"), "user profile id");
        assert!(Case::No.is_case("user profile id"));

        assert_eq!(Case::Capital.transform("user_profile"), "User Profile");
        assert!(Case::Capital.is_case("User Profile"));

        assert_eq!(
            Case::ScreamingSnake.transform("user_profile"),
            "USER_PROFILE"
        );
        assert!(Case::ScreamingSnake.is_case("USER_PROFILE"));
    }

    #[test]
    fn test_case_detection() {
        assert_eq!(Case::detect("user_profile"), Some(Case::Snake));
        assert_eq!(Case::detect("userProfile"), Some(Case::Camel));
        assert_eq!(Case::detect("UserProfile"), Some(Case::Pascal));
        assert_eq!(Case::detect("user-profile"), Some(Case::Kebab));
        assert_eq!(Case::detect("User-Profile"), Some(Case::Train));
        assert_eq!(Case::detect("user.profile"), Some(Case::Dot));
        assert_eq!(Case::detect("USER_PROFILE"), Some(Case::ScreamingSnake));
        assert_eq!(Case::detect(""), None);
    }

    #[test]
    fn test_case_from_str() {
        assert_eq!("snake".parse::<Case>().unwrap(), Case::Snake);
        assert_eq!("snake_case".parse::<Case>().unwrap(), Case::Snake);
        assert_eq!("snake-case".parse::<Case>().unwrap(), Case::Snake);
        assert_eq!("camel".parse::<Case>().unwrap(), Case::Camel);
        assert_eq!("camelCase".parse::<Case>().unwrap(), Case::Camel);
        assert_eq!("camel-case".parse::<Case>().unwrap(), Case::Camel);
        assert_eq!("pascal".parse::<Case>().unwrap(), Case::Pascal);
        assert_eq!("PascalCase".parse::<Case>().unwrap(), Case::Pascal);
        assert_eq!("pascal-case".parse::<Case>().unwrap(), Case::Pascal);
        assert_eq!("kebab".parse::<Case>().unwrap(), Case::Kebab);
        assert_eq!("kebab-case".parse::<Case>().unwrap(), Case::Kebab);
        assert_eq!("kebab_case".parse::<Case>().unwrap(), Case::Kebab);
        assert_eq!("train".parse::<Case>().unwrap(), Case::Train);
        assert_eq!("train-case".parse::<Case>().unwrap(), Case::Train);
        assert_eq!("train_case".parse::<Case>().unwrap(), Case::Train);
        assert_eq!("title".parse::<Case>().unwrap(), Case::Title);
        assert_eq!("title-case".parse::<Case>().unwrap(), Case::Title);
        assert_eq!("title_case".parse::<Case>().unwrap(), Case::Title);
        assert_eq!("sentence".parse::<Case>().unwrap(), Case::Sentence);
        assert_eq!("sentence-case".parse::<Case>().unwrap(), Case::Sentence);
        assert_eq!("sentence_case".parse::<Case>().unwrap(), Case::Sentence);
        assert_eq!("dot".parse::<Case>().unwrap(), Case::Dot);
        assert_eq!("dot-case".parse::<Case>().unwrap(), Case::Dot);
        assert_eq!("dot_case".parse::<Case>().unwrap(), Case::Dot);
        assert_eq!("no".parse::<Case>().unwrap(), Case::No);
        assert_eq!("no-case".parse::<Case>().unwrap(), Case::No);
        assert_eq!("no_case".parse::<Case>().unwrap(), Case::No);
        assert_eq!("capital".parse::<Case>().unwrap(), Case::Capital);
        assert_eq!("capital-case".parse::<Case>().unwrap(), Case::Capital);
        assert_eq!("capital_case".parse::<Case>().unwrap(), Case::Capital);
        assert_eq!(
            "screaming_snake".parse::<Case>().unwrap(),
            Case::ScreamingSnake
        );
        assert_eq!(
            "screaming_snake_case".parse::<Case>().unwrap(),
            Case::ScreamingSnake
        );
        assert_eq!(
            "screaming-snake".parse::<Case>().unwrap(),
            Case::ScreamingSnake
        );
        assert_eq!(
            "screaming-snake-case".parse::<Case>().unwrap(),
            Case::ScreamingSnake
        );

        let err = "invalid_case".parse::<Case>().unwrap_err();
        assert_eq!(err.invalid_name(), "invalid_case");
        assert_eq!(
            format!("{err}"),
            "unrecognized casing style: 'invalid_case'"
        );

        for case in Case::ALL {
            assert_eq!(format!("{case}"), case.as_str());
        }
    }

    #[test]
    fn test_split_compound_tail() {
        assert_eq!(
            split_compound_tail("user_profile"),
            Some(("user_", "profile"))
        );
        assert_eq!(
            split_compound_tail("user-profile"),
            Some(("user-", "profile"))
        );
        assert_eq!(
            split_compound_tail("user profile"),
            Some(("user ", "profile"))
        );
        assert_eq!(
            split_compound_tail("userProfile"),
            Some(("user", "Profile"))
        );
        assert_eq!(
            split_compound_tail("UserProfile"),
            Some(("User", "Profile"))
        );
        assert_eq!(
            split_compound_tail("HTTPResponse"),
            Some(("HTTP", "Response"))
        );
        assert_eq!(
            split_compound_tail("user2Profile"),
            Some(("user2", "Profile"))
        );
        assert_eq!(
            split_compound_tail("userProfile2"),
            Some(("userProfile", "2"))
        );
        assert_eq!(split_compound_tail("überUser"), Some(("über", "User")));
        assert_eq!(split_compound_tail("ÜberUser"), Some(("Über", "User")));
        assert_eq!(split_compound_tail("HTTPÜber"), Some(("HTTP", "Über")));
        assert_eq!(split_compound_tail("über123"), Some(("über", "123")));
        assert_eq!(split_compound_tail("123über"), Some(("123", "über")));
        assert_eq!(split_compound_tail("über"), None);
        assert_eq!(split_compound_tail("übers"), None);
        assert_eq!(split_compound_tail("ü"), None);
        assert_eq!(split_compound_tail("users"), None);
        assert_eq!(split_compound_tail(""), None);
    }
}
