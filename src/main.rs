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

    let _file_metrics: Vec<_> = files
        .iter()
        .filter_map(|f| {
            let content = std::fs::read_to_string(&f.path).ok()?;
            Some(metrics::loc::count_lines(&content, f.language))
        })
        .collect();
}
