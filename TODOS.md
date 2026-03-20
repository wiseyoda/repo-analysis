# TODOs

> Deferred items from reviews and development. Items here are tracked but not on the active roadmap.

---

## From CEO Review (2026-03-20)

### Compare Mode
**What:** `repostat compare repo-a repo-b` — side-by-side metrics comparison of two repos.
**Why:** Useful for evaluating codebases before merging teams or choosing between implementations.
**Pros:** Low effort (S), reuses all existing metrics, natural CLI extension.
**Cons:** Niche use case — most users analyze one repo at a time.
**Context:** Surfaced during Phase 9 CEO review cherry-pick ceremony. Deferred in favor of core improvements (risk scoring, parallel AI, diff mode). Can be built after Phase 9 ships.
**Effort:** S (human: ~1 day / CC: ~15 min)
**Priority:** P3
**Depends on:** Nothing — standalone feature.

### Watch Mode
**What:** `repostat --watch ./src` — re-run analysis on file changes, show only deltas.
**Why:** Useful during refactoring sessions to see if complexity is improving or worsening.
**Pros:** Great developer inner-loop tool, real-time feedback.
**Cons:** Requires `notify` crate dependency, debounce logic, partial re-analysis complexity.
**Context:** Surfaced during Phase 9 CEO review. Skipped because it adds a new dependency and non-trivial complexity (file watcher + debounce + incremental analysis). Better as Phase 10 with incremental analysis as prerequisite.
**Effort:** M (human: ~2 days / CC: ~30 min)
**Priority:** P2
**Depends on:** Incremental analysis mode (BACKLOG) would make this much more efficient.

## From Eng Review (2026-03-20)

### Shallow Clone Detection
**What:** Detect shallow clones (`git rev-parse --is-shallow-repository`) and note limited git history in `--verbose` output when computing churn-based risk scores.
**Why:** On shallow clones, churn counts are artificially low, making risk scores unreliable. Users should be warned.
**Pros:** Prevents misleading risk scores on CI environments that use shallow clones by default.
**Cons:** Minor — just a detection check and warning message.
**Context:** Found during Phase 9 eng review failure modes analysis. Shallow clones are common in CI (GitHub Actions defaults to `--depth 1`). Not blocking Phase 9 but worth tracking.
**Effort:** S (human: ~2 hours / CC: ~5 min)
**Priority:** P3
**Depends on:** Phase 9 churn collection (collect_file_churn).
