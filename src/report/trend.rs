//! Sparkline trend display across snapshots.

use std::io::{self, Write};

use crate::snapshot::Snapshot;

use super::color::ColorWriter;

/// Unicode block characters for sparklines (8 levels).
const SPARK_CHARS: [char; 8] = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

/// Render sparkline trends across multiple snapshots.
pub(crate) fn render(
    snapshots: &[Snapshot],
    writer: &mut dyn Write,
    color: bool,
) -> io::Result<()> {
    let mut cw = ColorWriter::new(writer, color);

    let count = snapshots.len();
    let first_date = snapshots
        .first()
        .map(|s| s.timestamp.format("%Y-%m-%d").to_string())
        .unwrap_or_default();
    let last_date = snapshots
        .last()
        .map(|s| s.timestamp.format("%Y-%m-%d").to_string())
        .unwrap_or_default();

    cw.bold(&format!(
        "repostat trend — {count} snapshots ({first_date} → {last_date})\n",
    ))?;
    cw.dim("───────────────────────────────────────────────\n")?;

    let code_lines: Vec<usize> = snapshots.iter().map(|s| s.total_lines.code).collect();
    let total_files: Vec<usize> = snapshots.iter().map(|s| s.total_files).collect();
    let languages: Vec<usize> = snapshots.iter().map(|s| s.by_language.len()).collect();

    render_sparkline_row(&mut cw, "Lines of code", &code_lines)?;
    render_sparkline_row(&mut cw, "Files", &total_files)?;
    render_sparkline_row(&mut cw, "Languages", &languages)?;

    cw.dim("───────────────────────────────────────────────\n")?;
    Ok(())
}

/// Render a single sparkline row with label, chart, and range.
fn render_sparkline_row(cw: &mut ColorWriter, label: &str, values: &[usize]) -> io::Result<()> {
    let spark = sparkline(values);
    let first = values.first().copied().unwrap_or(0);
    let last = values.last().copied().unwrap_or(0);
    cw.plain(&format!(
        "  {:<15} {}  {:>6} → {:<6}\n",
        label,
        spark,
        format_number(first),
        format_number(last),
    ))?;
    Ok(())
}

/// Generate a sparkline string from a series of values.
pub(crate) fn sparkline(values: &[usize]) -> String {
    if values.is_empty() {
        return String::new();
    }

    let min = *values.iter().min().unwrap_or(&0);
    let max = *values.iter().max().unwrap_or(&0);
    let range = if max == min { 1 } else { max - min };

    values
        .iter()
        .map(|&v| {
            let normalized = ((v - min) as f64 / range as f64 * 7.0) as usize;
            SPARK_CHARS[normalized.min(7)]
        })
        .collect()
}

/// Format a number with comma separators.
fn format_number(n: usize) -> String {
    if n < 1000 {
        return n.to_string();
    }
    let s = n.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    result.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sparkline_from_ascending_values() {
        let spark = sparkline(&[1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(spark, "▁▂▃▄▅▆▇█");
    }

    #[test]
    fn sparkline_from_flat_values() {
        let spark = sparkline(&[5, 5, 5, 5]);
        // All same → all map to index 0
        assert_eq!(spark, "▁▁▁▁");
    }

    #[test]
    fn sparkline_from_empty() {
        assert_eq!(sparkline(&[]), "");
    }

    #[test]
    fn sparkline_from_single_value() {
        let spark = sparkline(&[42]);
        assert_eq!(spark.len(), 3); // single unicode char
    }

    #[test]
    fn sparkline_from_two_values() {
        let spark = sparkline(&[0, 100]);
        assert_eq!(spark, "▁█");
    }

    #[test]
    fn format_number_small() {
        assert_eq!(format_number(42), "42");
        assert_eq!(format_number(999), "999");
    }

    #[test]
    fn format_number_thousands() {
        assert_eq!(format_number(1000), "1,000");
        assert_eq!(format_number(12345), "12,345");
        assert_eq!(format_number(1234567), "1,234,567");
    }
}
