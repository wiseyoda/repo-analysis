# Spec: Line Counting Engine

**Requirement IDs:** R-006
**Phase:** 1
**Date:** 2026-03-19

## Purpose

Count lines of code, comments, and blanks in a source file using language-specific
comment syntax. Feeds into per-language aggregation and the terminal dashboard.

## Inputs

| Input | Type |
|-------|------|
| File content | `&str` |
| Language | `Option<Language>` |

## Outputs

`LineMetrics` struct: `total_lines`, `code_lines`, `blank_lines`, `comment_lines` (all `usize`).

Invariant: `total_lines == code_lines + blank_lines + comment_lines`.

## Behavior

1. Split content into lines.
2. Track whether we're inside a block comment.
3. For each line:
   - If blank (only whitespace) → blank_lines++
   - If inside a block comment → comment_lines++ (check for block end)
   - If line starts with a single-line comment prefix (after trimming) → comment_lines++
   - If line contains a block comment start → comment_lines++ (enter block mode)
   - Otherwise → code_lines++
4. total_lines = number of lines.

## Comment Syntax by Language

**Single-line:** `//` (C-family, Rust, Go, Swift, Kotlin, Scala, Zig, Dart, Groovy, Protobuf, GraphQL),
`#` (Python, Ruby, Shell, Bash, Zsh, Fish, Perl, R, YAML, TOML, Makefile, Dockerfile, Terraform, CMake),
`--` (SQL, Haskell, Lua, Elm), `%` (Erlang), `;` (Clojure)

**Block comments:** `/* */` (C-family, Rust, Go, Swift, CSS, SCSS, Less, Kotlin, Scala, Groovy, Dart),
`{- -}` (Haskell), `(* *)` (OCaml, F#), `<!-- -->` (HTML, XML, Markdown, Vue, Svelte)

**None (no comment syntax):** JSON

**Unknown language (None):** all non-blank lines are code.

## Edge Cases

- Empty file → all zeros
- File with only blank lines → total_lines = N, blank_lines = N
- Mixed inline comments (code + comment on same line) → count as code (simplification)
- Nested block comments → not supported (matching first close)
- Block comment open/close on same line → comment_lines++
- No trailing newline → last line still counted

## Acceptance Criteria

- [ ] Counts total, code, blank, comment lines
- [ ] Invariant: total == code + blank + comment
- [ ] Handles // single-line comments
- [ ] Handles # single-line comments
- [ ] Handles /* */ block comments
- [ ] Handles <!-- --> block comments
- [ ] Unknown language counts all non-blank as code
- [ ] Empty file returns all zeros
- [ ] Blank-only file counts correctly
