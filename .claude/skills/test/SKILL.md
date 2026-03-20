---
name: test
version: 1.0.0
description: |
  Run the full quality gate for repostat: fmt, clippy, test. Shows structured
  pass/fail results. Use when asked to "test", "check tests", or "run the suite".
  For quick pass/fail only, use /check instead.
disable-model-invocation: false
user-invocable: true
allowed-tools:
  - Bash(cargo *)
  - Bash(wc *)
  - Read
  - Grep
  - Glob
---

## Arguments

- `/test` — run all tests with full quality gate
- `/test <name>` — run only tests matching `<name>` (passed to `cargo test <name>`)
- `/test --unit` — run only unit tests (`cargo test --lib`)
- `/test --integration` — run only integration tests (`cargo test --test '*'`)

## Preconditions

Check before running. Abort if any fail.

```bash
[ -f Cargo.toml ] && echo "CARGO_OK" || echo "NO_CARGO"
```

**If NO_CARGO:** "No Cargo.toml found. Are you in the project root?" — STOP.

## Steps

### Step 1: Format Check

```bash
cargo fmt --check 2>&1
```

If it fails, run `cargo fmt` to fix, then show what changed with `cargo fmt --check` again.

### Step 2: Lint Check

```bash
cargo clippy -- -D warnings 2>&1
```

If it fails, show the warnings grouped by file. For each warning, show:
- File:line
- Warning message
- Suggested fix (from clippy)

Do NOT auto-fix clippy warnings. Report them for the user to decide.

### Step 3: Test Suite

Based on $ARGUMENTS:
- No args: `cargo test 2>&1`
- `--unit`: `cargo test --lib 2>&1`
- `--integration`: `cargo test --test '*' 2>&1`
- Other: `cargo test $ARGUMENTS 2>&1`

If any test fails, for each failure report:
- Test name and location (file:line)
- Expected vs actual values
- Relevant source code context (read the test file)

### Step 4: Completion Summary

```
+======================================+
|        QUALITY GATE RESULTS          |
+======================================+
| Check    | Status | Details          |
|----------|--------|------------------|
| fmt      | PASS   |                  |
| clippy   | PASS   | 0 warnings       |
| test     | PASS   | 42 passed, 0 fail|
+--------------------------------------+
| VERDICT: GATE PASSED                 |
+======================================+
```

If any step failed:
```
| VERDICT: GATE FAILED — fix clippy   |
```

## Important Rules

1. **Never skip steps.** All three checks run even if one fails — the user needs the full picture.
2. **Never auto-fix clippy.** Report warnings with suggestions, let the user decide.
3. **Auto-fix formatting.** If `cargo fmt --check` fails, run `cargo fmt` and report what changed.
4. **Show test count.** Always report total tests run, passed, failed.
5. **Read failing test source.** When a test fails, read the test file to provide context.
