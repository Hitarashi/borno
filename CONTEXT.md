# `borno` Context

## Purpose

`borno` provides fast, allocation-conscious English grammatical inflections (pluralization and singularization), casing
transformations, URL slugification, and string utilities.

## Core Capabilities

1. **Pluralization & Singularization**:
    - Standard English grammar rules: regular endings (`-s`, `-es`, `-ies`, `-ves`).
    - Irregular nouns (`person` $\to$ `people`, `child` $\to$ `children`, `man` $\to$ `men`, `ox` $\to$ `oxen`,
      `mouse` $\to$ `mice`).
    - Uncountable nouns (`money`, `information`, `rice`, `series`, `species`).
    - Casing preservation across inflections (`User` $\to$ `Users`, `USER` $\to$ `USERS`).
    - Pluralization with numeric counts (`pluralize_with_count`).
    - Configurable rule engine (`Inflector`) supporting domain-specific irregulars, uncountables, and dynamic patterns.

2. **Casing Transformations & Predicates**:
    - `snake_case` (e.g. `user_account_id`) & `SCREAMING_SNAKE_CASE` (e.g. `USER_ACCOUNT_ID`).
    - `camelCase` (e.g. `userAccountId`).
    - `kebab-case` (e.g. `user-account-id`).
    - `PascalCase` / `StudlyCaps` (e.g. `UserAccountId`).
    - `Title Case` (e.g. `User Account Id`).
    - `Train-Case` / `Header-Case` (e.g. `User-Account-Id`).
    - `Sentence case` (e.g. `User account id`).
    - Relational schema naming conventions: `classify` (`user_profiles` $\to$ `UserProfile`), `tableize` (`UserProfile`
      $\to$ `user_profiles`), `foreign_key` (`UserProfile` $\to$ `user_profile_id`).
    - Acronym grouping support (`HTTPResponse` $\to$ `http_response`).
    - Zero-allocation predicates (`is_snake_case`, `is_camel_case`, `is_pascal_case`, etc.).
    - Zero-copy `Cow<'_, str>` return types returning `Cow::Borrowed` when the input already matches the target casing.

3. **URL Slugification**:
    - ASCII transliteration for Latin diacritics and accented characters.
    - Configurable delimiters, case folding, and character sanitization via `SlugOptions`.

4. **String Utilities**:
    - Truncation with whole-word preservation and custom ellipsis: `truncate`, `words`, and contextual `excerpt`.
    - Sensitive data masking: `mask`, `mask_email`.
    - Padding and boundary guards: `pad_start`, `pad_end`, `pad_both`, `start`, `finish`, `wrap`, `unwrap`.
    - Ordinal numbers (`1` $\to$ `1st`) and Oxford comma list formatting (`sentence`).

5. **Ergonomic Tri-Surface API**:
    - Direct top-level standalone functions.
    - Fluent `StrBuilder` pipeline for multi-step string manipulations.
    - Extension trait `StrExt` implementing fluent methods on `&str`.

## Feature Flags

- `regex` (default): Enables dynamic regular expression patterns in the custom rule engine.
- `serde`: Enables `Serialize` and `Deserialize` on configuration and option structs.
