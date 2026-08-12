//! Standalone analyze-command orchestration.

use std::path::Path;
use std::process;
use std::time::{Duration, Instant};

use crate::{analysis, cli, config, report, result, snapshot};

struct SnapshotState {
    current: snapshot::Snapshot,
    diff: Option<snapshot::diff::SnapshotDiff>,
    duration: Duration,
}

struct ReportContext<'a> {
    args: &'a cli::AnalyzeArgs,
    analysis: &'a snapshot::AnalysisResult,
    snapshot: &'a snapshot::Snapshot,
    diff: Option<&'a snapshot::diff::SnapshotDiff>,
}

struct CommandTimings {
    snapshot: Duration,
    report: Duration,
    total: Duration,
}

/// Run the default standalone analyze command.
pub(crate) fn run(args: &cli::AnalyzeArgs) {
    let total_start = Instant::now();
    let config = load_config(&args.path);
    let completed = run_analysis(&args.path, &config);
    let state = prepare_snapshot(args, &completed.result);
    let report = render_report(&ReportContext {
        args,
        analysis: &completed.result,
        snapshot: &state.current,
        diff: state.diff.as_ref(),
    });
    if args.verbose {
        print_timings(
            &completed,
            &CommandTimings {
                snapshot: state.duration,
                report,
                total: total_start.elapsed(),
            },
        );
    }
    exit_for_health(&config, &completed.result);
}

fn load_config(path: &Path) -> config::Config {
    match config::Config::load(path) {
        Ok(config) => config,
        Err(error) => exit_with_error(&error),
    }
}

fn run_analysis(path: &Path, config: &config::Config) -> analysis::CompletedAnalysis {
    match analysis::analyze(path, config) {
        Ok(completed) => completed,
        Err(error) => exit_with_error(&error),
    }
}

fn prepare_snapshot(args: &cli::AnalyzeArgs, analysis: &snapshot::AnalysisResult) -> SnapshotState {
    let started = Instant::now();
    let previous = if args.write_mode == cli::WriteMode::Save {
        snapshot::store::load_latest(&args.path).ok().flatten()
    } else {
        None
    };
    let current = snapshot::Snapshot::from_analysis(analysis);
    if args.write_mode == cli::WriteMode::Save {
        persist_snapshot(&args.path, &current);
    }
    let diff = previous.map(|previous| snapshot::diff::diff(&current, &previous));
    SnapshotState {
        current,
        diff,
        duration: started.elapsed(),
    }
}

fn persist_snapshot(path: &Path, snapshot: &snapshot::Snapshot) {
    match snapshot::store::write_snapshot(path, snapshot) {
        Ok(_) => snapshot::index::register_repo(path),
        Err(error) => eprintln!("warning: failed to write snapshot: {error}"),
    }
}

fn render_report(context: &ReportContext<'_>) -> Duration {
    let started = Instant::now();
    if context.args.json {
        render_json(&context.args.path, context.snapshot);
    } else if context.args.markdown {
        render_markdown(context.analysis, context.diff);
    } else if context.args.html {
        render_html(&context.args.path, context.analysis);
    } else {
        render_dashboard(context);
    }
    started.elapsed()
}

fn render_json(path: &Path, snapshot: &snapshot::Snapshot) {
    match serde_json::to_string_pretty(&result::MetricsResult::new(path, snapshot)) {
        Ok(json) => println!("{json}"),
        Err(error) => exit_with_report_error("serialize JSON", &error),
    }
}

fn render_markdown(
    analysis: &snapshot::AnalysisResult,
    diff: Option<&snapshot::diff::SnapshotDiff>,
) {
    let mut stdout = std::io::stdout().lock();
    if let Err(error) =
        report::markdown::render(&analysis.agg, diff, &analysis.risk_entries, &mut stdout)
    {
        exit_with_report_error("render markdown", &error);
    }
}

fn render_html(path: &Path, analysis: &snapshot::AnalysisResult) {
    let html_path = path.join("repostat-report.html");
    let data = report::html::HtmlData {
        agg: &analysis.agg,
        hotspots: &analysis.hotspots,
        risk_entries: &analysis.risk_entries,
    };
    let mut file = match std::fs::File::create(&html_path) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("error: failed to create {}: {error}", html_path.display());
            process::exit(2);
        }
    };
    if let Err(error) = report::html::render(&data, &mut file) {
        exit_with_report_error("write HTML report", &error);
    }
    eprintln!("HTML report written to {}", html_path.display());
}

fn render_dashboard(context: &ReportContext<'_>) {
    let (history_lines, history_files) = load_history(context.args);
    let data = report::dashboard::DashboardData {
        agg: &context.analysis.agg,
        diff: context.diff,
        hotspots: &context.analysis.hotspots,
        dep_summary: &context.analysis.dep_summary,
        doc_metrics: context.analysis.doc_metrics.as_ref(),
        ai_result: context.analysis.ai_result.as_ref(),
        history_lines,
        history_files,
        skipped_files: context.analysis.skipped_files,
        risk_entries: &context.analysis.risk_entries,
    };
    let color = report::color::is_color_enabled();
    let mut stdout = std::io::stdout().lock();
    if let Err(error) = report::dashboard::render(&data, &mut stdout, color) {
        exit_with_report_error("render dashboard", &error);
    }
}

fn load_history(args: &cli::AnalyzeArgs) -> (Vec<usize>, Vec<usize>) {
    if args.write_mode != cli::WriteMode::Save {
        return (Vec::new(), Vec::new());
    }
    let snapshots = snapshot::store::load_all(&args.path).unwrap_or_default();
    let lines = snapshots
        .iter()
        .map(|snapshot| snapshot.total_lines.code)
        .collect();
    let files = snapshots
        .iter()
        .map(|snapshot| snapshot.total_files)
        .collect();
    (lines, files)
}

fn print_timings(completed: &analysis::CompletedAnalysis, timings: &CommandTimings) {
    eprintln!(
        "  scanner:      {:.1}s",
        completed.timings.scanner.as_secs_f64()
    );
    eprintln!(
        "  metrics:      {:.1}s",
        completed.timings.metrics.as_secs_f64()
    );
    eprintln!(
        "  dependencies: {:.1}s",
        completed.timings.dependencies.as_secs_f64()
    );
    eprintln!(
        "  docs:         {:.1}s",
        completed.timings.documentation.as_secs_f64()
    );
    eprintln!("  snapshot:     {:.1}s", timings.snapshot.as_secs_f64());
    eprintln!("  report:       {:.1}s", timings.report.as_secs_f64());
    eprintln!("  total:        {:.1}s", timings.total.as_secs_f64());
}

fn exit_for_health(config: &config::Config, analysis: &snapshot::AnalysisResult) {
    let max_complexity = analysis
        .hotspots
        .iter()
        .map(|(_, function)| function.cyclomatic)
        .max()
        .unwrap_or(0);
    let max_function_lines = analysis
        .hotspots
        .iter()
        .map(|(_, function)| function.line_count)
        .max()
        .unwrap_or(0);
    let exit_code = config.health.evaluate(max_complexity, max_function_lines);
    if exit_code != 0 {
        process::exit(exit_code);
    }
}

fn exit_with_error(error: &dyn std::fmt::Display) -> ! {
    eprintln!("error: {error}");
    process::exit(1);
}

fn exit_with_report_error(action: &str, error: &dyn std::fmt::Display) -> ! {
    eprintln!("error: failed to {action}: {error}");
    process::exit(2);
}
