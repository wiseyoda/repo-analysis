# repostat — Project State

> Auto-maintained by /go. Persists across sessions.

## Current Task

- **Phase:** 9
- **Task:** Bug fixes: silent file read errors (log + count skipped files)
- **Status:** in-progress
- **Blocker:** none
- **Started:** 2026-03-20

## Progress

### Phase 1: Foundation & Core Metrics
- [x] All 15 tasks — completed 2026-03-19 (shipped as v0.2.0, PR #1 merged)

### Phase 2: Complexity Analysis
- [x] All 8 tasks — completed 2026-03-19 (shipped as v0.3.0, PR #2 merged)

### Phase 3: Dependency & Coupling Analysis
- [x] All 7 tasks — completed 2026-03-20 (shipped as v0.4.0, PR #3 merged)

### Phase 4: Documentation Analysis
- [x] All 6 tasks — completed 2026-03-20 (shipped as v0.5.0, PR #4 merged)

### Phase 5: AI-Augmented Analysis
- [x] All 13 tasks — completed 2026-03-20 (shipped as v0.6.0, PR #5 merged)

### Phase 6: History & Trends
- [x] All 7 tasks — completed 2026-03-20 (shipped as v0.7.0, PR #6 merged)

### Phase 7: Polish & Distribution
- [x] All 8 tasks — completed 2026-03-20 (shipped as v0.8.0, PR #7 merged)

### Phase 9: Developer Health Check
- [ ] Bug fixes: silent file read errors (log + count skipped files) — in progress
- [ ] Bug fixes: AnalysisResult builder struct
- [ ] Bug fixes: corrupt snapshot friendly error + silent index write warning
- [ ] Bug fixes: warn when 0 files analyzed after filtering
- [ ] Bug fixes: integration test speed (REPOSTAT_SKIP_AI env var)
- [ ] Bug fixes: AI module unit tests (skills.rs + schema.rs)
- [ ] Bug fixes: add --verbose flag with phase timing
- [ ] Bug fixes: sync Cargo.toml version to 0.9.0
- [ ] Report module unit tests
- [ ] Per-file churn collection
- [ ] Churn + complexity risk score
- [ ] Risk scores in snapshots
- [ ] Risk scores in dashboard, JSON, markdown
- [ ] Parallel AI skills
- [ ] Health score exit codes
- [ ] Health thresholds in config
- [ ] repostat init command
- [ ] repostat diff HEAD~N
- [ ] HTML dashboard output
- [ ] Graceful degradation

## Learnings

- 2026-03-19: tree-sitter 0.25 needed for grammar ABI compatibility.
- 2026-03-19: Language enum match arms inflate cyclomatic complexity scores.
- 2026-03-19: #[allow(dead_code)] on module declaration silences all items.
- 2026-03-20: Must use --model haiku for Claude CLI invocation to avoid defaulting to slow model.
- 2026-03-20: Dashboard render() hit clippy too_many_arguments; refactored to DashboardData struct.
- 2026-03-20: CLI refactored to subcommands while preserving backward compat (default = analyze).
- 2026-03-20: Sparkline uses 8-level Unicode block chars, needs min 3 snapshots to display.
- 2026-03-20: clap_complete and clap_mangen provide shell completions and man pages via subcommands.
- 2026-03-20: Claude CLI timeout extended from 60s to 180s with process kill on timeout.

## Session Log

| Date | Tasks Completed | Notes |
|------|----------------|-------|
| 2026-03-19 | Phase 1 (15 tasks) | Shipped as v0.2.0, PR #1 merged |
| 2026-03-19 | Phase 2 (8 tasks) | Shipped as v0.3.0, PR #2 merged |
| 2026-03-20 | Phase 3 (7 tasks) | Manifest parsing, lock files, coupling graph, fan-in/out, dashboard, snapshots |
| 2026-03-20 | Phase 4 (6 tasks) | Doc inventory, ratio, README scoring, dir coverage, dashboard, snapshots |
| 2026-03-20 | Phase 5 (13 tasks) | CLI detection, skill files, invocation, parsing, 6 skills, dashboard, snapshots |
| 2026-03-20 | Phase 6 (7 tasks) | Subcommands, trend sparklines, git history, cross-repo index, inline sparklines |
| 2026-03-20 | Phase 7 (8 tasks) | Shell completions, CI, README, Cargo.toml, releases, Homebrew, man pages, dogfooding |
