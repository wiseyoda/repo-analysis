# repostat — Project State

> Auto-maintained by /go. Persists across sessions.

## Current Task

- **Phase:** 1
- **Task:** Project scaffold: `Cargo.toml`, module structure, CI config
- **Status:** idle
- **Blocker:** none
- **Started:** —

## Progress

### Phase 1: Foundation & Core Metrics
- [ ] Project scaffold: `Cargo.toml`, module structure, CI config
- [ ] CLI argument parsing with `clap` (path argument, `--help`, `--version`)
- [ ] `.repostat.toml` config file loading and validation
- [ ] File scanner: recursive walk with gitignore + heuristic + config exclusions
- [ ] Language detection from file extensions (50+ languages)
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

> Things discovered during implementation that future sessions need to know.

(none yet)

## Session Log

| Date | Tasks Completed | Notes |
|------|----------------|-------|
| — | — | Project initialized with docs and .claude config |
