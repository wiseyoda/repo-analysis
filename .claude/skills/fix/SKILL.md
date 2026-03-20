---
name: fix
version: 1.0.0
description: |
  Debugger workflow. Reproduce issue, write a failing regression test, implement
  the minimal fix, verify, commit. Use when asked to "fix", "debug", "investigate",
  or when a bug is found during development.
disable-model-invocation: false
user-invocable: true
allowed-tools:
  - Bash
  - Read
  - Write
  - Edit
  - Glob
  - Grep
  - Skill
  - AskUserQuestion
---

## Arguments

- `/fix <description>` — describe the bug or paste the error message
- `/fix` with no args — STOP with "Usage: `/fix <bug description or error message>`"

## Preconditions

```bash
[ -f Cargo.toml ] && echo "CARGO_OK" || echo "NO_CARGO"
cargo test 2>&1 | tail -3
```

Note whether existing tests pass or fail — this tells us if the bug is in tested or untested code.

---

## Step 1: Reproduce — Understand the Bug

Read the description in $ARGUMENTS. Determine:

1. **What should happen** (expected behavior)
2. **What actually happens** (actual behavior)
3. **Where it likely lives** (grep for error messages, function names, relevant code)

```bash
# Search for relevant code
grep -rn "<keyword from bug description>" src/
```

Read the relevant source files. Trace the code path that triggers the bug.

Write a brief diagnosis:
```
BUG: <one-line description>
EXPECTED: <what should happen>
ACTUAL: <what happens instead>
ROOT CAUSE: <why it fails — the specific line/logic>
FIX: <what needs to change>
```

---

## Step 2: Write Failing Test (TDD — Red)

Write a regression test that reproduces the bug. The test MUST:

- Set up the precondition that triggers the bug
- Perform the action that exposes it
- Assert the CORRECT behavior (what it should do after the fix)
- Include an attribution comment:

```rust
#[test]
fn regression_description_of_bug() {
    // Regression: <brief description>
    // Found: YYYY-MM-DD
    // Root cause: <what was wrong>

    // ... test that currently FAILS
}
```

Run the test to confirm it fails:
```bash
cargo test <test_name> 2>&1
```

**If the test passes:** The bug isn't what we think, or it's already fixed.
Investigate further before proceeding.

---

## Step 3: Fix — Minimal Change

Make the **smallest change** that fixes the bug. Don't refactor, don't clean up,
don't improve adjacent code. Fix the bug and nothing else.

Follow `docs/coding-standard.md`:
- No `unwrap()` in non-test code
- Proper error handling
- Functions under 40 lines

---

## Step 4: Verify — Green

Run the regression test:
```bash
cargo test <test_name> 2>&1
```

**Must pass.** If it still fails, the fix is wrong — go back to Step 3.

Run the full suite to check for regressions:

Invoke **Skill: `/check`**

**If other tests broke:** The fix introduced a regression. Reconsider the approach.

---

## Step 5: Commit

```bash
git add <changed-files>
git commit -m "$(cat <<'INNEREOF'
fix(<scope>): <description>

Root cause: <what was wrong>
Regression test: <test name>

Co-Authored-By: Claude Opus 4.6 (1M context) <noreply@anthropic.com>
INNEREOF
)"
```

---

## Step 6: Completion Summary

```
+====================================================+
|              BUG FIX SUMMARY                        |
+====================================================+
| Bug          | <description>                        |
| Root cause   | <what was wrong>                     |
| Fix          | <what changed>                       |
+----------------------------------------------------+
| Files changed  | N                                  |
| Lines changed  | +M / -K                            |
| Regression test| <test name>                        |
+----------------------------------------------------+
| Quality gate   | PASS                               |
| Commit         | <sha>                              |
+====================================================+
```

---

## Important Rules

1. **Reproduce first.** Write a failing test BEFORE fixing. This is not optional.
2. **Minimal fix.** Change as little as possible. Don't refactor during a fix.
3. **Regression test required.** Every bug fix gets a test that would catch it if it recurred.
4. **Full suite must pass.** The fix must not break anything else.
5. **Commit message explains root cause.** Future developers need to understand WHY.
6. **One bug, one commit.** Don't bundle fixes.
