//! Command-line argument definitions.

use std::path::PathBuf;

use clap::Parser;

/// Analyze repository complexity, track coding progress, and produce AI-augmented reports.
#[derive(Parser, Debug)]
#[command(name = "repostat", version, about)]
pub(crate) struct Args {
    /// Path to the repository to analyze.
    pub path: Option<PathBuf>,
}

/// Parse command-line arguments.
pub(crate) fn parse() -> Args {
    Args::parse()
}
