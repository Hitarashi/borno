# Borno (বর্ণ)

Fast, allocation-conscious English grammatical inflections, casing transformations, URL slugification, and string utilities for Rust.

[![Crates.io](https://img.shields.io/crates/v/borno.svg)](https://crates.io/crates/borno)
[![Documentation](https://docs.rs/borno/badge.svg)](https://docs.rs/borno)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)

## Features

- **English Grammatical Inflections**: Pluralization and singularization with regular suffix rules, irregular noun dictionaries, uncountable noun lookups, count-based matching, and letter-casing preservation (`User` $\to$ `Users`, `USER` $\to$ `USERS`).
- **Zero-Allocation Casing**: Fast conversions (`to_snake_case`, `to_camel_case`, `to_pascal_case`, `to_kebab_case`, `to_title_case`, `to_dot_case`, etc.) returning `Cow<'_, str>` to eliminate heap allocations when the input already conforms to the target casing.
- **Acronym Recognition**: Automatically groups uppercase acronym sequences (e.g. `HTTPResponse` $\to$ `http_response`).
- **Schema & Identifier Conventions**: Database table naming (`tableize`, `classify`), foreign key extraction and validation (`foreign_key`, `is_foreign_key`), UI-friendly titles (`headline`, `humanize`), and namespace stripping (`demodulize`, `deconstantize`).
- **Delimiter-Aware Slicing**: Zero-allocation substring extraction (`before`, `before_last`, `after`, `after_last`, `between`, `between_first`).
- **Normalization & Sanitization**: Whitespace collapsing (`squish`), cross-platform file path separator normalization (`normalize_slashes`), and HTML entity encoding/decoding (`html_escape`, `html_unescape`).
- **Terminal & Text Formatting**: Configurable word wrapping (`word_wrap`, `WordWrapOptions`) and multi-column tabular justification with alignment options (`justify_columns`, `Alignment`).
- **Template Interpolation**: Fast placeholder substitution supporting Mustache-style `{{ key }}`, `{ key }`, and colon `:key` parameters with brace escaping.
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
use borno::{pluralize, pluralize_with_count, pluralize_with_count_inclusive, singularize};

assert_eq!(pluralize("user"), "users");
assert_eq!(pluralize("person"), "people");
assert_eq!(pluralize("Category"), "Categories"); // Preserves casing
assert_eq!(singularize("cacti"), "cactus");
assert_eq!(singularize("boxes"), "box");

// Count-aware pluralization
assert_eq!(pluralize_with_count("file", 1), "file");
assert_eq!(pluralize_with_count("file", 5), "files");
assert_eq!(pluralize_with_count_inclusive("file", 1), "1 file");
assert_eq!(pluralize_with_count_inclusive("file", 5), "5 files");
```

### Casing Transformations

```rust
use borno::{to_camel_case, to_dot_case, to_kebab_case, to_pascal_case, to_snake_case};
use std::borrow::Cow;

// Returns Cow::Borrowed when input is already in target casing (zero allocation)
let borrowed = to_snake_case("already_snake");
assert!(matches!(borrowed, Cow::Borrowed(_)));

// Handles acronyms cleanly
assert_eq!(to_snake_case("HTTPResponseCode"), "http_response_code");
assert_eq!(to_camel_case("user_account_id"), "userAccountId");
assert_eq!(to_kebab_case("UserProfilePage"), "user-profile-page");
assert_eq!(to_pascal_case("order_items"), "OrderItems");
assert_eq!(to_dot_case("user_profile_setting"), "user.profile.setting");
```

### Identifier & Schema Conventions

```rust
use borno::{classify, deconstantize, demodulize, foreign_key, headline, humanize, is_foreign_key, tableize};

assert_eq!(tableize("UserProfile"), "user_profiles");
assert_eq!(classify("user_profiles"), "UserProfile");
assert_eq!(foreign_key("UserProfile"), "user_profile_id");
assert!(is_foreign_key("user_id"));

assert_eq!(headline("user_profile_id"), "User Profile Id");
assert_eq!(humanize("employee_salary"), "Employee salary");
assert_eq!(demodulize("App::Services::Notification"), "Notification");
assert_eq!(deconstantize("App::Services::Notification"), "App::Services");
```

### Slicing & Normalization

```rust
use borno::{after, before, between, normalize_slashes, squish};

assert_eq!(before("user@example.com", "@"), "user");
assert_eq!(after("user@example.com", "@"), "example.com");
assert_eq!(between("name: [Jane];", "[", "]"), "Jane");

assert_eq!(squish("   lots   of   spaces  \n\t "), "lots of spaces");
assert_eq!(normalize_slashes(r"path\to\file.txt"), "path/to/file.txt");
```

### Text Formatting & Interpolation

```rust
use borno::{html_escape, html_unescape, interpolate, word_wrap};

assert_eq!(html_escape("<b>Hello & Welcome</b>"), "&lt;b&gt;Hello &amp; Welcome&lt;/b&gt;");
assert_eq!(html_unescape("&lt;b&gt;bold&lt;/b&gt;"), "<b>bold</b>");

let wrapped = word_wrap("The quick brown fox jumps over the lazy dog", 15);
assert_eq!(wrapped, "The quick brown\nfox jumps over\nthe lazy dog");

let message = interpolate("Hello {{ name }}, your role is :role.", &[
    ("name", "Alice"),
    ("role", "admin"),
]);
assert_eq!(message, "Hello Alice, your role is admin.");
```

### URL Slugification

```rust
use borno::{SlugOptions, slugify, slugify_with_options};

assert_eq!(slugify("Hello, World!"), "hello-world");
assert_eq!(slugify("Café & Restaurant"), "cafe-and-restaurant");

let options = SlugOptions::new().with_delimiter('_');
assert_eq!(slugify_with_options("Hello World", &options), "hello_world");
```

### String Utilities

```rust
use borno::{mask_email, ordinal, sentence, truncate};

assert_eq!(mask_email("user@example.com", '*'), "u***@example.com");
assert_eq!(ordinal(1), "1st");
assert_eq!(ordinal(22), "22nd");
assert_eq!(sentence(&["apple", "banana", "cherry"]), "apple, banana, and cherry");
assert_eq!(truncate("The quick brown fox", 10), "The quick...");
```

### Fluent Extension Trait (`StrExt`)

```rust
use borno::StrExt;

assert_eq!("user_id".to_pascal_case(), "UserId");
assert_eq!("child".to_plural(), "children");
assert_eq!("user_profiles".to_singular(), "user_profile");
assert_eq!("user_id".before("_"), "user");
assert_eq!("  messy   text  ".squish(), "messy text");
assert_eq!("Hello World".to_slug(), "hello-world");
```

### Fluent Pipeline (`StrBuilder`)

```rust
use borno::StrBuilder;

let result = StrBuilder::new("  hello_world  ")
    .trim()
    .pascal_case()
    .pluralize()
    .build();

assert_eq!(result, "HelloWorlds");
```

## Feature Flags

`borno` provides fine-grained feature flags so callers can opt in only to what they need, minimizing compile times and binary size. By default, the `full` feature bundle is enabled.

### Composite Bundles

| Bundle | Description |
|---|---|
| `full` *(default)* | Enables all computational casing, conventions, grammar, slug, utility modules, fluent builder, extension trait, and regex support. |
| `casing` | Enables all casing transformations (`case-*`). |
| `utils` | Enables all string utilities (`util-*`). |
| `default` | Alias for `["full"]`. |

### Fine-Grained Features

| Feature | Capabilities |
|---|---|
| `case-snake` | `to_snake_case`, `to_screaming_snake_case`, `is_snake_case`, `is_screaming_snake_case`. |
| `case-camel` | `to_camel_case`, `is_camel_case`. |
| `case-pascal` | `to_pascal_case`, `is_pascal_case`. |
| `case-kebab` | `to_kebab_case`, `is_kebab_case`. |
| `case-train` | `to_train_case`, `is_train_case`. |
| `case-title` | `to_title_case`, `is_title_case`. |
| `case-sentence` | `to_sentence_case`, `is_sentence_case`. |
| `case-dot` | `to_dot_case`, `is_dot_case`. |
| `conventions` | Database and identifier conventions (`tableize`, `classify`, `foreign_key`, `headline`, `humanize`, `demodulize`, `deconstantize`). |
| `grammar` | English grammatical inflection (`pluralize`, `singularize`, `pluralize_with_count`, `is_plural`, `is_singular`, `Inflector`). |
| `regex` | Enables dynamic regex rule registration in `Inflector` (`add_plural_rule`, `add_singular_rule`). |
| `slug` | URL slugification (`slugify`, `slugify_with_options`, `SlugOptions`). |
| `util-slice` | Delimiter-aware slicing (`before`, `after`, `between`, etc.). |
| `util-normalize` | Whitespace squishing (`squish`) and file path normalization (`normalize_slashes`). |
| `util-interpolate` | String template placeholder interpolation (`interpolate`, `interpolate_pairs`). |
| `util-text` | Word wrapping (`word_wrap`), HTML escaping (`html_escape`), and tabular column justification (`justify_columns`). |
| `util-truncate` | Contextual excerpting (`excerpt`), character truncation (`truncate`), and word-count limiting (`words`). |
| `util-mask` | Character masking (`mask`) and email masking (`mask_email`). |
| `util-pad` | Padding (`pad_start`, `pad_end`, `pad_both`) and boundary guards (`start`, `finish`, `wrap`, `unwrap`). |
| `util-format` | Ordinals (`ordinal`) and Oxford comma list formatting (`sentence`). |
| `builder` | Fluent multi-step string transformation pipeline (`StrBuilder`), conditionally exposing methods matching active features. |
| `ext` | Extension trait on `&str` (`StrExt`), conditionally exposing methods matching active features. |
| `serde` | Implements `Serialize` and `Deserialize` on option structs (`SlugOptions`, `SentenceOptions`, `TruncateOptions`, `ExcerptOptions`, etc.). |

### Opting In To Specific Features

To minimize dependencies and compilation time, disable default features and choose only what your project needs:

```toml
[dependencies]
# Only use snake_case and kebab-case conversions
borno = { version = "0.1.0", default-features = false, features = ["case-snake", "case-kebab"] }

# Or only use grammar inflections with the fluent extension trait
borno = { version = "0.1.0", default-features = false, features = ["grammar", "ext"] }
```

## License

Dual-licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
