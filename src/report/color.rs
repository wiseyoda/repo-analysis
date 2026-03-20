//! Terminal color support with `NO_COLOR` respect.

use std::io::{self, Write};

/// ANSI color codes.
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const GREEN: &str = "\x1b[32m";
const RED: &str = "\x1b[31m";
const CYAN: &str = "\x1b[36m";
const DIM: &str = "\x1b[2m";

/// Check if color output is enabled.
///
/// Respects the `NO_COLOR` environment variable (https://no-color.org/).
pub(crate) fn is_color_enabled() -> bool {
    std::env::var("NO_COLOR").is_err()
}

/// A writer that optionally applies ANSI color codes.
pub(crate) struct ColorWriter<'a> {
    inner: &'a mut dyn Write,
    color: bool,
}

impl<'a> ColorWriter<'a> {
    /// Create a new color writer.
    pub(crate) fn new(writer: &'a mut dyn Write, color: bool) -> Self {
        Self {
            inner: writer,
            color,
        }
    }

    /// Write bold text.
    pub(crate) fn bold(&mut self, text: &str) -> io::Result<()> {
        if self.color {
            write!(self.inner, "{BOLD}{text}{RESET}")
        } else {
            write!(self.inner, "{text}")
        }
    }

    /// Write a positive delta (green).
    pub(crate) fn positive(&mut self, text: &str) -> io::Result<()> {
        if self.color {
            write!(self.inner, "{GREEN}{text}{RESET}")
        } else {
            write!(self.inner, "{text}")
        }
    }

    /// Write a negative delta (red).
    pub(crate) fn negative(&mut self, text: &str) -> io::Result<()> {
        if self.color {
            write!(self.inner, "{RED}{text}{RESET}")
        } else {
            write!(self.inner, "{text}")
        }
    }

    /// Write a delta value, coloring positive green and negative red.
    pub(crate) fn delta(&mut self, value: i64) -> io::Result<()> {
        let text = format!("({value:+})");
        if value > 0 {
            self.positive(&text)
        } else if value < 0 {
            self.negative(&text)
        } else {
            write!(self.inner, "{text}")
        }
    }

    /// Write a header/title (cyan).
    pub(crate) fn header(&mut self, text: &str) -> io::Result<()> {
        if self.color {
            write!(self.inner, "{CYAN}{text}{RESET}")
        } else {
            write!(self.inner, "{text}")
        }
    }

    /// Write dimmed text.
    pub(crate) fn dim(&mut self, text: &str) -> io::Result<()> {
        if self.color {
            write!(self.inner, "{DIM}{text}{RESET}")
        } else {
            write!(self.inner, "{text}")
        }
    }

    /// Write plain text (no color).
    pub(crate) fn plain(&mut self, text: &str) -> io::Result<()> {
        write!(self.inner, "{text}")
    }

    /// Write a newline.
    pub(crate) fn newline(&mut self) -> io::Result<()> {
        writeln!(self.inner)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_color_strips_ansi() {
        let mut buf = Vec::new();
        let mut cw = ColorWriter::new(&mut buf, false);
        cw.bold("hello").unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert_eq!(output, "hello");
        assert!(!output.contains("\x1b"));
    }

    #[test]
    fn color_adds_ansi() {
        let mut buf = Vec::new();
        let mut cw = ColorWriter::new(&mut buf, true);
        cw.bold("hello").unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("\x1b[1m"));
        assert!(output.contains("hello"));
    }

    #[test]
    fn positive_delta_green() {
        let mut buf = Vec::new();
        let mut cw = ColorWriter::new(&mut buf, true);
        cw.delta(5).unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("\x1b[32m"));
        assert!(output.contains("(+5)"));
    }

    #[test]
    fn negative_delta_red() {
        let mut buf = Vec::new();
        let mut cw = ColorWriter::new(&mut buf, true);
        cw.delta(-3).unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("\x1b[31m"));
        assert!(output.contains("(-3)"));
    }

    #[test]
    fn zero_delta_no_color() {
        let mut buf = Vec::new();
        let mut cw = ColorWriter::new(&mut buf, true);
        cw.delta(0).unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert!(!output.contains("\x1b[32m"));
        assert!(!output.contains("\x1b[31m"));
        assert!(output.contains("(+0)"));
    }
}
