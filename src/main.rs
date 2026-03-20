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
    match cli::parse_and_validate() {
        Ok(_path) => {
            // Analysis pipeline will go here
        }
        Err(e) => {
            eprintln!("error: {e}");
            process::exit(1);
        }
    }
}
