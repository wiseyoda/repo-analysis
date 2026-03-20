---
name: refactor
version: 1.0.0
description: |
  Refactor code while preserving all behavior. Tests must pass after every change.
  Includes self-regulation to stop before making things worse. Use when asked to
  "refactor", "clean up", "simplify", or "restructure".
disable-model-invocation: true
user-invocable: true
allowed-tools:
  - Read
  - Write
  - Edit
  - Bash(cargo *)
  - Bash(git diff*)
  - Bash(git status*)
  - Glob
  - Grep
  - AskUserQuestion
---

## Arguments

- `/refactor <target>` — refactor the specified file, module, or function
- `/refactor` — scan the codebase for the highest-impact refactoring opportunity
- If no argument: scan `src/` for files exceeding thresholds (500+ lines, functions 40+ lines, deep nesting)

## Preconditions

```bash
[ -f Cargo.toml ] && echo "CARGO_OK" || echo "NO_CARGO"
cargo test 2>&1 | tail -1
```

**If NO_CARGO:** "No Cargo.toml found." — STOP.
**If tests fail:** "Tests are already failing. Fix them before refactoring." — STOP.

## Step 0: Baseline Metrics

Capture before-state for comparison:

```bash
# Count lines per file in target area
wc -l src/**/*.rs 2>/dev/null | sort -rn | head -20
# Count total tests
cargo test 2>&1 | grep "test result"
```

Record: total lines, file count, test count, largest file, longest function.

## Step 1: Identify Targets

Scan the target code for refactoring opportunities. Check for:

- [ ] Functions over 40 lines → extract sub-functions
- [ ] More than 4 parameters → introduce options struct
- [ ] Deep nesting (3+ levels) → early returns
- [ ] Duplicated logic (3+ occurrences) → extract shared function
- [ ] Raw types where newtypes would add clarity
- [ ] `String` parameters that should be `&str`
- [ ] Missing error context in `?` propagation → add `.map_err()` or `.context()`
- [ ] Dead code → delete it (don't comment it out)
- [ ] `pub` that should be `pub(crate)`
- [ ] Missing doc comments on `pub` items

Rank by impact. Prioritize: correctness fixes > readability > style.

## Step 2: Refactoring Loop

For each target (in priority order):

### 2a. Describe the change
One sentence: what will change and why.

### 2b. Make ONE change
Apply one focused refactoring. Do not bundle multiple refactorings.

### 2c. Run tests
```bash
cargo test 2>&1
```
- **PASS** → continue to 2d
- **FAIL** → revert immediately: `git checkout -- <changed-files>`. Log the failure. Move to next target.

### 2d. Self-Regulation Check

Run this after every 5 changes or after any revert:

```
WTF-LIKELIHOOD:
  Start at 0%
  Each revert:                     +20%
  Each change touching >3 files:   +10%
  After change 15:                 +2% per additional change
  Touching files outside target:   +15%
```

**If WTF > 25%:** STOP immediately. Show what you've done. Ask the user:
- "I've made N changes with M reverts. Continuing risks introducing bugs."
- A) Continue carefully — I trust the test suite
- B) Stop here — ship what's done
- RECOMMENDATION: Choose B because refactoring should leave things better, not riskier.

**Hard cap: 25 changes.** After 25 changes, stop regardless.

## Step 3: After Metrics

Capture after-state:

```bash
wc -l src/**/*.rs 2>/dev/null | sort -rn | head -20
cargo test 2>&1 | grep "test result"
```

## Step 4: Full Quality Gate

```bash
cargo fmt --check && cargo clippy -- -D warnings && cargo test
```

All three must pass. Fix any formatting issues.

## Step 5: Completion Summary

```
+====================================================+
|            REFACTORING SUMMARY                      |
+====================================================+
| Target       | <what was refactored>                |
| Changes      | N applied, M reverted                |
| WTF score    | X%                                   |
+----------------------------------------------------+
| BEFORE                | AFTER                       |
|-----------------------|-----------------------------|
| Largest file: 420 ln  | Largest file: 280 ln        |
| Longest fn:   67 ln   | Longest fn:    38 ln        |
| Total lines:  2,100   | Total lines:   1,950        |
| Tests:        42      | Tests:         42           |
+----------------------------------------------------+
| Changes applied:                                    |
|  1. Extracted parse_config() from main() (67→38 ln) |
|  2. Replaced String params with &str in scanner     |
|  3. Deleted dead code in metrics/loc.rs (-23 lines) |
+----------------------------------------------------+
| fmt:    PASS                                        |
| clippy: PASS                                        |
| tests:  PASS (42 passed, 0 failed)                  |
+----------------------------------------------------+
| VERDICT: REFACTORING COMPLETE                       |
+====================================================+
```

## Important Rules

1. **Tests must pass before you start.** If tests fail, STOP.
2. **Tests must pass after EVERY change.** No exceptions.
3. **No behavior changes.** Refactoring changes structure, not behavior.
4. **Revert on failure.** If a change breaks tests, revert it immediately.
5. **One change at a time.** Each refactoring is independently verifiable.
6. **Self-regulate.** Follow the WTF-likelihood heuristic. When in doubt, stop.
7. **Never bundle reverts.** If you revert, log what happened and move on.
8. **Hard cap: 25 changes.** Stop after 25 regardless of remaining opportunities.
