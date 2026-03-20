# Changelog

All notable changes to repostat will be documented in this file.

Format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
Versioning follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.9.0] - 2026-03-20

### Added
- **Risk scoring**: per-file risk scores combining git churn and cyclomatic complexity — find your riskiest code
- **Parallel AI analysis**: 6 Claude CLI skills now run concurrently (~60s → ~15s)
- **Health exit codes**: exit 0 (healthy), 10 (warning), 20 (critical) for zero-config CI integration
- **`repostat init`**: generate a commented `.repostat.toml` config file with `--force` for overwrites
- **`repostat diff HEAD~N`**: analyze only files changed in recent commits
- **`--html` flag**: self-contained HTML report with SVG bar charts, no JavaScript
- **`--verbose` flag**: per-phase timing on stderr (scanner, metrics, AI, report)
- **`REPOSTAT_SKIP_AI` env var**: skip AI analysis for fast runs and CI
- Health thresholds configurable via `[health]` section in `.repostat.toml`
- Risk hotspots section in terminal dashboard, markdown, and JSON output
- Risk data persisted in snapshots as raw inputs (churn_count, max_complexity)

### Fixed
- File read errors now logged to stderr with count instead of silently dropped
- Corrupt snapshot files produce friendly warnings instead of cryptic errors
- Index write failures now warn instead of being silently ignored
- Zero files after filtering now shows a helpful warning message
- Claude CLI timeout extended from 60s to 180s with process kill on timeout
- Integration tests now run in 0.7s (was 222s) via REPOSTAT_SKIP_AI
- Version mismatch between Cargo.toml and VERSION file resolved

### Changed
- `Snapshot::from_aggregate` replaced with `AnalysisResult` builder struct
- AI module test coverage expanded to 30 tests
- Report module test coverage expanded to 30 tests
- Total test count: 251 (236 unit + 15 integration)

## [0.8.0] - 2026-03-20

### Added
- Shell completions for bash, zsh, and fish via `repostat completions <shell>`
- Man page generation via `repostat manpage`
- GitHub Actions CI pipeline: fmt, clippy, test, build on every PR
- GitHub Actions release workflow: pre-built binaries for macOS (arm64/x64) and Linux (x64)
- Homebrew formula template for installation via `brew install`
- Cargo.toml metadata for `cargo install` from crates.io
- Complete README with install instructions, usage examples, configuration, and dogfooding output

### Changed
- Bumped Cargo.toml version to match release tags
- CI now runs unit tests only (skips integration tests that require Claude CLI)

## [0.7.0] - 2026-03-20

### Added
- `repostat trend` subcommand with Unicode sparkline charts (▁▂▃▄▅▆▇█) across all snapshots
- Git log integration: total commits, unique contributors, per-week activity (commits, lines added/removed)
- Snapshot comparison support via `load_all()` for loading entire snapshot history
- Cross-repo index at `~/.repostat/repos.json` tracking all analyzed repositories
- `repostat list` subcommand showing tracked repos with last-analyzed date and snapshot count
- Inline sparklines in the main dashboard (Files/Lines rows) when 3+ snapshots exist
- CLI refactored to subcommands while preserving backward compatibility (default = analyze)
- Feature spec: docs/specs/history-trends.md

## [0.6.0] - 2026-03-20

### Added
- Claude CLI detection (`which claude`) with graceful skip when unavailable
- Skill file system: 6 analysis prompts loaded from `~/.repostat/skills/`, defaults written on first run
- Claude CLI invocation via `claude -p --model haiku --output-format json` with 60s timeout
- Lenient JSON response parsing: direct parse, code block extraction, brace-delimited fallback
- Architecture summary skill: project description, patterns, design approach
- Feature inventory skill: feature list with completeness status
- Code quality review skill: anti-patterns, dead code, inconsistencies with overall score
- Effort estimation skill: dev-hours for existing code and remaining work
- Stale documentation detection skill: finds docs referencing removed code
- Documentation quality scoring skill: per-file clarity/completeness ratings
- AI Analysis section in terminal dashboard (architecture, features, quality, effort)
- AI analysis results stored in JSON snapshots for historical comparison
- Graceful degradation: all other analysis continues normally when Claude CLI is missing
- Refactored dashboard render to use `DashboardData` struct (clippy compliance)
- Feature spec: docs/specs/ai-augmented-analysis.md

## [0.5.0] - 2026-03-20

### Added
- Markdown file inventory: count, total lines, and total characters across all .md files
- Doc-to-code ratio calculation (documentation lines vs code lines)
- README completeness checker scoring 5 sections: install, usage, API, contributing, license
- Per-directory documentation coverage (source dirs with accompanying docs)
- Documentation section in terminal dashboard showing all doc metrics
- Documentation metrics stored in JSON snapshots for trend tracking
- Feature spec: docs/specs/documentation-analysis.md

## [0.4.0] - 2026-03-20

### Added
- Dependency manifest parser for 8 ecosystems (Cargo.toml, package.json, requirements.txt, Pipfile, go.mod, Gemfile, pom.xml, build.gradle)
- Direct dependency counting per manifest with project-wide totals
- Transitive dependency counting from lock files (Cargo.lock, package-lock.json, yarn.lock, go.sum, Gemfile.lock, poetry.lock)
- Internal coupling analysis: import/require/use statement parsing for 8+ languages
- Fan-in / fan-out metrics per module from the coupling graph
- Dependencies section in terminal dashboard showing manifest counts and dep totals
- Dependency and coupling data stored in JSON snapshots for trend tracking

## [0.3.0] - 2026-03-20

### Added
- Tree-sitter integration with compiled grammars for 10 languages (Rust, Python, JS, TS, Go, Java, C, C++, Swift, Ruby)
- Cyclomatic complexity calculation per function and per file
- Cognitive complexity calculation with nesting depth penalties (SonarQube-style)
- Function extraction: name, line count, cyclomatic and cognitive scores
- File and function size threshold flagging (configurable, defaults: 50/500 lines)
- Complexity hotspots section in the terminal dashboard (top 10 most complex functions)
- Regex-based complexity fallback for languages without tree-sitter grammars
- Complexity hotspot data stored in JSON snapshots for trend tracking

## [0.2.0] - 2026-03-19

### Added
- CLI with path argument, `--help`, `--version`, `--json`, `--markdown` flags
- `.repostat.toml` config file with custom include/exclude glob patterns
- Recursive file scanner with three-layer exclusion (gitignore, heuristics, config)
- Language detection for 52+ programming languages by file extension
- Line counting engine with language-aware comment detection (single + block)
- Generated/minified file detection and exclusion from metrics
- Per-language metric aggregation with file counts and line breakdowns
- JSON snapshot storage in `.repostat/snapshots/` with timestamps and git SHA
- Snapshot diffing showing deltas from the previous analysis run
- Terminal dashboard with box-drawn output, per-language table, and diff display
- Markdown report generation via `--markdown` flag
- Color output with `NO_COLOR` environment variable support
- Parallel file processing via rayon for multi-core performance
- GitHub Actions CI pipeline (fmt, clippy, test, build)

## [0.1.0] - 2026-03-19

### Added
- Project documentation: constitution, requirements, tech-stack, coding-standard, decisions
- ROADMAP with 7 phases and exit criteria
- BACKLOG for deferred items
- Claude Code project configuration: hooks, rules, skills, settings
- 11 custom skills: /go, /test, /review, /add-feature, /refactor, /check,
  /analyze-arch, /add-module, /ship, /fix, /status, /spec
- Automated enforcement hooks: Rust safety, constitution protection, pre-commit gate
- Persistent project state tracking (.repostat/state.md)
