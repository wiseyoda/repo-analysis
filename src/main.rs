//! repostat — Analyze repository complexity and track coding progress.

use std::process;

mod cli;
mod config;
mod errors;
mod metrics;
mod report;
mod scanner;
mod snapshot;

fn main() {
    let path = match cli::parse_and_validate() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("error: {e}");
            process::exit(1);
        }
    };

    let config = match config::Config::load(&path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("error: {e}");
            process::exit(1);
        }
    };

    let files = match scanner::scan(&path, &config) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("error: {e}");
            process::exit(1);
        }
    };

    let file_results: Vec<_> = files
        .iter()
        .filter(|f| !f.is_minified && !f.is_generated)
        .filter_map(|f| {
            let content = std::fs::read_to_string(&f.path).ok()?;
            let lines = metrics::loc::count_lines(&content, f.language);
            Some(metrics::aggregate::FileResult {
                language: f.language,
                lines,
            })
        })
        .collect();

    let agg = metrics::aggregate::aggregate(&file_results);

    let previous = snapshot::store::load_latest(&path).ok().flatten();

    let snap = snapshot::Snapshot::from_aggregate(&agg, snapshot::current_git_sha());
    if let Err(e) = snapshot::store::write_snapshot(&path, &snap) {
        eprintln!("warning: failed to write snapshot: {e}");
    }

    let diff = previous.map(|prev| snapshot::diff::diff(&snap, &prev));

    let mut stdout = std::io::stdout().lock();
    if let Err(e) = report::dashboard::render(&agg, diff.as_ref(), &mut stdout) {
        eprintln!("error: failed to render dashboard: {e}");
        process::exit(2);
    }
}
