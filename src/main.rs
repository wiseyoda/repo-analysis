//! repostat — Analyze repository complexity and track coding progress.

use std::path::Path;
use std::process;

mod ai;
mod analysis;
mod analyze_command;
mod cli;
mod config;
mod errors;
mod metrics;
mod report;
mod result;
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
        cli::ValidatedCommand::Analyze(args) => analyze_command::run(&args),
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
        cli::ValidatedCommand::Extension(args) => run_extension(&args),
    }
}

/// Run the no-write Engine tool adapter.
fn run_extension(args: &cli::ExtensionArgs) {
    let config = match config::Config::load(&args.path) {
        Ok(config) => config,
        Err(error) => exit_with_error(&error),
    };
    let completed = match analysis::analyze(&args.path, &config) {
        Ok(completed) => completed,
        Err(error) => exit_with_error(&error),
    };
    let snapshot = snapshot::Snapshot::from_analysis(&completed.result);
    match serde_json::to_string_pretty(&result::MetricsResult::new(&args.path, &snapshot)) {
        Ok(json) => println!("{json}"),
        Err(error) => exit_with_report_error("serialize extension result", &error),
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
# See https://github.com/wiseyoda/ai-mux-repostat for documentation.

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
