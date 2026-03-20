---
name: test
description: Run the full test suite with formatting and linting checks
disable-model-invocation: true
user-invocable: true
allowed-tools: Bash(cargo *), Read, Grep
---

Run the full quality gate for repostat:

1. **Format check**: `cargo fmt --check`
   - If it fails, run `cargo fmt` to fix, then show what changed.

2. **Lint check**: `cargo clippy -- -D warnings`
   - If it fails, fix all warnings. No exceptions.

3. **Test suite**: `cargo test`
   - Run all unit and integration tests.
   - If any fail, analyze the failure and report:
     - Which test failed
     - Expected vs actual
     - The relevant source code

4. **Summary**: Report pass/fail for each step.

If $ARGUMENTS contains a specific test name or module, run only that:
`cargo test $ARGUMENTS`

Always follow the project coding standard in `docs/coding-standard.md`.
