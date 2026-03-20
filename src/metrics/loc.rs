//! Line counting: code, comments, and blanks.

use crate::scanner::language::Language;

/// Per-file line count metrics.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct LineMetrics {
    /// Total number of lines in the file.
    pub total_lines: usize,
    /// Lines containing executable code (not blank, not pure comment).
    pub code_lines: usize,
    /// Lines containing only whitespace.
    pub blank_lines: usize,
    /// Lines that are pure comments.
    pub comment_lines: usize,
}

/// Comment syntax definition for a language.
struct CommentSyntax {
    single: &'static [&'static str],
    block_start: &'static str,
    block_end: &'static str,
}

/// Count lines of code, comments, and blanks in source content.
///
/// Uses language-specific comment syntax. For unknown languages (`None`),
/// all non-blank lines are counted as code.
pub(crate) fn count_lines(content: &str, language: Option<Language>) -> LineMetrics {
    if content.is_empty() {
        return LineMetrics::default();
    }

    let syntax = language.and_then(comment_syntax);
    let lines: Vec<&str> = content.lines().collect();
    let total_lines = lines.len();
    let mut code_lines = 0;
    let mut blank_lines = 0;
    let mut comment_lines = 0;
    let mut in_block = false;

    for line in &lines {
        let trimmed = line.trim();

        if trimmed.is_empty() {
            blank_lines += 1;
            continue;
        }

        if let Some(ref syn) = syntax {
            if in_block {
                comment_lines += 1;
                if trimmed.contains(syn.block_end) {
                    in_block = false;
                }
                continue;
            }

            if syn.single.iter().any(|prefix| trimmed.starts_with(prefix)) {
                comment_lines += 1;
                continue;
            }

            if !syn.block_start.is_empty() && trimmed.contains(syn.block_start) {
                comment_lines += 1;
                if !trimmed.contains(syn.block_end)
                    || trimmed.find(syn.block_start) >= trimmed.find(syn.block_end)
                {
                    in_block = true;
                }
                continue;
            }
        }

        code_lines += 1;
    }

    LineMetrics {
        total_lines,
        code_lines,
        blank_lines,
        comment_lines,
    }
}

/// Return comment syntax for a language, or None if no comment syntax.
fn comment_syntax(lang: Language) -> Option<CommentSyntax> {
    use Language::*;
    match lang {
        Rust | Go | Swift | Dart | Zig | Protobuf | GraphQL => Some(CommentSyntax {
            single: &["//"],
            block_start: "/*",
            block_end: "*/",
        }),
        C | Cpp | Java | CSharp | Kotlin | Scala | Groovy | JavaScript | TypeScript => {
            Some(CommentSyntax {
                single: &["//"],
                block_start: "/*",
                block_end: "*/",
            })
        }
        CSS | SCSS | Less => Some(CommentSyntax {
            single: &[],
            block_start: "/*",
            block_end: "*/",
        }),
        Sass => Some(CommentSyntax {
            single: &["//"],
            block_start: "/*",
            block_end: "*/",
        }),
        Python | Ruby | Shell | Bash | Zsh | Fish | Perl | R | YAML | TOML | Makefile
        | Dockerfile | Terraform | CMake => Some(CommentSyntax {
            single: &["#"],
            block_start: "",
            block_end: "",
        }),
        SQL | Lua => Some(CommentSyntax {
            single: &["--"],
            block_start: "",
            block_end: "",
        }),
        Haskell => Some(CommentSyntax {
            single: &["--"],
            block_start: "{-",
            block_end: "-}",
        }),
        Erlang => Some(CommentSyntax {
            single: &["%"],
            block_start: "",
            block_end: "",
        }),
        Clojure => Some(CommentSyntax {
            single: &[";"],
            block_start: "",
            block_end: "",
        }),
        OCaml | FSharp => Some(CommentSyntax {
            single: &[],
            block_start: "(*",
            block_end: "*)",
        }),
        HTML | XML | Markdown | Vue | Svelte => Some(CommentSyntax {
            single: &[],
            block_start: "<!--",
            block_end: "-->",
        }),
        PHP => Some(CommentSyntax {
            single: &["//", "#"],
            block_start: "/*",
            block_end: "*/",
        }),
        Elixir => Some(CommentSyntax {
            single: &["#"],
            block_start: "",
            block_end: "",
        }),
        Nim => Some(CommentSyntax {
            single: &["#"],
            block_start: "#[",
            block_end: "]#",
        }),
        Julia => Some(CommentSyntax {
            single: &["#"],
            block_start: "#=",
            block_end: "=#",
        }),
        ObjectiveC => Some(CommentSyntax {
            single: &["//"],
            block_start: "/*",
            block_end: "*/",
        }),
        PowerShell => Some(CommentSyntax {
            single: &["#"],
            block_start: "<#",
            block_end: "#>",
        }),
        JSON => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::language::Language;

    #[test]
    fn counts_empty_file() {
        let m = count_lines("", None);
        assert_eq!(m.total_lines, 0);
        assert_eq!(m.code_lines, 0);
        assert_eq!(m.blank_lines, 0);
        assert_eq!(m.comment_lines, 0);
    }

    #[test]
    fn counts_blank_only_file() {
        let m = count_lines("  \n\n  \n", None);
        assert_eq!(m.total_lines, 3);
        assert_eq!(m.blank_lines, 3);
        assert_eq!(m.code_lines, 0);
        assert_eq!(m.comment_lines, 0);
    }

    #[test]
    fn invariant_total_equals_sum() {
        let content = "fn main() {\n  // hello\n\n  let x = 1;\n}\n";
        let m = count_lines(content, Some(Language::Rust));
        assert_eq!(
            m.total_lines,
            m.code_lines + m.blank_lines + m.comment_lines
        );
    }

    #[test]
    fn counts_slash_slash_comments() {
        let content = "// comment\ncode\n// another\n";
        let m = count_lines(content, Some(Language::Rust));
        assert_eq!(m.comment_lines, 2);
        assert_eq!(m.code_lines, 1);
    }

    #[test]
    fn counts_hash_comments() {
        let content = "# comment\ncode\n# another\n";
        let m = count_lines(content, Some(Language::Python));
        assert_eq!(m.comment_lines, 2);
        assert_eq!(m.code_lines, 1);
    }

    #[test]
    fn counts_dash_dash_comments() {
        let content = "-- comment\nSELECT 1\n-- another\n";
        let m = count_lines(content, Some(Language::SQL));
        assert_eq!(m.comment_lines, 2);
        assert_eq!(m.code_lines, 1);
    }

    #[test]
    fn counts_block_comments_c_style() {
        let content = "/* start\n  middle\n  end */\ncode\n";
        let m = count_lines(content, Some(Language::Rust));
        assert_eq!(m.comment_lines, 3);
        assert_eq!(m.code_lines, 1);
    }

    #[test]
    fn counts_block_comment_single_line() {
        let content = "/* single line block */\ncode\n";
        let m = count_lines(content, Some(Language::C));
        assert_eq!(m.comment_lines, 1);
        assert_eq!(m.code_lines, 1);
    }

    #[test]
    fn counts_html_block_comments() {
        let content = "<!-- comment\n  still comment\n-->\n<div>\n";
        let m = count_lines(content, Some(Language::HTML));
        assert_eq!(m.comment_lines, 3);
        assert_eq!(m.code_lines, 1);
    }

    #[test]
    fn unknown_language_all_nonblank_is_code() {
        let content = "line1\n\nline3\n";
        let m = count_lines(content, None);
        assert_eq!(m.total_lines, 3);
        assert_eq!(m.code_lines, 2);
        assert_eq!(m.blank_lines, 1);
        assert_eq!(m.comment_lines, 0);
    }

    #[test]
    fn json_has_no_comments() {
        let content = "{\n  \"key\": \"value\"\n}\n";
        let m = count_lines(content, Some(Language::JSON));
        assert_eq!(m.comment_lines, 0);
        assert_eq!(m.code_lines, 3);
    }

    #[test]
    fn mixed_code_and_comment_on_same_line_counts_as_code() {
        let content = "let x = 1; // inline comment\n";
        let m = count_lines(content, Some(Language::Rust));
        assert_eq!(m.code_lines, 1);
        assert_eq!(m.comment_lines, 0);
    }
}
