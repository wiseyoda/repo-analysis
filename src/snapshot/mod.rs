//! Snapshot storage, serialization, and diffing.

pub(crate) mod diff;
pub(crate) mod index;
pub(crate) mod store;

use std::collections::BTreeMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::metrics::aggregate::AggregateMetrics;
use crate::metrics::complexity::FunctionInfo;
use crate::metrics::dependencies::DependencySummary;
use crate::metrics::documentation::DocumentationMetrics;
use crate::metrics::risk::RiskEntry;

/// Collected outputs from all analysis passes.
///
/// Built incrementally in `main.rs` as each analysis phase completes,
/// then passed to `Snapshot::from_analysis()` for persistence.
pub(crate) struct AnalysisResult {
    /// Aggregate line/file metrics.
    pub(crate) agg: AggregateMetrics,
    /// Git SHA at time of analysis.
    pub(crate) git_sha: Option<String>,
    /// Complexity hotspots: (relative path, function info).
    pub(crate) hotspots: Vec<(String, FunctionInfo)>,
    /// External dependency summary.
    pub(crate) dep_summary: DependencySummary,
    /// Documentation metrics (None if not computed).
    pub(crate) doc_metrics: Option<DocumentationMetrics>,
    /// AI analysis results (None if Claude CLI unavailable).
    pub(crate) ai_result: Option<crate::ai::schema::AiAnalysisResult>,
    /// Number of files skipped due to read errors.
    pub(crate) skipped_files: usize,
    /// Per-file risk entries (churn * complexity).
    pub(crate) risk_entries: Vec<RiskEntry>,
}

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
    /// Complexity hotspots (top N most complex functions).
    #[serde(default)]
    pub(crate) hotspots: Vec<SnapshotHotspot>,
    /// Dependency information.
    #[serde(default)]
    pub(crate) dependencies: Option<SnapshotDependencies>,
    /// Documentation metrics.
    #[serde(default)]
    pub(crate) documentation: Option<SnapshotDocumentation>,
    /// AI analysis results.
    #[serde(default)]
    pub(crate) ai_analysis: Option<crate::ai::schema::AiAnalysisResult>,
    /// Number of files skipped due to read errors.
    #[serde(default)]
    pub(crate) skipped_files: usize,
    /// Per-file risk data (raw inputs: churn + complexity).
    #[serde(default)]
    pub(crate) risk_hotspots: Vec<SnapshotRiskEntry>,
}

/// Per-file risk data stored in a snapshot.
///
/// Stores raw inputs (churn_count, max_complexity) rather than computed
/// scores so the formula can change without invalidating history.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct SnapshotRiskEntry {
    /// File path (relative to repo root).
    pub(crate) file: String,
    /// Number of commits in the last 6 months.
    pub(crate) churn_count: usize,
    /// Maximum cyclomatic complexity of any function.
    pub(crate) max_complexity: usize,
}

/// Dependency data stored in a snapshot.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct SnapshotDependencies {
    /// Number of manifest files found.
    pub(crate) manifest_count: usize,
    /// Total direct dependencies.
    pub(crate) direct: usize,
    /// Total transitive dependencies (from lock files).
    pub(crate) transitive: Option<usize>,
    /// Per-manifest breakdown.
    pub(crate) manifests: Vec<SnapshotManifest>,
}

/// Per-manifest entry in a snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct SnapshotManifest {
    /// Manifest filename.
    pub(crate) name: String,
    /// Ecosystem type.
    pub(crate) ecosystem: String,
    /// Number of direct dependencies.
    pub(crate) deps: usize,
}

/// Documentation metrics stored in a snapshot.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct SnapshotDocumentation {
    /// Number of markdown files.
    pub(crate) file_count: usize,
    /// Total lines across all markdown files.
    pub(crate) total_lines: usize,
    /// Total characters across all markdown files.
    pub(crate) total_chars: usize,
    /// Doc-to-code ratio.
    pub(crate) doc_to_code_ratio: f64,
    /// README completeness score (0.0 to 1.0).
    pub(crate) readme_score: f64,
    /// README sections that were found.
    pub(crate) readme_sections: Vec<String>,
    /// Per-directory documentation coverage (0.0 to 1.0).
    pub(crate) dir_coverage: f64,
}

/// A complexity hotspot stored in a snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct SnapshotHotspot {
    /// File path (relative to repo root).
    pub(crate) file: String,
    /// Function name.
    pub(crate) function: String,
    /// Cyclomatic complexity.
    pub(crate) cyclomatic: usize,
    /// Cognitive complexity.
    pub(crate) cognitive: usize,
    /// Number of lines.
    pub(crate) lines: usize,
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
    /// Build a snapshot from aggregate metrics, hotspots, dependency, doc, and AI data.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn from_aggregate(
        agg: &crate::metrics::aggregate::AggregateMetrics,
        git_sha: Option<String>,
        hotspots: &[(String, crate::metrics::complexity::FunctionInfo)],
        dep_summary: &crate::metrics::dependencies::DependencySummary,
        doc_metrics: Option<&crate::metrics::documentation::DocumentationMetrics>,
        ai_result: Option<&crate::ai::schema::AiAnalysisResult>,
        skipped_files: usize,
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

        let snapshot_hotspots = hotspots
            .iter()
            .map(|(path, func)| SnapshotHotspot {
                file: path.clone(),
                function: func.name.clone(),
                cyclomatic: func.cyclomatic,
                cognitive: func.cognitive,
                lines: func.line_count,
            })
            .collect();

        let dependencies = if dep_summary.manifests.is_empty() {
            None
        } else {
            Some(SnapshotDependencies {
                manifest_count: dep_summary.manifests.len(),
                direct: dep_summary.total_direct,
                transitive: dep_summary.total_transitive,
                manifests: dep_summary
                    .manifests
                    .iter()
                    .map(|m| SnapshotManifest {
                        name: m
                            .file_path
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("unknown")
                            .to_string(),
                        ecosystem: format!("{:?}", m.manifest_type),
                        deps: m.direct_deps.len(),
                    })
                    .collect(),
            })
        };

        let documentation = doc_metrics.map(|docs| SnapshotDocumentation {
            file_count: docs.inventory.file_count,
            total_lines: docs.inventory.total_lines,
            total_chars: docs.inventory.total_chars,
            doc_to_code_ratio: docs.doc_to_code.ratio,
            readme_score: docs.readme_score.score,
            readme_sections: docs
                .readme_score
                .sections
                .iter()
                .filter(|s| s.present)
                .map(|s| s.name.to_string())
                .collect(),
            dir_coverage: docs.dir_coverage.coverage,
        });

        Self {
            timestamp: Utc::now(),
            git_sha,
            total_files: agg.total_files,
            total_lines: SnapshotLineMetrics::from_line_metrics(&agg.total_lines),
            by_language,
            hotspots: snapshot_hotspots,
            dependencies,
            documentation,
            ai_analysis: ai_result.cloned(),
            skipped_files,
            risk_hotspots: vec![],
        }
    }

    /// Build a snapshot from an `AnalysisResult`.
    pub(crate) fn from_analysis(result: &AnalysisResult) -> Self {
        let mut snap = Self::from_aggregate(
            &result.agg,
            result.git_sha.clone(),
            &result.hotspots,
            &result.dep_summary,
            result.doc_metrics.as_ref(),
            result.ai_result.as_ref(),
            result.skipped_files,
        );
        snap.risk_hotspots = result
            .risk_entries
            .iter()
            .map(|r| SnapshotRiskEntry {
                file: r.file.clone(),
                churn_count: r.churn_count,
                max_complexity: r.max_complexity,
            })
            .collect();
        snap
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

        let dep_default = crate::metrics::dependencies::DependencySummary::default();
        let snap = Snapshot::from_aggregate(
            &agg,
            Some("abc123".to_string()),
            &[],
            &dep_default,
            None,
            None,
            2,
        );
        assert_eq!(snap.total_files, 3);
        assert_eq!(snap.total_lines.code, 80);
        assert_eq!(snap.git_sha, Some("abc123".to_string()));
        assert!(snap.by_language.contains_key("Rust"));
        assert_eq!(snap.by_language["Rust"].files, 3);
        assert_eq!(snap.skipped_files, 2);
    }

    #[test]
    fn snapshot_from_analysis_result() {
        let mut by_language = BTreeMap::new();
        by_language.insert(
            Language::Rust,
            LanguageMetrics {
                file_count: 5,
                lines: LineMetrics {
                    total_lines: 200,
                    code_lines: 160,
                    blank_lines: 20,
                    comment_lines: 20,
                },
            },
        );

        let agg = AggregateMetrics {
            total_files: 5,
            total_lines: LineMetrics {
                total_lines: 200,
                code_lines: 160,
                blank_lines: 20,
                comment_lines: 20,
            },
            by_language,
            unknown_language: LanguageMetrics::default(),
        };

        let result = AnalysisResult {
            agg,
            git_sha: Some("def456".to_string()),
            hotspots: vec![],
            dep_summary: crate::metrics::dependencies::DependencySummary::default(),
            doc_metrics: None,
            ai_result: None,
            skipped_files: 1,
            risk_entries: vec![],
        };

        let snap = Snapshot::from_analysis(&result);
        assert_eq!(snap.total_files, 5);
        assert_eq!(snap.total_lines.code, 160);
        assert_eq!(snap.git_sha, Some("def456".to_string()));
        assert_eq!(snap.skipped_files, 1);
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
            hotspots: vec![],
            dependencies: None,
            documentation: None,
            ai_analysis: None,
            skipped_files: 0,
            risk_hotspots: vec![],
        };

        let json = serde_json::to_string_pretty(&snap).unwrap();
        assert!(json.contains("\"total_files\": 1"));
        assert!(json.contains("\"Rust\""));
    }
}

#[cfg(test)]
mod doc_tests {
    use super::*;

    #[test]
    fn snapshot_with_documentation_serializes() {
        let doc = SnapshotDocumentation {
            file_count: 5,
            total_lines: 200,
            total_chars: 8000,
            doc_to_code_ratio: 0.15,
            readme_score: 0.8,
            readme_sections: vec!["install".to_string(), "usage".to_string()],
            dir_coverage: 0.75,
        };

        let json = serde_json::to_string_pretty(&doc).unwrap();
        assert!(json.contains("\"file_count\": 5"));
        assert!(json.contains("\"doc_to_code_ratio\": 0.15"));
        assert!(json.contains("\"readme_score\": 0.8"));
        assert!(json.contains("\"install\""));
    }

    #[test]
    fn snapshot_documentation_roundtrips() {
        let doc = SnapshotDocumentation {
            file_count: 3,
            total_lines: 100,
            total_chars: 5000,
            doc_to_code_ratio: 0.1,
            readme_score: 0.6,
            readme_sections: vec!["install".to_string(), "license".to_string()],
            dir_coverage: 0.5,
        };

        let json = serde_json::to_string(&doc).unwrap();
        let parsed: SnapshotDocumentation = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.file_count, 3);
        assert_eq!(parsed.readme_sections.len(), 2);
        assert!((parsed.dir_coverage - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn snapshot_without_documentation_deserializes() {
        // Simulate an old snapshot without documentation field
        let json = r#"{
            "timestamp": "2024-01-01T00:00:00Z",
            "git_sha": null,
            "total_files": 1,
            "total_lines": {"total": 10, "code": 8, "blank": 1, "comment": 1},
            "by_language": {},
            "hotspots": []
        }"#;

        let snap: Snapshot = serde_json::from_str(json).unwrap();
        assert!(snap.documentation.is_none());
    }
}
