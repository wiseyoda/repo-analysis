//! Self-contained HTML report with inline CSS and SVG charts.

use std::io::{self, Write};

use crate::metrics::aggregate::AggregateMetrics;
use crate::metrics::complexity::FunctionInfo;
use crate::metrics::risk::RiskEntry;

/// All data needed to render the HTML report.
pub(crate) struct HtmlData<'a> {
    /// Aggregate code metrics.
    pub(crate) agg: &'a AggregateMetrics,
    /// Complexity hotspots.
    pub(crate) hotspots: &'a [(String, FunctionInfo)],
    /// Risk entries.
    pub(crate) risk_entries: &'a [RiskEntry],
}

/// Render a self-contained HTML report to a writer.
pub(crate) fn render(data: &HtmlData<'_>, writer: &mut dyn Write) -> io::Result<()> {
    write_header(writer)?;
    write_summary(data.agg, writer)?;
    write_language_chart(data.agg, writer)?;
    if !data.hotspots.is_empty() {
        write_hotspots(data.hotspots, writer)?;
    }
    if !data.risk_entries.is_empty() {
        write_risk_table(data.risk_entries, writer)?;
    }
    write_footer(writer)?;
    Ok(())
}

/// Write the HTML head and opening body.
fn write_header(w: &mut dyn Write) -> io::Result<()> {
    write!(
        w,
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>repostat report</title>
<style>
body {{ font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
  max-width: 800px; margin: 2rem auto; padding: 0 1rem; color: #1a1a1a; }}
h1 {{ border-bottom: 2px solid #e0e0e0; padding-bottom: 0.5rem; }}
h2 {{ margin-top: 2rem; color: #333; }}
table {{ border-collapse: collapse; width: 100%; margin: 1rem 0; }}
th, td {{ text-align: left; padding: 0.4rem 0.8rem; border-bottom: 1px solid #e0e0e0; }}
th {{ background: #f5f5f5; font-weight: 600; }}
td.num {{ text-align: right; font-variant-numeric: tabular-nums; }}
.metric {{ display: inline-block; margin: 0.5rem 1.5rem 0.5rem 0; }}
.metric .value {{ font-size: 1.5rem; font-weight: 700; }}
.metric .label {{ font-size: 0.85rem; color: #666; }}
svg {{ margin: 1rem 0; }}
</style>
</head>
<body>
<h1>repostat report</h1>
"#
    )
}

/// Write the summary metrics section.
fn write_summary(agg: &AggregateMetrics, w: &mut dyn Write) -> io::Result<()> {
    write!(
        w,
        r#"<h2>Summary</h2>
<div>
<div class="metric"><div class="value">{}</div><div class="label">Files</div></div>
<div class="metric"><div class="value">{}</div><div class="label">Code lines</div></div>
<div class="metric"><div class="value">{}</div><div class="label">Comments</div></div>
<div class="metric"><div class="value">{}</div><div class="label">Blanks</div></div>
<div class="metric"><div class="value">{}</div><div class="label">Languages</div></div>
</div>
"#,
        agg.total_files,
        agg.total_lines.code_lines,
        agg.total_lines.comment_lines,
        agg.total_lines.blank_lines,
        agg.by_language.len(),
    )
}

/// Write the language breakdown as an SVG horizontal bar chart.
fn write_language_chart(agg: &AggregateMetrics, w: &mut dyn Write) -> io::Result<()> {
    writeln!(w, "<h2>Language Breakdown</h2>")?;

    let mut langs: Vec<_> = agg.by_language.iter().collect();
    langs.sort_by(|a, b| b.1.lines.code_lines.cmp(&a.1.lines.code_lines));

    let max_code = langs
        .first()
        .map(|(_, m)| m.lines.code_lines)
        .unwrap_or(1)
        .max(1);
    let bar_count = langs.len().min(15);
    let chart_height = bar_count * 28 + 10;

    writeln!(
        w,
        r#"<svg width="700" height="{chart_height}" xmlns="http://www.w3.org/2000/svg">"#,
    )?;

    for (i, (lang, metrics)) in langs.iter().take(bar_count).enumerate() {
        let y = i * 28 + 5;
        let code = metrics.lines.code_lines;
        let bw = (code as f64 / max_code as f64 * 450.0).max(2.0) as usize;
        let name = lang.display_name();
        let text_y = y + 16;
        writeln!(
            w,
            "  <text x=\"0\" y=\"{text_y}\" font-size=\"13\" fill=\"#333\">{name}</text>",
        )?;
        writeln!(
            w,
            "  <rect x=\"120\" y=\"{y}\" width=\"{bw}\" height=\"20\" fill=\"#4a90d9\" rx=\"3\"/>",
        )?;
        let label_x = 120 + bw + 8;
        let label_y = y + 15;
        writeln!(
            w,
            "  <text x=\"{label_x}\" y=\"{label_y}\" font-size=\"12\" fill=\"#666\">{code}</text>",
        )?;
    }

    writeln!(w, "</svg>")?;
    Ok(())
}

/// Write the complexity hotspots table.
fn write_hotspots(hotspots: &[(String, FunctionInfo)], w: &mut dyn Write) -> io::Result<()> {
    writeln!(w, "<h2>Complexity Hotspots</h2>")?;
    writeln!(
        w,
        "<table><tr><th>File</th><th>Function</th>\
         <th>Cyclomatic</th><th>Cognitive</th><th>Lines</th></tr>"
    )?;
    for (file, func) in hotspots.iter().take(10) {
        writeln!(
            w,
            "<tr><td>{file}</td><td>{}</td>\
             <td class=\"num\">{}</td><td class=\"num\">{}</td>\
             <td class=\"num\">{}</td></tr>",
            func.name, func.cyclomatic, func.cognitive, func.line_count,
        )?;
    }
    writeln!(w, "</table>")?;
    Ok(())
}

/// Write the risk hotspots table.
fn write_risk_table(entries: &[RiskEntry], w: &mut dyn Write) -> io::Result<()> {
    writeln!(w, "<h2>Risk Hotspots</h2>")?;
    writeln!(
        w,
        "<table><tr><th>File</th><th>Risk</th>\
         <th>Churn</th><th>Complexity</th></tr>"
    )?;
    for entry in entries.iter().take(10) {
        writeln!(
            w,
            "<tr><td>{}</td><td class=\"num\">{}</td>\
             <td class=\"num\">{}</td><td class=\"num\">{}</td></tr>",
            entry.file, entry.risk_score, entry.churn_count, entry.max_complexity,
        )?;
    }
    writeln!(w, "</table>")?;
    Ok(())
}

/// Write the closing HTML tags.
fn write_footer(w: &mut dyn Write) -> io::Result<()> {
    writeln!(
        w,
        "<footer style=\"margin-top:2rem;color:#999;font-size:0.8rem\">\
         Generated by repostat</footer>\n</body>\n</html>"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metrics::aggregate::LanguageMetrics;
    use crate::metrics::loc::LineMetrics;
    use crate::scanner::language::Language;
    use std::collections::BTreeMap;

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
    fn renders_complete_html() {
        let agg = make_agg();
        let data = HtmlData {
            agg: &agg,
            hotspots: &[],
            risk_entries: &[],
        };
        let mut buf = Vec::new();
        render(&data, &mut buf).expect("render failed");
        let output = String::from_utf8(buf).expect("invalid utf8");
        assert!(output.contains("<!DOCTYPE html>"));
        assert!(output.contains("</html>"));
        assert!(output.contains("repostat report"));
    }

    #[test]
    fn html_contains_summary_metrics() {
        let agg = make_agg();
        let data = HtmlData {
            agg: &agg,
            hotspots: &[],
            risk_entries: &[],
        };
        let mut buf = Vec::new();
        render(&data, &mut buf).expect("render failed");
        let output = String::from_utf8(buf).expect("invalid utf8");
        assert!(output.contains("150"));
        assert!(output.contains("Code lines"));
    }

    #[test]
    fn html_contains_svg_chart() {
        let agg = make_agg();
        let data = HtmlData {
            agg: &agg,
            hotspots: &[],
            risk_entries: &[],
        };
        let mut buf = Vec::new();
        render(&data, &mut buf).expect("render failed");
        let output = String::from_utf8(buf).expect("invalid utf8");
        assert!(output.contains("<svg"));
        assert!(output.contains("</svg>"));
        assert!(output.contains("Rust"));
    }

    #[test]
    fn html_self_contained_no_external_refs() {
        let agg = make_agg();
        let data = HtmlData {
            agg: &agg,
            hotspots: &[],
            risk_entries: &[],
        };
        let mut buf = Vec::new();
        render(&data, &mut buf).expect("render failed");
        let output = String::from_utf8(buf).expect("invalid utf8");
        // SVG xmlns is allowed, but no external resource URLs
        let without_xmlns = output.replace("http://www.w3.org/2000/svg", "");
        assert!(!without_xmlns.contains("http://"));
        assert!(!without_xmlns.contains("https://"));
        assert!(!output.contains("<script"));
    }
}
