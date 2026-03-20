//! Skill file loading and default skill definitions.

use std::path::{Path, PathBuf};

/// A loaded skill file with name and prompt content.
#[derive(Debug, Clone)]
pub(crate) struct SkillFile {
    /// Skill name (e.g., "architecture").
    pub(crate) name: String,
    /// Full prompt content.
    pub(crate) prompt: String,
}

/// Ensure the skills directory exists and contains default skill files.
///
/// Returns the path to the skills directory, or `None` if it cannot be created.
pub(crate) fn ensure_skills_dir() -> Option<PathBuf> {
    let dir = skills_dir()?;

    if !dir.exists() {
        if std::fs::create_dir_all(&dir).is_err() {
            eprintln!(
                "warning: could not create skills directory: {}",
                dir.display()
            );
            return None;
        }
        write_default_skills(&dir);
    }

    Some(dir)
}

/// Get the default skills directory path.
fn skills_dir() -> Option<PathBuf> {
    dirs::home_dir().map(|h| h.join(".repostat").join("skills"))
}

/// Load all skill files from a directory.
///
/// Reads all `.md` files and returns them as `SkillFile` entries.
pub(crate) fn load_skills(dir: &Path) -> Vec<SkillFile> {
    let mut skills = Vec::new();

    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return builtin_skills(),
    };

    for entry in entries.filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }
        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();
        if let Ok(prompt) = std::fs::read_to_string(&path) {
            if !prompt.trim().is_empty() {
                skills.push(SkillFile { name, prompt });
            }
        }
    }

    if skills.is_empty() {
        builtin_skills()
    } else {
        skills
    }
}

/// Return built-in skill definitions (used when disk skills are unavailable).
pub(crate) fn builtin_skills() -> Vec<SkillFile> {
    vec![
        SkillFile {
            name: "architecture".to_string(),
            prompt: ARCHITECTURE_PROMPT.to_string(),
        },
        SkillFile {
            name: "features".to_string(),
            prompt: FEATURES_PROMPT.to_string(),
        },
        SkillFile {
            name: "quality".to_string(),
            prompt: QUALITY_PROMPT.to_string(),
        },
        SkillFile {
            name: "effort".to_string(),
            prompt: EFFORT_PROMPT.to_string(),
        },
        SkillFile {
            name: "stale-docs".to_string(),
            prompt: STALE_DOCS_PROMPT.to_string(),
        },
        SkillFile {
            name: "doc-quality".to_string(),
            prompt: DOC_QUALITY_PROMPT.to_string(),
        },
    ]
}

/// Write default skill files to the given directory.
fn write_default_skills(dir: &Path) {
    let defaults = [
        ("architecture.md", ARCHITECTURE_PROMPT),
        ("features.md", FEATURES_PROMPT),
        ("quality.md", QUALITY_PROMPT),
        ("effort.md", EFFORT_PROMPT),
        ("stale-docs.md", STALE_DOCS_PROMPT),
        ("doc-quality.md", DOC_QUALITY_PROMPT),
    ];

    for (filename, content) in &defaults {
        let path = dir.join(filename);
        if let Err(e) = std::fs::write(&path, content) {
            eprintln!("warning: could not write skill file {filename}: {e}");
        }
    }
}

const ARCHITECTURE_PROMPT: &str = r#"Analyze this codebase and provide an architecture summary.

Respond with ONLY a JSON object (no markdown, no explanation):

{
  "description": "One paragraph describing the project's purpose and architecture",
  "patterns": ["list", "of", "design patterns", "used"],
  "design_approach": "A short phrase like 'layered', 'microservices', 'monolith', 'event-driven'"
}
"#;

const FEATURES_PROMPT: &str = r#"Analyze this codebase and provide a feature inventory.

Respond with ONLY a JSON object (no markdown, no explanation):

{
  "features": [
    {
      "name": "Feature name",
      "status": "complete or wip or planned",
      "description": "Brief description"
    }
  ]
}
"#;

const QUALITY_PROMPT: &str = r#"Review this codebase for quality issues.

Respond with ONLY a JSON object (no markdown, no explanation):

{
  "issues": [
    {
      "category": "anti-pattern or dead-code or inconsistency",
      "description": "What the issue is",
      "file": "path/to/file.rs or null"
    }
  ],
  "overall_score": "good or fair or poor"
}
"#;

const EFFORT_PROMPT: &str = r#"Estimate the development effort for this codebase.

Respond with ONLY a JSON object (no markdown, no explanation):

{
  "existing_hours": 123.0,
  "remaining_hours": 45.0,
  "summary": "Brief effort summary"
}

existing_hours: approximate developer-hours already invested.
remaining_hours: approximate hours to reach a 1.0 release (null if unknown).
"#;

const STALE_DOCS_PROMPT: &str = r#"Check this codebase for stale documentation.

Look for docs that reference functions, files, or APIs that no longer exist.

Respond with ONLY a JSON object (no markdown, no explanation):

{
  "stale_files": [
    {
      "file": "path/to/doc.md",
      "reason": "References function X which was removed"
    }
  ]
}

Return an empty stale_files array if all docs are up to date.
"#;

const DOC_QUALITY_PROMPT: &str = r#"Score the documentation quality of this codebase.

Respond with ONLY a JSON object (no markdown, no explanation):

{
  "overall_score": "good or fair or poor",
  "files": [
    {
      "file": "path/to/doc.md",
      "score": "good or fair or poor",
      "feedback": "Brief feedback"
    }
  ]
}
"#;

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::TempDir;

    use super::*;

    #[test]
    fn builtin_skills_returns_all_six() {
        let skills = builtin_skills();
        assert_eq!(skills.len(), 6);

        let names: Vec<&str> = skills.iter().map(|s| s.name.as_str()).collect();
        assert!(names.contains(&"architecture"));
        assert!(names.contains(&"features"));
        assert!(names.contains(&"quality"));
        assert!(names.contains(&"effort"));
        assert!(names.contains(&"stale-docs"));
        assert!(names.contains(&"doc-quality"));
    }

    #[test]
    fn builtin_skills_have_nonempty_prompts() {
        for skill in builtin_skills() {
            assert!(
                !skill.prompt.trim().is_empty(),
                "skill '{}' has empty prompt",
                skill.name,
            );
        }
    }

    #[test]
    fn load_skills_from_directory() {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join("test-skill.md"), "Analyze this code.").unwrap();

        let skills = load_skills(dir.path());
        assert_eq!(skills.len(), 1);
        assert_eq!(skills[0].name, "test-skill");
        assert_eq!(skills[0].prompt, "Analyze this code.");
    }

    #[test]
    fn load_skills_ignores_non_md_files() {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join("skill.md"), "prompt content").unwrap();
        fs::write(dir.path().join("notes.txt"), "not a skill").unwrap();

        let skills = load_skills(dir.path());
        assert_eq!(skills.len(), 1);
        assert_eq!(skills[0].name, "skill");
    }

    #[test]
    fn load_skills_falls_back_to_builtins_for_empty_dir() {
        let dir = TempDir::new().unwrap();
        let skills = load_skills(dir.path());
        assert_eq!(skills.len(), 6, "should fall back to builtins");
    }

    #[test]
    fn load_skills_skips_empty_files() {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join("empty.md"), "").unwrap();
        fs::write(dir.path().join("whitespace.md"), "   \n  \n").unwrap();

        let skills = load_skills(dir.path());
        // Both are empty after trim, so falls back to builtins
        assert_eq!(skills.len(), 6);
    }

    #[test]
    fn write_default_skills_creates_files() {
        let dir = TempDir::new().unwrap();
        write_default_skills(dir.path());

        let files: Vec<String> = fs::read_dir(dir.path())
            .unwrap()
            .filter_map(|e| e.ok())
            .map(|e| e.file_name().to_string_lossy().to_string())
            .collect();

        assert!(files.contains(&"architecture.md".to_string()));
        assert!(files.contains(&"features.md".to_string()));
        assert!(files.contains(&"quality.md".to_string()));
        assert!(files.contains(&"effort.md".to_string()));
        assert!(files.contains(&"stale-docs.md".to_string()));
        assert!(files.contains(&"doc-quality.md".to_string()));
        assert_eq!(files.len(), 6);
    }
}
