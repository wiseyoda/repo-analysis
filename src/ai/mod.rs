//! AI-augmented analysis: Claude CLI integration, skill files, response parsing.

pub(crate) mod claude;
pub(crate) mod schema;
pub(crate) mod skills;

use std::path::Path;

use schema::AiAnalysisResult;

/// Check if AI analysis is disabled via environment variable.
///
/// Returns `true` if `REPOSTAT_SKIP_AI` is set to "1" or "true" (case-insensitive).
fn is_ai_disabled() -> bool {
    std::env::var("REPOSTAT_SKIP_AI")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

/// Run AI analysis on the target directory.
///
/// Returns `None` if Claude CLI is unavailable, AI is disabled via
/// `REPOSTAT_SKIP_AI` env var, or all skills fail.
/// Individual skill failures are logged to stderr and skipped.
pub(crate) fn run_ai_analysis(target_dir: &Path) -> Option<AiAnalysisResult> {
    if is_ai_disabled() {
        return None;
    }

    let cli_path = claude::detect_cli()?;

    let skills_dir = skills::ensure_skills_dir();
    let skill_files = match &skills_dir {
        Some(dir) => skills::load_skills(dir),
        None => skills::builtin_skills(),
    };

    let mut result = AiAnalysisResult::default();

    for skill in &skill_files {
        match claude::invoke(&cli_path, target_dir, &skill.prompt) {
            Ok(output) => {
                schema::merge_skill_result(&mut result, &skill.name, &output);
            }
            Err(e) => {
                eprintln!("warning: AI skill '{}' failed: {e}", skill.name);
            }
        }
    }

    if result.is_empty() {
        None
    } else {
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: env var tests must use unsafe in Rust 2024 edition.
    // These tests are serial-unsafe but acceptable for test code.

    #[test]
    fn is_ai_disabled_returns_true_for_1() {
        unsafe { std::env::set_var("REPOSTAT_SKIP_AI", "1") };
        assert!(is_ai_disabled());
        unsafe { std::env::remove_var("REPOSTAT_SKIP_AI") };
    }

    #[test]
    fn is_ai_disabled_returns_true_for_true() {
        unsafe { std::env::set_var("REPOSTAT_SKIP_AI", "true") };
        assert!(is_ai_disabled());
        unsafe { std::env::remove_var("REPOSTAT_SKIP_AI") };
    }

    #[test]
    fn is_ai_disabled_returns_true_for_true_uppercase() {
        unsafe { std::env::set_var("REPOSTAT_SKIP_AI", "TRUE") };
        assert!(is_ai_disabled());
        unsafe { std::env::remove_var("REPOSTAT_SKIP_AI") };
    }

    #[test]
    fn is_ai_disabled_returns_false_for_0() {
        unsafe { std::env::set_var("REPOSTAT_SKIP_AI", "0") };
        assert!(!is_ai_disabled());
        unsafe { std::env::remove_var("REPOSTAT_SKIP_AI") };
    }

    #[test]
    fn is_ai_disabled_returns_false_when_unset() {
        unsafe { std::env::remove_var("REPOSTAT_SKIP_AI") };
        assert!(!is_ai_disabled());
    }
}
