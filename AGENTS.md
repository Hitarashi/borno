# `borno` Agent Instructions

## Scope

`borno` provides fast, allocation-conscious English grammatical inflections, casing transformations, URL slugification,
and string utilities. It operates purely as computational algorithms with zero network, filesystem, or asynchronous
dependencies.

## Design Rules

1. **Deterministic & Unicode Safe**:
    - Transformations must be deterministic and safely handle Unicode scalar boundaries without panicking or slicing
      mid-character.
2. **Extensive Grammar Rule Tables**:
    - Maintain comprehensive regular and irregular rule tables for English inflection.
    - Support casing preservation (e.g. `User` pluralizes to `Users`, `user` to `users`, `USER` to `USERS`).
3. **Allocation Awareness**:
    - Provide zero-allocation predicates (e.g. `is_singular`, `is_plural`, `is_snake_case`, `is_camel_case`,
      `is_pascal_case`, etc.).
    - For casing conversions where input already conforms to the target casing, return `Cow::Borrowed` to eliminate heap
      allocations.
4. **Clean Feature Separation**:
    - Core casing, inflection tables, uncountables, irregulars, slugification, and string utilities work without external
      dependencies.
    - Dynamic regex patterns require the optional `regex` feature (enabled by default).
    - Serializability of options and configurations requires the optional `serde` feature.
    - All tests must pass with `--all-features` as well as `--no-default-features`.
5. **Code Style and Lints**:
    - Follow Google's Rust Style Guide.
    - Do not suppress Clippy warnings inline (`#[allow(...)]` or `#[expect(...)]`). Resolve the warning or restructure
      the code instead.
    - Avoid using leading underscores (`_`) on identifiers, variables, parameters, or struct fields.
    - Code formatting must be verified via `cargo +nightly fmt --check`.
6. **Framework Agnostic**:
    - Do not mention any web frameworks in documentation, codebase comments, identifiers, or error messages.
    - Reference only official standards, RFCs, Unicode specifications, and English grammar conventions.
