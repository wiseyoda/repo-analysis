---
paths:
  - "tests/**/*.rs"
  - "src/**/*test*"
---

# Test File Rules

When writing or editing tests:

- Test names describe behavior: `fn counts_code_lines_excluding_comments()`
- Start test name with a verb in present tense
- One assertion per concept (multiple asserts for one behavior is fine)
- Each test is independent — sets up its own state, no shared mutable state
- No sleeping or timing — tests must be deterministic
- Use `tempfile::tempdir()` for filesystem tests, never write to real paths
- Use `assert_cmd` for integration tests that run the binary
- Use `insta` for snapshot testing terminal output and reports
- Test edge cases: empty input, Unicode, very large files, permission errors
- `unwrap()` and `expect()` are allowed in test code
