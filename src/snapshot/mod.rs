//! Snapshot storage, serialization, and diffing.

pub(crate) mod diff;
pub(crate) mod store;

use std::collections::BTreeMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A point-in-time snapshot of repository metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Snapshot {
    /// When this snapshot was taken.
    pub(crate) timestamp: DateTime<Utc>,
    /// Git SHA at time of snapshot (if available).
    pub(crate) git_sha: Option<String>,
    /// Total files analyzed.
    pub(crate) total_files: usize,
    /// Total line metrics.
    pub(crate) total_lines: SnapshotLineMetrics,
    /// Per-language breakdown.
    pub(crate) by_language: BTreeMap<String, SnapshotLanguageEntry>,
}

/// Line metrics as stored in a snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct SnapshotLineMetrics {
    /// Total lines.
    pub(crate) total: usize,
    /// Code lines.
    pub(crate) code: usize,
    /// Blank lines.
    pub(crate) blank: usize,
    /// Comment lines.
    pub(crate) comment: usize,
}

/// Per-language entry in a snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct SnapshotLanguageEntry {
    /// Number of files.
    pub(crate) files: usize,
    /// Line metrics for this language.
    pub(crate) lines: SnapshotLineMetrics,
}

impl Snapshot {
    /// Build a snapshot from aggregate metrics.
    pub(crate) fn from_aggregate(
        agg: &crate::metrics::aggregate::AggregateMetrics,
        git_sha: Option<String>,
    ) -> Self {
        let mut by_language = BTreeMap::new();
        for (lang, metrics) in &agg.by_language {
            by_language.insert(
                lang.display_name().to_string(),
                SnapshotLanguageEntry {
                    files: metrics.file_count,
                    lines: SnapshotLineMetrics::from_line_metrics(&metrics.lines),
                },
            );
        }
        if agg.unknown_language.file_count > 0 {
            by_language.insert(
                "Other".to_string(),
                SnapshotLanguageEntry {
                    files: agg.unknown_language.file_count,
                    lines: SnapshotLineMetrics::from_line_metrics(&agg.unknown_language.lines),
                },
            );
        }

        Self {
            timestamp: Utc::now(),
            git_sha,
            total_files: agg.total_files,
            total_lines: SnapshotLineMetrics::from_line_metrics(&agg.total_lines),
            by_language,
        }
    }
}

impl SnapshotLineMetrics {
    /// Convert from internal LineMetrics.
    fn from_line_metrics(m: &crate::metrics::loc::LineMetrics) -> Self {
        Self {
            total: m.total_lines,
            code: m.code_lines,
            blank: m.blank_lines,
            comment: m.comment_lines,
        }
    }
}

/// Get the current git SHA, if in a git repository.
pub(crate) fn current_git_sha() -> Option<String> {
    let output = std::process::Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()?;
    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metrics::aggregate::{AggregateMetrics, LanguageMetrics};
    use crate::metrics::loc::LineMetrics;
    use crate::scanner::language::Language;

    #[test]
    fn snapshot_from_aggregate_populates_fields() {
        let mut by_language = BTreeMap::new();
        by_language.insert(
            Language::Rust,
            LanguageMetrics {
                file_count: 3,
                lines: LineMetrics {
                    total_lines: 100,
                    code_lines: 80,
                    blank_lines: 10,
                    comment_lines: 10,
                },
            },
        );

        let agg = AggregateMetrics {
            total_files: 3,
            total_lines: LineMetrics {
                total_lines: 100,
                code_lines: 80,
                blank_lines: 10,
                comment_lines: 10,
            },
            by_language,
            unknown_language: LanguageMetrics::default(),
        };

        let snap = Snapshot::from_aggregate(&agg, Some("abc123".to_string()));
        assert_eq!(snap.total_files, 3);
        assert_eq!(snap.total_lines.code, 80);
        assert_eq!(snap.git_sha, Some("abc123".to_string()));
        assert!(snap.by_language.contains_key("Rust"));
        assert_eq!(snap.by_language["Rust"].files, 3);
    }

    #[test]
    fn snapshot_serializes_to_json() {
        let mut by_language = BTreeMap::new();
        by_language.insert(
            "Rust".to_string(),
            SnapshotLanguageEntry {
                files: 1,
                lines: SnapshotLineMetrics {
                    total: 10,
                    code: 8,
                    blank: 1,
                    comment: 1,
                },
            },
        );

        let snap = Snapshot {
            timestamp: Utc::now(),
            git_sha: None,
            total_files: 1,
            total_lines: SnapshotLineMetrics {
                total: 10,
                code: 8,
                blank: 1,
                comment: 1,
            },
            by_language,
        };

        let json = serde_json::to_string_pretty(&snap).unwrap();
        assert!(json.contains("\"total_files\": 1"));
        assert!(json.contains("\"Rust\""));
    }
}
