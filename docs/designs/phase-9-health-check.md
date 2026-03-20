# Phase 9: Developer Health Check

> Design document promoted from CEO review on 2026-03-20.
> Mode: Selective Expansion | Approach: B (Developer Health Check)

## Vision

Make repostat the smartest CLI code analyzer. Don't just report problems — guide
developers toward fixing them. The 10x version answers: "What code is most likely
to cause your next production incident?"

Combined with churn data, repostat becomes the only CLI tool that produces a
**risk score** per file: high churn + high complexity = ticking time bomb.

## Goals

1. **Churn + Complexity Risk Score** — the signature feature
2. **Parallel AI Skills** — 60s → ~15s analysis time
3. **Health Score Exit Codes** — zero-cost CI integration
4. **`repostat init`** — professional first-run experience
5. **`repostat diff HEAD~N`** — scoped analysis for recent changes
6. **HTML Dashboard** — rich visualization without web infrastructure

## Bug Fixes (from CEO review)

1. Fix silent file read errors (log to stderr + count skipped)
2. Kill child process on AI timeout (extend timeout to 3 min)
3. Create AnalysisResult builder struct (from_aggregate has 6 params)
4. Fix corrupt snapshot error messages (friendly instead of cryptic)
5. Fix silent index write failures (warn on ~/.repostat/repos.json error)
6. Warn when 0 files analyzed after filtering
7. Fix integration test speed (REPOSTAT_SKIP_AI env var)
8. Add AI module unit tests (skills.rs + schema.rs)
9. Sync Cargo.toml version to 0.9.0
10. Add --verbose flag with phase timing

## Scope Decisions

| # | Proposal | Effort | Decision | Reasoning |
|---|----------|--------|----------|-----------|
| 1 | Churn + Complexity Risk Score | M | ACCEPTED | Most differentiating feature |
| 2 | Parallel AI Skills | S | ACCEPTED | Single biggest UX improvement |
| 3 | Health Score Exit Code | S | ACCEPTED | Zero-cost CI bridge |
| 4 | `repostat init` command | S | ACCEPTED | First-run polish |
| 5 | `repostat diff HEAD~N` | M | ACCEPTED | Scoped analysis |
| 6 | HTML dashboard output | M | ACCEPTED | Richer visualization |
| 7 | `--watch` mode | M | SKIPPED | Adds dependency + complexity |
| 8 | `repostat compare` | S | DEFERRED | Lower priority |

## Architecture Notes (updated by eng review)

### Risk Scores
- **Per-file churn is a NEW data pipeline** (not pure reuse of git_history.rs)
- New function: `collect_file_churn()` via single `git log --name-only --since=6months`
- Returns `BTreeMap<PathBuf, usize>` (file path → commit count)
- Snapshots store **raw inputs** (churn_count, max_complexity per file), NOT computed scores
- Score formula (`churn * complexity`) is computed at display time — formula changes don't invalidate history
- Graceful degradation: no git → skip risk scores, assess complexity only

### Parallel AI
- Use `rayon::scope` (already in project) for 6 concurrent `claude::invoke()` calls
- No custom thread pool — boring technology, proven in this codebase
- Timeout extended to 3 minutes with child process kill on timeout

### Exit Codes
- `0` = success (healthy)
- `1` = tool error (bad path, config parse failure, render error)
- `10` = health warning (thresholds exceeded)
- `20` = health critical (severe thresholds exceeded)
- Highest severity wins when multiple thresholds are exceeded

### Health Thresholds (defaults, overridable via `[health]` in .repostat.toml)
- **WARNING:** cyclomatic > 25 OR function > 60 lines OR risk > 50th percentile
- **CRITICAL:** cyclomatic > 50 OR function > 100 lines OR risk > 90th percentile
- Graceful degradation: missing data sources → assess available metrics only

### HTML Output
- Pure SVG charts generated in Rust — no JavaScript, no external dependencies
- Self-contained single HTML file with inline CSS and SVG bar/line charts
- Deterministic output, testable with insta snapshot tests

### Diff Mode
- `repostat diff HEAD~N` — commits only (no branch names, dates, or tags in v1)
- Uses `git diff --name-only HEAD~N..HEAD` to get changed file paths
- Filters scanner output to those paths, runs normal analysis pipeline on subset
- Non-git repos → clear error message

### Init Command
- `repostat init` creates `.repostat.toml` with commented defaults + `[health]` section
- Error if `.repostat.toml` exists — use `--force` to overwrite
- Non-writable directory → propagated IO error

### Report Module Tests (pulled from TODOS.md)
- Add unit tests for dashboard.rs, markdown.rs, trend.rs, html.rs before modifying
- "Make the change easy, then make the easy change" pattern

## NOT in scope

- Watch mode (needs `notify` dependency, better after incremental analysis)
- Compare mode (niche use case, deferred to TODOS.md)
- SQLite storage (not needed until 100+ snapshots)
- CI integration GitHub Action (Phase 10 territory)
- Web dashboard (out of scope for CLI identity)
- Direct Anthropic API (Claude CLI works)
- Branch/date/tag support in diff mode (v1 is commits only)
- Interactive HTML charts (pure SVG chosen for determinism)
- TOML merge in init (overwrite-only with --force)
- Computed risk scores in snapshots (raw inputs stored instead)

## Eng Review Decisions (2026-03-20)

All open questions resolved:

1. **Risk score formula:** Store raw inputs (churn_count, max_complexity) in snapshots.
   Compute `score = churn * complexity` at display time. Formula is code, not data.
2. **HTML approach:** Pure SVG charts generated in Rust. No JS. Deterministic, testable.
3. **Health thresholds:** Defaults ship out of box (complexity >25/50, function >60/100).
   Override via `[health]` section in `.repostat.toml`.
4. **Parallel AI:** Use `rayon::scope` — already in project, boring technology.
5. **Diff scope:** Commits only (`HEAD~N`). No branches/dates/tags in v1.
6. **Exit codes:** 0=ok, 1=error, 10=warning, 20=critical (distinct from tool errors).
7. **Init behavior:** Error on existing config, `--force` to overwrite.
8. **Degradation:** Graceful — assess available data, skip what's missing.
9. **File churn:** Single `git log --name-only` pass, not per-file queries.
10. **Report tests:** Bundled into Phase 9 (moved from TODOS.md).

## Exit Criteria

- Risk scores appear in terminal dashboard, JSON, markdown, and HTML output
- AI analysis completes in <20s (parallel skills via rayon::scope)
- Exit codes: 0 (ok), 1 (tool error), 10 (warning), 20 (critical)
- `repostat init` creates commented `.repostat.toml`; errors on existing; `--force` overwrites
- `repostat diff HEAD~5` shows only changed-file metrics; errors on non-git dirs
- HTML output is self-contained SVG file, deterministic, snapshot-testable
- Integration tests complete in <10s (REPOSTAT_SKIP_AI env var)
- AI module has >80% test coverage
- Report module has unit tests (dashboard, markdown, trend, html)
- All bug fixes verified
- `--verbose` shows phase timing to stderr
