//! Metric aggregation: totals, per-language breakdowns, file counts.

use std::collections::BTreeMap;

use crate::scanner::language::Language;

use super::loc::LineMetrics;

/// Per-language aggregated metrics.
#[derive(Debug, Clone, Default)]
pub(crate) struct LanguageMetrics {
    /// Number of files in this language.
    pub(crate) file_count: usize,
    /// Summed line metrics across all files.
    pub(crate) lines: LineMetrics,
}

/// Complete analysis result aggregated from all scanned files.
#[derive(Debug, Clone)]
pub(crate) struct AggregateMetrics {
    /// Total number of files analyzed (excluding generated/minified).
    pub(crate) total_files: usize,
    /// Total line metrics across all files.
    pub(crate) total_lines: LineMetrics,
    /// Per-language breakdown, sorted by language name.
    pub(crate) by_language: BTreeMap<Language, LanguageMetrics>,
    /// Files with no recognized language.
    pub(crate) unknown_language: LanguageMetrics,
}

/// A single file's metrics paired with its language.
pub(crate) struct FileResult {
    /// Detected language (None for unrecognized extensions).
    pub(crate) language: Option<Language>,
    /// Line metrics for this file.
    pub(crate) lines: LineMetrics,
}

/// Aggregate per-file metrics into totals and per-language breakdowns.
pub(crate) fn aggregate(results: &[FileResult]) -> AggregateMetrics {
    let mut total_lines = LineMetrics::default();
    let mut by_language: BTreeMap<Language, LanguageMetrics> = BTreeMap::new();
    let mut unknown_language = LanguageMetrics::default();

    for result in results {
        total_lines = total_lines.add(&result.lines);

        match result.language {
            Some(lang) => {
                let entry = by_language.entry(lang).or_default();
                entry.file_count += 1;
                entry.lines = entry.lines.add(&result.lines);
            }
            None => {
                unknown_language.file_count += 1;
                unknown_language.lines = unknown_language.lines.add(&result.lines);
            }
        }
    }

    AggregateMetrics {
        total_files: results.len(),
        total_lines,
        by_language,
        unknown_language,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aggregates_empty_input() {
        let agg = aggregate(&[]);
        assert_eq!(agg.total_files, 0);
        assert_eq!(agg.total_lines.total_lines, 0);
        assert!(agg.by_language.is_empty());
    }

    #[test]
    fn aggregates_single_file() {
        let results = vec![FileResult {
            language: Some(Language::Rust),
            lines: LineMetrics {
                total_lines: 10,
                code_lines: 7,
                blank_lines: 2,
                comment_lines: 1,
            },
        }];
        let agg = aggregate(&results);
        assert_eq!(agg.total_files, 1);
        assert_eq!(agg.total_lines.code_lines, 7);
        assert_eq!(agg.by_language[&Language::Rust].file_count, 1);
        assert_eq!(agg.by_language[&Language::Rust].lines.code_lines, 7);
    }

    #[test]
    fn aggregates_multiple_languages() {
        let results = vec![
            FileResult {
                language: Some(Language::Rust),
                lines: LineMetrics {
                    total_lines: 10,
                    code_lines: 8,
                    blank_lines: 1,
                    comment_lines: 1,
                },
            },
            FileResult {
                language: Some(Language::Python),
                lines: LineMetrics {
                    total_lines: 20,
                    code_lines: 15,
                    blank_lines: 3,
                    comment_lines: 2,
                },
            },
            FileResult {
                language: Some(Language::Rust),
                lines: LineMetrics {
                    total_lines: 5,
                    code_lines: 4,
                    blank_lines: 1,
                    comment_lines: 0,
                },
            },
        ];
        let agg = aggregate(&results);
        assert_eq!(agg.total_files, 3);
        assert_eq!(agg.total_lines.total_lines, 35);
        assert_eq!(agg.total_lines.code_lines, 27);
        assert_eq!(agg.by_language[&Language::Rust].file_count, 2);
        assert_eq!(agg.by_language[&Language::Rust].lines.code_lines, 12);
        assert_eq!(agg.by_language[&Language::Python].file_count, 1);
    }

    #[test]
    fn unknown_language_tracked_separately() {
        let results = vec![FileResult {
            language: None,
            lines: LineMetrics {
                total_lines: 5,
                code_lines: 5,
                blank_lines: 0,
                comment_lines: 0,
            },
        }];
        let agg = aggregate(&results);
        assert_eq!(agg.unknown_language.file_count, 1);
        assert_eq!(agg.unknown_language.lines.code_lines, 5);
        assert!(agg.by_language.is_empty());
    }

    #[test]
    fn total_invariant_holds() {
        let results = vec![
            FileResult {
                language: Some(Language::Go),
                lines: LineMetrics {
                    total_lines: 100,
                    code_lines: 80,
                    blank_lines: 10,
                    comment_lines: 10,
                },
            },
            FileResult {
                language: None,
                lines: LineMetrics {
                    total_lines: 50,
                    code_lines: 40,
                    blank_lines: 10,
                    comment_lines: 0,
                },
            },
        ];
        let agg = aggregate(&results);
        assert_eq!(
            agg.total_lines.total_lines,
            agg.total_lines.code_lines
                + agg.total_lines.blank_lines
                + agg.total_lines.comment_lines
        );
    }
}
