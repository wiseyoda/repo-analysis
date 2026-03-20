# repostat

Fast CLI for repository complexity analysis, progress tracking, and AI-augmented insights. Built in Rust.

## What It Does

Point it at any repo. Get back:

- **Line counts** -- code, comments, blanks, per language, per file
- **Complexity** -- cyclomatic + cognitive complexity via tree-sitter (10 languages)
- **Dependencies** -- direct + transitive counts from manifests and lockfiles (8 ecosystems)
- **Documentation** -- coverage ratio, README completeness, per-directory coverage
- **AI insights** -- architecture summary, feature inventory, effort estimation (via Claude CLI)
- **Progress tracking** -- JSON snapshots with sparkline trends over time
- **Cross-repo index** -- track all analyzed repos from one place

Smart enough to skip `node_modules`, `vendor`, `build`, generated files, and minified code.

## Install

### From source

```bash
git clone https://github.com/wiseyoda/repo-analysis.git
cd repo-analysis
cargo build --release
cp target/release/repostat ~/.local/bin/
```

### With cargo

```bash
cargo install repostat
```

### Shell completions

```bash
# Bash
repostat completions bash > ~/.local/share/bash-completion/completions/repostat

# Zsh
repostat completions zsh > ~/.zfunc/_repostat

# Fish
repostat completions fish > ~/.config/fish/completions/repostat.fish
```

## Usage

```bash
# Analyze a repo (default: current directory)
repostat ./path/to/repo

# JSON output for scripts
repostat -j ./path/to/repo

# Markdown report
repostat -m ./path/to/repo

# View trends over time (sparkline charts)
repostat trend ./path/to/repo

# List all tracked repositories
repostat list

# Generate man page
repostat manpage > repostat.1
```

## How It Works

1. **Scan** -- Walk the file tree, respect `.gitignore`, detect languages, exclude generated code
2. **Measure** -- Count lines, calculate complexity, parse dependency manifests
3. **Document** -- Score README completeness, measure doc-to-code ratio
4. **Analyze** -- (Optional) Claude CLI generates architecture summary and effort estimates
5. **Store** -- Save a JSON snapshot in `.repostat/snapshots/`
6. **Report** -- Compact terminal dashboard with delta from last run and sparkline trends

## Dogfooding

repostat analyzing itself:

```
Files: 60          Lines: 10,251
  Code:    7,893     Blank:  1,534     Comment: 824

Language          Files    Code     %
Rust                 29    5794  73.4%
Markdown             23    1897  24.0%
YAML                  2      78   1.0%
TOML                  1      43   0.5%

Documentation: 23 files, 1897 lines, doc-to-code 0.33
README score: 4/5, Dir coverage: 2/6
```

## Configuration

Create `.repostat.toml` in your repo root:

```toml
exclude_patterns = ["generated/**", "vendor/**"]
include_patterns = ["vendor/important.rs"]
```

## Project Status

| Phase | Status |
|-------|--------|
| 1. Foundation & Core Metrics | Shipped (v0.2.0) |
| 2. Complexity Analysis | Shipped (v0.3.0) |
| 3. Dependency & Coupling | Shipped (v0.4.0) |
| 4. Documentation Analysis | Shipped (v0.5.0) |
| 5. AI-Augmented Analysis | Shipped (v0.6.0) |
| 6. History & Trends | Shipped (v0.7.0) |
| 7. Polish & Distribution | Shipped (v0.8.0) |

## Contributing

Read [docs/constitution.md](docs/constitution.md) first. Then:

1. Fork the repo
2. Create a feature branch
3. Write tests first (TDD)
4. Run `cargo fmt && cargo clippy -- -D warnings && cargo test`
5. Open a PR

## License

[MIT](LICENSE)
