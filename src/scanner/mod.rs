//! File system scanner with gitignore-aware walking and exclusion logic.

use std::path::{Path, PathBuf};

use globset::{Glob, GlobSet, GlobSetBuilder};
use ignore::WalkBuilder;

use crate::config::Config;

/// Directories excluded by built-in heuristics (Layer 2 of ADR-005).
const HEURISTIC_EXCLUDES: &[&str] = &[
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

/// Errors that can occur during file scanning.
#[derive(Debug, thiserror::Error)]
pub(crate) enum ScanError {
    /// Failed to build a glob pattern from config.
    #[error("invalid glob pattern '{pattern}': {source}")]
    InvalidPattern {
        /// The pattern that failed.
        pattern: String,
        /// Underlying glob error.
        source: globset::Error,
    },
}

/// Scan a directory for files to analyze, applying three-layer exclusion.
///
/// Layer 1: `.gitignore` rules (via `ignore` crate).
/// Layer 2: Built-in heuristic directory exclusions.
/// Layer 3: Config-based `exclude_patterns` / `include_patterns`.
///
/// Returns a sorted list of regular file paths.
pub(crate) fn scan(dir: &Path, config: &Config) -> Result<Vec<PathBuf>, ScanError> {
    let exclude_set = build_glob_set(&config.exclude_patterns)?;
    let include_set = build_glob_set(&config.include_patterns)?;

    let mut files: Vec<PathBuf> = WalkBuilder::new(dir)
        .hidden(false)
        .filter_entry(|entry| !is_heuristic_excluded(entry.file_name()))
        .build()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_some_and(|ft| ft.is_file()))
        .map(|entry| entry.into_path())
        .filter(|path| passes_config_filter(path, dir, &exclude_set, &include_set))
        .collect();

    files.sort();
    Ok(files)
}

/// Check if a directory name matches a heuristic exclusion.
fn is_heuristic_excluded(name: &std::ffi::OsStr) -> bool {
    let name = name.to_string_lossy();
    HEURISTIC_EXCLUDES.iter().any(|&excluded| name == excluded)
}

/// Check if a file passes the config exclude/include filter (Layer 3).
fn passes_config_filter(
    path: &Path,
    root: &Path,
    exclude_set: &GlobSet,
    include_set: &GlobSet,
) -> bool {
    let relative = path.strip_prefix(root).unwrap_or(path);

    if exclude_set.is_match(relative) {
        // Excluded — but check if include overrides
        return include_set.is_match(relative);
    }

    true
}

/// Build a `GlobSet` from a list of pattern strings.
fn build_glob_set(patterns: &[String]) -> Result<GlobSet, ScanError> {
    let mut builder = GlobSetBuilder::new();
    for pattern in patterns {
        let glob = Glob::new(pattern).map_err(|e| ScanError::InvalidPattern {
            pattern: pattern.clone(),
            source: e,
        })?;
        builder.add(glob);
    }
    builder.build().map_err(|e| ScanError::InvalidPattern {
        pattern: "<combined>".to_string(),
        source: e,
    })
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::TempDir;

    use crate::config::Config;

    use super::*;

    /// Helper to create a file inside a temp dir at a relative path.
    fn create_file(dir: &std::path::Path, relative: &str) {
        let path = dir.join(relative);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(&path, "content").unwrap();
    }

    #[test]
    fn walks_directory_and_returns_files() {
        let dir = TempDir::new().unwrap();
        create_file(dir.path(), "a.rs");
        create_file(dir.path(), "b.rs");
        create_file(dir.path(), "sub/c.rs");

        let config = Config::default();
        let files = scan(dir.path(), &config).unwrap();

        assert_eq!(files.len(), 3);
        assert!(files.iter().any(|p| p.ends_with("a.rs")));
        assert!(files.iter().any(|p| p.ends_with("b.rs")));
        assert!(files.iter().any(|p| p.ends_with("sub/c.rs")));
    }

    #[test]
    fn respects_gitignore() {
        let dir = TempDir::new().unwrap();
        create_file(dir.path(), "keep.rs");
        create_file(dir.path(), "ignored.log");
        fs::write(dir.path().join(".gitignore"), "*.log\n").unwrap();

        // ignore crate needs a git repo to respect .gitignore
        std::process::Command::new("git")
            .args(["init"])
            .current_dir(dir.path())
            .output()
            .unwrap();

        let config = Config::default();
        let files = scan(dir.path(), &config).unwrap();

        let names: Vec<_> = files
            .iter()
            .filter_map(|p| p.file_name())
            .map(|n| n.to_string_lossy().to_string())
            .collect();
        assert!(names.contains(&"keep.rs".to_string()));
        assert!(!names.contains(&"ignored.log".to_string()));
    }

    #[test]
    fn excludes_heuristic_directories() {
        let dir = TempDir::new().unwrap();
        create_file(dir.path(), "src/main.rs");
        create_file(dir.path(), "node_modules/pkg/index.js");
        create_file(dir.path(), "vendor/lib.go");
        create_file(dir.path(), "__pycache__/mod.pyc");
        create_file(dir.path(), ".venv/bin/python");

        let config = Config::default();
        let files = scan(dir.path(), &config).unwrap();

        let paths: Vec<String> = files.iter().map(|p| p.display().to_string()).collect();
        assert!(
            paths.iter().any(|p| p.contains("main.rs")),
            "should include src/main.rs"
        );
        assert!(
            !paths.iter().any(|p| p.contains("node_modules")),
            "should exclude node_modules"
        );
        assert!(
            !paths.iter().any(|p| p.contains("vendor")),
            "should exclude vendor"
        );
        assert!(
            !paths.iter().any(|p| p.contains("__pycache__")),
            "should exclude __pycache__"
        );
        assert!(
            !paths.iter().any(|p| p.contains(".venv")),
            "should exclude .venv"
        );
    }

    #[test]
    fn applies_config_exclude_patterns() {
        let dir = TempDir::new().unwrap();
        create_file(dir.path(), "src/main.rs");
        create_file(dir.path(), "generated/output.rs");

        let config = Config {
            exclude_patterns: vec!["generated/**".to_string()],
            include_patterns: vec![],
        };
        let files = scan(dir.path(), &config).unwrap();

        let paths: Vec<String> = files.iter().map(|p| p.display().to_string()).collect();
        assert!(paths.iter().any(|p| p.contains("main.rs")));
        assert!(!paths.iter().any(|p| p.contains("generated")));
    }

    #[test]
    fn include_overrides_exclude() {
        let dir = TempDir::new().unwrap();
        create_file(dir.path(), "generated/output.rs");
        create_file(dir.path(), "generated/keep.rs");

        let config = Config {
            exclude_patterns: vec!["generated/**".to_string()],
            include_patterns: vec!["generated/keep.rs".to_string()],
        };
        let files = scan(dir.path(), &config).unwrap();

        let paths: Vec<String> = files.iter().map(|p| p.display().to_string()).collect();
        assert!(
            paths.iter().any(|p| p.contains("keep.rs")),
            "include should override exclude"
        );
        assert!(
            !paths.iter().any(|p| p.contains("output.rs")),
            "non-included file should stay excluded"
        );
    }

    #[test]
    fn returns_only_regular_files() {
        let dir = TempDir::new().unwrap();
        create_file(dir.path(), "file.rs");
        fs::create_dir_all(dir.path().join("empty_dir")).unwrap();

        let config = Config::default();
        let files = scan(dir.path(), &config).unwrap();

        assert!(files.iter().all(|p| p.is_file()));
    }

    #[test]
    fn returns_sorted_paths() {
        let dir = TempDir::new().unwrap();
        create_file(dir.path(), "z.rs");
        create_file(dir.path(), "a.rs");
        create_file(dir.path(), "m/b.rs");

        let config = Config::default();
        let files = scan(dir.path(), &config).unwrap();

        let sorted: Vec<_> = {
            let mut v = files.clone();
            v.sort();
            v
        };
        assert_eq!(files, sorted, "results should be sorted");
    }

    #[test]
    fn handles_empty_directory() {
        let dir = TempDir::new().unwrap();
        let config = Config::default();
        let files = scan(dir.path(), &config).unwrap();
        assert!(files.is_empty());
    }
}
