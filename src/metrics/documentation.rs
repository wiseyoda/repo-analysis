//! Documentation analysis: markdown inventory, doc-to-code ratio,
//! README completeness, and per-directory coverage.

use std::path::{Path, PathBuf};

use ignore::WalkBuilder;

/// Directories to skip when scanning for documentation files.
const EXCLUDE_DIRS: &[&str] = &[
    "node_modules",
    "vendor",
    "build",
    "dist",
    ".next",
    "Pods",
    "target",
    ".git",
    "__pycache__",
    ".venv",
    "venv",
];

/// Markdown file inventory: count, lines, and characters.
#[derive(Debug, Clone, Default)]
pub(crate) struct DocInventory {
    /// Total markdown files found.
    pub(crate) file_count: usize,
    /// Total lines across all markdown files.
    pub(crate) total_lines: usize,
    /// Total characters (bytes) across all markdown files.
    pub(crate) total_chars: usize,
}

/// Doc-to-code ratio metrics.
#[derive(Debug, Clone)]
pub(crate) struct DocToCodeRatio {
    /// Lines of documentation (markdown).
    #[allow(dead_code)] // kept for JSON/markdown report consumers
    pub(crate) doc_lines: usize,
    /// Lines of code.
    #[allow(dead_code)] // kept for JSON/markdown report consumers
    pub(crate) code_lines: usize,
    /// Ratio: doc_lines / code_lines (0.0 if no code).
    pub(crate) ratio: f64,
}

/// A README section check result.
#[derive(Debug, Clone)]
pub(crate) struct ReadmeSection {
    /// Section name.
    pub(crate) name: &'static str,
    /// Whether the section was found.
    pub(crate) present: bool,
}

/// README completeness score.
#[derive(Debug, Clone)]
pub(crate) struct ReadmeScore {
    /// Path to the README file (None if not found).
    #[allow(dead_code)] // kept for JSON/markdown report consumers
    pub(crate) readme_path: Option<PathBuf>,
    /// Sections checked and whether they were found.
    pub(crate) sections: Vec<ReadmeSection>,
    /// Overall score: present_count / total_sections (0.0 to 1.0).
    pub(crate) score: f64,
}

/// Per-directory documentation coverage entry.
#[derive(Debug, Clone)]
pub(crate) struct DirCoverageEntry {
    /// Directory path (relative to repo root).
    #[allow(dead_code)] // kept for JSON/markdown report consumers
    pub(crate) dir: PathBuf,
    /// Whether documentation exists for this directory.
    pub(crate) has_docs: bool,
}

/// Per-directory documentation coverage.
#[derive(Debug, Clone)]
pub(crate) struct DirCoverage {
    /// Per-directory entries.
    pub(crate) entries: Vec<DirCoverageEntry>,
    /// Overall coverage: dirs_with_docs / total_dirs (0.0 to 1.0).
    pub(crate) coverage: f64,
}

/// Aggregate documentation metrics for a repository.
#[derive(Debug, Clone)]
pub(crate) struct DocumentationMetrics {
    /// Markdown file inventory.
    pub(crate) inventory: DocInventory,
    /// Doc-to-code ratio.
    pub(crate) doc_to_code: DocToCodeRatio,
    /// README completeness score.
    pub(crate) readme_score: ReadmeScore,
    /// Per-directory documentation coverage.
    pub(crate) dir_coverage: DirCoverage,
}

/// Analyze documentation in a directory.
///
/// `code_lines` is the total code line count from `AggregateMetrics`,
/// used to compute the doc-to-code ratio.
pub(crate) fn analyze_documentation(dir: &Path, code_lines: usize) -> DocumentationMetrics {
    let inventory = scan_markdown_inventory(dir);
    let doc_to_code = compute_doc_to_code_ratio(inventory.total_lines, code_lines);
    let readme_score = score_readme(dir);
    let dir_coverage = compute_dir_coverage(dir);

    DocumentationMetrics {
        inventory,
        doc_to_code,
        readme_score,
        dir_coverage,
    }
}

/// Scan a directory for markdown files and compute inventory stats.
pub(crate) fn scan_markdown_inventory(dir: &Path) -> DocInventory {
    let mut file_count = 0usize;
    let mut total_lines = 0usize;
    let mut total_chars = 0usize;

    for entry in WalkBuilder::new(dir)
        .hidden(false)
        .filter_entry(|e| {
            let name = e.file_name().to_string_lossy();
            !EXCLUDE_DIRS.iter().any(|&d| name == d)
        })
        .build()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if !entry.file_type().is_some_and(|ft| ft.is_file()) {
            continue;
        }
        let ext = path.extension().and_then(|e| e.to_str());
        if ext != Some("md") {
            continue;
        }
        if let Ok(content) = std::fs::read_to_string(path) {
            file_count += 1;
            total_lines += content.lines().count();
            total_chars += content.len();
        }
    }

    DocInventory {
        file_count,
        total_lines,
        total_chars,
    }
}

/// Compute doc-to-code ratio from doc lines and code lines.
pub(crate) fn compute_doc_to_code_ratio(doc_lines: usize, code_lines: usize) -> DocToCodeRatio {
    let ratio = if code_lines == 0 {
        0.0
    } else {
        doc_lines as f64 / code_lines as f64
    };

    DocToCodeRatio {
        doc_lines,
        code_lines,
        ratio,
    }
}

/// Known README filenames in priority order.
const README_NAMES: &[&str] = &["README.md", "readme.md", "Readme.md", "README"];

/// Score a README file for completeness.
///
/// Checks for five key sections: installation, usage, API, contributing, and license.
pub(crate) fn score_readme(dir: &Path) -> ReadmeScore {
    let readme_path = README_NAMES
        .iter()
        .map(|name| dir.join(name))
        .find(|path| path.is_file());

    let content = readme_path
        .as_ref()
        .and_then(|p| std::fs::read_to_string(p).ok())
        .unwrap_or_default();

    let lower = content.to_lowercase();

    let sections = vec![
        ReadmeSection {
            name: "install",
            present: has_section(
                &lower,
                &["install", "setup", "getting started", "quick start"],
            ),
        },
        ReadmeSection {
            name: "usage",
            present: has_section(&lower, &["usage", "how to use", "examples", "quick start"]),
        },
        ReadmeSection {
            name: "api",
            present: has_section(&lower, &["api", "reference", "interface", "endpoints"]),
        },
        ReadmeSection {
            name: "contributing",
            present: has_section(&lower, &["contributing", "contribute", "development"]),
        },
        ReadmeSection {
            name: "license",
            present: has_license_section(&lower),
        },
    ];

    let present_count = sections.iter().filter(|s| s.present).count();
    let score = if sections.is_empty() {
        0.0
    } else {
        present_count as f64 / sections.len() as f64
    };

    ReadmeScore {
        readme_path,
        sections,
        score,
    }
}

/// Check if a README contains a section matching any of the given keywords.
///
/// Looks for keywords in heading lines (starting with `#`).
fn has_section(lower_content: &str, keywords: &[&str]) -> bool {
    for line in lower_content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('#') {
            let heading = trimmed.trim_start_matches('#').trim();
            if keywords.iter().any(|kw| heading.contains(kw)) {
                return true;
            }
        }
    }
    false
}

/// Check if a README contains a license section.
///
/// Looks for a "license" heading or common license identifiers in the body.
fn has_license_section(lower_content: &str) -> bool {
    let license_keywords = &[
        "mit license",
        "apache license",
        "gpl",
        "bsd license",
        "isc license",
    ];

    for line in lower_content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('#') {
            let heading = trimmed.trim_start_matches('#').trim();
            if heading.contains("license") {
                return true;
            }
        }
    }

    // Fallback: check for license identifiers anywhere in text
    license_keywords.iter().any(|kw| lower_content.contains(kw))
}

/// Compute per-directory documentation coverage.
///
/// A source directory is considered "covered" if it contains a `.md` file,
/// or if a `.md` file exists in its parent directory.
pub(crate) fn compute_dir_coverage(dir: &Path) -> DirCoverage {
    let mut source_dirs = std::collections::BTreeSet::new();
    let mut doc_dirs = std::collections::BTreeSet::new();

    for entry in WalkBuilder::new(dir)
        .hidden(false)
        .filter_entry(|e| {
            let name = e.file_name().to_string_lossy();
            !EXCLUDE_DIRS.iter().any(|&d| name == d)
        })
        .build()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if !entry.file_type().is_some_and(|ft| ft.is_file()) {
            continue;
        }

        let relative = path.strip_prefix(dir).unwrap_or(path);
        let parent = relative.parent().unwrap_or(Path::new(""));

        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        if ext == "md" {
            doc_dirs.insert(parent.to_path_buf());
        } else if is_source_extension(ext) {
            source_dirs.insert(parent.to_path_buf());
        }
    }

    let entries: Vec<DirCoverageEntry> = source_dirs
        .iter()
        .map(|src_dir| {
            let has_docs = doc_dirs.contains(src_dir)
                || src_dir
                    .parent()
                    .is_some_and(|parent| doc_dirs.contains(parent));
            DirCoverageEntry {
                dir: src_dir.clone(),
                has_docs,
            }
        })
        .collect();

    let total = entries.len();
    let covered = entries.iter().filter(|e| e.has_docs).count();
    let coverage = if total == 0 {
        0.0
    } else {
        covered as f64 / total as f64
    };

    DirCoverage { entries, coverage }
}

/// Check if a file extension indicates a source code file.
fn is_source_extension(ext: &str) -> bool {
    matches!(
        ext,
        "rs" | "py"
            | "js"
            | "ts"
            | "tsx"
            | "jsx"
            | "go"
            | "java"
            | "c"
            | "cpp"
            | "h"
            | "hpp"
            | "swift"
            | "rb"
            | "php"
            | "cs"
            | "kt"
            | "scala"
            | "ex"
            | "exs"
            | "zig"
            | "lua"
            | "sh"
            | "bash"
    )
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::TempDir;

    use super::*;

    fn create_file(dir: &Path, relative: &str, content: &str) {
        let path = dir.join(relative);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(&path, content).unwrap();
    }

    // === Markdown Inventory Tests (R-300) ===

    #[test]
    fn inventory_counts_markdown_files() {
        let dir = TempDir::new().unwrap();
        create_file(dir.path(), "README.md", "# Hello\nWorld\n");
        create_file(dir.path(), "docs/guide.md", "# Guide\nStep 1\nStep 2\n");
        create_file(dir.path(), "src/main.rs", "fn main() {}");

        let inv = scan_markdown_inventory(dir.path());
        assert_eq!(inv.file_count, 2);
    }

    #[test]
    fn inventory_counts_lines_and_chars() {
        let dir = TempDir::new().unwrap();
        let content = "# Title\n\nParagraph one.\nParagraph two.\n";
        create_file(dir.path(), "doc.md", content);

        let inv = scan_markdown_inventory(dir.path());
        assert_eq!(inv.file_count, 1);
        assert_eq!(inv.total_lines, 4);
        assert_eq!(inv.total_chars, content.len());
    }

    #[test]
    fn inventory_returns_zeros_for_no_markdown() {
        let dir = TempDir::new().unwrap();
        create_file(dir.path(), "main.rs", "fn main() {}");

        let inv = scan_markdown_inventory(dir.path());
        assert_eq!(inv.file_count, 0);
        assert_eq!(inv.total_lines, 0);
        assert_eq!(inv.total_chars, 0);
    }

    #[test]
    fn inventory_skips_excluded_dirs() {
        let dir = TempDir::new().unwrap();
        create_file(dir.path(), "README.md", "# Hi\n");
        create_file(dir.path(), "node_modules/pkg/README.md", "# Pkg\n");
        create_file(dir.path(), "target/doc/index.md", "# Index\n");

        let inv = scan_markdown_inventory(dir.path());
        assert_eq!(inv.file_count, 1);
    }

    #[test]
    fn inventory_handles_empty_directory() {
        let dir = TempDir::new().unwrap();
        let inv = scan_markdown_inventory(dir.path());
        assert_eq!(inv.file_count, 0);
    }

    #[test]
    fn inventory_handles_nested_markdown() {
        let dir = TempDir::new().unwrap();
        create_file(dir.path(), "README.md", "# Root\n");
        create_file(dir.path(), "docs/guide.md", "# Guide\n");
        create_file(dir.path(), "docs/api/ref.md", "# API Ref\n");

        let inv = scan_markdown_inventory(dir.path());
        assert_eq!(inv.file_count, 3);
    }

    // === Doc-to-Code Ratio Tests (R-301) ===

    #[test]
    fn ratio_with_both_doc_and_code() {
        let ratio = compute_doc_to_code_ratio(100, 1000);
        assert_eq!(ratio.doc_lines, 100);
        assert_eq!(ratio.code_lines, 1000);
        assert!((ratio.ratio - 0.1).abs() < f64::EPSILON);
    }

    #[test]
    fn ratio_with_zero_code_lines() {
        let ratio = compute_doc_to_code_ratio(50, 0);
        assert_eq!(ratio.ratio, 0.0);
    }

    #[test]
    fn ratio_with_zero_doc_lines() {
        let ratio = compute_doc_to_code_ratio(0, 500);
        assert_eq!(ratio.ratio, 0.0);
    }

    #[test]
    fn ratio_with_both_zero() {
        let ratio = compute_doc_to_code_ratio(0, 0);
        assert_eq!(ratio.ratio, 0.0);
    }

    // === README Completeness Tests (R-302) ===

    #[test]
    fn readme_score_all_sections_present() {
        let dir = TempDir::new().unwrap();
        create_file(
            dir.path(),
            "README.md",
            "# My Project\n\n\
             ## Installation\n\nRun `cargo install`.\n\n\
             ## Usage\n\n`repostat ./path`\n\n\
             ## API\n\nSee reference docs.\n\n\
             ## Contributing\n\nPRs welcome.\n\n\
             ## License\n\nMIT License\n",
        );

        let score = score_readme(dir.path());
        assert!(score.readme_path.is_some());
        assert_eq!(score.sections.len(), 5);
        assert!(
            score.sections.iter().all(|s| s.present),
            "all sections should be present"
        );
        assert!((score.score - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn readme_score_no_readme() {
        let dir = TempDir::new().unwrap();
        create_file(dir.path(), "main.rs", "fn main() {}");

        let score = score_readme(dir.path());
        assert!(score.readme_path.is_none());
        assert_eq!(score.score, 0.0);
    }

    #[test]
    fn readme_score_empty_readme() {
        let dir = TempDir::new().unwrap();
        create_file(dir.path(), "README.md", "");

        let score = score_readme(dir.path());
        assert!(score.readme_path.is_some());
        assert_eq!(score.score, 0.0);
    }

    #[test]
    fn readme_score_partial_sections() {
        let dir = TempDir::new().unwrap();
        create_file(
            dir.path(),
            "README.md",
            "# Project\n\n## Installation\n\nDo stuff.\n\n## License\n\nMIT License\n",
        );

        let score = score_readme(dir.path());
        let present: Vec<&str> = score
            .sections
            .iter()
            .filter(|s| s.present)
            .map(|s| s.name)
            .collect();
        assert!(present.contains(&"install"));
        assert!(present.contains(&"license"));
        assert!(!present.contains(&"api"));
        assert!((score.score - 2.0 / 5.0).abs() < f64::EPSILON);
    }

    #[test]
    fn readme_score_detects_setup_as_install() {
        let dir = TempDir::new().unwrap();
        create_file(
            dir.path(),
            "README.md",
            "# App\n\n## Setup\n\nInstall deps.\n",
        );

        let score = score_readme(dir.path());
        let install = score.sections.iter().find(|s| s.name == "install").unwrap();
        assert!(install.present, "setup heading should count as install");
    }

    #[test]
    fn readme_score_detects_license_in_body() {
        let dir = TempDir::new().unwrap();
        create_file(
            dir.path(),
            "README.md",
            "# App\n\nThis project is under the MIT License.\n",
        );

        let score = score_readme(dir.path());
        let license = score.sections.iter().find(|s| s.name == "license").unwrap();
        assert!(
            license.present,
            "MIT License in body should count as license"
        );
    }

    #[test]
    fn readme_score_case_insensitive() {
        let dir = TempDir::new().unwrap();
        create_file(
            dir.path(),
            "README.md",
            "# app\n\n## INSTALLATION\n\nStuff.\n\n## USAGE\n\nThings.\n",
        );

        let score = score_readme(dir.path());
        let install = score.sections.iter().find(|s| s.name == "install").unwrap();
        let usage = score.sections.iter().find(|s| s.name == "usage").unwrap();
        assert!(install.present);
        assert!(usage.present);
    }

    // === Per-Directory Coverage Tests (R-303) ===

    #[test]
    fn dir_coverage_all_covered() {
        let dir = TempDir::new().unwrap();
        create_file(dir.path(), "src/main.rs", "fn main() {}");
        create_file(dir.path(), "src/README.md", "# Src docs\n");

        let cov = compute_dir_coverage(dir.path());
        assert_eq!(cov.entries.len(), 1);
        assert!(cov.entries[0].has_docs);
        assert!((cov.coverage - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn dir_coverage_none_covered() {
        let dir = TempDir::new().unwrap();
        create_file(dir.path(), "src/main.rs", "fn main() {}");
        create_file(dir.path(), "lib/util.rs", "pub fn help() {}");

        let cov = compute_dir_coverage(dir.path());
        assert_eq!(cov.entries.len(), 2);
        assert!(cov.entries.iter().all(|e| !e.has_docs));
        assert_eq!(cov.coverage, 0.0);
    }

    #[test]
    fn dir_coverage_parent_doc_counts() {
        let dir = TempDir::new().unwrap();
        create_file(dir.path(), "docs/guide.md", "# Guide\n");
        create_file(dir.path(), "docs/src/code.rs", "fn x() {}");

        let cov = compute_dir_coverage(dir.path());
        let src_entry = cov
            .entries
            .iter()
            .find(|e| e.dir.to_string_lossy().contains("src"))
            .unwrap();
        assert!(
            src_entry.has_docs,
            "parent dir docs should cover child source dir"
        );
    }

    #[test]
    fn dir_coverage_no_source_files() {
        let dir = TempDir::new().unwrap();
        create_file(dir.path(), "docs/readme.md", "# Hello\n");

        let cov = compute_dir_coverage(dir.path());
        assert!(cov.entries.is_empty());
        assert_eq!(cov.coverage, 0.0);
    }

    #[test]
    fn dir_coverage_skips_excluded_dirs() {
        let dir = TempDir::new().unwrap();
        create_file(dir.path(), "src/main.rs", "fn main() {}");
        create_file(dir.path(), "src/docs.md", "# Docs\n");
        create_file(dir.path(), "node_modules/pkg/index.js", "export {}");

        let cov = compute_dir_coverage(dir.path());
        assert!(
            !cov.entries
                .iter()
                .any(|e| e.dir.to_string_lossy().contains("node_modules")),
            "should not include node_modules"
        );
    }

    // === Aggregate analyze_documentation Tests ===

    #[test]
    fn analyze_documentation_integrates_all_metrics() {
        let dir = TempDir::new().unwrap();
        create_file(
            dir.path(),
            "README.md",
            "# Project\n\n## Installation\n\nDo it.\n\n## Usage\n\nUse it.\n",
        );
        create_file(dir.path(), "docs/guide.md", "# Guide\nStep 1\nStep 2\n");
        create_file(dir.path(), "src/main.rs", "fn main() {}");

        let metrics = analyze_documentation(dir.path(), 500);

        assert_eq!(metrics.inventory.file_count, 2);
        assert!(metrics.inventory.total_lines > 0);
        assert!(metrics.inventory.total_chars > 0);
        assert!(metrics.doc_to_code.ratio > 0.0);
        assert!(metrics.readme_score.readme_path.is_some());
        assert!(metrics.readme_score.score > 0.0);
    }
}
