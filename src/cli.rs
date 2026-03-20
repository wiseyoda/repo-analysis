//! Command-line argument definitions.

use std::path::PathBuf;

use clap::Parser;

use crate::errors::RepostatError;

/// Analyze repository complexity, track coding progress, and produce AI-augmented reports.
#[derive(Parser, Debug)]
#[command(name = "repostat", version, about)]
pub(crate) struct Args {
    /// Path to the repository to analyze [default: current directory].
    pub path: Option<PathBuf>,
}

/// Parse command-line arguments and validate the target path.
///
/// Defaults to the current working directory if no path is provided.
/// Returns the validated, canonicalized path.
pub(crate) fn parse_and_validate() -> anyhow::Result<PathBuf> {
    let args = Args::parse();

    let path = match args.path {
        Some(p) => p,
        None => std::env::current_dir()?,
    };

    if !path.exists() {
        return Err(RepostatError::PathNotFound(path).into());
    }

    if !path.is_dir() {
        return Err(RepostatError::NotADirectory(path).into());
    }

    Ok(path)
}
