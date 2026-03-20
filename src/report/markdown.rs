//! Markdown report generation.

use std::io::{self, Write};

use crate::metrics::aggregate::AggregateMetrics;
use crate::snapshot::diff::SnapshotDiff;

/// Render a Markdown report to a writer.
pub(crate) fn render(
    agg: &AggregateMetrics,
    diff: Option<&SnapshotDiff>,
    writer: &mut dyn Write,
) -> io::Result<()> {
    writeln!(writer, "# Repository Analysis")?;
    writeln!(writer)?;
    render_summary(agg, diff, writer)?;
    render_language_table(agg, writer)?;
    Ok(())
}

/// Render the summary section.
fn render_summary(
    agg: &AggregateMetrics,
    diff: Option<&SnapshotDiff>,
    w: &mut dyn Write,
) -> io::Result<()> {
    writeln!(w, "## Summary")?;
    writeln!(w)?;
    write!(w, "- **Files:** {}", agg.total_files)?;
    if let Some(d) = diff {
        write!(w, " ({:+})", d.files_delta)?;
    }
    writeln!(w)?;

    write!(w, "- **Total lines:** {}", agg.total_lines.total_lines)?;
    if let Some(d) = diff {
        write!(w, " ({:+})", d.lines_delta.total)?;
    }
    writeln!(w)?;

    writeln!(w, "- **Code:** {}", agg.total_lines.code_lines)?;
    writeln!(w, "- **Blank:** {}", agg.total_lines.blank_lines)?;
    writeln!(w, "- **Comment:** {}", agg.total_lines.comment_lines)?;
    writeln!(w)?;
    Ok(())
}

/// Render the per-language breakdown as a Markdown table.
fn render_language_table(agg: &AggregateMetrics, w: &mut dyn Write) -> io::Result<()> {
    writeln!(w, "## Language Breakdown")?;
    writeln!(w)?;
    writeln!(w, "| Language | Files | Code | % |")?;
    writeln!(w, "|----------|------:|-----:|--:|")?;

    let total_code = agg.total_lines.code_lines.max(1);

    let mut langs: Vec<_> = agg.by_language.iter().collect();
    langs.sort_by(|a, b| b.1.lines.code_lines.cmp(&a.1.lines.code_lines));

    for (lang, metrics) in &langs {
        let pct = (metrics.lines.code_lines as f64 / total_code as f64) * 100.0;
        writeln!(
            w,
            "| {} | {} | {} | {:.1}% |",
            lang.display_name(),
            metrics.file_count,
            metrics.lines.code_lines,
            pct,
        )?;
    }

    if agg.unknown_language.file_count > 0 {
        let pct = (agg.unknown_language.lines.code_lines as f64 / total_code as f64) * 100.0;
        writeln!(
            w,
            "| Other | {} | {} | {:.1}% |",
            agg.unknown_language.file_count, agg.unknown_language.lines.code_lines, pct,
        )?;
    }

    writeln!(w)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::metrics::aggregate::LanguageMetrics;
    use crate::metrics::loc::LineMetrics;
    use crate::scanner::language::Language;

    use super::*;

    fn make_agg() -> AggregateMetrics {
        let mut by_language = BTreeMap::new();
        by_language.insert(
            Language::Rust,
            LanguageMetrics {
                file_count: 5,
                lines: LineMetrics {
                    total_lines: 200,
                    code_lines: 150,
                    blank_lines: 30,
                    comment_lines: 20,
                },
            },
        );
        AggregateMetrics {
            total_files: 5,
            total_lines: LineMetrics {
                total_lines: 200,
                code_lines: 150,
                blank_lines: 30,
                comment_lines: 20,
            },
            by_language,
            unknown_language: LanguageMetrics::default(),
        }
    }

    #[test]
    fn renders_markdown_with_header() {
        let agg = make_agg();
        let mut buf = Vec::new();
        render(&agg, None, &mut buf).unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert!(output.starts_with("# Repository Analysis"));
    }

    #[test]
    fn renders_language_table() {
        let agg = make_agg();
        let mut buf = Vec::new();
        render(&agg, None, &mut buf).unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("| Language | Files | Code | % |"));
        assert!(output.contains("Rust"));
    }

    #[test]
    fn renders_summary_section() {
        let agg = make_agg();
        let mut buf = Vec::new();
        render(&agg, None, &mut buf).expect("render failed");
        let output = String::from_utf8(buf).expect("invalid utf8");
        assert!(output.contains("## Summary"));
        assert!(output.contains("**Files:** 5"));
        assert!(output.contains("**Code:** 150"));
        assert!(output.contains("**Blank:** 30"));
        assert!(output.contains("**Comment:** 20"));
    }

    #[test]
    fn renders_without_diff_no_deltas() {
        let agg = make_agg();
        let mut buf = Vec::new();
        render(&agg, None, &mut buf).expect("render failed");
        let output = String::from_utf8(buf).expect("invalid utf8");
        assert!(
            !output.contains("(+"),
            "should not contain delta markers without diff"
        );
    }

    #[test]
    fn renders_negative_diff_deltas() {
        let agg = make_agg();
        let diff = SnapshotDiff {
            files_delta: -2,
            lines_delta: crate::snapshot::diff::LinesDelta {
                total: -10,
                code: -8,
                blank: -1,
                comment: -1,
            },
        };
        let mut buf = Vec::new();
        render(&agg, Some(&diff), &mut buf).expect("render failed");
        let output = String::from_utf8(buf).expect("invalid utf8");
        assert!(output.contains("(-2)"));
        assert!(output.contains("(-10)"));
    }

    #[test]
    fn renders_diff_deltas() {
        let agg = make_agg();
        let diff = SnapshotDiff {
            files_delta: 1,
            lines_delta: crate::snapshot::diff::LinesDelta {
                total: 20,
                code: 15,
                blank: 3,
                comment: 2,
            },
        };
        let mut buf = Vec::new();
        render(&agg, Some(&diff), &mut buf).unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("(+1)"));
        assert!(output.contains("(+20)"));
    }
}
