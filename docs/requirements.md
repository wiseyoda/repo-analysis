# Requirements

> Canonical requirements for `repostat`. Each requirement has a unique ID, priority,
> and phase assignment. Implementation order follows phase numbering.

## Notation

- **P0** — Must have for the phase. Blocks release.
- **P1** — Should have. Ship without if needed, but close immediately after.
- **P2** — Nice to have. Can defer to backlog.

---

## Phase 1: Foundation & Core Metrics

> Goal: `repostat ./path` produces accurate line counts, language breakdown, and file statistics
> with a terminal dashboard. Snapshots are stored. The skeleton is tested and shippable.

| ID | Requirement | Priority |
|----|-------------|----------|
| R-001 | CLI accepts a path argument and validates it exists and is a directory | P0 |
| R-002 | Recursive file walker respects `.gitignore` rules | P0 |
| R-003 | Built-in heuristic exclusions: `node_modules`, `vendor`, `build`, `dist`, `.next`, `Pods`, `target`, `.git`, `__pycache__`, `.venv`, `venv` | P0 |
| R-004 | Detect minified files (avg line length > 200 chars) and generated files (header comments) and exclude from user code metrics | P0 |
| R-005 | `.repostat.toml` config file support for custom include/exclude patterns | P0 |
| R-006 | Line counting: total lines, code lines, blank lines, comment lines — per file and aggregated per language | P0 |
| R-007 | Language detection by file extension with a curated mapping (50+ languages) | P0 |
| R-008 | Language breakdown: percentage of codebase per language, file count per language | P0 |
| R-009 | Terminal dashboard output: compact box-drawn display fitting one screen | P0 |
| R-010 | JSON snapshot storage in `.repostat/snapshots/` with timestamp, git SHA (if available), all metrics, and config used | P0 |
| R-011 | Auto-diff: show delta vs most recent snapshot at bottom of dashboard | P0 |
| R-012 | `--markdown` flag generates a Markdown report file | P1 |
| R-013 | `--json` flag outputs raw JSON to stdout | P1 |
| R-014 | Parallel file traversal using rayon for multi-core performance | P1 |
| R-015 | Colored terminal output with graceful fallback for non-color terminals | P1 |
| R-016 | `--help` with clear usage examples and flag descriptions | P0 |

## Phase 2: Complexity Analysis

> Goal: Tree-sitter powered complexity metrics. Cyclomatic, cognitive, function size, file size.
> Hotspot identification.

| ID | Requirement | Priority |
|----|-------------|----------|
| R-100 | Tree-sitter integration with compiled grammars for: TypeScript, JavaScript, Python, Rust, Go, Swift, Java, C, C++, Ruby | P0 |
| R-101 | Cyclomatic complexity calculation per function, per file, and project average | P0 |
| R-102 | Cognitive complexity calculation (SonarQube-style, nested logic weighted higher) | P0 |
| R-103 | Function size analysis: flag functions exceeding configurable threshold (default: 50 lines) | P0 |
| R-104 | File size analysis: flag files exceeding configurable threshold (default: 500 lines) | P0 |
| R-105 | Complexity hotspots: top N most complex files/functions in the dashboard | P0 |
| R-106 | Dynamic tree-sitter grammar loading for languages beyond the top 10 | P1 |
| R-107 | Regex-based complexity heuristic fallback for languages without tree-sitter grammars | P1 |
| R-108 | Complexity trend in snapshots: track per-file complexity over time | P1 |

## Phase 3: Dependency & Coupling Analysis

> Goal: Understand the dependency landscape — both external packages and internal coupling.

| ID | Requirement | Priority |
|----|-------------|----------|
| R-200 | Parse dependency manifests: `package.json`, `Cargo.toml`, `requirements.txt`, `Pipfile`, `go.mod`, `Package.swift`, `Gemfile`, `pom.xml`, `build.gradle` | P0 |
| R-201 | Direct dependency count per manifest | P0 |
| R-202 | Transitive dependency count from lock files (`package-lock.json`, `Cargo.lock`, `poetry.lock`, `go.sum`, etc.) | P1 |
| R-203 | Internal coupling: parse import/require/use statements to build an internal dependency graph | P1 |
| R-204 | Fan-in / fan-out metrics per module | P1 |
| R-205 | Dependency staleness: flag deps with no updates in 12+ months (from lock file metadata where available) | P2 |

## Phase 4: Documentation Analysis

> Goal: Measure documentation quality, coverage, and freshness.

| ID | Requirement | Priority |
|----|-------------|----------|
| R-300 | Markdown file inventory: count, total lines, total characters | P0 |
| R-301 | Doc-to-code ratio: lines of documentation vs lines of code | P0 |
| R-302 | README completeness scoring: check for install instructions, usage, API docs, contributing guide, license | P0 |
| R-303 | Per-module doc coverage: does each source directory have accompanying documentation? | P1 |
| R-304 | Stale doc detection (AI): identify docs referencing functions/files that no longer exist | P1 |
| R-305 | Doc quality scoring (AI): clarity, completeness, accuracy rating | P2 |

## Phase 5: AI-Augmented Analysis

> Goal: Claude CLI integration for architecture summary, feature inventory, quality review,
> and effort estimation.

| ID | Requirement | Priority |
|----|-------------|----------|
| R-400 | Detect Claude CLI availability (`which claude`) with graceful skip if missing | P0 |
| R-401 | Skill file system: load analysis prompts from `~/.repostat/skills/` | P0 |
| R-402 | Invoke `claude -p` in the target directory with skill content and JSON output mode | P0 |
| R-403 | Lenient JSON response parsing: extract what's available, use defaults for missing fields | P0 |
| R-404 | Architecture summary: high-level project description, patterns, design approach | P0 |
| R-405 | Feature inventory: list of features, their completeness status, WIP items | P0 |
| R-406 | Code quality review: anti-patterns, dead code, inconsistencies | P1 |
| R-407 | Effort estimation: approximate dev-hours for existing code and remaining work | P1 |
| R-408 | Store AI analysis results in snapshots for historical comparison | P0 |
| R-409 | AI results displayed in a dedicated section of the dashboard | P0 |

## Phase 6: History & Trends

> Goal: Sparkline trend visualization, explicit snapshot comparison, cross-repo index.

| ID | Requirement | Priority |
|----|-------------|----------|
| R-500 | `repostat trend` subcommand: sparkline charts of key metrics across all snapshots | P0 |
| R-501 | Git-aware history: lines added/removed per period, commit frequency, contributor count | P0 |
| R-502 | Snapshot comparison: diff any two snapshots by timestamp or git SHA | P1 |
| R-503 | Cross-repo index in `~/.repostat/repos.json`: track all analyzed repos for quick listing | P1 |
| R-504 | `repostat list` subcommand: show all tracked repos with last-analyzed date | P1 |

## Phase 7: Polish & Distribution

> Goal: Production quality CLI experience, packaging, and documentation.

| ID | Requirement | Priority |
|----|-------------|----------|
| R-600 | Shell completions: bash, zsh, fish | P1 |
| R-601 | Man page generation | P2 |
| R-602 | `cargo install` from crates.io | P1 |
| R-603 | Homebrew formula | P2 |
| R-604 | GitHub Releases with pre-built binaries (macOS arm64/x64, Linux x64) | P1 |
| R-605 | CI pipeline: test, lint, format, clippy on every PR | P0 |
| R-606 | README with install instructions, usage examples, and screenshots | P0 |

## Cross-Cutting Requirements

These apply to ALL phases and are never deferred.

| ID | Requirement |
|----|-------------|
| R-X01 | All features are developed test-first (TDD). Tests exist before implementation. |
| R-X02 | All features have a spec before code (SDD). Specs live in `docs/specs/`. |
| R-X03 | `cargo clippy -- -D warnings` passes with zero warnings. |
| R-X04 | `cargo fmt --check` passes. |
| R-X05 | Error messages are actionable. The user knows what went wrong and what to do. |
| R-X06 | No `unwrap()` or `expect()` in non-test code. Use proper error handling. |
| R-X07 | All public APIs have doc comments. |
| R-X08 | Performance: full analysis of a 50k-line repo completes in under 5 seconds (excluding AI). |
