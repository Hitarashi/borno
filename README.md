# Borno

Fast, allocation-conscious English grammatical inflections, casing transformations, URL slugification, and string utilities for Rust.

[![Crates.io](https://img.shields.io/crates/v/borno.svg)](https://crates.io/crates/borno)
[![Documentation](https://docs.rs/borno/badge.svg)](https://docs.rs/borno)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)

## Features

- **English Grammatical Inflections**: Pluralization and singularization with regular suffix rules, irregular noun dictionaries, uncountable noun lookups, count-based matching, and letter-casing preservation (`User` $\to$ `Users`, `USER` $\to$ `USERS`).
- **Zero-Allocation Casing**: Fast conversions (`to_snake_case`, `to_camel_case`, `to_pascal_case`, `to_kebab_case`, `to_title_case`, etc.) returning `Cow<'_, str>` to eliminate heap allocations when the input already conforms to the target casing.
- **Acronym Recognition**: Automatically groups uppercase acronym sequences (e.g. `HTTPResponse` $\to$ `http_response`).
- **Schema & Relational Helpers**: Common conventions (`tableize`, `classify`, `foreign_key`) for code generation, ORMs, and scaffolding.
- **URL Slugification**: Latin diacritic transliteration, symbol normalization, and configurable delimiters via `SlugOptions`.
- **String Utilities**: Sensitive data masking (`mask`, `mask_email`), word-boundary truncation (`truncate`, `words`), contextual excerpting (`excerpt`), ordinals (`1` $\to$ `1st`), and Oxford comma list formatting (`sentence`).
- **Tri-Surface Ergonomics**: Standalone top-level functions, fluent `StrBuilder` pipeline, and `StrExt` extension trait for `&str`.

## Installation

Add `borno` to your `Cargo.toml`:

```toml
[dependencies]
borno = "0.1.0"
```

## Quick Start

### Grammatical Inflection

```rust
use borno::{pluralize, pluralize_with_count, singularize};

assert_eq!(pluralize("user"), "users");
assert_eq!(pluralize("person"), "people");
assert_eq!(pluralize("Category"), "Categories"); // Preserves casing
assert_eq!(singularize("cacti"), "cactus");
assert_eq!(singularize("boxes"), "box");

// Count-aware pluralization
assert_eq!(pluralize_with_count("file", 1), "1 file");
assert_eq!(pluralize_with_count("file", 5), "5 files");
```

### Casing Transformations

```rust
use borno::{to_camel_case, to_kebab_case, to_snake_case, to_pascal_case};
use std::borrow::Cow;

// Returns Cow::Borrowed when input is already in target casing (zero allocation)
let borrowed = to_snake_case("already_snake");
assert!(matches!(borrowed, Cow::Borrowed(_)));

// Handles acronyms cleanly
assert_eq!(to_snake_case("HTTPResponseCode"), "http_response_code");
assert_eq!(to_camel_case("user_account_id"), "userAccountId");
assert_eq!(to_kebab_case("UserProfilePage"), "user-profile-page");
assert_eq!(to_pascal_case("order_items"), "OrderItems");
```

### URL Slugification

```rust
use borno::{slugify, slugify_with_options, SlugOptions};

assert_eq!(slugify("Hello, World!"), "hello-world");
assert_eq!(slugify("Café & Restaurant"), "cafe-and-restaurant");

let options = SlugOptions::new().with_delimiter('_');
assert_eq!(slugify_with_options("Hello World", &options), "hello_world");
```

### String Utilities

```rust
use borno::{mask_email, ordinal, sentence, truncate};

assert_eq!(mask_email("user@example.com"), "u***@example.com");
assert_eq!(ordinal(1), "1st");
assert_eq!(ordinal(22), "22nd");
assert_eq!(sentence(&["apple", "banana", "cherry"]), "apple, banana, and cherry");
assert_eq!(truncate("The quick brown fox", 10), "The quick...");
```

### Fluent Extension Trait (`StrExt`)

```rust
use borno::StrExt;

assert_eq!("user_id".to_pascal_case(), "UserId");
assert_eq!("child".pluralize(), "children");
assert_eq!("Hello World".slugify(), "hello-world");
```

### Fluent Pipeline (`StrBuilder`)

```rust
use borno::StrBuilder;

let result = StrBuilder::new("  hello_world  ")
    .trim()
    .to_pascal_case()
    .pluralize()
    .build();

assert_eq!(result, "HelloWorlds");
```

## Feature Flags

- `regex` *(default)*: Enables regular expression matching for custom dynamic inflection rules.
- `serde`: Derives `Serialize` and `Deserialize` for option types (`SlugOptions`, `TruncateOptions`, etc.).

## License

Dual-licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
