//! AI response parsing: lenient JSON extraction with typed defaults.

use serde::{Deserialize, Serialize};

/// Aggregate AI analysis result from all skills.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub(crate) struct AiAnalysisResult {
    /// Architecture summary.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) architecture: Option<ArchitectureSummary>,
    /// Feature inventory.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) features: Option<FeatureInventory>,
    /// Code quality review.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) quality: Option<QualityReview>,
    /// Effort estimation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) effort: Option<EffortEstimate>,
    /// Stale documentation detection.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) stale_docs: Option<StaleDocs>,
    /// Documentation quality scoring.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) doc_quality: Option<DocQuality>,
}

impl AiAnalysisResult {
    /// Check if all fields are None (no results).
    pub(crate) fn is_empty(&self) -> bool {
        self.architecture.is_none()
            && self.features.is_none()
            && self.quality.is_none()
            && self.effort.is_none()
            && self.stale_docs.is_none()
            && self.doc_quality.is_none()
    }
}

/// Architecture summary from AI analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ArchitectureSummary {
    /// High-level project description.
    #[serde(default)]
    pub(crate) description: String,
    /// Design patterns identified.
    #[serde(default)]
    pub(crate) patterns: Vec<String>,
    /// Overall design approach.
    #[serde(default)]
    pub(crate) design_approach: String,
}

/// Feature inventory from AI analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct FeatureInventory {
    /// List of identified features.
    #[serde(default)]
    pub(crate) features: Vec<Feature>,
}

/// A single feature identified by AI.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Feature {
    /// Feature name.
    #[serde(default)]
    pub(crate) name: String,
    /// Status: "complete", "wip", "planned".
    #[serde(default)]
    pub(crate) status: String,
    /// Brief description.
    #[serde(default)]
    pub(crate) description: String,
}

/// Code quality review from AI analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct QualityReview {
    /// Quality issues found.
    #[serde(default)]
    pub(crate) issues: Vec<QualityIssue>,
    /// Overall score: "good", "fair", "poor".
    #[serde(default)]
    pub(crate) overall_score: String,
}

/// A single quality issue.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct QualityIssue {
    /// Category: "anti-pattern", "dead-code", "inconsistency".
    #[serde(default)]
    pub(crate) category: String,
    /// Description of the issue.
    #[serde(default)]
    pub(crate) description: String,
    /// File path (optional).
    #[serde(default)]
    pub(crate) file: Option<String>,
}

/// Effort estimation from AI analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EffortEstimate {
    /// Approximate developer-hours already invested.
    #[serde(default)]
    pub(crate) existing_hours: f64,
    /// Approximate hours remaining to 1.0.
    #[serde(default)]
    pub(crate) remaining_hours: Option<f64>,
    /// Brief summary.
    #[serde(default)]
    pub(crate) summary: String,
}

/// Stale documentation detection from AI analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct StaleDocs {
    /// Files with stale documentation.
    #[serde(default)]
    pub(crate) stale_files: Vec<StaleDocEntry>,
}

/// A single stale documentation entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct StaleDocEntry {
    /// File path.
    #[serde(default)]
    pub(crate) file: String,
    /// Reason it's considered stale.
    #[serde(default)]
    pub(crate) reason: String,
}

/// Documentation quality scoring from AI analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct DocQuality {
    /// Overall quality score.
    #[serde(default)]
    pub(crate) overall_score: String,
    /// Per-file scores.
    #[serde(default)]
    pub(crate) files: Vec<DocQualityEntry>,
}

/// A single documentation quality entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct DocQualityEntry {
    /// File path.
    #[serde(default)]
    pub(crate) file: String,
    /// Quality score.
    #[serde(default)]
    pub(crate) score: String,
    /// Feedback.
    #[serde(default)]
    pub(crate) feedback: String,
}

/// Extract JSON from raw output that may contain markdown code blocks.
///
/// Tries these strategies in order:
/// 1. Parse the entire string as JSON.
/// 2. Extract content from ```json ... ``` blocks.
/// 3. Find the first `{` to last `}` and try parsing that substring.
pub(crate) fn extract_json(raw: &str) -> Option<serde_json::Value> {
    let trimmed = raw.trim();

    // Strategy 1: direct parse
    if let Ok(v) = serde_json::from_str::<serde_json::Value>(trimmed) {
        return Some(v);
    }

    // Strategy 2: extract from markdown code block
    if let Some(json_str) = extract_from_code_block(trimmed) {
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(json_str) {
            return Some(v);
        }
    }

    // Strategy 3: find first { to last }
    if let (Some(start), Some(end)) = (trimmed.find('{'), trimmed.rfind('}')) {
        if start < end {
            if let Ok(v) = serde_json::from_str::<serde_json::Value>(&trimmed[start..=end]) {
                return Some(v);
            }
        }
    }

    None
}

/// Extract content from a markdown JSON code block.
fn extract_from_code_block(text: &str) -> Option<&str> {
    let start_marker = "```json";
    let end_marker = "```";

    let start = text.find(start_marker)?;
    let content_start = start + start_marker.len();
    let rest = &text[content_start..];
    let end = rest.find(end_marker)?;

    Some(rest[..end].trim())
}

/// Merge a skill's parsed output into the aggregate result.
///
/// Uses lenient parsing: if the JSON doesn't match the expected schema
/// for a skill, the result for that skill is simply skipped.
pub(crate) fn merge_skill_result(result: &mut AiAnalysisResult, skill_name: &str, raw: &str) {
    let Some(json) = extract_json(raw) else {
        eprintln!("warning: could not extract JSON from '{skill_name}' response");
        return;
    };

    let json_str = json.to_string();

    match skill_name {
        "architecture" => {
            if let Ok(v) = serde_json::from_str::<ArchitectureSummary>(&json_str) {
                result.architecture = Some(v);
            }
        }
        "features" => {
            if let Ok(v) = serde_json::from_str::<FeatureInventory>(&json_str) {
                result.features = Some(v);
            }
        }
        "quality" => {
            if let Ok(v) = serde_json::from_str::<QualityReview>(&json_str) {
                result.quality = Some(v);
            }
        }
        "effort" => {
            if let Ok(v) = serde_json::from_str::<EffortEstimate>(&json_str) {
                result.effort = Some(v);
            }
        }
        "stale-docs" => {
            if let Ok(v) = serde_json::from_str::<StaleDocs>(&json_str) {
                result.stale_docs = Some(v);
            }
        }
        "doc-quality" => {
            if let Ok(v) = serde_json::from_str::<DocQuality>(&json_str) {
                result.doc_quality = Some(v);
            }
        }
        _ => {
            eprintln!("warning: unknown skill '{skill_name}'");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_json_from_plain_json() {
        let raw = r#"{"description": "A web app", "patterns": ["MVC"]}"#;
        let v = extract_json(raw);
        assert!(v.is_some());
        assert_eq!(
            v.as_ref()
                .and_then(|v| v.get("description"))
                .and_then(|v| v.as_str()),
            Some("A web app")
        );
    }

    #[test]
    fn extract_json_from_code_block() {
        let raw = "Here's the analysis:\n```json\n{\"description\": \"test\"}\n```\nDone.";
        let v = extract_json(raw);
        assert!(v.is_some());
    }

    #[test]
    fn extract_json_from_embedded_braces() {
        let raw = "The architecture is:\n{\"description\": \"test\"}\nThat's it.";
        let v = extract_json(raw);
        assert!(v.is_some());
    }

    #[test]
    fn extract_json_returns_none_for_garbage() {
        assert!(extract_json("not json at all").is_none());
        assert!(extract_json("").is_none());
        assert!(extract_json("}{").is_none());
    }

    #[test]
    fn merge_architecture_result() {
        let mut result = AiAnalysisResult::default();
        let raw = r#"{"description": "A CLI tool", "patterns": ["Builder"], "design_approach": "layered"}"#;
        merge_skill_result(&mut result, "architecture", raw);

        assert!(result.architecture.is_some());
        let arch = result.architecture.as_ref().unwrap();
        assert_eq!(arch.description, "A CLI tool");
        assert_eq!(arch.patterns, vec!["Builder"]);
        assert_eq!(arch.design_approach, "layered");
    }

    #[test]
    fn merge_features_result() {
        let mut result = AiAnalysisResult::default();
        let raw =
            r#"{"features": [{"name": "Auth", "status": "complete", "description": "Login"}]}"#;
        merge_skill_result(&mut result, "features", raw);

        assert!(result.features.is_some());
        assert_eq!(result.features.as_ref().unwrap().features.len(), 1);
    }

    #[test]
    fn merge_quality_result() {
        let mut result = AiAnalysisResult::default();
        let raw = r#"{"issues": [], "overall_score": "good"}"#;
        merge_skill_result(&mut result, "quality", raw);

        assert!(result.quality.is_some());
        assert_eq!(result.quality.as_ref().unwrap().overall_score, "good");
    }

    #[test]
    fn merge_effort_result() {
        let mut result = AiAnalysisResult::default();
        let raw =
            r#"{"existing_hours": 240.0, "remaining_hours": 80.0, "summary": "Moderate effort"}"#;
        merge_skill_result(&mut result, "effort", raw);

        assert!(result.effort.is_some());
        let eff = result.effort.as_ref().unwrap();
        assert!((eff.existing_hours - 240.0).abs() < f64::EPSILON);
    }

    #[test]
    fn merge_stale_docs_result() {
        let mut result = AiAnalysisResult::default();
        let raw = r#"{"stale_files": [{"file": "old.md", "reason": "references removed fn"}]}"#;
        merge_skill_result(&mut result, "stale-docs", raw);

        assert!(result.stale_docs.is_some());
        assert_eq!(result.stale_docs.as_ref().unwrap().stale_files.len(), 1);
    }

    #[test]
    fn merge_doc_quality_result() {
        let mut result = AiAnalysisResult::default();
        let raw = r#"{"overall_score": "fair", "files": []}"#;
        merge_skill_result(&mut result, "doc-quality", raw);

        assert!(result.doc_quality.is_some());
        assert_eq!(result.doc_quality.as_ref().unwrap().overall_score, "fair");
    }

    #[test]
    fn merge_unknown_skill_is_ignored() {
        let mut result = AiAnalysisResult::default();
        merge_skill_result(&mut result, "unknown-skill", r#"{}"#);
        assert!(result.is_empty());
    }

    #[test]
    fn merge_with_invalid_json_skips() {
        let mut result = AiAnalysisResult::default();
        merge_skill_result(&mut result, "architecture", "not json");
        assert!(result.architecture.is_none());
    }

    #[test]
    fn lenient_parsing_uses_defaults_for_missing_fields() {
        let mut result = AiAnalysisResult::default();
        // Partial JSON — missing "patterns" and "design_approach"
        let raw = r#"{"description": "A tool"}"#;
        merge_skill_result(&mut result, "architecture", raw);

        assert!(result.architecture.is_some());
        let arch = result.architecture.as_ref().unwrap();
        assert_eq!(arch.description, "A tool");
        assert!(arch.patterns.is_empty()); // default
        assert!(arch.design_approach.is_empty()); // default
    }

    #[test]
    fn ai_result_is_empty_when_all_none() {
        let result = AiAnalysisResult::default();
        assert!(result.is_empty());
    }

    #[test]
    fn ai_result_not_empty_with_any_field() {
        let mut result = AiAnalysisResult::default();
        result.architecture = Some(ArchitectureSummary {
            description: "test".to_string(),
            patterns: vec![],
            design_approach: String::new(),
        });
        assert!(!result.is_empty());
    }

    #[test]
    fn ai_result_serializes_roundtrip() {
        let result = AiAnalysisResult {
            architecture: Some(ArchitectureSummary {
                description: "A CLI tool".to_string(),
                patterns: vec!["Builder".to_string()],
                design_approach: "layered".to_string(),
            }),
            features: None,
            quality: Some(QualityReview {
                issues: vec![],
                overall_score: "good".to_string(),
            }),
            effort: None,
            stale_docs: None,
            doc_quality: None,
        };

        let json = serde_json::to_string(&result).unwrap();
        let parsed: AiAnalysisResult = serde_json::from_str(&json).unwrap();
        assert!(parsed.architecture.is_some());
        assert!(parsed.features.is_none());
        assert!(parsed.quality.is_some());
    }
}
