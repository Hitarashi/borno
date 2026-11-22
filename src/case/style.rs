//! Unified casing style representation providing transformation, validation, and detection.

use std::{borrow::Cow, fmt, str::FromStr};

use super::{
    predicates::{
        is_camel_or_pascal, is_delimited_ascii, is_delimited_title, is_sentence_case_internal,
    },
    transform::{
        transform_camel_or_pascal, transform_cased, transform_sentence_case, transform_title,
    },
};

/// Error returned when parsing an unrecognized casing style string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseCaseError {
    invalid_name: String,
}

impl ParseCaseError {
    /// Returns the invalid string that failed to parse as a casing style.
    #[must_use]
    pub fn invalid_name(&self) -> &str {
        &self.invalid_name
    }
}

impl fmt::Display for ParseCaseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "unrecognized casing style: '{}'",
            self.invalid_name
        )
    }
}

impl std::error::Error for ParseCaseError {}

/// Unified casing style representation providing transformation, validation, and detection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Case {
    /// `snake_case` (e.g. `user_account_id`)
    #[cfg(feature = "case-snake")]
    Snake,
    /// `SCREAMING_SNAKE_CASE` (e.g. `USER_ACCOUNT_ID`)
    #[cfg(feature = "case-screaming-snake")]
    ScreamingSnake,
    /// `camelCase` (e.g. `userAccountId`)
    #[cfg(feature = "case-camel")]
    Camel,
    /// `PascalCase` / `StudlyCaps` (e.g. `UserAccountId`)
    #[cfg(feature = "case-pascal")]
    Pascal,
    /// `kebab-case` (e.g. `user-account-id`)
    #[cfg(feature = "case-kebab")]
    Kebab,
    /// `Train-Case` / `Header-Case` (e.g. `User-Account-Id`)
    #[cfg(feature = "case-train")]
    Train,
    /// `Title Case` (e.g. `User Account Id`)
    #[cfg(feature = "case-title")]
    Title,
    /// `Sentence case` (e.g. `User account id`)
    #[cfg(feature = "case-sentence")]
    Sentence,
    /// `dot.case` (e.g. `user.account.id`)
    #[cfg(feature = "case-dot")]
    Dot,
    /// `no case` (e.g. `user account id`)
    #[cfg(feature = "case-no")]
    No,
    /// `Capital Case` (e.g. `User Account Id`)
    #[cfg(feature = "case-capital")]
    Capital,
}

impl Case {
    /// Slice of all available casing styles under current feature flags.
    pub const ALL: &'static [Case] = &[
        #[cfg(feature = "case-snake")]
        Case::Snake,
        #[cfg(feature = "case-screaming-snake")]
        Case::ScreamingSnake,
        #[cfg(feature = "case-camel")]
        Case::Camel,
        #[cfg(feature = "case-pascal")]
        Case::Pascal,
        #[cfg(feature = "case-kebab")]
        Case::Kebab,
        #[cfg(feature = "case-train")]
        Case::Train,
        #[cfg(feature = "case-title")]
        Case::Title,
        #[cfg(feature = "case-sentence")]
        Case::Sentence,
        #[cfg(feature = "case-dot")]
        Case::Dot,
        #[cfg(feature = "case-no")]
        Case::No,
        #[cfg(feature = "case-capital")]
        Case::Capital,
    ];

    /// Canonical string identifier for this casing style.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            #[cfg(feature = "case-snake")]
            Case::Snake => "snake_case",
            #[cfg(feature = "case-screaming-snake")]
            Case::ScreamingSnake => "SCREAMING_SNAKE_CASE",
            #[cfg(feature = "case-camel")]
            Case::Camel => "camelCase",
            #[cfg(feature = "case-pascal")]
            Case::Pascal => "PascalCase",
            #[cfg(feature = "case-kebab")]
            Case::Kebab => "kebab-case",
            #[cfg(feature = "case-train")]
            Case::Train => "Train-Case",
            #[cfg(feature = "case-title")]
            Case::Title => "Title Case",
            #[cfg(feature = "case-sentence")]
            Case::Sentence => "Sentence case",
            #[cfg(feature = "case-dot")]
            Case::Dot => "dot.case",
            #[cfg(feature = "case-no")]
            Case::No => "no case",
            #[cfg(feature = "case-capital")]
            Case::Capital => "Capital Case",
        }
    }

    /// Transforms an input string slice into this casing style.
    ///
    /// Returns `Cow::Borrowed` with zero allocations if the input already conforms to this casing style.
    #[must_use]
    pub fn transform<'a>(&self, input: &'a str) -> Cow<'a, str> {
        if self.is_case(input) {
            return Cow::Borrowed(input);
        }

        match self {
            #[cfg(feature = "case-snake")]
            Case::Snake => transform_cased(input, '_', false),
            #[cfg(feature = "case-screaming-snake")]
            Case::ScreamingSnake => transform_cased(input, '_', true),
            #[cfg(feature = "case-camel")]
            Case::Camel => transform_camel_or_pascal(input, false),
            #[cfg(feature = "case-pascal")]
            Case::Pascal => transform_camel_or_pascal(input, true),
            #[cfg(feature = "case-kebab")]
            Case::Kebab => transform_cased(input, '-', false),
            #[cfg(feature = "case-train")]
            Case::Train => transform_title(input, '-'),
            #[cfg(feature = "case-title")]
            Case::Title => transform_title(input, ' '),
            #[cfg(feature = "case-sentence")]
            Case::Sentence => transform_sentence_case(input),
            #[cfg(feature = "case-dot")]
            Case::Dot => transform_cased(input, '.', false),
            #[cfg(feature = "case-no")]
            Case::No => transform_cased(input, ' ', false),
            #[cfg(feature = "case-capital")]
            Case::Capital => transform_title(input, ' '),
        }
    }

    /// Checks whether an input string slice conforms to this casing style without allocating.
    #[must_use]
    pub fn is_case(&self, input: &str) -> bool {
        match self {
            #[cfg(feature = "case-snake")]
            Case::Snake => is_delimited_ascii(input, b'_', false),
            #[cfg(feature = "case-screaming-snake")]
            Case::ScreamingSnake => is_delimited_ascii(input, b'_', true),
            #[cfg(feature = "case-camel")]
            Case::Camel => is_camel_or_pascal(input, false),
            #[cfg(feature = "case-pascal")]
            Case::Pascal => is_camel_or_pascal(input, true),
            #[cfg(feature = "case-kebab")]
            Case::Kebab => is_delimited_ascii(input, b'-', false),
            #[cfg(feature = "case-train")]
            Case::Train => is_delimited_title(input, '-'),
            #[cfg(feature = "case-title")]
            Case::Title => is_delimited_title(input, ' '),
            #[cfg(feature = "case-sentence")]
            Case::Sentence => is_sentence_case_internal(input),
            #[cfg(feature = "case-dot")]
            Case::Dot => is_delimited_ascii(input, b'.', false),
            #[cfg(feature = "case-no")]
            Case::No => is_delimited_ascii(input, b' ', false),
            #[cfg(feature = "case-capital")]
            Case::Capital => is_delimited_title(input, ' '),
        }
    }

    /// Detects the casing style that the input string slice conforms to, if any.
    #[must_use]
    pub fn detect(input: &str) -> Option<Case> {
        if input.is_empty() {
            return None;
        }

        Self::ALL
            .iter()
            .copied()
            .find(|candidate| candidate.is_case(input))
    }
}

impl fmt::Display for Case {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for Case {
    type Err = ParseCaseError;

    fn from_str(string_value: &str) -> Result<Self, Self::Err> {
        let normalized = string_value.trim().to_ascii_lowercase();
        match normalized.as_str() {
            #[cfg(feature = "case-snake")]
            "snake" | "snake_case" | "snake-case" => Ok(Case::Snake),
            #[cfg(feature = "case-screaming-snake")]
            "screaming_snake"
            | "screaming_snake_case"
            | "screaming-snake"
            | "screaming-snake-case"
            | "macro"
            | "constant" => Ok(Case::ScreamingSnake),
            #[cfg(feature = "case-camel")]
            "camel" | "camel_case" | "camelcase" | "camel-case" => Ok(Case::Camel),
            #[cfg(feature = "case-pascal")]
            "pascal" | "pascal_case" | "pascalcase" | "pascal-case" | "studly" | "class" => {
                Ok(Case::Pascal)
            }
            #[cfg(feature = "case-kebab")]
            "kebab" | "kebab_case" | "kebabcase" | "kebab-case" | "slug" => Ok(Case::Kebab),
            #[cfg(feature = "case-train")]
            "train" | "train_case" | "traincase" | "train-case" | "header" | "header-case" => {
                Ok(Case::Train)
            }
            #[cfg(feature = "case-title")]
            "title" | "title_case" | "titlecase" | "title-case" => Ok(Case::Title),
            #[cfg(feature = "case-sentence")]
            "sentence" | "sentence_case" | "sentencecase" | "sentence-case" => Ok(Case::Sentence),
            #[cfg(feature = "case-dot")]
            "dot" | "dot_case" | "dotcase" | "dot-case" => Ok(Case::Dot),
            #[cfg(feature = "case-no")]
            "no" | "no_case" | "nocase" | "no-case" => Ok(Case::No),
            #[cfg(feature = "case-capital")]
            "capital" | "capital_case" | "capitalcase" | "capital-case" => Ok(Case::Capital),
            _ => Err(ParseCaseError {
                invalid_name: string_value.to_string(),
            }),
        }
    }
}
