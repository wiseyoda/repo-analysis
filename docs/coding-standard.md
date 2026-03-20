# Coding Standard

> Enforceable rules for all code in this project. These standards apply to every contributor,
> human or AI. When in doubt, refer to the Constitution.

## Rust Style

### Formatting

- **rustfmt** is the single source of truth. Run `cargo fmt` before every commit.
- No manual formatting overrides (`#[rustfmt::skip]`) without a comment explaining why.
- Line length: 100 characters (rustfmt default).

### Naming

| Item | Convention | Example |
|------|------------|---------|
| Crates | `snake_case` | `repo_stat` |
| Modules | `snake_case` | `line_counter` |
| Types | `PascalCase` | `AnalysisResult` |
| Functions | `snake_case` | `count_lines` |
| Constants | `SCREAMING_SNAKE` | `MAX_LINE_LENGTH` |
| Enum variants | `PascalCase` | `Language::TypeScript` |
| Trait names | `PascalCase`, adjective-like | `Parseable`, `Renderable` |
| Builder methods | `with_*` | `with_config()` |
| Conversion methods | `to_*`, `into_*`, `as_*`, `from_*` | `to_markdown()` |
| Fallible constructors | `new` returns `Result` or use `try_new` | `Config::try_new()` |

### Error Handling

```rust
// CORRECT: Define domain errors with thiserror
#[derive(Debug, thiserror::Error)]
pub enum ScanError {
    #[error("path does not exist: {0}")]
    PathNotFound(PathBuf),

    #[error("failed to read file {path}: {source}")]
    ReadFailed {
        path: PathBuf,
        source: std::io::Error,
    },
}

// CORRECT: Propagate with ?
fn scan_file(path: &Path) -> Result<FileMetrics, ScanError> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| ScanError::ReadFailed {
            path: path.to_owned(),
            source: e,
        })?;
    Ok(analyze(content))
}

// WRONG: Never in non-test code
let content = std::fs::read_to_string(path).unwrap();
let content = std::fs::read_to_string(path).expect("should exist");
```

- `thiserror` for library/module error types (structured, matchable).
- `anyhow` only at the application boundary (`main.rs`, CLI layer).
- No `unwrap()` or `expect()` outside of `#[cfg(test)]` code.
- No `panic!()` in library code. Return `Result` or `Option`.
- Error messages are lowercase, no trailing punctuation (Rust convention).

### Structure

- **One type, one file** when the type + impls exceed 100 lines.
- **Module files** (`mod.rs`) re-export public API and contain no logic.
- **Public API surface is minimal.** Default to `pub(crate)`. Only `pub` what consumers need.
- **Imports** are grouped: std → external crates → internal modules, separated by blank lines.

```rust
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use tree_sitter::Parser;

use crate::config::Config;
use crate::scanner::Language;
```

### Functions

- **Max 40 lines** per function. If longer, extract sub-functions.
- **Max 4 parameters.** Use a config/options struct beyond that.
- **Early returns** over nested `if/else`. Flatten the happy path.
- **No side effects in getters.** A method named `get_*` or `is_*` must be pure.

```rust
// CORRECT: Early return, flat
fn process_file(path: &Path, config: &Config) -> Result<Option<FileMetrics>> {
    if !path.exists() {
        return Ok(None);
    }

    let language = match Language::detect(path) {
        Some(lang) => lang,
        None => return Ok(None),
    };

    if config.is_excluded(path) {
        return Ok(None);
    }

    let metrics = analyze(path, language)?;
    Ok(Some(metrics))
}
```

### Types & Data

- **Structs are immutable by default.** Fields are private; expose via methods.
- **Builder pattern** for complex construction (more than 3 fields with optional values).
- Use `Cow<'_, str>` when a function may or may not need to allocate.
- Prefer `&str` over `String` in function parameters.
- Use newtypes for domain concepts: `struct LineCount(usize)`, not raw `usize`.

### Documentation

- All `pub` items have `///` doc comments.
- Doc comments describe WHAT and WHY, not HOW (the code shows how).
- Include a one-line summary, then a blank line, then details if needed.
- Add `# Examples` in doc comments for non-obvious APIs.
- No doc comments on private items unless the logic is non-obvious.

```rust
/// Counts lines of code, comments, and blanks in a source file.
///
/// Uses language-specific comment syntax to distinguish code from comments.
/// Blank lines are lines containing only whitespace.
pub fn count_lines(content: &str, language: Language) -> LineMetrics {
    // ...
}
```

## Testing Standard

### TDD Workflow

1. **Red**: Write a failing test that describes the desired behavior.
2. **Green**: Write the minimum code to make the test pass.
3. **Refactor**: Clean up while keeping tests green.

Every PR must show this cycle. Tests are committed alongside (or before) implementation.

### Test Organization

```
src/
  metrics/
    loc.rs              # Implementation
    loc.rs → #[cfg(test)] mod tests  # Unit tests inline

tests/
  integration/
    cli_basic.rs        # Full binary tests
    cli_output.rs       # Output format tests
  fixtures/
    sample_repo/        # Minimal test repos
    complex_repo/       # Edge case repos
  snapshots/            # insta snapshot files
```

### Test Naming

```rust
#[test]
fn counts_code_lines_excluding_comments() { }

#[test]
fn returns_error_for_nonexistent_path() { }

#[test]
fn excludes_node_modules_by_default() { }
```

- Name describes the behavior, not the implementation.
- Starts with a verb in present tense.
- Reads as a sentence: "it [test name]".

### Test Quality

- **One assertion per concept** (multiple `assert!` calls are fine if testing one behavior).
- **No test interdependence.** Each test sets up its own state.
- **No sleeping or timing.** Tests must be deterministic.
- **Test edge cases**: empty input, Unicode, very large files, permission errors.
- **Fixture repos** are committed, minimal, and documented.

## CLI Standard

### User Experience

- Every flag has a long form (`--markdown`) and a short form (`-m`) where unambiguous.
- `--help` is comprehensive with usage examples.
- `--version` outputs `repostat x.y.z`.
- Exit code 0 on success, 1 on user error, 2 on internal error.
- Errors go to stderr, data goes to stdout.
- No output on success for scriptable commands (unless `--verbose`).
- Progress indicators for operations exceeding 1 second.

### Output

- Colored output by default, respects `NO_COLOR` environment variable.
- Machine-readable output available via `--json`.
- Terminal width is detected and layouts adapt (no hardcoded 80-col assumption).
- Unicode box-drawing characters for the dashboard, with ASCII fallback.

## Git & Commit Standard

- **Conventional Commits**: `feat:`, `fix:`, `refactor:`, `test:`, `docs:`, `chore:`, `perf:`
- Scope is optional but encouraged: `feat(complexity): add cognitive complexity`
- Subject line: imperative mood, lowercase, no period, max 72 chars.
- Body: explain WHY, not WHAT (the diff shows what).
- Breaking changes: `feat!:` or `BREAKING CHANGE:` footer.

## Code Review Checklist

Before merging any code, verify:

- [ ] Tests exist and pass (`cargo test`)
- [ ] No clippy warnings (`cargo clippy -- -D warnings`)
- [ ] Formatted (`cargo fmt --check`)
- [ ] No `unwrap()`/`expect()` outside tests
- [ ] Error messages are actionable
- [ ] Public APIs have doc comments
- [ ] No unnecessary dependencies added
- [ ] Performance impact considered for hot paths
