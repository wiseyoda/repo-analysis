//! Cross-repo index: track all analyzed repositories.

use std::io::{self, Write};
use std::path::Path;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::report::color::ColorWriter;

/// Path to the global repos index file.
fn index_path() -> Option<std::path::PathBuf> {
    dirs::home_dir().map(|h| h.join(".repostat").join("repos.json"))
}

/// A tracked repository entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct RepoEntry {
    /// Absolute path to the repository.
    pub(crate) path: String,
    /// Repository name (directory basename).
    pub(crate) name: String,
    /// When it was last analyzed.
    pub(crate) last_analyzed: DateTime<Utc>,
    /// Number of snapshots.
    pub(crate) snapshot_count: usize,
}

/// The global repo index.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct RepoIndex {
    /// Tracked repositories.
    pub(crate) repos: Vec<RepoEntry>,
}

/// Register a repository in the global index.
///
/// Updates the entry if it already exists, or adds a new one.
pub(crate) fn register_repo(target_dir: &Path) {
    let Some(index_file) = index_path() else {
        return;
    };

    let mut index = load_index(&index_file).unwrap_or_default();

    let abs_path = std::fs::canonicalize(target_dir).unwrap_or_else(|_| target_dir.to_path_buf());
    let path_str = abs_path.display().to_string();
    let name = abs_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    let snapshot_count = super::store::load_all(target_dir)
        .map(|s| s.len())
        .unwrap_or(0);

    if let Some(entry) = index.repos.iter_mut().find(|r| r.path == path_str) {
        entry.last_analyzed = Utc::now();
        entry.snapshot_count = snapshot_count;
    } else {
        index.repos.push(RepoEntry {
            path: path_str,
            name,
            last_analyzed: Utc::now(),
            snapshot_count,
        });
    }

    save_index(&index_file, &index);
}

/// Render the list of tracked repositories.
pub(crate) fn render_list(writer: &mut dyn Write, color: bool) -> io::Result<()> {
    let mut cw = ColorWriter::new(writer, color);

    let index = index_path()
        .and_then(|p| load_index(&p))
        .unwrap_or_default();

    if index.repos.is_empty() {
        cw.plain("No tracked repositories. Run `repostat` on a repo first.\n")?;
        return Ok(());
    }

    cw.bold("repostat — tracked repositories\n")?;
    cw.dim("─────────────────────────────────────────────────────────\n")?;

    for entry in &index.repos {
        cw.plain(&format!(
            "  {:<20} {:<35} {}  {} snapshots\n",
            entry.name,
            truncate_path(&entry.path, 35),
            entry.last_analyzed.format("%Y-%m-%d"),
            entry.snapshot_count,
        ))?;
    }

    cw.dim("─────────────────────────────────────────────────────────\n")?;
    Ok(())
}

/// Truncate a path string for display.
fn truncate_path(path: &str, max: usize) -> String {
    if path.len() <= max {
        path.to_string()
    } else {
        format!("...{}", &path[path.len() - max + 3..])
    }
}

/// Load the repo index from disk.
fn load_index(path: &Path) -> Option<RepoIndex> {
    let content = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(&content).ok()
}

/// Save the repo index to disk.
fn save_index(path: &Path, index: &RepoIndex) {
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Ok(json) = serde_json::to_string_pretty(index) {
        let _ = std::fs::write(path, json);
    }
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::*;

    #[test]
    fn truncate_path_short() {
        assert_eq!(truncate_path("/short", 20), "/short");
    }

    #[test]
    fn truncate_path_long() {
        let long = "/very/long/path/that/exceeds/the/limit";
        let result = truncate_path(long, 20);
        assert!(result.len() <= 20);
        assert!(result.starts_with("..."));
    }

    #[test]
    fn repo_index_serializes_roundtrip() {
        let index = RepoIndex {
            repos: vec![RepoEntry {
                path: "/tmp/test".to_string(),
                name: "test".to_string(),
                last_analyzed: Utc::now(),
                snapshot_count: 3,
            }],
        };

        let json = serde_json::to_string(&index).unwrap();
        let parsed: RepoIndex = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.repos.len(), 1);
        assert_eq!(parsed.repos[0].name, "test");
    }

    #[test]
    fn render_list_empty_index() {
        let mut buf = Vec::new();
        render_list(&mut buf, false).unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("No tracked repositories"));
    }

    #[test]
    fn save_and_load_index() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("repos.json");

        let index = RepoIndex {
            repos: vec![RepoEntry {
                path: "/test/repo".to_string(),
                name: "repo".to_string(),
                last_analyzed: Utc::now(),
                snapshot_count: 5,
            }],
        };

        save_index(&path, &index);
        let loaded = load_index(&path).unwrap();
        assert_eq!(loaded.repos.len(), 1);
        assert_eq!(loaded.repos[0].snapshot_count, 5);
    }
}
