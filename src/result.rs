//! Stable structured output for suite and standalone consumers.

use std::collections::BTreeMap;
use std::path::Path;

use serde::Serialize;

use crate::snapshot::{Snapshot, SnapshotLineMetrics};

/// Version of the stable metrics payload.
pub(crate) const METRICS_SCHEMA_VERSION: &str = "1";
/// Artifact type advertised to Engine.
pub(crate) const METRICS_ARTIFACT_TYPE: &str = "repostat.metrics.v1";

/// Canonical identity of the analyzed source.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SourceIdentity<'a> {
    canonical_root: String,
    git_sha: Option<&'a str>,
}

#[derive(Serialize)]
struct LineMetrics {
    total: usize,
    code: usize,
    blank: usize,
    comment: usize,
}

impl From<&SnapshotLineMetrics> for LineMetrics {
    fn from(value: &SnapshotLineMetrics) -> Self {
        Self {
            total: value.total,
            code: value.code,
            blank: value.blank,
            comment: value.comment,
        }
    }
}

#[derive(Serialize)]
struct LanguageMetrics {
    files: usize,
    lines: LineMetrics,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Hotspot<'a> {
    file: &'a str,
    function: &'a str,
    cyclomatic: usize,
    cognitive: usize,
    lines: usize,
}

#[derive(Serialize)]
struct ManifestMetrics<'a> {
    name: &'a str,
    ecosystem: &'a str,
    deps: usize,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct DependencyMetrics<'a> {
    manifest_count: usize,
    direct: usize,
    transitive: Option<usize>,
    manifests: Vec<ManifestMetrics<'a>>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct DocumentationMetrics<'a> {
    file_count: usize,
    total_lines: usize,
    total_chars: usize,
    doc_to_code_ratio: f64,
    readme_score: f64,
    readme_sections: &'a [String],
    dir_coverage: f64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct RiskHotspot<'a> {
    file: &'a str,
    churn_count: usize,
    max_complexity: usize,
}

/// Deterministic metrics payload. Engine owns timestamps and artifact identity.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MetricsResult<'a> {
    schema_version: &'static str,
    artifact_type: &'static str,
    source: SourceIdentity<'a>,
    total_files: usize,
    total_lines: LineMetrics,
    by_language: BTreeMap<&'a str, LanguageMetrics>,
    hotspots: Vec<Hotspot<'a>>,
    dependencies: Option<DependencyMetrics<'a>>,
    documentation: Option<DocumentationMetrics<'a>>,
    skipped_files: usize,
    risk_hotspots: Vec<RiskHotspot<'a>>,
}

impl<'a> MetricsResult<'a> {
    /// Build stable output from a historical snapshot representation.
    pub(crate) fn new(root: &Path, snapshot: &'a Snapshot) -> Self {
        Self {
            schema_version: METRICS_SCHEMA_VERSION,
            artifact_type: METRICS_ARTIFACT_TYPE,
            source: SourceIdentity {
                canonical_root: root.display().to_string(),
                git_sha: snapshot.git_sha.as_deref(),
            },
            total_files: snapshot.total_files,
            total_lines: LineMetrics::from(&snapshot.total_lines),
            by_language: language_metrics(snapshot),
            hotspots: hotspot_metrics(snapshot),
            dependencies: dependency_metrics(snapshot),
            documentation: documentation_metrics(snapshot),
            skipped_files: snapshot.skipped_files,
            risk_hotspots: risk_metrics(snapshot),
        }
    }
}

fn language_metrics(snapshot: &Snapshot) -> BTreeMap<&str, LanguageMetrics> {
    snapshot
        .by_language
        .iter()
        .map(|(language, metrics)| {
            (
                language.as_str(),
                LanguageMetrics {
                    files: metrics.files,
                    lines: LineMetrics::from(&metrics.lines),
                },
            )
        })
        .collect()
}

fn hotspot_metrics(snapshot: &Snapshot) -> Vec<Hotspot<'_>> {
    snapshot
        .hotspots
        .iter()
        .map(|hotspot| Hotspot {
            file: &hotspot.file,
            function: &hotspot.function,
            cyclomatic: hotspot.cyclomatic,
            cognitive: hotspot.cognitive,
            lines: hotspot.lines,
        })
        .collect()
}

fn dependency_metrics(snapshot: &Snapshot) -> Option<DependencyMetrics<'_>> {
    snapshot
        .dependencies
        .as_ref()
        .map(|dependencies| DependencyMetrics {
            manifest_count: dependencies.manifest_count,
            direct: dependencies.direct,
            transitive: dependencies.transitive,
            manifests: dependencies
                .manifests
                .iter()
                .map(|manifest| ManifestMetrics {
                    name: &manifest.name,
                    ecosystem: &manifest.ecosystem,
                    deps: manifest.deps,
                })
                .collect(),
        })
}

fn documentation_metrics(snapshot: &Snapshot) -> Option<DocumentationMetrics<'_>> {
    snapshot
        .documentation
        .as_ref()
        .map(|documentation| DocumentationMetrics {
            file_count: documentation.file_count,
            total_lines: documentation.total_lines,
            total_chars: documentation.total_chars,
            doc_to_code_ratio: documentation.doc_to_code_ratio,
            readme_score: documentation.readme_score,
            readme_sections: &documentation.readme_sections,
            dir_coverage: documentation.dir_coverage,
        })
}

fn risk_metrics(snapshot: &Snapshot) -> Vec<RiskHotspot<'_>> {
    snapshot
        .risk_hotspots
        .iter()
        .map(|risk| RiskHotspot {
            file: &risk.file,
            churn_count: risk.churn_count,
            max_complexity: risk.max_complexity,
        })
        .collect()
}
