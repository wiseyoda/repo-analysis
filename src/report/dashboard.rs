//! Terminal dashboard: compact box-drawn output with all Phase 1 metrics.

use std::io::{self, Write};

use crate::metrics::aggregate::AggregateMetrics;
use crate::snapshot::diff::SnapshotDiff;

/// Render the terminal dashboard to stdout.
pub(crate) fn render(
    agg: &AggregateMetrics,
    diff: Option<&SnapshotDiff>,
    writer: &mut dyn Write,
) -> io::Result<()> {
    render_header(writer)?;
    render_summary(agg, diff, writer)?;
    render_language_breakdown(agg, writer)?;
    Ok(())
}

/// Render the dashboard header.
fn render_header(w: &mut dyn Write) -> io::Result<()> {
    writeln!(w, "┌────────────────────────────────────────┐")?;
    writeln!(w, "│           repostat analysis             │")?;
    writeln!(w, "├────────────────────────────────────────┤")?;
    Ok(())
}

/// Render the summary section with totals and optional diff.
fn render_summary(
    agg: &AggregateMetrics,
    diff: Option<&SnapshotDiff>,
    w: &mut dyn Write,
) -> io::Result<()> {
    write!(w, "│ Files: {:<10}", agg.total_files)?;
    if let Some(d) = diff {
        write!(w, " ({:+})", d.files_delta)?;
    }
    writeln!(w)?;

    write!(w, "│ Lines: {:<10}", agg.total_lines.total_lines)?;
    if let Some(d) = diff {
        write!(w, " ({:+})", d.lines_delta.total)?;
    }
    writeln!(w)?;

    write!(w, "│   Code:    {:<10}", agg.total_lines.code_lines)?;
    if let Some(d) = diff {
        write!(w, " ({:+})", d.lines_delta.code)?;
    }
    writeln!(w)?;

    write!(w, "│   Blank:   {:<10}", agg.total_lines.blank_lines)?;
    if let Some(d) = diff {
        write!(w, " ({:+})", d.lines_delta.blank)?;
    }
    writeln!(w)?;

    write!(w, "│   Comment: {:<10}", agg.total_lines.comment_lines)?;
    if let Some(d) = diff {
        write!(w, " ({:+})", d.lines_delta.comment)?;
    }
    writeln!(w)?;

    writeln!(w, "├────────────────────────────────────────┤")?;
    Ok(())
}

/// Render per-language breakdown sorted by code lines descending.
fn render_language_breakdown(agg: &AggregateMetrics, w: &mut dyn Write) -> io::Result<()> {
    writeln!(w, "│ Language          Files    Code     %  │")?;
    writeln!(w, "│ ─────────────────────────────────────  │")?;

    let total_code = agg.total_lines.code_lines.max(1);

    // Sort languages by code lines descending
    let mut langs: Vec<_> = agg.by_language.iter().collect();
    langs.sort_by(|a, b| b.1.lines.code_lines.cmp(&a.1.lines.code_lines));

    for (lang, metrics) in &langs {
        let pct = (metrics.lines.code_lines as f64 / total_code as f64) * 100.0;
        writeln!(
            w,
            "│ {:<17} {:>5}  {:>6}  {:>4.1}%  │",
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
            "│ {:<17} {:>5}  {:>6}  {:>4.1}%  │",
            "Other", agg.unknown_language.file_count, agg.unknown_language.lines.code_lines, pct,
        )?;
    }

    writeln!(w, "└────────────────────────────────────────┘")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::metrics::aggregate::LanguageMetrics;
    use crate::metrics::loc::LineMetrics;
    use crate::scanner::language::Language;
    use crate::snapshot::diff::LinesDelta;

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
        by_language.insert(
            Language::Python,
            LanguageMetrics {
                file_count: 3,
                lines: LineMetrics {
                    total_lines: 100,
                    code_lines: 70,
                    blank_lines: 20,
                    comment_lines: 10,
                },
            },
        );

        AggregateMetrics {
            total_files: 8,
            total_lines: LineMetrics {
                total_lines: 300,
                code_lines: 220,
                blank_lines: 50,
                comment_lines: 30,
            },
            by_language,
            unknown_language: LanguageMetrics::default(),
        }
    }

    #[test]
    fn renders_without_diff() {
        let agg = make_agg();
        let mut buf = Vec::new();
        render(&agg, None, &mut buf).unwrap();
        let output = String::from_utf8(buf).unwrap();

        assert!(output.contains("repostat"));
        assert!(output.contains("Files: 8"));
        assert!(output.contains("Lines: 300"));
        assert!(output.contains("Rust"));
        assert!(output.contains("Python"));
    }

    #[test]
    fn renders_with_diff() {
        let agg = make_agg();
        let diff = SnapshotDiff {
            files_delta: 2,
            lines_delta: LinesDelta {
                total: 50,
                code: 30,
                blank: 10,
                comment: 10,
            },
        };
        let mut buf = Vec::new();
        render(&agg, Some(&diff), &mut buf).unwrap();
        let output = String::from_utf8(buf).unwrap();

        assert!(output.contains("(+2)"));
        assert!(output.contains("(+50)"));
    }

    #[test]
    fn languages_sorted_by_code_lines_descending() {
        let agg = make_agg();
        let mut buf = Vec::new();
        render(&agg, None, &mut buf).unwrap();
        let output = String::from_utf8(buf).unwrap();

        let rust_pos = output.find("Rust").unwrap();
        let python_pos = output.find("Python").unwrap();
        assert!(rust_pos < python_pos, "Rust should appear before Python");
    }
}
