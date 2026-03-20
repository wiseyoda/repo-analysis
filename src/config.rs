//! Configuration file parsing and validation for `.repostat.toml`.

use std::path::Path;

use serde::Deserialize;

/// The config file name looked for in the target directory.
const CONFIG_FILE_NAME: &str = ".repostat.toml";

/// Project-level configuration loaded from `.repostat.toml`.
#[derive(Debug, Default)]
pub(crate) struct Config {
    /// Glob patterns for files/directories to exclude from analysis.
    pub(crate) exclude_patterns: Vec<String>,
    /// Glob patterns for files/directories to force-include (overrides exclude).
    pub(crate) include_patterns: Vec<String>,
}

/// Raw TOML structure — deserialized then converted to `Config`.
#[derive(Deserialize)]
struct RawConfig {
    exclude: Option<PatternList>,
    include: Option<PatternList>,
}

/// A table containing a list of glob patterns.
#[derive(Deserialize)]
struct PatternList {
    patterns: Vec<String>,
}

/// Errors that can occur when loading configuration.
#[derive(Debug, thiserror::Error)]
pub(crate) enum ConfigError {
    /// Failed to read the config file.
    #[error("failed to read config at {path}: {source}")]
    ReadFailed {
        /// Path to the config file.
        path: std::path::PathBuf,
        /// Underlying I/O error.
        source: std::io::Error,
    },

    /// Failed to parse the config file as TOML.
    #[error("failed to parse config at {path}: {source}")]
    ParseFailed {
        /// Path to the config file.
        path: std::path::PathBuf,
        /// Underlying TOML parse error.
        source: toml::de::Error,
    },
}

impl Config {
    /// Load configuration from `.repostat.toml` in the given directory.
    ///
    /// Returns default config if the file does not exist.
    pub(crate) fn load(dir: &Path) -> Result<Self, ConfigError> {
        let path = dir.join(CONFIG_FILE_NAME);

        if !path.exists() {
            return Ok(Self::default());
        }

        let content = std::fs::read_to_string(&path).map_err(|e| ConfigError::ReadFailed {
            path: path.clone(),
            source: e,
        })?;

        if content.trim().is_empty() {
            return Ok(Self::default());
        }

        let raw: RawConfig =
            toml::from_str(&content).map_err(|e| ConfigError::ParseFailed { path, source: e })?;

        Ok(Self {
            exclude_patterns: raw.exclude.map(|p| p.patterns).unwrap_or_default(),
            include_patterns: raw.include.map(|p| p.patterns).unwrap_or_default(),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::TempDir;

    use super::*;

    #[test]
    fn returns_default_when_no_config_file_exists() {
        let dir = TempDir::new().unwrap();
        let config = Config::load(dir.path()).unwrap();
        assert!(config.exclude_patterns.is_empty());
        assert!(config.include_patterns.is_empty());
    }

    #[test]
    fn parses_exclude_patterns() {
        let dir = TempDir::new().unwrap();
        fs::write(
            dir.path().join(".repostat.toml"),
            r#"
[exclude]
patterns = ["*.generated.*", "vendor/**"]
"#,
        )
        .unwrap();

        let config = Config::load(dir.path()).unwrap();
        assert_eq!(config.exclude_patterns, vec!["*.generated.*", "vendor/**"]);
        assert!(config.include_patterns.is_empty());
    }

    #[test]
    fn parses_include_patterns() {
        let dir = TempDir::new().unwrap();
        fs::write(
            dir.path().join(".repostat.toml"),
            r#"
[include]
patterns = ["vendor/important/**"]
"#,
        )
        .unwrap();

        let config = Config::load(dir.path()).unwrap();
        assert!(config.exclude_patterns.is_empty());
        assert_eq!(config.include_patterns, vec!["vendor/important/**"]);
    }

    #[test]
    fn parses_both_exclude_and_include() {
        let dir = TempDir::new().unwrap();
        fs::write(
            dir.path().join(".repostat.toml"),
            r#"
[exclude]
patterns = ["dist/**"]

[include]
patterns = ["dist/keep/**"]
"#,
        )
        .unwrap();

        let config = Config::load(dir.path()).unwrap();
        assert_eq!(config.exclude_patterns, vec!["dist/**"]);
        assert_eq!(config.include_patterns, vec!["dist/keep/**"]);
    }

    #[test]
    fn handles_empty_config_file() {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join(".repostat.toml"), "").unwrap();

        let config = Config::load(dir.path()).unwrap();
        assert!(config.exclude_patterns.is_empty());
        assert!(config.include_patterns.is_empty());
    }

    #[test]
    fn returns_error_for_malformed_toml() {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join(".repostat.toml"), "not valid [[ toml").unwrap();

        let result = Config::load(dir.path());
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(
            err.contains("failed to parse"),
            "error should mention parse failure, got: {err}"
        );
    }

    #[test]
    fn ignores_unknown_keys() {
        let dir = TempDir::new().unwrap();
        fs::write(
            dir.path().join(".repostat.toml"),
            r#"
[exclude]
patterns = ["build/**"]

[some_future_section]
key = "value"
"#,
        )
        .unwrap();

        let config = Config::load(dir.path()).unwrap();
        assert_eq!(config.exclude_patterns, vec!["build/**"]);
    }
}
