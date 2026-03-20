# repostat — Project State

> Auto-maintained by /go. Persists across sessions.

## Current Task

- **Phase:** 1
- **Task:** Line counting engine: code, comments, blanks — per file, per language
- **Status:** idle
- **Blocker:** none
- **Started:** —

## Progress

### Phase 1: Foundation & Core Metrics
- [x] Project scaffold: `Cargo.toml`, module structure, CI config — completed 2026-03-19
- [x] CLI argument parsing with `clap` (path argument, `--help`, `--version`) — completed 2026-03-19
- [x] `.repostat.toml` config file loading and validation — completed 2026-03-19
- [x] File scanner: recursive walk with gitignore + heuristic + config exclusions — completed 2026-03-19
- [x] Language detection from file extensions (50+ languages) — completed 2026-03-19
- [ ] Line counting engine: code, comments, blanks — per file, per language
- [ ] Generated/minified file detection and exclusion
- [ ] Metric aggregation: totals, per-language breakdowns, file counts
- [ ] Snapshot storage: write JSON to `.repostat/snapshots/`
- [ ] Snapshot diffing: compare current run to most recent snapshot
- [ ] Terminal dashboard: compact box-drawn output with all Phase 1 metrics
- [ ] `--json` flag for machine-readable output
- [ ] `--markdown` flag for report generation
- [ ] Parallel file processing with `rayon`
- [ ] Color support with `NO_COLOR` respect

### Phase 2–7
- [ ] Not started

## Learnings

- 2026-03-19: Rust 1.93.1 on this machine; edition 2024 compiles without issues.
- 2026-03-19: Clippy treats pub(crate) fields as dead_code if not read in non-test code. Wire new modules into main.rs immediately to avoid this.
- 2026-03-19: The `ignore` crate needs a git init in the test dir for .gitignore to be respected.
- 2026-03-19: Added `globset` crate for config pattern matching (Layer 3 of ADR-005).
- 2026-03-19: Clippy upper_case_acronyms lint fires on enum variants like PHP, SQL, HTML. Use #[allow(clippy::upper_case_acronyms)] on the enum.

## Session Log

| Date | Tasks Completed | Notes |
|------|----------------|-------|
| — | — | Project initialized with docs and .claude config |
| 2026-03-19 | Project scaffold | Cargo.toml, 9 source files, CI config, 3 integration tests. |
| 2026-03-19 | CLI argument parsing | Path validation, error handling, spec + 4 new integration tests. |
| 2026-03-19 | Config loading | .repostat.toml parsing with exclude/include patterns, 7 unit tests. |
| 2026-03-19 | File scanner | 3-layer exclusion (gitignore, heuristics, config), globset, 8 unit tests. |
| 2026-03-19 | Language detection | 52 languages, ScannedFile struct, 11 tests. |
