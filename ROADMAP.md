# Roadmap

> Execution plan for `repostat`. Phases are sequential — each builds on the prior.
> See `docs/requirements.md` for detailed requirements per phase.

## Phase 1: Foundation & Core Metrics

**Goal**: `repostat ./path` produces accurate line counts, language breakdown, and file
statistics with a terminal dashboard. Snapshots are stored and diffed.

- [x] Project scaffold: `Cargo.toml`, module structure, CI config
- [x] CLI argument parsing with `clap` (path argument, `--help`, `--version`)
- [x] `.repostat.toml` config file loading and validation
- [x] File scanner: recursive walk with gitignore + heuristic + config exclusions
- [x] Language detection from file extensions (50+ languages)
- [x] Line counting engine: code, comments, blanks — per file, per language
- [x] Generated/minified file detection and exclusion
- [x] Metric aggregation: totals, per-language breakdowns, file counts
- [x] Snapshot storage: write JSON to `.repostat/snapshots/`
- [x] Snapshot diffing: compare current run to most recent snapshot
- [x] Terminal dashboard: compact box-drawn output with all Phase 1 metrics
- [x] `--json` flag for machine-readable output
- [x] `--markdown` flag for report generation
- [x] Parallel file processing with `rayon`
- [x] Color support with `NO_COLOR` respect

**Exit Criteria**: Run `repostat ./path` on 3+ real repos. Output is accurate, fast (<2s
for 50k lines), and the dashboard is readable. Snapshots persist and diffs display.

---

## Phase 2: Complexity Analysis

**Goal**: Tree-sitter powered cyclomatic and cognitive complexity. Hotspot identification.

- [x] Tree-sitter integration and grammar compilation (top 10 languages)
- [x] Cyclomatic complexity calculation per function and per file
- [x] Cognitive complexity calculation (nested-logic weighting)
- [x] Function extraction: name, line count, complexity per function
- [x] File size and function size threshold flagging
- [x] Complexity hotspots section in the dashboard (top N worst)
- [x] Regex fallback for unsupported languages
- [x] Complexity data in snapshots for trend tracking

**Exit Criteria**: Complexity scores match manual calculation on test fixtures.
Hotspots correctly identify the most complex code. All 10 bundled grammars work.

---

## Phase 3: Dependency & Coupling Analysis

**Goal**: External dependency counts and internal coupling metrics.

- [x] Dependency manifest parser (package.json, Cargo.toml, requirements.txt, go.mod, etc.)
- [x] Direct dependency counting per manifest
- [x] Lock file parsing for transitive dependency counts
- [x] Import/require/use statement parsing for internal dependency graph
- [ ] Fan-in / fan-out calculation per module
- [ ] Dependencies section in dashboard
- [ ] Dependency data in snapshots

**Exit Criteria**: Dependency counts match manual verification. Internal coupling
graph identifies the most-connected modules.

---

## Phase 4: Documentation Analysis

**Goal**: Measure and score documentation coverage and quality.

- [ ] Markdown file inventory (count, lines, characters)
- [ ] Doc-to-code ratio calculation
- [ ] README completeness checker (install, usage, API, contributing, license sections)
- [ ] Per-directory documentation coverage
- [ ] Documentation section in dashboard
- [ ] Documentation metrics in snapshots

**Exit Criteria**: README scoring matches manual evaluation. Doc-to-code ratio is accurate.

---

## Phase 5: AI-Augmented Analysis

**Goal**: Claude CLI integration for architecture, features, quality, and effort estimation.

- [ ] Claude CLI detection (`which claude`)
- [ ] Skill file system: load from `~/.repostat/skills/`, write defaults on first run
- [ ] Claude CLI invocation: `claude -p` in target dir with skill content and JSON output
- [ ] Lenient JSON response parsing with defaults for missing fields
- [ ] Architecture summary skill
- [ ] Feature inventory skill
- [ ] Code quality review skill
- [ ] Effort estimation skill
- [ ] Stale documentation detection skill
- [ ] Doc quality scoring skill
- [ ] AI results section in dashboard
- [ ] AI results stored in snapshots
- [ ] Graceful degradation when Claude CLI unavailable

**Exit Criteria**: AI analysis runs on 3+ repos and returns useful, parseable results.
Graceful skip works when Claude CLI is missing. Results are stored in snapshots.

---

## Phase 6: History & Trends

**Goal**: Sparkline trends, git history integration, cross-repo tracking.

- [ ] `repostat trend` subcommand with sparkline charts
- [ ] Git log integration: lines added/removed per period
- [ ] Commit frequency and contributor count from git
- [ ] Snapshot comparison by timestamp or SHA
- [ ] Cross-repo index at `~/.repostat/repos.json`
- [ ] `repostat list` subcommand
- [ ] Trend data in terminal (inline sparklines in dashboard)

**Exit Criteria**: Sparklines render correctly for 5+ snapshots. Git history
analysis matches `git log --stat`. Cross-repo listing works.

---

## Phase 7: Polish & Distribution

**Goal**: Production-quality CLI experience and packaging.

- [ ] Shell completions (bash, zsh, fish)
- [ ] CI pipeline: fmt, clippy, test, build on every PR
- [ ] README with install instructions, usage examples, screenshots
- [ ] `cargo install` publishing to crates.io
- [ ] GitHub Releases with pre-built binaries (macOS arm64/x64, Linux x64)
- [ ] Homebrew formula
- [ ] Man page generation
- [ ] `repostat` dogfooding: analyze itself, include report in README

**Exit Criteria**: Install works via `cargo install` and `brew install`. CI is green.
README is complete. The tool can analyze its own codebase.
