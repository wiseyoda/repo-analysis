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
| `clap_complete` + `clap_mangen` | Shell and man-page generation | Keeps generated CLI documentation aligned with clap definitions. |
| `tree-sitter` | Source code parsing | Multi-language AST parsing for complexity analysis. Used by GitHub, Neovim. |
| `tree-sitter-{lang}` | Language grammars | Compiled grammars for top 10 languages. |
| `rayon` | Parallelism | Data-parallel file processing. Zero-config thread pool. |
| `serde` + `serde_json` | Serialization | Historical snapshots and stable `repostat.metrics.v1` output. |
| `toml` | Config parsing | `.repostat.toml` configuration file. |
| `ignore` | Gitignore-aware walking | From the ripgrep ecosystem. Handles `.gitignore`, `.ignore`, nested overrides. |
| `globset` | Configured include/exclude matching | Compiles user patterns once for deterministic scans. |
| `crossterm` | Terminal rendering | Cross-platform terminal manipulation for the dashboard. |
| `chrono` | Timestamps | Snapshot timestamps, date formatting in reports. |
| `dirs` | Global index location | Locates the user data directory for explicit standalone history. |
| `thiserror` | Error types | Derive macro for clean, idiomatic error enums. |
| `anyhow` | Error propagation | Ergonomic error handling in application code (not library code). |

## Dev Dependencies

| Crate | Purpose |
|-------|---------|
| `assert_cmd` | CLI integration testing — run the binary and assert on output |
| `predicates` | Fluent assertions for integration tests |
| `tempfile` | Temporary directories for test fixtures |
| `pretty_assertions` | Readable test diffs |

## External Tools

| Tool | Purpose | Integration |
|------|---------|-------------|
| git | Source identity and history analysis | Invoked with the target repository as its working directory |

Repostat has no model or provider runtime dependency. Optional AI enrichment is
an explicit ai-mux Engine workflow, outside the Repostat process.

## Architecture Layers

```
┌─────────────────────────────────────────────┐
│                    CLI                       │  clap argument parsing, output formatting
├─────────────────────────────────────────────┤
│                  Report                      │  Dashboard rendering, markdown generation
├─────────────────────────────────────────────┤
│          Shared deterministic analysis       │  One scanner for standalone + extension
├────────────┬────────────┬───────────────────┤
│  Metrics   │ Complexity │  Stable result     │  LOC, tree-sitter, repostat.metrics.v1
├────────────┼────────────┼───────────────────┤
│  Scanner   │  Git read  │ Snapshot (opt-in) │  Read-only core, explicit history writes
└────────────┴────────────┴───────────────────┘
```

## Directory Structure

```
repostat/
├── src/
│   ├── main.rs              # Entry point, CLI setup
│   ├── analyze_command.rs   # Standalone analyze orchestration
│   ├── analysis.rs          # Shared deterministic analysis
│   ├── cli.rs               # Argument definitions (clap)
│   ├── config.rs            # .repostat.toml parsing
│   ├── result.rs            # Stable suite/standalone JSON
│   ├── scanner/
│   │   ├── mod.rs           # File walker, exclusion logic
│   │   ├── language.rs      # Language detection
│   │   └── filter.rs        # Generated/minified detection
│   ├── metrics/
│   │   ├── mod.rs           # Metric aggregation
│   │   ├── loc.rs           # Line counting
│   │   ├── complexity.rs    # Cyclomatic + cognitive complexity
│   │   ├── dependencies.rs  # Dependency manifest parsing
│   │   ├── documentation.rs # Documentation metrics
│   │   ├── git_history.rs   # Git history and deterministic churn
│   │   └── risk.rs          # Churn/complexity risk scoring
│   ├── ai/
│   │   ├── mod.rs           # Historical compatibility boundary
│   │   └── schema.rs        # Historical AI snapshot schema
│   ├── snapshot/
│   │   ├── mod.rs           # Snapshot management
│   │   ├── store.rs         # Read/write JSON snapshots
│   │   ├── index.rs         # Explicit cross-repo history index
│   │   └── diff.rs          # Snapshot comparison
│   ├── report/
│   │   ├── mod.rs           # Report orchestration
│   │   ├── dashboard.rs     # Terminal dashboard rendering
│   │   ├── markdown.rs      # Markdown report generation
│   │   ├── html.rs          # Explicit HTML export
│   │   └── trend.rs         # Sparkline trend display
│   └── errors.rs            # Error types
├── tests/
│   ├── cli_basic.rs         # Legacy CLI integration tests
│   └── suite_tool.rs        # Determinism, no-write, manifest conformance
├── schemas/
│   └── repostat-metrics-v1.schema.json
├── ai-mux.extension.json
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
| Property | `proptest` (if needed) | Edge cases in parsing, counting, complexity math |
| Suite conformance | `assert_cmd` + mutation traps | No providers, no writes, byte stability, manifest/schema closure |

## CI Pipeline

```
cargo fmt --check
  → cargo clippy -- -D warnings
    → cargo test
      → cargo build --release
```

All four must pass before merge. No exceptions.
