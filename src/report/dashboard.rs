//! Terminal dashboard: compact box-drawn output with all metrics.

use std::io::{self, Write};

use crate::metrics::aggregate::AggregateMetrics;
use crate::metrics::complexity::FunctionInfo;
use crate::metrics::dependencies::DependencySummary;
use crate::metrics::documentation::DocumentationMetrics;
use crate::snapshot::diff::SnapshotDiff;

use super::color::ColorWriter;

/// Hotspot entry: file path + function info.
pub(crate) type Hotspot = (String, FunctionInfo);

/// Render the terminal dashboard to stdout.
pub(crate) fn render(
    agg: &AggregateMetrics,
    diff: Option<&SnapshotDiff>,
    hotspots: &[Hotspot],
    dep_summary: &DependencySummary,
    doc_metrics: Option<&DocumentationMetrics>,
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
    if !dep_summary.manifests.is_empty() {
        render_dependencies(dep_summary, &mut cw)?;
    }
    if let Some(docs) = doc_metrics {
        render_documentation(docs, &mut cw)?;
    }
    Ok(())
}

/// Render dependencies section.
fn render_dependencies(summary: &DependencySummary, cw: &mut ColorWriter) -> io::Result<()> {
    cw.dim("├────────────────────────────────────────┤")?;
    cw.newline()?;
    cw.plain("│ ")?;
    cw.bold("Dependencies")?;
    cw.newline()?;
    cw.dim("│ ─────────────────────────────────────  │")?;
    cw.newline()?;

    for manifest in &summary.manifests {
        let name = manifest
            .file_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");
        cw.plain(&format!(
            "│  {:<20} {:>3} deps\n",
            name,
            manifest.direct_deps.len(),
        ))?;
    }

    cw.plain(&format!("│  Total direct: {}\n", summary.total_direct))?;
    if let Some(transitive) = summary.total_transitive {
        cw.plain(&format!("│  Total transitive: {transitive}\n"))?;
    }

    cw.dim("└────────────────────────────────────────┘")?;
    cw.newline()?;
    Ok(())
}

/// Render documentation metrics section.
fn render_documentation(docs: &DocumentationMetrics, cw: &mut ColorWriter) -> io::Result<()> {
    cw.dim("├────────────────────────────────────────┤")?;
    cw.newline()?;
    cw.plain("│ ")?;
    cw.bold("Documentation")?;
    cw.newline()?;
    cw.dim("│ ─────────────────────────────────────  │")?;
    cw.newline()?;

    cw.plain(&format!(
        "│  Markdown files: {}\n",
        docs.inventory.file_count,
    ))?;
    cw.plain(&format!(
        "│  Doc lines:      {}\n",
        docs.inventory.total_lines,
    ))?;

    let ratio_pct = docs.doc_to_code.ratio * 100.0;
    cw.plain(&format!(
        "│  Doc-to-code:    {:.2} ({:.0}%)\n",
        docs.doc_to_code.ratio, ratio_pct,
    ))?;

    let present = docs
        .readme_score
        .sections
        .iter()
        .filter(|s| s.present)
        .count();
    let total = docs.readme_score.sections.len();
    let score_pct = docs.readme_score.score * 100.0;
    cw.plain(&format!(
        "│  README score:   {}/{} ({:.0}%)\n",
        present, total, score_pct,
    ))?;

    let covered = docs
        .dir_coverage
        .entries
        .iter()
        .filter(|e| e.has_docs)
        .count();
    let total_dirs = docs.dir_coverage.entries.len();
    let cov_pct = docs.dir_coverage.coverage * 100.0;
    cw.plain(&format!(
        "│  Dir coverage:   {}/{} ({:.0}%)\n",
        covered, total_dirs, cov_pct,
    ))?;

    cw.dim("└────────────────────────────────────────┘")?;
    cw.newline()?;
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
        render(
            &agg,
            None,
            &[],
            &DependencySummary::default(),
            None,
            &mut buf,
            false,
        )
        .unwrap();
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
        render(
            &agg,
            Some(&diff),
            &[],
            &DependencySummary::default(),
            None,
            &mut buf,
            false,
        )
        .unwrap();
        let output = String::from_utf8(buf).unwrap();

        assert!(output.contains("(+2)"));
        assert!(output.contains("(+50)"));
    }

    #[test]
    fn languages_sorted_by_code_lines_descending() {
        let agg = make_agg();
        let mut buf = Vec::new();
        render(
            &agg,
            None,
            &[],
            &DependencySummary::default(),
            None,
            &mut buf,
            false,
        )
        .unwrap();
        let output = String::from_utf8(buf).unwrap();

        let rust_pos = output.find("Rust").unwrap();
        let python_pos = output.find("Python").unwrap();
        assert!(rust_pos < python_pos, "Rust should appear before Python");
    }

    #[test]
    fn color_mode_adds_ansi_codes() {
        let agg = make_agg();
        let mut buf = Vec::new();
        render(
            &agg,
            None,
            &[],
            &DependencySummary::default(),
            None,
            &mut buf,
            true,
        )
        .unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("\x1b["), "should contain ANSI codes");
    }

    #[test]
    fn no_color_mode_has_no_ansi() {
        let agg = make_agg();
        let mut buf = Vec::new();
        render(
            &agg,
            None,
            &[],
            &DependencySummary::default(),
            None,
            &mut buf,
            false,
        )
        .unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert!(!output.contains("\x1b["), "should not contain ANSI codes");
    }

    #[test]
    fn renders_documentation_section() {
        use crate::metrics::documentation::*;

        let agg = make_agg();
        let docs = DocumentationMetrics {
            inventory: DocInventory {
                file_count: 3,
                total_lines: 150,
                total_chars: 5000,
            },
            doc_to_code: DocToCodeRatio {
                doc_lines: 150,
                code_lines: 1000,
                ratio: 0.15,
            },
            readme_score: ReadmeScore {
                readme_path: None,
                sections: vec![
                    ReadmeSection {
                        name: "install",
                        present: true,
                    },
                    ReadmeSection {
                        name: "usage",
                        present: true,
                    },
                    ReadmeSection {
                        name: "api",
                        present: false,
                    },
                    ReadmeSection {
                        name: "contributing",
                        present: true,
                    },
                    ReadmeSection {
                        name: "license",
                        present: true,
                    },
                ],
                score: 0.8,
            },
            dir_coverage: DirCoverage {
                entries: vec![
                    DirCoverageEntry {
                        dir: "src".into(),
                        has_docs: true,
                    },
                    DirCoverageEntry {
                        dir: "lib".into(),
                        has_docs: false,
                    },
                ],
                coverage: 0.5,
            },
        };
        let mut buf = Vec::new();
        render(
            &agg,
            None,
            &[],
            &DependencySummary::default(),
            Some(&docs),
            &mut buf,
            false,
        )
        .unwrap();
        let output = String::from_utf8(buf).unwrap();

        assert!(output.contains("Documentation"));
        assert!(output.contains("Markdown files: 3"));
        assert!(output.contains("Doc lines:      150"));
        assert!(output.contains("Doc-to-code:    0.15"));
        assert!(output.contains("README score:   4/5"));
        assert!(output.contains("Dir coverage:   1/2"));
    }
}
