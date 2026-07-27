# repostat

Fast, deterministic repository complexity and health analysis. Repostat is
token-free and read-only by default.

## What It Does

Point it at any repo. Get back:

- **Line counts** -- code, comments, blanks, per language, per file
- **Complexity** -- cyclomatic + cognitive complexity via tree-sitter (10 languages)
- **Dependencies** -- direct + transitive counts from manifests and lockfiles (8 ecosystems)
- **Documentation** -- coverage ratio, README completeness, per-directory coverage
- **Stable JSON** -- versioned `repostat.metrics.v1` output for tools and CI
- **Progress tracking** -- opt-in JSON snapshots with sparkline trends over time
- **Cross-repo index** -- track all analyzed repos from one place

Smart enough to skip `node_modules`, `vendor`, `build`, generated files, and minified code.

## Install

### From source

```bash
git clone https://github.com/wiseyoda/ai-mux-repostat.git
cd ai-mux-repostat
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
# Read-only analysis (default: current directory)
repostat ./path/to/repo

# Stable, byte-reproducible JSON for scripts
repostat --json --no-write ./path/to/repo

# Markdown to stdout
repostat -m ./path/to/repo

# Explicitly save history and update the cross-repo index
repostat --save ./path/to/repo

# View trends over time (sparkline charts)
repostat trend ./path/to/repo

# List all tracked repositories
repostat list

# Generate man page
repostat manpage > repostat.1

# Suite tool adapter; JSON-only stdout, no writes
repostat extension ./path/to/repo
```

## How It Works

1. **Scan** -- Walk the file tree, respect `.gitignore`, detect languages, exclude generated code
2. **Measure** -- Count lines, calculate complexity, parse dependency manifests
3. **Document** -- Score README completeness, measure doc-to-code ratio
4. **Report** -- Emit a compact dashboard, Markdown, HTML, or stable JSON
5. **Store when asked** -- `--save` writes `.repostat/snapshots/` and updates the index

Repostat never starts a model. Optional AI enrichment belongs to an explicit
ai-mux Engine workflow so token use, accounts, validation, and artifacts remain
visible to the control plane.

## Write Behavior

Ordinary analysis, `--json`, `--markdown`, and `extension` do not write to the
target repository or Repostat's global data directory.

- `--save` explicitly writes snapshot history and the cross-repo index.
- `--html` explicitly exports `repostat-report.html` to the target.
- `--no-write` enforces the read-only contract and conflicts with both write
  flags.

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
[exclude]
patterns = ["generated/**", "vendor/**"]

[include]
patterns = ["vendor/important.rs"]
```

## Project Status

| Phase | Status |
|-------|--------|
| 1. Foundation & Core Metrics | Shipped (v0.2.0) |
| 2. Complexity Analysis | Shipped (v0.3.0) |
| 3. Dependency & Coupling | Shipped (v0.4.0) |
| 4. Documentation Analysis | Shipped (v0.5.0) |
| 5. Direct AI analysis | Historical v0.6.0 behavior; retired from execution |
| 6. History & Trends | Shipped (v0.7.0) |
| 7. Polish & Distribution | Shipped (v0.8.0) |
| 9. Developer Health Check | Shipped (v0.9.0) |
| Suite deterministic tool boundary | Candidate on `codex/suite-integration` |

## Contributing

Read [docs/constitution.md](docs/constitution.md) first. Then:

1. Fork the repo
2. Create a feature branch
3. Write tests first (TDD)
4. Run `scripts/verify.sh`
5. Open a PR

`scripts/verify.sh` is the repository verification entrypoint. It runs
`cargo fmt --check`, Clippy with warnings denied, and the complete test suite.

## License

[MIT](LICENSE)
