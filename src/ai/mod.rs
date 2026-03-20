//! AI-augmented analysis: Claude CLI integration, skill files, response parsing.

pub(crate) mod claude;
pub(crate) mod schema;
pub(crate) mod skills;

use std::path::Path;

use schema::AiAnalysisResult;

/// Run AI analysis on the target directory.
///
/// Returns `None` if Claude CLI is unavailable or all skills fail.
/// Individual skill failures are logged to stderr and skipped.
pub(crate) fn run_ai_analysis(target_dir: &Path) -> Option<AiAnalysisResult> {
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
