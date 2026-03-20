# Changelog

All notable changes to repostat will be documented in this file.

Format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
Versioning follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
