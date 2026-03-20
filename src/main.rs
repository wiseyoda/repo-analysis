//! repostat — Analyze repository complexity and track coding progress.

use std::path::Path;
use std::process;
use std::time::Instant;

use rayon::prelude::*;

mod ai;
mod cli;
mod config;
mod errors;
mod metrics;
mod report;
mod scanner;
mod snapshot;

fn main() {
    let command = match cli::parse_and_validate() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("error: {e}");
            process::exit(1);
        }
    };

    match command {
        cli::ValidatedCommand::Analyze(args) => run_analyze(&args),
        cli::ValidatedCommand::Trend(args) => run_trend(&args.path),
        cli::ValidatedCommand::List => run_list(),
        cli::ValidatedCommand::Completions(shell) => cli::generate_completions(shell),
        cli::ValidatedCommand::Manpage => {
            if let Err(e) = cli::generate_manpage() {
                eprintln!("error: failed to generate man page: {e}");
                process::exit(2);
            }
        }
        cli::ValidatedCommand::Init { force } => run_init(force),
        cli::ValidatedCommand::Diff(args) => run_diff(&args),
    }
}

/// Run the default analyze command.
fn run_analyze(args: &cli::AnalyzeArgs) {
    let total_start = Instant::now();

    let config = match config::Config::load(&args.path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("error: {e}");
            process::exit(1);
        }
    };

    let t = Instant::now();
    let files = match scanner::scan(&args.path, &config) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("error: {e}");
            process::exit(1);
        }
    };
    let scanner_dur = t.elapsed();

    let t = Instant::now();
    let skipped_count = std::sync::atomic::AtomicUsize::new(0);
    let analyzed: Vec<_> = files
        .par_iter()
        .filter(|f| !f.is_minified && !f.is_generated)
        .filter_map(|f| {
            let content = match std::fs::read_to_string(&f.path) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("warning: skipped {}: {e}", f.path.display());
                    skipped_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    return None;
                }
            };
            let lines = metrics::loc::count_lines(&content, f.language);
            let functions = f
                .language
                .and_then(|lang| metrics::complexity::extract_functions(&content, lang))
                .unwrap_or_default();
            Some((f, lines, functions))
        })
        .collect();
    let skipped_files = skipped_count.load(std::sync::atomic::Ordering::Relaxed);
    let metrics_dur = t.elapsed();

    if analyzed.is_empty() {
        eprintln!(
            "warning: no source files found after filtering. \
             Check your .repostat.toml exclude patterns."
        );
    }

    let file_results: Vec<_> = analyzed
        .iter()
        .map(|(f, lines, _)| metrics::aggregate::FileResult {
            language: f.language,
            lines: *lines,
        })
        .collect();

    let agg = metrics::aggregate::aggregate(&file_results);

    // Collect hotspots: top 10 most complex functions across all files
    let mut all_functions: Vec<_> = analyzed
        .iter()
        .flat_map(|(f, _, functions)| {
            let path = f
                .path
                .strip_prefix(&args.path)
                .unwrap_or(&f.path)
                .display()
                .to_string();
            functions
                .iter()
                .map(move |func| (path.clone(), func.clone()))
        })
        .collect();
    all_functions.sort_by(|a, b| b.1.cyclomatic.cmp(&a.1.cyclomatic));
    let hotspots: Vec<_> = all_functions.into_iter().take(10).collect();

    let t = Instant::now();
    let dep_summary = metrics::dependencies::summarize_dependencies(&args.path);
    let deps_dur = t.elapsed();

    let t = Instant::now();
    let doc_metrics =
        metrics::documentation::analyze_documentation(&args.path, agg.total_lines.code_lines);
    let docs_dur = t.elapsed();

    let t = Instant::now();
    let ai_result = ai::run_ai_analysis(&args.path);
    let ai_dur = t.elapsed();

    // Compute risk scores from churn + complexity
    let file_churn = metrics::git_history::collect_file_churn(&args.path);
    let complexity_map = metrics::risk::file_complexity_map(&hotspots);
    let risk_entries = match &file_churn {
        Some(churn) => metrics::risk::compute_risk_scores(churn, &complexity_map),
        None => vec![],
    };

    let analysis = snapshot::AnalysisResult {
        agg,
        git_sha: snapshot::current_git_sha(),
        hotspots,
        dep_summary,
        doc_metrics: Some(doc_metrics),
        ai_result,
        skipped_files,
        risk_entries,
    };

    let t = Instant::now();
    let previous = snapshot::store::load_latest(&args.path).ok().flatten();

    let snap = snapshot::Snapshot::from_analysis(&analysis);
    if let Err(e) = snapshot::store::write_snapshot(&args.path, &snap) {
        eprintln!("warning: failed to write snapshot: {e}");
    }

    // Register in cross-repo index
    snapshot::index::register_repo(&args.path);
    let snapshot_dur = t.elapsed();

    let diff = previous.map(|prev| snapshot::diff::diff(&snap, &prev));

    let t = Instant::now();
    if args.json {
        match serde_json::to_string_pretty(&snap) {
            Ok(json) => println!("{json}"),
            Err(e) => {
                eprintln!("error: failed to serialize JSON: {e}");
                process::exit(2);
            }
        }
    } else if args.markdown {
        let mut stdout = std::io::stdout().lock();
        if let Err(e) = report::markdown::render(
            &analysis.agg,
            diff.as_ref(),
            &analysis.risk_entries,
            &mut stdout,
        ) {
            eprintln!("error: failed to render markdown: {e}");
            process::exit(2);
        }
    } else {
        let color = report::color::is_color_enabled();
        let mut stdout = std::io::stdout().lock();

        // Load snapshot history for sparklines
        let all_snapshots = snapshot::store::load_all(&args.path).unwrap_or_default();
        let history_lines: Vec<usize> = all_snapshots.iter().map(|s| s.total_lines.code).collect();
        let history_files: Vec<usize> = all_snapshots.iter().map(|s| s.total_files).collect();

        let dashboard_data = report::dashboard::DashboardData {
            agg: &analysis.agg,
            diff: diff.as_ref(),
            hotspots: &analysis.hotspots,
            dep_summary: &analysis.dep_summary,
            doc_metrics: analysis.doc_metrics.as_ref(),
            ai_result: analysis.ai_result.as_ref(),
            history_lines,
            history_files,
            skipped_files: analysis.skipped_files,
            risk_entries: &analysis.risk_entries,
        };
        if let Err(e) = report::dashboard::render(&dashboard_data, &mut stdout, color) {
            eprintln!("error: failed to render dashboard: {e}");
            process::exit(2);
        }
    }
    let report_dur = t.elapsed();

    if args.verbose {
        let total_dur = total_start.elapsed();
        eprintln!("  scanner:      {:.1}s", scanner_dur.as_secs_f64());
        eprintln!("  metrics:      {:.1}s", metrics_dur.as_secs_f64());
        eprintln!("  dependencies: {:.1}s", deps_dur.as_secs_f64());
        eprintln!("  docs:         {:.1}s", docs_dur.as_secs_f64());
        eprintln!("  AI:           {:.1}s", ai_dur.as_secs_f64());
        eprintln!("  snapshot:     {:.1}s", snapshot_dur.as_secs_f64());
        eprintln!("  report:       {:.1}s", report_dur.as_secs_f64());
        eprintln!("  total:        {:.1}s", total_dur.as_secs_f64());
    }

    // Evaluate health exit code from analysis metrics
    let max_complexity = analysis
        .hotspots
        .iter()
        .map(|(_, f)| f.cyclomatic)
        .max()
        .unwrap_or(0);
    let max_func_lines = analysis
        .hotspots
        .iter()
        .map(|(_, f)| f.line_count)
        .max()
        .unwrap_or(0);
    let health_exit = config.health.evaluate(max_complexity, max_func_lines);
    if health_exit != 0 {
        process::exit(health_exit);
    }
}

/// Run the trend subcommand.
fn run_trend(target_dir: &Path) {
    let snapshots = match snapshot::store::load_all(target_dir) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: failed to load snapshots: {e}");
            process::exit(1);
        }
    };

    if snapshots.is_empty() {
        eprintln!("No snapshots found. Run `repostat` first to create one.");
        return;
    }

    let color = report::color::is_color_enabled();
    let mut stdout = std::io::stdout().lock();
    if let Err(e) = report::trend::render(&snapshots, &mut stdout, color) {
        eprintln!("error: failed to render trends: {e}");
        process::exit(2);
    }
}

/// Run the list subcommand.
fn run_list() {
    let color = report::color::is_color_enabled();
    let mut stdout = std::io::stdout().lock();
    if let Err(e) = snapshot::index::render_list(&mut stdout, color) {
        eprintln!("error: failed to list repos: {e}");
        process::exit(2);
    }
}

/// Default config template with commented defaults.
const DEFAULT_CONFIG: &str = r#"# repostat configuration
# See https://github.com/wiseyoda/repo-analysis for documentation.

# Exclude files/directories from analysis (glob patterns).
# These are applied on top of .gitignore and built-in heuristics.
# [exclude]
# patterns = ["generated/**", "vendor/**"]

# Force-include files that would otherwise be excluded.
# [include]
# patterns = ["vendor/important/**"]

# Health score thresholds for exit codes.
# Exit 0 = healthy, 10 = warning, 20 = critical.
# [health]
# warn_complexity = 25
# crit_complexity = 50
# warn_function_lines = 60
# crit_function_lines = 100
"#;

/// Run the init subcommand.
fn run_init(force: bool) {
    let path = std::path::Path::new(".repostat.toml");

    if path.exists() && !force {
        eprintln!("error: .repostat.toml already exists. Use --force to overwrite.");
        process::exit(1);
    }

    if let Err(e) = std::fs::write(path, DEFAULT_CONFIG) {
        eprintln!("error: failed to write .repostat.toml: {e}");
        process::exit(1);
    }

    eprintln!("Created .repostat.toml with default settings.");
}

/// Run the diff subcommand — analyze only changed files.
fn run_diff(args: &cli::DiffArgs) {
    let changed = match metrics::git_history::changed_files(&args.path, &args.revspec) {
        Ok(files) => files,
        Err(e) => {
            eprintln!("error: {e}");
            process::exit(1);
        }
    };

    if changed.is_empty() {
        eprintln!("No files changed in {}..HEAD", args.revspec);
        return;
    }

    let config = match config::Config::load(&args.path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("error: {e}");
            process::exit(1);
        }
    };

    let files = match scanner::scan(&args.path, &config) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("error: {e}");
            process::exit(1);
        }
    };

    // Filter to only changed files
    let changed_set: std::collections::HashSet<_> = changed.iter().map(|p| p.as_path()).collect();

    let analyzed: Vec<_> = files
        .iter()
        .filter(|f| !f.is_minified && !f.is_generated)
        .filter(|f| changed_set.contains(f.path.as_path()))
        .filter_map(|f| {
            let content = match std::fs::read_to_string(&f.path) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("warning: skipped {}: {e}", f.path.display());
                    return None;
                }
            };
            let lines = metrics::loc::count_lines(&content, f.language);
            let functions = f
                .language
                .and_then(|lang| metrics::complexity::extract_functions(&content, lang))
                .unwrap_or_default();
            Some((f, lines, functions))
        })
        .collect();

    eprintln!(
        "Analyzing {} changed file{} ({}..HEAD)",
        analyzed.len(),
        if analyzed.len() == 1 { "" } else { "s" },
        args.revspec,
    );

    let file_results: Vec<_> = analyzed
        .iter()
        .map(|(f, lines, _)| metrics::aggregate::FileResult {
            language: f.language,
            lines: *lines,
        })
        .collect();

    let agg = metrics::aggregate::aggregate(&file_results);

    let color = report::color::is_color_enabled();
    let mut stdout = std::io::stdout().lock();

    let dep_summary = metrics::dependencies::DependencySummary::default();
    let dashboard_data = report::dashboard::DashboardData {
        agg: &agg,
        diff: None,
        hotspots: &[],
        dep_summary: &dep_summary,
        doc_metrics: None,
        ai_result: None,
        history_lines: vec![],
        history_files: vec![],
        skipped_files: 0,
        risk_entries: &[],
    };
    if let Err(e) = report::dashboard::render(&dashboard_data, &mut stdout, color) {
        eprintln!("error: failed to render dashboard: {e}");
        process::exit(2);
    }
}
