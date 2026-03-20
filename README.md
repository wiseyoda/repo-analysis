# repostat

Fast CLI for repository complexity analysis, progress tracking, and AI-augmented insights. Built in Rust.

```
┌───────────────────────────────────┐
│  my-app  v0.3.0                   │
├───────────────────────────────────┤
│ Code     12,450 lines             │
│ Docs      1,230 lines             │
│ Files       142                   │
│ Complexity  6.2 avg               │
│ Deps         47 (12 deep)         │
│ Effort    ~180 dev-hours          │
│ Health    ███████▓░░ 72/100       │
└───────────────────────────────────┘
Δ since last: +1,200 lines, +8 files
```

## What It Does

Point it at any repo. Get back:

- **Line counts** — code, comments, blanks, per language, per file
- **Complexity** — cyclomatic + cognitive complexity via tree-sitter (10+ languages)
- **Dependencies** — direct + transitive counts from manifests and lockfiles
- **Documentation** — coverage ratio, README completeness, staleness detection
- **AI insights** — architecture summary, feature inventory, effort estimation (via Claude CLI)
- **Progress tracking** — JSON snapshots with sparkline trends over time

Smart enough to skip `node_modules`, `vendor`, `build`, generated files, and minified code.

## Install

> Coming soon. Currently in development (Phase 1).

```bash
# From source
git clone https://github.com/wiseyoda/repo-analysis.git
cd repo-analysis
cargo build --release
```

## Usage

```bash
# Analyze a repo
repostat ./path/to/repo

# JSON output for scripts
repostat ./path/to/repo --json

# Markdown report
repostat ./path/to/repo --markdown

# View trends over time
repostat trend ./path/to/repo
```

## How It Works

1. **Scan** — Walk the file tree, respect `.gitignore`, detect languages, exclude generated code
2. **Measure** — Count lines, calculate complexity, parse dependency manifests
3. **Analyze** — (Optional) Claude CLI generates architecture summary and effort estimates
4. **Store** — Save a JSON snapshot in `.repostat/snapshots/`
5. **Report** — Compact terminal dashboard with delta from last run

## Project Status

See [ROADMAP.md](ROADMAP.md) for the full plan. Currently in **Phase 1: Foundation & Core Metrics**.

| Phase | Status |
|-------|--------|
| 1. Foundation & Core Metrics | In progress |
| 2. Complexity Analysis | Planned |
| 3. Dependency & Coupling | Planned |
| 4. Documentation Analysis | Planned |
| 5. AI-Augmented Analysis | Planned |
| 6. History & Trends | Planned |
| 7. Polish & Distribution | Planned |

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

[MIT](LICENSE)
