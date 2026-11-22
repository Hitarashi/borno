# Domain Glossary (`GLOSSARY.md`)

This document establishes the official ubiquitous domain language for `borno`. All modules, interfaces, comments, and documentation must adhere to these terms.

## Domain Concepts

### `CasingStyle` (or `Case`)
A recognized naming convention for formatting compound words and identifiers across programming languages, database schemas, and protocols.
- **`Snake` (`snake_case`)**: Lowercase words separated by single underscores (e.g. `user_account_id`).
- **`ScreamingSnake` (`SCREAMING_SNAKE_CASE`)**: Uppercase words separated by single underscores (e.g. `USER_ACCOUNT_ID`).
- **`Camel` (`camelCase`)**: Lowercase initial word followed by capitalized subsequent words with no delimiters (e.g. `userAccountId`).
- **`Pascal` (`PascalCase`)**: Capitalized initial word followed by capitalized subsequent words with no delimiters (e.g. `UserAccountId`).
- **`Kebab` (`kebab-case`)**: Lowercase words separated by single hyphens (e.g. `user-account-id`).
- **`Train` (`Train-Case` / `Header-Case`)**: Capitalized words separated by single hyphens (e.g. `User-Account-Id`).
- **`Title` (`Title Case`)**: Capitalized words separated by single spaces (e.g. `User Account Id`).
- **`Sentence` (`Sentence case`)**: Capitalized first word with lowercase subsequent words separated by single spaces (e.g. `User account id`).
- **`Dot` (`dot.case`)**: Lowercase words separated by single dots (e.g. `user.account.id`).
- **`No` (`no case`)**: Lowercase words separated by single spaces (e.g. `user account id`).
- **`Capital` (`Capital Case`)**: Capitalized words separated by single spaces (e.g. `User Account Id`).

### `WordBoundary`
A transition point between lexical tokens inside an identifier, determined by explicit separator characters (whitespace, `_`, `-`, `.`, `/`, `\`, `:`), casing transitions (lowercase followed by uppercase), or uppercase acronym runs followed by lowercase words (e.g. `HTTPResponse` $\to$ `HTTP` + `Response`).

### `Inflection`
The English grammatical modification of nouns between singular and plural forms, preserving letter-casing style and honoring irregular dictionaries, uncountable nouns, and regular suffix transformation rules.

### `CompoundIdentifier`
An identifier composed of multiple semantic tokens, joined either by delimiters (`user_profile`, `user-profile`) or casing shifts (`userProfile`, `UserProfile`). During inflection, only the tail word (stem) undergoes inflection while the prefix remains invariant.

### `Affix`
Prefix, suffix, or enclosing delimiters applied or removed idempotently from a string slice without duplicate insertions or partial slice corruptions.

### `Slugification`
The deterministic normalization of text into URL-safe ASCII strings, incorporating Latin diacritic transliteration, optional symbol expansion, delimiter formatting, and case folding.
