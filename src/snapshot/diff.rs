//! Snapshot comparison: compute deltas between two snapshots.

use super::{Snapshot, SnapshotLineMetrics};

/// Delta between two snapshots.
#[derive(Debug, Clone)]
pub(crate) struct SnapshotDiff {
    /// Change in total file count.
    pub(crate) files_delta: i64,
    /// Change in line metrics.
    pub(crate) lines_delta: LinesDelta,
}

/// Delta in line metrics.
#[derive(Debug, Clone)]
pub(crate) struct LinesDelta {
    /// Change in total lines.
    pub(crate) total: i64,
    /// Change in code lines.
    pub(crate) code: i64,
    /// Change in blank lines.
    pub(crate) blank: i64,
    /// Change in comment lines.
    pub(crate) comment: i64,
}

/// Compute the diff between a current snapshot and a previous one.
pub(crate) fn diff(current: &Snapshot, previous: &Snapshot) -> SnapshotDiff {
    SnapshotDiff {
        files_delta: current.total_files as i64 - previous.total_files as i64,
        lines_delta: diff_lines(&current.total_lines, &previous.total_lines),
    }
}

/// Compute delta between two line metrics.
fn diff_lines(current: &SnapshotLineMetrics, previous: &SnapshotLineMetrics) -> LinesDelta {
    LinesDelta {
        total: current.total as i64 - previous.total as i64,
        code: current.code as i64 - previous.code as i64,
        blank: current.blank as i64 - previous.blank as i64,
        comment: current.comment as i64 - previous.comment as i64,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use chrono::Utc;

    use super::*;

    fn make_snapshot(files: usize, total: usize, code: usize) -> Snapshot {
        Snapshot {
            timestamp: Utc::now(),
            git_sha: None,
            total_files: files,
            total_lines: SnapshotLineMetrics {
                total,
                code,
                blank: total - code,
                comment: 0,
            },
            by_language: BTreeMap::new(),
            hotspots: vec![],
            dependencies: None,
            documentation: None,
            ai_analysis: None,
        }
    }

    #[test]
    fn diff_detects_growth() {
        let prev = make_snapshot(5, 100, 80);
        let curr = make_snapshot(8, 150, 120);
        let d = diff(&curr, &prev);
        assert_eq!(d.files_delta, 3);
        assert_eq!(d.lines_delta.total, 50);
        assert_eq!(d.lines_delta.code, 40);
    }

    #[test]
    fn diff_detects_shrinkage() {
        let prev = make_snapshot(10, 200, 160);
        let curr = make_snapshot(8, 150, 120);
        let d = diff(&curr, &prev);
        assert_eq!(d.files_delta, -2);
        assert_eq!(d.lines_delta.total, -50);
        assert_eq!(d.lines_delta.code, -40);
    }

    #[test]
    fn diff_identical_is_zero() {
        let snap = make_snapshot(5, 100, 80);
        let d = diff(&snap, &snap);
        assert_eq!(d.files_delta, 0);
        assert_eq!(d.lines_delta.total, 0);
        assert_eq!(d.lines_delta.code, 0);
    }
}
