# repostat — Project State

> Auto-maintained by /go. Persists across sessions.

## Current Task

- **Phase:** 3
- **Task:** Phase complete — all 7 tasks done
- **Status:** idle
- **Blocker:** none
- **Started:** —

## Progress

### Phase 1: Foundation & Core Metrics
- [x] All 15 tasks — completed 2026-03-19 (shipped as v0.2.0, PR #1 merged)

### Phase 2: Complexity Analysis
- [x] All 8 tasks — completed 2026-03-19 (shipped as v0.3.0, PR #2 merged)

### Phase 3: Dependency & Coupling Analysis
- [x] Dependency manifest parser (8 ecosystems) — completed 2026-03-20
- [x] Direct dependency counting — completed 2026-03-20
- [x] Lock file parsing (transitive deps) — completed 2026-03-20
- [x] Import/use statement parsing (coupling graph) — completed 2026-03-20
- [x] Fan-in / fan-out calculation — completed 2026-03-20
- [x] Dependencies section in dashboard — completed 2026-03-20
- [x] Dependency data in snapshots — completed 2026-03-20

### Phase 4–7
- [ ] Not started

## Learnings

- 2026-03-19: tree-sitter 0.25 needed for grammar ABI compatibility.
- 2026-03-19: Language enum match arms inflate cyclomatic complexity scores.
- 2026-03-19: #[allow(dead_code)] on module declaration silences all items.
- 2026-03-20: Cargo.toml dep parsing uses simple line-by-line under [dependencies] sections — doesn't handle inline tables perfectly but works for counting.
- 2026-03-20: Lock file parsers are ecosystem-specific; Cargo.lock and poetry.lock use [[package]] TOML blocks, package-lock.json uses JSON objects.

## Session Log

| Date | Tasks Completed | Notes |
|------|----------------|-------|
| 2026-03-19 | Phase 1 (15 tasks) | Shipped as v0.2.0, PR #1 merged |
| 2026-03-19 | Phase 2 (8 tasks) | Shipped as v0.3.0, PR #2 merged |
| 2026-03-20 | Phase 3 (7 tasks) | Manifest parsing, lock files, coupling graph, fan-in/out, dashboard, snapshots |
