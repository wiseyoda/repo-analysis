# repostat — Project State

> Auto-maintained by /go. Persists across sessions.

## Current Task

- **Phase:** 2
- **Task:** Tree-sitter integration and grammar compilation (top 10 languages)
- **Status:** in-progress
- **Blocker:** none
- **Started:** 2026-03-19

## Progress

### Phase 1: Foundation & Core Metrics
- [x] All 15 tasks — completed 2026-03-19 (shipped as v0.2.0, PR #1 merged)

### Phase 2: Complexity Analysis
- [ ] Tree-sitter integration and grammar compilation (top 10 languages)
- [ ] Cyclomatic complexity calculation per function and per file
- [ ] Cognitive complexity calculation (nested-logic weighting)
- [ ] Function extraction: name, line count, complexity per function
- [ ] File size and function size threshold flagging
- [ ] Complexity hotspots section in the dashboard (top N worst)
- [ ] Regex fallback for unsupported languages
- [ ] Complexity data in snapshots for trend tracking

### Phase 3–7
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
| 2026-03-19 | Phase 1 (15 tasks) | Shipped as v0.2.0, PR #1 merged |
