# Changelog

All notable changes to this project are documented in this file.

## [0.2.0]

### Breaking

- `Expression::ast` no longer has a `Rule::file` match arm. Previously, calling it directly on a `Rule::file` pair would silently return only the *first* expression of a multi-expression file; it now panics (`unreachable!()`) instead. Use the new `parse_expressions` function to handle multi-expression input.

### Added

- `parse_expression(input: &str) -> Result<Expression, ParserError>` — parses a single expression directly, without touching Pest's `Pairs`/`Rule` types.
- `parse_expressions(input: &str) -> Result<Vec<(&str, Expression)>, ParserError>` — parses every expression in a multi-line input into `(source_line, Expression)` pairs.
- `Expression` now derives `Clone` and `PartialEq`.
- `keywords`/`categories` added to `Cargo.toml` for crates.io discoverability.
- CI via GitHub Actions (`.github/workflows/ci.yml`): fmt, clippy (`-D warnings`), test, and a release build on push/PR to `main`.
- Test coverage for AST evaluation (`tests/ast_evaluation_tests.rs`) and truth-table generation (`tests/truth_table_tests.rs`) — previously only grammar/syntax parsing was tested.

### Changed

- Truth-table rendering now uses the `comfy-table` crate (`ASCII_MARKDOWN` preset) instead of hand-rolled `Display` string-building.
- `--file` input is now newline-normalized the same way `--expression` input already was, so a file missing a trailing newline on its last line no longer fails with a misleading parse error.
- `parse` subcommand's `--file`/`--expression` mutual exclusivity now uses an explicit `ArgGroup`, producing a clearer "one of these is required" error when neither is passed.
- Running the CLI with no arguments now prints help instead of silently exiting.
- Build tooling migrated from `Makefile` to `Taskfile.yml` (go-task).

## [0.1.4] and earlier

See git history.
