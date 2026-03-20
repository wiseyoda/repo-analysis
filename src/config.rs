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
    /// Health score thresholds for exit codes.
    pub(crate) health: HealthThresholds,
}

/// Thresholds for health score exit codes.
#[derive(Debug, Clone)]
pub(crate) struct HealthThresholds {
    /// Warning threshold for max cyclomatic complexity.
    pub(crate) warn_complexity: usize,
    /// Critical threshold for max cyclomatic complexity.
    pub(crate) crit_complexity: usize,
    /// Warning threshold for max function line count.
    pub(crate) warn_function_lines: usize,
    /// Critical threshold for max function line count.
    pub(crate) crit_function_lines: usize,
}

impl Default for HealthThresholds {
    fn default() -> Self {
        Self {
            warn_complexity: 25,
            crit_complexity: 50,
            warn_function_lines: 60,
            crit_function_lines: 100,
        }
    }
}

/// Exit codes for health assessment.
pub(crate) const EXIT_HEALTHY: i32 = 0;
/// Exit code for health warning.
pub(crate) const EXIT_WARNING: i32 = 10;
/// Exit code for health critical.
pub(crate) const EXIT_CRITICAL: i32 = 20;

impl HealthThresholds {
    /// Evaluate health status from max complexity and function line count.
    ///
    /// Returns the exit code: 0 (healthy), 10 (warning), or 20 (critical).
    pub(crate) fn evaluate(&self, max_complexity: usize, max_function_lines: usize) -> i32 {
        if max_complexity > self.crit_complexity || max_function_lines > self.crit_function_lines {
            return EXIT_CRITICAL;
        }
        if max_complexity > self.warn_complexity || max_function_lines > self.warn_function_lines {
            return EXIT_WARNING;
        }
        EXIT_HEALTHY
    }
}

/// Raw TOML structure — deserialized then converted to `Config`.
#[derive(Deserialize)]
struct RawConfig {
    exclude: Option<PatternList>,
    include: Option<PatternList>,
    health: Option<RawHealthThresholds>,
}

/// Raw health thresholds from TOML.
#[derive(Deserialize)]
struct RawHealthThresholds {
    warn_complexity: Option<usize>,
    crit_complexity: Option<usize>,
    warn_function_lines: Option<usize>,
    crit_function_lines: Option<usize>,
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

        let defaults = HealthThresholds::default();
        let health = match raw.health {
            Some(h) => HealthThresholds {
                warn_complexity: h.warn_complexity.unwrap_or(defaults.warn_complexity),
                crit_complexity: h.crit_complexity.unwrap_or(defaults.crit_complexity),
                warn_function_lines: h
                    .warn_function_lines
                    .unwrap_or(defaults.warn_function_lines),
                crit_function_lines: h
                    .crit_function_lines
                    .unwrap_or(defaults.crit_function_lines),
            },
            None => defaults,
        };

        Ok(Self {
            exclude_patterns: raw.exclude.map(|p| p.patterns).unwrap_or_default(),
            include_patterns: raw.include.map(|p| p.patterns).unwrap_or_default(),
            health,
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

    #[test]
    fn health_defaults_applied_without_section() {
        let dir = TempDir::new().expect("tmpdir");
        let config = Config::load(dir.path()).expect("load");
        assert_eq!(config.health.warn_complexity, 25);
        assert_eq!(config.health.crit_complexity, 50);
        assert_eq!(config.health.warn_function_lines, 60);
        assert_eq!(config.health.crit_function_lines, 100);
    }

    #[test]
    fn health_config_parsed_from_toml() {
        let dir = TempDir::new().expect("tmpdir");
        fs::write(
            dir.path().join(".repostat.toml"),
            r#"
[health]
warn_complexity = 10
crit_complexity = 30
"#,
        )
        .expect("write");

        let config = Config::load(dir.path()).expect("load");
        assert_eq!(config.health.warn_complexity, 10);
        assert_eq!(config.health.crit_complexity, 30);
        // Unset fields use defaults
        assert_eq!(config.health.warn_function_lines, 60);
        assert_eq!(config.health.crit_function_lines, 100);
    }

    #[test]
    fn evaluate_healthy() {
        let h = HealthThresholds::default();
        assert_eq!(h.evaluate(10, 30), EXIT_HEALTHY);
    }

    #[test]
    fn evaluate_warning_complexity() {
        let h = HealthThresholds::default();
        assert_eq!(h.evaluate(30, 30), EXIT_WARNING);
    }

    #[test]
    fn evaluate_critical_complexity() {
        let h = HealthThresholds::default();
        assert_eq!(h.evaluate(55, 30), EXIT_CRITICAL);
    }

    #[test]
    fn evaluate_warning_function_lines() {
        let h = HealthThresholds::default();
        assert_eq!(h.evaluate(10, 70), EXIT_WARNING);
    }

    #[test]
    fn evaluate_critical_function_lines() {
        let h = HealthThresholds::default();
        assert_eq!(h.evaluate(10, 110), EXIT_CRITICAL);
    }

    #[test]
    fn evaluate_critical_wins_over_warning() {
        let h = HealthThresholds::default();
        // Both complexity warning and function critical
        assert_eq!(h.evaluate(30, 110), EXIT_CRITICAL);
    }
}
