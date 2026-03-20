//! Command-line argument definitions with subcommand support.

use std::path::PathBuf;

use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::Shell;

use crate::errors::RepostatError;

/// Analyze repository complexity, track coding progress, and produce AI-augmented reports.
#[derive(Parser, Debug)]
#[command(name = "repostat", version, about)]
pub(crate) struct Args {
    /// Subcommand to run (default: analyze).
    #[command(subcommand)]
    pub(crate) command: Option<Command>,

    /// Path to the repository to analyze [default: current directory].
    pub(crate) path: Option<PathBuf>,

    /// Output raw JSON to stdout instead of the dashboard.
    #[arg(long, short)]
    pub(crate) json: bool,

    /// Generate a Markdown report file.
    #[arg(long, short)]
    pub(crate) markdown: bool,
}

/// Available subcommands.
#[derive(Subcommand, Debug)]
pub(crate) enum Command {
    /// Show sparkline trends across all snapshots.
    Trend {
        /// Path to the repository [default: current directory].
        path: Option<PathBuf>,
    },
    /// List all tracked repositories.
    List,
    /// Generate shell completions.
    Completions {
        /// Shell to generate completions for.
        shell: Shell,
    },
    /// Generate a man page.
    Manpage,
}

/// Parsed and validated CLI arguments.
pub(crate) enum ValidatedCommand {
    /// Analyze a repository (default behavior).
    Analyze(AnalyzeArgs),
    /// Show trends.
    Trend(TrendArgs),
    /// List tracked repos.
    List,
    /// Generate shell completions.
    Completions(Shell),
    /// Generate man page.
    Manpage,
}

/// Arguments for the analyze (default) subcommand.
pub(crate) struct AnalyzeArgs {
    /// Validated target directory path.
    pub(crate) path: PathBuf,
    /// Whether to output JSON.
    pub(crate) json: bool,
    /// Whether to output Markdown.
    pub(crate) markdown: bool,
}

/// Arguments for the trend subcommand.
pub(crate) struct TrendArgs {
    /// Validated target directory path.
    pub(crate) path: PathBuf,
}

/// Parse command-line arguments and validate.
pub(crate) fn parse_and_validate() -> anyhow::Result<ValidatedCommand> {
    let args = Args::parse();

    match args.command {
        Some(Command::Trend { path }) => {
            let path = resolve_path(path)?;
            Ok(ValidatedCommand::Trend(TrendArgs { path }))
        }
        Some(Command::List) => Ok(ValidatedCommand::List),
        Some(Command::Completions { shell }) => Ok(ValidatedCommand::Completions(shell)),
        Some(Command::Manpage) => Ok(ValidatedCommand::Manpage),
        None => {
            let path = resolve_path(args.path)?;
            Ok(ValidatedCommand::Analyze(AnalyzeArgs {
                path,
                json: args.json,
                markdown: args.markdown,
            }))
        }
    }
}

/// Generate shell completions to stdout.
pub(crate) fn generate_completions(shell: Shell) {
    let mut cmd = Args::command();
    clap_complete::generate(shell, &mut cmd, "repostat", &mut std::io::stdout());
}

/// Generate a man page to stdout.
pub(crate) fn generate_manpage() -> anyhow::Result<()> {
    let cmd = Args::command();
    let man = clap_mangen::Man::new(cmd);
    man.render(&mut std::io::stdout())?;
    Ok(())
}

/// Resolve and validate a path argument.
fn resolve_path(path: Option<PathBuf>) -> anyhow::Result<PathBuf> {
    let path = match path {
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
