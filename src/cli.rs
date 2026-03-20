//! Command-line argument definitions.

use std::path::PathBuf;

use clap::Parser;

use crate::errors::RepostatError;

/// Analyze repository complexity, track coding progress, and produce AI-augmented reports.
#[derive(Parser, Debug)]
#[command(name = "repostat", version, about)]
pub(crate) struct Args {
    /// Path to the repository to analyze [default: current directory].
    pub(crate) path: Option<PathBuf>,

    /// Output raw JSON to stdout instead of the dashboard.
    #[arg(long, short)]
    pub(crate) json: bool,

    /// Generate a Markdown report file.
    #[arg(long, short)]
    pub(crate) markdown: bool,
}

/// Parsed and validated CLI arguments.
pub(crate) struct ValidatedArgs {
    /// Validated target directory path.
    pub(crate) path: PathBuf,
    /// Whether to output JSON.
    pub(crate) json: bool,
    /// Whether to output Markdown.
    pub(crate) markdown: bool,
}

/// Parse command-line arguments and validate the target path.
///
/// Defaults to the current working directory if no path is provided.
pub(crate) fn parse_and_validate() -> anyhow::Result<ValidatedArgs> {
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

    Ok(ValidatedArgs {
        path,
        json: args.json,
        markdown: args.markdown,
    })
}
