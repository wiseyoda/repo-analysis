---
paths:
  - "src/**/*.rs"
---

# Rust Source Rules

When editing Rust source files:

- No `unwrap()` or `expect()` outside `#[cfg(test)]` blocks
- No `panic!()` in library code (only `main.rs`)
- All `pub` items must have `///` doc comments
- Functions must be under 40 lines — extract if longer
- Max 4 parameters — use an options struct beyond that
- Use early returns over nested conditionals
- Imports grouped: std blank line, external crates blank line, internal crate imports
- Use `thiserror` for module error types, `anyhow` only in `main.rs`/CLI layer
- Prefer `&str` over `String` in function parameters
- Prefer `pub(crate)` over `pub` unless the item is part of the public API
- Run `cargo clippy -- -D warnings` after making changes
