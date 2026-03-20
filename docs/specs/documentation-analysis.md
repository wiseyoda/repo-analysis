# Documentation Analysis — Feature Spec

> Phase 4 of the repostat roadmap. Measures documentation quality, coverage, and freshness.

## Purpose

Add documentation analysis to repostat so users understand their project's documentation health:
how much docs exist, the doc-to-code ratio, whether the README covers essentials, and whether
source directories have accompanying documentation.

## Requirements

| ID | Description | Priority |
|----|-------------|----------|
| R-300 | Markdown file inventory: count, total lines, total characters | P0 |
| R-301 | Doc-to-code ratio: lines of documentation vs lines of code | P0 |
| R-302 | README completeness scoring (install, usage, API, contributing, license) | P0 |
| R-303 | Per-module doc coverage: does each source directory have documentation? | P1 |

## Data Model

### DocInventory

Collected from walking the repository for `.md` files.

```rust
struct DocInventory {
    /// Total markdown files found.
    file_count: usize,
    /// Total lines across all markdown files.
    total_lines: usize,
    /// Total characters (bytes) across all markdown files.
    total_chars: usize,
}
```

### DocToCodeRatio

```rust
struct DocToCodeRatio {
    /// Lines of documentation (markdown).
    doc_lines: usize,
    /// Lines of code (from AggregateMetrics).
    code_lines: usize,
    /// Ratio: doc_lines / code_lines (0.0 if no code).
    ratio: f64,
}
```

### ReadmeScore

Checks a README file for the presence of key sections by scanning headings and content
for keywords.

```rust
struct ReadmeSection {
    name: &'static str,
    present: bool,
}

struct ReadmeScore {
    /// Path to the README file (None if not found).
    readme_path: Option<PathBuf>,
    /// Sections checked and whether they were found.
    sections: Vec<ReadmeSection>,
    /// Overall score: present_count / total_sections (0.0 to 1.0).
    score: f64,
}
```

Sections to check:
- **Installation**: headings or content matching "install", "setup", "getting started"
- **Usage**: headings matching "usage", "how to use", "quick start", "examples"
- **API**: headings matching "api", "reference", "interface", "endpoints"
- **Contributing**: headings matching "contributing", "development", "contribute"
- **License**: headings matching "license", content matching "MIT", "Apache", "GPL", etc.

### DirCoverage

For each source directory, check if any `.md` file exists in it or its parent.

```rust
struct DirCoverageEntry {
    /// Directory path (relative to repo root).
    dir: PathBuf,
    /// Whether documentation exists for this directory.
    has_docs: bool,
}

struct DirCoverage {
    /// Per-directory entries.
    entries: Vec<DirCoverageEntry>,
    /// Overall coverage: dirs_with_docs / total_dirs (0.0 to 1.0).
    coverage: f64,
}
```

### DocumentationMetrics (aggregate)

```rust
struct DocumentationMetrics {
    inventory: DocInventory,
    doc_to_code: DocToCodeRatio,
    readme_score: ReadmeScore,
    dir_coverage: DirCoverage,
}
```

## Behavior

### Markdown Inventory (R-300)

1. Walk the repository using the same exclusion rules as the scanner.
2. Find all files with `.md` extension.
3. For each file, count lines and characters (UTF-8 byte length).
4. Return totals.

### Doc-to-Code Ratio (R-301)

1. `doc_lines` = total lines from markdown inventory.
2. `code_lines` = total code lines from `AggregateMetrics`.
3. `ratio` = `doc_lines as f64 / code_lines as f64` (0.0 if code_lines is 0).

### README Completeness (R-302)

1. Look for a README file: `README.md`, `readme.md`, `README`, `Readme.md`.
2. If not found, score is 0.0 with all sections missing.
3. Parse the README content, looking at headings (`#`, `##`, `###`) and body text.
4. For each section, check if matching keywords appear in headings or nearby content.
5. Score = sections_present / total_sections.

### Per-Directory Coverage (R-303)

1. Identify source directories: directories containing source code files.
2. For each source directory, check if a `.md` file exists in:
   - The directory itself
   - The parent directory (e.g., `src/` has a `docs/` sibling)
3. Report which directories have docs and overall coverage percentage.

## Dashboard Section

```
├────────────────────────────────────────┤
│ Documentation                          │
│ ─────────────────────────────────────  │
│  Markdown files: 12                    │
│  Doc lines:      1,234                 │
│  Doc-to-code:    0.15 (15%)            │
│  README score:   4/5 (80%)             │
│  Dir coverage:   6/8 (75%)             │
└────────────────────────────────────────┘
```

## Snapshot Schema

```json
{
  "documentation": {
    "file_count": 12,
    "total_lines": 1234,
    "total_chars": 45678,
    "doc_to_code_ratio": 0.15,
    "readme_score": 0.8,
    "readme_sections": ["install", "usage", "api", "contributing"],
    "dir_coverage": 0.75
  }
}
```

## Edge Cases

- Repository with no markdown files: inventory returns zeros, ratio is 0.0.
- Repository with no code files: ratio is 0.0 (not infinity).
- README exists but is empty: all sections missing, score 0.0.
- Multiple READMEs: use the one at root level.
- Binary files with .md extension: read_to_string will fail; skip gracefully.

## Acceptance Criteria

- [ ] `analyze_documentation(dir, code_lines)` returns `DocumentationMetrics`.
- [ ] Markdown inventory matches `find . -name "*.md" | wc -l` on test fixtures.
- [ ] README scoring matches manual evaluation on 3+ test cases.
- [ ] Dir coverage correctly identifies undocumented source directories.
- [ ] Dashboard renders documentation section when metrics are available.
- [ ] Snapshot includes documentation data and survives round-trip serialization.
