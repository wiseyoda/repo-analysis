//! Application error types.

use std::path::PathBuf;

/// Errors that can occur during repostat operations.
#[derive(Debug, thiserror::Error)]
pub(crate) enum RepostatError {
    /// The specified path does not exist.
    #[error("path does not exist: {0}")]
    PathNotFound(PathBuf),

    /// The specified path is not a directory.
    #[error("path is not a directory: {0}")]
    NotADirectory(PathBuf),
}
