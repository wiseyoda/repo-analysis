//! Read and write JSON snapshots to `.repostat/snapshots/`.

use std::path::{Path, PathBuf};

use super::Snapshot;

/// Errors that can occur during snapshot I/O.
#[derive(Debug, thiserror::Error)]
#[allow(dead_code)] // variants used by load_latest (next task wires it)
pub(crate) enum SnapshotError {
    /// Failed to create the snapshots directory.
    #[error("failed to create snapshot directory: {0}")]
    CreateDir(std::io::Error),

    /// Failed to write a snapshot file.
    #[error("failed to write snapshot to {path}: {source}")]
    WriteFailed {
        /// Path we tried to write.
        path: PathBuf,
        /// Underlying I/O error.
        source: std::io::Error,
    },

    /// Failed to serialize snapshot as JSON.
    #[error("failed to serialize snapshot: {0}")]
    SerializeFailed(serde_json::Error),

    /// Failed to read a snapshot file.
    #[error("failed to read snapshot from {path}: {source}")]
    ReadFailed {
        /// Path we tried to read.
        path: PathBuf,
        /// Underlying I/O error.
        source: std::io::Error,
    },

    /// Failed to parse a snapshot file.
    #[error("failed to parse snapshot from {path}: {source}")]
    ParseFailed {
        /// Path we tried to parse.
        path: PathBuf,
        /// Underlying JSON error.
        source: serde_json::Error,
    },
}

/// Directory name for snapshots inside the target repo.
const SNAPSHOT_DIR: &str = ".repostat/snapshots";

/// Write a snapshot to `.repostat/snapshots/<timestamp>.json`.
pub(crate) fn write_snapshot(
    target_dir: &Path,
    snapshot: &Snapshot,
) -> Result<PathBuf, SnapshotError> {
    let dir = target_dir.join(SNAPSHOT_DIR);
    std::fs::create_dir_all(&dir).map_err(SnapshotError::CreateDir)?;

    let filename = snapshot.timestamp.format("%Y%m%d-%H%M%S.json").to_string();
    let path = dir.join(filename);

    let json = serde_json::to_string_pretty(snapshot).map_err(SnapshotError::SerializeFailed)?;

    std::fs::write(&path, json).map_err(|e| SnapshotError::WriteFailed {
        path: path.clone(),
        source: e,
    })?;

    Ok(path)
}

/// Load the most recent snapshot from `.repostat/snapshots/`.
///
/// Returns `None` if no snapshots exist or the directory doesn't exist.
#[allow(dead_code)] // used by snapshot diffing (next task)
pub(crate) fn load_latest(target_dir: &Path) -> Result<Option<Snapshot>, SnapshotError> {
    let dir = target_dir.join(SNAPSHOT_DIR);
    if !dir.exists() {
        return Ok(None);
    }

    let mut entries: Vec<PathBuf> = std::fs::read_dir(&dir)
        .map_err(|e| SnapshotError::ReadFailed {
            path: dir.clone(),
            source: e,
        })?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|ext| ext == "json"))
        .collect();

    entries.sort();
    let latest = match entries.last() {
        Some(p) => p,
        None => return Ok(None),
    };

    let content = std::fs::read_to_string(latest).map_err(|e| SnapshotError::ReadFailed {
        path: latest.clone(),
        source: e,
    })?;

    let snapshot: Snapshot =
        serde_json::from_str(&content).map_err(|e| SnapshotError::ParseFailed {
            path: latest.clone(),
            source: e,
        })?;

    Ok(Some(snapshot))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn make_test_snapshot() -> Snapshot {
        use crate::metrics::aggregate::{AggregateMetrics, LanguageMetrics};
        use crate::metrics::loc::LineMetrics;
        use std::collections::BTreeMap;

        let agg = AggregateMetrics {
            total_files: 1,
            total_lines: LineMetrics {
                total_lines: 10,
                code_lines: 8,
                blank_lines: 1,
                comment_lines: 1,
            },
            by_language: BTreeMap::new(),
            unknown_language: LanguageMetrics::default(),
        };
        Snapshot::from_aggregate(&agg, None)
    }

    #[test]
    fn writes_snapshot_file() {
        let dir = TempDir::new().unwrap();
        let snap = make_test_snapshot();
        let path = write_snapshot(dir.path(), &snap).unwrap();
        assert!(path.exists());
        assert!(path.extension().unwrap() == "json");
    }

    #[test]
    fn load_latest_returns_none_when_no_snapshots() {
        let dir = TempDir::new().unwrap();
        let result = load_latest(dir.path()).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn load_latest_returns_written_snapshot() {
        let dir = TempDir::new().unwrap();
        let snap = make_test_snapshot();
        write_snapshot(dir.path(), &snap).unwrap();
        let loaded = load_latest(dir.path()).unwrap().unwrap();
        assert_eq!(loaded.total_files, 1);
        assert_eq!(loaded.total_lines.code, 8);
    }

    #[test]
    fn roundtrip_preserves_data() {
        let dir = TempDir::new().unwrap();
        let snap = make_test_snapshot();
        write_snapshot(dir.path(), &snap).unwrap();
        let loaded = load_latest(dir.path()).unwrap().unwrap();
        assert_eq!(snap.total_files, loaded.total_files);
        assert_eq!(snap.total_lines.total, loaded.total_lines.total);
        assert_eq!(snap.total_lines.code, loaded.total_lines.code);
    }
}
