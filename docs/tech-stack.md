# Tech Stack

> Canonical technology choices for `repostat`. Changes require a decision record in `decisions.md`.

## Language & Toolchain

| Component | Choice | Rationale |
|-----------|--------|-----------|
| Language | **Rust (stable)** | Single binary, zero runtime deps, maximum performance, strong type system |
| Edition | **2024** | Latest stable Rust edition |
| Min Rust Version | **1.85+** | Required for edition 2024 |
| Build | **cargo** | Standard Rust build system |
| Formatter | **rustfmt** | Enforced in CI, default config |
| Linter | **clippy** | `-D warnings` — all warnings are errors |

## Core Dependencies

> Each dependency must satisfy the criteria in `constitution.md` §8 (Dependency Discipline).

| Crate | Purpose | Justification |
|-------|---------|---------------|
| `clap` (derive) | CLI argument parsing | De facto standard. Derive macros reduce boilerplate. Generates help, completions. |
| `tree-sitter` | Source code parsing | Multi-language AST parsing for complexity analysis. Used by GitHub, Neovim. |
| `tree-sitter-{lang}` | Language grammars | Compiled grammars for top 10 languages. |
| `rayon` | Parallelism | Data-parallel file processing. Zero-config thread pool. |
| `serde` + `serde_json` | Serialization | Snapshot storage, config parsing, AI response handling. |
| `toml` | Config parsing | `.repostat.toml` configuration file. |
| `ignore` | Gitignore-aware walking | From the ripgrep ecosystem. Handles `.gitignore`, `.ignore`, nested overrides. |
| `crossterm` | Terminal rendering | Cross-platform terminal manipulation for the dashboard. |
| `chrono` | Timestamps | Snapshot timestamps, date formatting in reports. |
| `thiserror` | Error types | Derive macro for clean, idiomatic error enums. |
| `anyhow` | Error propagation | Ergonomic error handling in application code (not library code). |

## Dev Dependencies

| Crate | Purpose |
|-------|---------|
| `assert_cmd` | CLI integration testing — run the binary and assert on output |
| `predicates` | Fluent assertions for integration tests |
| `tempfile` | Temporary directories for test fixtures |
| `insta` | Snapshot testing for terminal output and reports |
| `pretty_assertions` | Readable test diffs |

## External Tools

| Tool | Purpose | Integration |
|------|---------|-------------|
| Claude CLI | AI-augmented analysis | Invoked via `claude -p` subprocess in target directory |
| git | History analysis | Invoked via subprocess for log/diff/rev-parse |

## Architecture Layers

```
┌─────────────────────────────────────────────┐
│                    CLI                       │  clap argument parsing, output formatting
├─────────────────────────────────────────────┤
│                  Report                      │  Dashboard rendering, markdown generation
├─────────────────────────────────────────────┤
│                 Analysis                     │  Orchestrates all analysis passes
├────────────┬────────────┬───────────────────┤
│  Metrics   │ Complexity │   AI Provider     │  LOC counting, tree-sitter, Claude CLI
├────────────┼────────────┼───────────────────┤
│  Scanner   │  Parsers   │   Snapshot Store  │  File walking, language detection, JSON I/O
└────────────┴────────────┴───────────────────┘
```

## Directory Structure

```
repostat/
├── src/
│   ├── main.rs              # Entry point, CLI setup
│   ├── cli.rs               # Argument definitions (clap)
│   ├── config.rs            # .repostat.toml parsing
│   ├── scanner/
│   │   ├── mod.rs           # File walker, exclusion logic
│   │   ├── language.rs      # Language detection
│   │   └── gitignore.rs     # Gitignore integration
│   ├── metrics/
│   │   ├── mod.rs           # Metric aggregation
│   │   ├── loc.rs           # Line counting
│   │   ├── complexity.rs    # Cyclomatic + cognitive complexity
│   │   ├── functions.rs     # Function size analysis
│   │   └── dependencies.rs  # Dependency manifest parsing
│   ├── ai/
│   │   ├── mod.rs           # AI orchestration
│   │   ├── claude.rs        # Claude CLI invocation
│   │   ├── skills.rs        # Skill file loading
│   │   └── schema.rs        # Response parsing + validation
│   ├── snapshot/
│   │   ├── mod.rs           # Snapshot management
│   │   ├── store.rs         # Read/write JSON snapshots
│   │   └── diff.rs          # Snapshot comparison
│   ├── report/
│   │   ├── mod.rs           # Report orchestration
│   │   ├── dashboard.rs     # Terminal dashboard rendering
│   │   ├── markdown.rs      # Markdown report generation
│   │   └── trend.rs         # Sparkline trend display
│   └── errors.rs            # Error types
├── tests/
│   ├── integration/         # End-to-end CLI tests
│   ├── fixtures/            # Test repos and sample data
│   └── snapshots/           # insta snapshot files
├── docs/
│   ├── constitution.md
│   ├── requirements.md
│   ├── tech-stack.md
│   ├── coding-standard.md
│   ├── decisions.md
│   └── specs/               # Feature specifications (SDD)
├── .repostat.toml           # Self-referential config (dogfooding)
├── CLAUDE.md
├── ROADMAP.md
├── BACKLOG.md
├── Cargo.toml
└── Cargo.lock
```

## Testing Strategy

| Level | Tool | Scope |
|-------|------|-------|
| Unit | `#[cfg(test)]` modules | Individual functions, parsers, calculations |
| Integration | `assert_cmd` | Full CLI invocation against fixture repos |
| Snapshot | `insta` | Terminal output, markdown reports, JSON snapshots |
| Property | `proptest` (if needed) | Edge cases in parsing, counting, complexity math |

## CI Pipeline

```
cargo fmt --check
  → cargo clippy -- -D warnings
    → cargo test
      → cargo build --release
```

All four must pass before merge. No exceptions.
