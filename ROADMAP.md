# Roadmap

> Execution plan for `repostat`. Phases are sequential — each builds on the prior.
> See `docs/requirements.md` for detailed requirements per phase.

## Phase 1: Foundation & Core Metrics

**Goal**: `repostat ./path` produces accurate line counts, language breakdown, and file
statistics with a terminal dashboard. Snapshots are stored and diffed.

- [x] Project scaffold: `Cargo.toml`, module structure, CI config
- [x] CLI argument parsing with `clap` (path argument, `--help`, `--version`)
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

**Exit Criteria**: Run `repostat ./path` on 3+ real repos. Output is accurate, fast (<2s
for 50k lines), and the dashboard is readable. Snapshots persist and diffs display.

---

## Phase 2: Complexity Analysis

**Goal**: Tree-sitter powered cyclomatic and cognitive complexity. Hotspot identification.

- [ ] Tree-sitter integration and grammar compilation (top 10 languages)
- [ ] Cyclomatic complexity calculation per function and per file
- [ ] Cognitive complexity calculation (nested-logic weighting)
- [ ] Function extraction: name, line count, complexity per function
- [ ] File size and function size threshold flagging
- [ ] Complexity hotspots section in the dashboard (top N worst)
- [ ] Regex fallback for unsupported languages
- [ ] Complexity data in snapshots for trend tracking

**Exit Criteria**: Complexity scores match manual calculation on test fixtures.
Hotspots correctly identify the most complex code. All 10 bundled grammars work.

---

## Phase 3: Dependency & Coupling Analysis

**Goal**: External dependency counts and internal coupling metrics.

- [ ] Dependency manifest parser (package.json, Cargo.toml, requirements.txt, go.mod, etc.)
- [ ] Direct dependency counting per manifest
- [ ] Lock file parsing for transitive dependency counts
- [ ] Import/require/use statement parsing for internal dependency graph
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
