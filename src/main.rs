//! repostat — Analyze repository complexity and track coding progress.

mod cli;
mod config;
mod errors;
mod metrics;
mod report;
mod scanner;
mod snapshot;

fn main() -> anyhow::Result<()> {
    let _args = cli::parse();
    Ok(())
}
