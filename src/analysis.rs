//! Shared deterministic repository analysis.

use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

use rayon::prelude::*;

use crate::config::Config;
use crate::{metrics, scanner, snapshot};

struct AnalyzedFile<'a> {
    file: &'a scanner::ScannedFile,
    lines: metrics::loc::LineMetrics,
    functions: Vec<metrics::complexity::FunctionInfo>,
}

struct DerivedMetrics {
    aggregate: metrics::aggregate::AggregateMetrics,
    hotspots: Vec<(String, metrics::complexity::FunctionInfo)>,
    dep_summary: metrics::dependencies::DependencySummary,
    doc_metrics: metrics::documentation::DocumentationMetrics,
    risk_entries: Vec<metrics::risk::RiskEntry>,
    dependencies_duration: Duration,
    documentation_duration: Duration,
}

/// Per-phase timings for optional CLI diagnostics.
pub(crate) struct AnalysisTimings {
    pub(crate) scanner: Duration,
    pub(crate) metrics: Duration,
    pub(crate) dependencies: Duration,
    pub(crate) documentation: Duration,
}

/// Deterministic analysis plus timing metadata excluded from structured output.
pub(crate) struct CompletedAnalysis {
    pub(crate) result: snapshot::AnalysisResult,
    pub(crate) timings: AnalysisTimings,
}

/// Analyze a canonical repository root without writing or invoking a model.
pub(crate) fn analyze(path: &Path, config: &Config) -> anyhow::Result<CompletedAnalysis> {
    let started = Instant::now();
    let files = scanner::scan(path, config)?;
    let scanner_duration = started.elapsed();

    let started = Instant::now();
    let (analyzed, skipped_files) = analyze_files(&files);
    let metrics_duration = started.elapsed();
    warn_if_empty(&analyzed);
    let derived = derive_metrics(path, &analyzed);

    Ok(CompletedAnalysis {
        result: snapshot::AnalysisResult {
            agg: derived.aggregate,
            git_sha: snapshot::current_git_sha(path),
            hotspots: derived.hotspots,
            dep_summary: derived.dep_summary,
            doc_metrics: Some(derived.doc_metrics),
            ai_result: None,
            skipped_files,
            risk_entries: derived.risk_entries,
        },
        timings: AnalysisTimings {
            scanner: scanner_duration,
            metrics: metrics_duration,
            dependencies: derived.dependencies_duration,
            documentation: derived.documentation_duration,
        },
    })
}

fn analyze_files(files: &[scanner::ScannedFile]) -> (Vec<AnalyzedFile<'_>>, usize) {
    let skipped_count = AtomicUsize::new(0);
    let analyzed: Vec<_> = files
        .par_iter()
        .filter(|file| !file.is_minified && !file.is_generated)
        .filter_map(|file| analyze_file(file, &skipped_count))
        .collect();
    (analyzed, skipped_count.load(Ordering::Relaxed))
}

fn analyze_file<'a>(
    file: &'a scanner::ScannedFile,
    skipped_count: &AtomicUsize,
) -> Option<AnalyzedFile<'a>> {
    let content = match std::fs::read_to_string(&file.path) {
        Ok(content) => content,
        Err(error) => {
            eprintln!("warning: skipped {}: {error}", file.path.display());
            skipped_count.fetch_add(1, Ordering::Relaxed);
            return None;
        }
    };
    let lines = metrics::loc::count_lines(&content, file.language);
    let functions = file
        .language
        .and_then(|language| metrics::complexity::extract_functions(&content, language))
        .unwrap_or_default();
    Some(AnalyzedFile {
        file,
        lines,
        functions,
    })
}

fn warn_if_empty(analyzed: &[AnalyzedFile<'_>]) {
    if analyzed.is_empty() {
        eprintln!(
            "warning: no source files found after filtering. \
             Check your .repostat.toml exclude patterns."
        );
    }
}

fn derive_metrics(path: &Path, analyzed: &[AnalyzedFile<'_>]) -> DerivedMetrics {
    let aggregate = aggregate_files(analyzed);
    let hotspots = collect_hotspots(path, analyzed);
    let started = Instant::now();
    let dep_summary = metrics::dependencies::summarize_dependencies(path);
    let dependencies_duration = started.elapsed();
    let started = Instant::now();
    let doc_metrics =
        metrics::documentation::analyze_documentation(path, aggregate.total_lines.code_lines);
    let documentation_duration = started.elapsed();
    let complexity_map = metrics::risk::file_complexity_map(&hotspots);
    let risk_entries = metrics::git_history::collect_file_churn(path)
        .as_ref()
        .map(|churn| metrics::risk::compute_risk_scores(churn, &complexity_map))
        .unwrap_or_default();

    DerivedMetrics {
        aggregate,
        hotspots,
        dep_summary,
        doc_metrics,
        risk_entries,
        dependencies_duration,
        documentation_duration,
    }
}

fn aggregate_files(analyzed: &[AnalyzedFile<'_>]) -> metrics::aggregate::AggregateMetrics {
    let file_results: Vec<_> = analyzed
        .iter()
        .map(|analyzed_file| metrics::aggregate::FileResult {
            language: analyzed_file.file.language,
            lines: analyzed_file.lines,
        })
        .collect();
    metrics::aggregate::aggregate(&file_results)
}

fn collect_hotspots(
    path: &Path,
    analyzed: &[AnalyzedFile<'_>],
) -> Vec<(String, metrics::complexity::FunctionInfo)> {
    let mut all_functions: Vec<_> = analyzed
        .iter()
        .flat_map(|analyzed_file| {
            let relative_path = analyzed_file
                .file
                .path
                .strip_prefix(path)
                .unwrap_or(&analyzed_file.file.path)
                .display()
                .to_string();
            analyzed_file
                .functions
                .iter()
                .map(move |function| (relative_path.clone(), function.clone()))
        })
        .collect();
    all_functions.sort_by(|left, right| {
        right
            .1
            .cyclomatic
            .cmp(&left.1.cyclomatic)
            .then_with(|| left.0.cmp(&right.0))
            .then_with(|| left.1.name.cmp(&right.1.name))
    });
    all_functions.into_iter().take(10).collect()
}
