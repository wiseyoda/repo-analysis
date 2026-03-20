//! Terminal dashboard: compact box-drawn output with all metrics.

use std::io::{self, Write};

use crate::metrics::aggregate::AggregateMetrics;
use crate::metrics::complexity::FunctionInfo;
use crate::snapshot::diff::SnapshotDiff;

use super::color::ColorWriter;

/// Hotspot entry: file path + function info.
pub(crate) type Hotspot = (String, FunctionInfo);

/// Render the terminal dashboard to stdout.
pub(crate) fn render(
    agg: &AggregateMetrics,
    diff: Option<&SnapshotDiff>,
    hotspots: &[Hotspot],
    writer: &mut dyn Write,
    color: bool,
) -> io::Result<()> {
    let mut cw = ColorWriter::new(writer, color);
    render_header(&mut cw)?;
    render_summary(agg, diff, &mut cw)?;
    render_language_breakdown(agg, &mut cw)?;
    if !hotspots.is_empty() {
        render_hotspots(hotspots, &mut cw)?;
    }
    Ok(())
}

/// Render complexity hotspots section.
fn render_hotspots(hotspots: &[Hotspot], cw: &mut ColorWriter) -> io::Result<()> {
    cw.dim("├────────────────────────────────────────┤")?;
    cw.newline()?;
    cw.plain("│ ")?;
    cw.bold("Complexity Hotspots")?;
    cw.newline()?;
    cw.dim("│ ─────────────────────────────────────  │")?;
    cw.newline()?;

    for (path, func) in hotspots.iter().take(10) {
        cw.plain(&format!(
            "│  CC={:<3} Cog={:<3} {:>4}L  {}::{}\n",
            func.cyclomatic, func.cognitive, func.line_count, path, func.name,
        ))?;
    }

    cw.dim("└────────────────────────────────────────┘")?;
    cw.newline()?;
    Ok(())
}

/// Render the dashboard header.
fn render_header(cw: &mut ColorWriter) -> io::Result<()> {
    cw.dim("┌────────────────────────────────────────┐")?;
    cw.newline()?;
    cw.dim("│")?;
    cw.header("           repostat analysis             ")?;
    cw.dim("│")?;
    cw.newline()?;
    cw.dim("├────────────────────────────────────────┤")?;
    cw.newline()?;
    Ok(())
}

/// Render the summary section with totals and optional diff.
fn render_summary(
    agg: &AggregateMetrics,
    diff: Option<&SnapshotDiff>,
    cw: &mut ColorWriter,
) -> io::Result<()> {
    cw.plain("│ ")?;
    cw.bold("Files:")?;
    cw.plain(&format!(" {:<10}", agg.total_files))?;
    if let Some(d) = diff {
        cw.plain(" ")?;
        cw.delta(d.files_delta)?;
    }
    cw.newline()?;

    cw.plain("│ ")?;
    cw.bold("Lines:")?;
    cw.plain(&format!(" {:<10}", agg.total_lines.total_lines))?;
    if let Some(d) = diff {
        cw.plain(" ")?;
        cw.delta(d.lines_delta.total)?;
    }
    cw.newline()?;

    cw.plain(&format!("│   Code:    {:<10}", agg.total_lines.code_lines))?;
    if let Some(d) = diff {
        cw.plain(" ")?;
        cw.delta(d.lines_delta.code)?;
    }
    cw.newline()?;

    cw.plain(&format!("│   Blank:   {:<10}", agg.total_lines.blank_lines))?;
    if let Some(d) = diff {
        cw.plain(" ")?;
        cw.delta(d.lines_delta.blank)?;
    }
    cw.newline()?;

    cw.plain(&format!(
        "│   Comment: {:<10}",
        agg.total_lines.comment_lines
    ))?;
    if let Some(d) = diff {
        cw.plain(" ")?;
        cw.delta(d.lines_delta.comment)?;
    }
    cw.newline()?;

    cw.dim("├────────────────────────────────────────┤")?;
    cw.newline()?;
    Ok(())
}

/// Render per-language breakdown sorted by code lines descending.
fn render_language_breakdown(agg: &AggregateMetrics, cw: &mut ColorWriter) -> io::Result<()> {
    cw.plain("│ ")?;
    cw.bold("Language          Files    Code     %")?;
    cw.plain("  │")?;
    cw.newline()?;

    cw.dim("│ ─────────────────────────────────────  │")?;
    cw.newline()?;

    let total_code = agg.total_lines.code_lines.max(1);

    let mut langs: Vec<_> = agg.by_language.iter().collect();
    langs.sort_by(|a, b| b.1.lines.code_lines.cmp(&a.1.lines.code_lines));

    for (lang, metrics) in &langs {
        let pct = (metrics.lines.code_lines as f64 / total_code as f64) * 100.0;
        cw.plain(&format!(
            "│ {:<17} {:>5}  {:>6}  {:>4.1}%  │",
            lang.display_name(),
            metrics.file_count,
            metrics.lines.code_lines,
            pct,
        ))?;
        cw.newline()?;
    }

    if agg.unknown_language.file_count > 0 {
        let pct = (agg.unknown_language.lines.code_lines as f64 / total_code as f64) * 100.0;
        cw.plain(&format!(
            "│ {:<17} {:>5}  {:>6}  {:>4.1}%  │",
            "Other", agg.unknown_language.file_count, agg.unknown_language.lines.code_lines, pct,
        ))?;
        cw.newline()?;
    }

    cw.dim("└────────────────────────────────────────┘")?;
    cw.newline()?;
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
        render(&agg, None, &[], &mut buf, false).unwrap();
        let output = String::from_utf8(buf).unwrap();

        assert!(output.contains("repostat"));
        assert!(output.contains("Files:"));
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
        render(&agg, Some(&diff), &[], &mut buf, false).unwrap();
        let output = String::from_utf8(buf).unwrap();

        assert!(output.contains("(+2)"));
        assert!(output.contains("(+50)"));
    }

    #[test]
    fn languages_sorted_by_code_lines_descending() {
        let agg = make_agg();
        let mut buf = Vec::new();
        render(&agg, None, &[], &mut buf, false).unwrap();
        let output = String::from_utf8(buf).unwrap();

        let rust_pos = output.find("Rust").unwrap();
        let python_pos = output.find("Python").unwrap();
        assert!(rust_pos < python_pos, "Rust should appear before Python");
    }

    #[test]
    fn color_mode_adds_ansi_codes() {
        let agg = make_agg();
        let mut buf = Vec::new();
        render(&agg, None, &[], &mut buf, true).unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("\x1b["), "should contain ANSI codes");
    }

    #[test]
    fn no_color_mode_has_no_ansi() {
        let agg = make_agg();
        let mut buf = Vec::new();
        render(&agg, None, &[], &mut buf, false).unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert!(!output.contains("\x1b["), "should not contain ANSI codes");
    }
}
