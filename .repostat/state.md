# repostat — Project State

> Auto-maintained by /go. Persists across sessions.

## Current Task

- **Phase:** 2
- **Task:** Phase complete — all 8 tasks done
- **Status:** idle
- **Blocker:** none
- **Started:** —

## Progress

### Phase 1: Foundation & Core Metrics
- [x] All 15 tasks — completed 2026-03-19 (shipped as v0.2.0, PR #1 merged)

### Phase 2: Complexity Analysis
- [x] Tree-sitter integration (10 grammars) — completed 2026-03-19
- [x] Cyclomatic complexity — completed 2026-03-19
- [x] Cognitive complexity — completed 2026-03-19
- [x] Function extraction — completed 2026-03-19
- [x] Threshold flagging — completed 2026-03-19
- [x] Dashboard hotspots — completed 2026-03-19
- [x] Regex fallback — completed 2026-03-19
- [x] Complexity in snapshots — completed 2026-03-19

### Phase 3–7
- [ ] Not started

## Learnings

- 2026-03-19: tree-sitter 0.25 needed (0.24 has ABI mismatch with newer grammar crates like python 0.25, go 0.25).
- 2026-03-19: Language enum match arms for display_name/from_extension have high CC (50+) but are trivially simple — match exhaustiveness inflates cyclomatic.
- 2026-03-19: #[allow(dead_code)] on module declaration in mod.rs silences all items in that module.

## Session Log

| Date | Tasks Completed | Notes |
|------|----------------|-------|
| 2026-03-19 | Phase 1 (15 tasks) | Shipped as v0.2.0, PR #1 merged |
| 2026-03-19 | Phase 2 (8 tasks) | Tree-sitter, cyclomatic, cognitive, functions, thresholds, hotspots, regex, snapshots |
