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

    let _config = match config::Config::load(&path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("error: {e}");
            process::exit(1);
        }
    };
}
