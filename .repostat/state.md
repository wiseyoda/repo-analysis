# repostat — Project State

> Auto-maintained by /go. Persists across sessions.

## Current Task

- **Phase:** 1
- **Task:** Phase complete — all 15 tasks done
- **Status:** idle
- **Blocker:** none
- **Started:** —

## Progress

### Phase 1: Foundation & Core Metrics
- [x] Project scaffold — completed 2026-03-19
- [x] CLI argument parsing — completed 2026-03-19
- [x] Config file loading — completed 2026-03-19
- [x] File scanner — completed 2026-03-19
- [x] Language detection — completed 2026-03-19
- [x] Line counting engine — completed 2026-03-19
- [x] Generated/minified detection — completed 2026-03-19
- [x] Metric aggregation — completed 2026-03-19
- [x] Snapshot storage — completed 2026-03-19
- [x] Snapshot diffing — completed 2026-03-19
- [x] Terminal dashboard — completed 2026-03-19
- [x] --json flag — completed 2026-03-19
- [x] --markdown flag — completed 2026-03-19
- [x] Parallel processing (rayon) — completed 2026-03-19
- [x] Color support (NO_COLOR) — completed 2026-03-19

### Phase 2–7
- [ ] Not started

## Learnings

- 2026-03-19: Rust 1.93.1 on this machine; edition 2024 compiles without issues.
- 2026-03-19: Clippy treats pub(crate) fields as dead_code if not read in non-test code.
- 2026-03-19: The `ignore` crate needs a git init in test dirs for .gitignore to work.
- 2026-03-19: Added `globset` crate for config pattern matching.
- 2026-03-19: Clippy upper_case_acronyms lint — use #[allow] on the Language enum.
- 2026-03-19: Block comment detection: check if close marker appears after open on same line.

## Session Log

| Date | Tasks Completed | Notes |
|------|----------------|-------|
| 2026-03-19 | Tasks 1-5 | Scaffold, CLI, config, scanner, language detection |
| 2026-03-19 | Tasks 6-15 | LOC, filters, aggregation, snapshots, dashboard, JSON, markdown, rayon, color |
