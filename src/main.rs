//! repostat — Analyze repository complexity and track coding progress.

use std::process;

use rayon::prelude::*;

mod cli;
mod config;
mod errors;
mod metrics;
mod report;
mod scanner;
mod snapshot;

fn main() {
    let args = match cli::parse_and_validate() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("error: {e}");
            process::exit(1);
        }
    };

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

    let file_results: Vec<_> = files
        .par_iter()
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

    let previous = snapshot::store::load_latest(&args.path).ok().flatten();

    let snap = snapshot::Snapshot::from_aggregate(&agg, snapshot::current_git_sha());
    if let Err(e) = snapshot::store::write_snapshot(&args.path, &snap) {
        eprintln!("warning: failed to write snapshot: {e}");
    }

    let diff = previous.map(|prev| snapshot::diff::diff(&snap, &prev));

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
        if let Err(e) = report::markdown::render(&agg, diff.as_ref(), &mut stdout) {
            eprintln!("error: failed to render markdown: {e}");
            process::exit(2);
        }
    } else {
        let color = report::color::is_color_enabled();
        let mut stdout = std::io::stdout().lock();
        if let Err(e) = report::dashboard::render(&agg, diff.as_ref(), &mut stdout, color) {
            eprintln!("error: failed to render dashboard: {e}");
            process::exit(2);
        }
    }
}
