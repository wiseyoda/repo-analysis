---
name: review
version: 1.0.0
description: |
  Pre-commit code review against project standards. Fix-first: auto-fixes mechanical
  issues, asks about judgment calls. Use before committing or when asked to "review",
  "check my code", or "audit". For post-commit PR review, run on a feature branch.
disable-model-invocation: false
user-invocable: true
allowed-tools:
  - Read
  - Edit
  - Grep
  - Glob
  - Bash(cargo *)
  - Bash(git diff*)
  - Bash(git log*)
  - Bash(git status*)
  - AskUserQuestion
---

## Arguments

- `/review` — review all uncommitted changes
- `/review <file>` — review only the specified file or module
- `/review --staged` — review only staged changes (`git diff --cached`)

## Preconditions

```bash
[ -f Cargo.toml ] && echo "CARGO_OK" || echo "NO_CARGO"
git diff --stat HEAD 2>/dev/null | tail -1 || echo "NO_CHANGES"
```

**If NO_CARGO:** "No Cargo.toml found." — STOP.
**If NO_CHANGES and no args:** "No uncommitted changes to review." — STOP.

## Step 0: Gather Context

Read these files to calibrate the review:

1. `docs/constitution.md` — immutable principles (the highest bar)
2. `docs/coding-standard.md` — specific code rules
3. `ROADMAP.md` — confirm changes align with active phase

```bash
git diff --stat HEAD
git diff HEAD
```

If $ARGUMENTS specifies a file, scope to that file only.

## Step 1: Two-Pass Review

### Pass 1: CRITICAL (blocks commit)

These are constitution violations and hard coding standard rules:

- [ ] No `unwrap()` or `expect()` in non-test code (constitution §5)
- [ ] No `panic!()` in library code (constitution §5)
- [ ] All new features have tests (constitution §2 — TDD)
- [ ] Errors propagated with `Result<T, E>`, never swallowed (constitution §5)
- [ ] No unnecessary abstractions — Rule of Three (constitution §4)
- [ ] New dependencies justified per §8 (run `cargo tree -d` if Cargo.toml changed)
- [ ] Functions under 40 lines (coding-standard)
- [ ] Max 4 parameters per function (coding-standard)
- [ ] All `pub` items have `///` doc comments (coding-standard)
- [ ] Error types use `thiserror` (coding-standard)

### Pass 2: INFORMATIONAL (should fix, doesn't block)

- [ ] Imports grouped: std → external → internal (coding-standard)
- [ ] Early returns over nested conditionals (coding-standard)
- [ ] `pub(crate)` preferred over `pub` (coding-standard)
- [ ] `&str` over `String` in function parameters (coding-standard)
- [ ] Naming conventions followed (coding-standard)
- [ ] Code is in the correct module per `docs/tech-stack.md`
- [ ] Module `mod.rs` files only re-export, no logic
- [ ] Dead code removed (not commented out)
- [ ] Errors go to stderr, data to stdout (CLI standard)
- [ ] Changes align with current roadmap phase

## Step 2: Classify Findings

For each finding, classify as:

- **AUTO-FIX**: Mechanical issues with one obvious fix (missing doc comment, import ordering,
  `pub` → `pub(crate)`, formatting). Apply immediately.
- **ASK**: Judgment calls, architectural decisions, or changes that alter behavior
  (function extraction, error handling strategy, dependency additions). Batch into
  one AskUserQuestion.

## Step 3: Auto-Fix All AUTO-FIX Items

Apply each fix directly. For each, output one line:
```
[AUTO-FIXED] file:line — Problem → what you did
```

## Step 4: Batch-Ask About ASK Items

If ASK items exist, present them in ONE AskUserQuestion:

```
I auto-fixed N issues. M need your input:

1. [CRITICAL] src/scanner/mod.rs:42 — Function exceeds 40 lines (67 lines)
   Fix: Extract lines 28-52 into a helper function
   → A) Fix as recommended  B) Skip

2. [INFORMATIONAL] src/metrics/loc.rs:15 — No test for edge case (empty file)
   Fix: Add unit test for empty input
   → A) Fix  B) Skip

RECOMMENDATION: Fix both — #1 is a coding standard violation, #2 prevents a regression.
```

## Step 5: Apply User-Approved Fixes

Apply fixes for items where the user chose "Fix."

## Step 6: Health Score

Score each category (0-100), then compute weighted average:

| Category | Weight | Score Method |
|----------|--------|-------------|
| Constitution compliance | 30% | -25 per CRITICAL finding |
| Coding standard | 25% | -15 per violation |
| Test coverage | 20% | -20 per untested new function |
| Documentation | 15% | -10 per undocumented pub item |
| Architecture fit | 10% | -15 per structural violation |

## Step 7: Completion Summary

```
+====================================================+
|              CODE REVIEW SUMMARY                    |
+====================================================+
| Scope          | 5 files, 142 lines changed         |
| Findings       | 8 total (3 critical, 5 info)       |
| Auto-fixed     | 5                                  |
| Asked          | 2 (both fixed)                     |
| Skipped        | 1                                  |
+----------------------------------------------------+
| Category               | Score |                   |
|------------------------|-------|                   |
| Constitution           | 100   |                   |
| Coding standard        |  85   |                   |
| Test coverage          |  80   |                   |
| Documentation          |  90   |                   |
| Architecture fit       | 100   |                   |
+----------------------------------------------------+
| HEALTH SCORE: 91/100                               |
| VERDICT: APPROVED — ready to commit                |
+====================================================+
```

Verdicts:
- **APPROVED** (90+): Ready to commit.
- **APPROVED WITH NOTES** (70-89): Commit OK, but address skipped items soon.
- **CHANGES REQUESTED** (<70): Fix critical issues before committing.

## Important Rules

1. **Read the FULL diff before commenting.** Do not flag issues already addressed.
2. **Fix-first, not read-only.** AUTO-FIX items are applied directly. Never commit or push — that's the user's job.
3. **Be terse.** One line per finding. No preamble.
4. **Only flag real problems.** Skip anything that's fine.
5. **Constitution violations are always CRITICAL.** No exceptions.
6. **Cross-reference ROADMAP.md.** Flag changes that don't align with the active phase.
7. **Never modify `docs/constitution.md`.** It is immutable. If you see an edit to it in the diff, flag as CRITICAL.
