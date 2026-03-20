//! Risk score computation: churn * complexity per file.

use std::collections::BTreeMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// A single file's risk assessment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct RiskEntry {
    /// File path (relative to repo root).
    pub(crate) file: String,
    /// Number of commits touching this file in the last 6 months.
    pub(crate) churn_count: usize,
    /// Maximum cyclomatic complexity of any function in this file.
    pub(crate) max_complexity: usize,
    /// Risk score: churn_count * max_complexity.
    pub(crate) risk_score: usize,
}

/// Compute risk scores by combining churn and complexity data.
///
/// Returns entries sorted by risk score descending.
/// Files with zero risk score are excluded.
pub(crate) fn compute_risk_scores(
    churn: &BTreeMap<PathBuf, usize>,
    file_complexity: &BTreeMap<String, usize>,
) -> Vec<RiskEntry> {
    let mut entries: Vec<RiskEntry> = churn
        .iter()
        .map(|(path, &churn_count)| {
            let path_str = path.display().to_string();
            let max_complexity = file_complexity.get(&path_str).copied().unwrap_or(1);
            let risk_score = churn_count * max_complexity;
            RiskEntry {
                file: path_str,
                churn_count,
                max_complexity,
                risk_score,
            }
        })
        .filter(|e| e.risk_score > 0)
        .collect();

    entries.sort_by(|a, b| b.risk_score.cmp(&a.risk_score));
    entries
}

/// Extract per-file max complexity from hotspot data.
///
/// Hotspots are (file_path, FunctionInfo) pairs. This returns the
/// maximum cyclomatic complexity for each file.
pub(crate) fn file_complexity_map(
    hotspots: &[(String, super::complexity::FunctionInfo)],
) -> BTreeMap<String, usize> {
    let mut map: BTreeMap<String, usize> = BTreeMap::new();
    for (file, func) in hotspots {
        let entry = map.entry(file.clone()).or_insert(0);
        if func.cyclomatic > *entry {
            *entry = func.cyclomatic;
        }
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_risk_scores_basic() {
        let mut churn = BTreeMap::new();
        churn.insert(PathBuf::from("src/main.rs"), 10);
        churn.insert(PathBuf::from("src/lib.rs"), 5);

        let mut complexity = BTreeMap::new();
        complexity.insert("src/main.rs".to_string(), 20);
        complexity.insert("src/lib.rs".to_string(), 3);

        let scores = compute_risk_scores(&churn, &complexity);
        assert_eq!(scores.len(), 2);
        // main.rs: 10 * 20 = 200, lib.rs: 5 * 3 = 15
        assert_eq!(scores[0].file, "src/main.rs");
        assert_eq!(scores[0].risk_score, 200);
        assert_eq!(scores[1].file, "src/lib.rs");
        assert_eq!(scores[1].risk_score, 15);
    }

    #[test]
    fn compute_risk_scores_default_complexity() {
        let mut churn = BTreeMap::new();
        churn.insert(PathBuf::from("README.md"), 8);

        let complexity = BTreeMap::new(); // no complexity data

        let scores = compute_risk_scores(&churn, &complexity);
        assert_eq!(scores.len(), 1);
        // 8 * 1 (default) = 8
        assert_eq!(scores[0].risk_score, 8);
        assert_eq!(scores[0].max_complexity, 1);
    }

    #[test]
    fn compute_risk_scores_empty_churn() {
        let churn = BTreeMap::new();
        let complexity = BTreeMap::new();
        let scores = compute_risk_scores(&churn, &complexity);
        assert!(scores.is_empty());
    }

    #[test]
    fn compute_risk_scores_sorted_descending() {
        let mut churn = BTreeMap::new();
        churn.insert(PathBuf::from("a.rs"), 1);
        churn.insert(PathBuf::from("b.rs"), 10);
        churn.insert(PathBuf::from("c.rs"), 5);

        let mut complexity = BTreeMap::new();
        complexity.insert("a.rs".to_string(), 10);
        complexity.insert("b.rs".to_string(), 1);
        complexity.insert("c.rs".to_string(), 4);

        let scores = compute_risk_scores(&churn, &complexity);
        // a: 1*10=10, b: 10*1=10, c: 5*4=20
        assert_eq!(scores[0].file, "c.rs");
        assert_eq!(scores[0].risk_score, 20);
    }

    #[test]
    fn file_complexity_map_picks_max() {
        use super::super::complexity::FunctionInfo;

        let hotspots = vec![
            (
                "src/main.rs".to_string(),
                FunctionInfo {
                    name: "foo".to_string(),
                    cyclomatic: 5,
                    cognitive: 3,
                    line_count: 10,
                },
            ),
            (
                "src/main.rs".to_string(),
                FunctionInfo {
                    name: "bar".to_string(),
                    cyclomatic: 15,
                    cognitive: 8,
                    line_count: 20,
                },
            ),
        ];

        let map = file_complexity_map(&hotspots);
        assert_eq!(map["src/main.rs"], 15);
    }

    #[test]
    fn file_complexity_map_empty_hotspots() {
        let map = file_complexity_map(&[]);
        assert!(map.is_empty());
    }
}
