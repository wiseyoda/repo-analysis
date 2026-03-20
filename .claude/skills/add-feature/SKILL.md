---
name: add-feature
version: 1.0.0
description: |
  Add a new feature using the full TDD + SDD workflow. Creates spec, writes
  failing tests, implements, refactors, and verifies. Use when asked to
  "add", "implement", "build", or "create" a feature.
disable-model-invocation: false
user-invocable: true
allowed-tools:
  - Read
  - Write
  - Edit
  - Bash(cargo *)
  - Bash(mkdir *)
  - Bash(git diff*)
  - Bash(git status*)
  - Glob
  - Grep
  - AskUserQuestion
---

## Arguments

- `/add-feature <description>` — describe the feature to add
- If no argument: STOP with "Usage: `/add-feature <description>`"

## Preconditions

```bash
[ -f Cargo.toml ] && echo "CARGO_OK" || echo "NO_CARGO"
cargo test 2>&1 | tail -1
```

**If NO_CARGO:** "No Cargo.toml found." — STOP.
**If tests fail:** "Existing tests are failing. Fix them first before adding new features." — STOP.

## Step 0: Scope Check

Read `docs/requirements.md` and `ROADMAP.md`.

1. **Find matching requirement IDs** for $ARGUMENTS. If the feature maps to specific R-NNN IDs, note them.
2. **Check roadmap phase.** Is this feature in the current active phase? If not, use AskUserQuestion:
   - "This feature appears to be in Phase N, but the active phase is Phase M."
   - A) Proceed anyway — it's related to current work
   - B) Abort — focus on the active phase
   - RECOMMENDATION: Choose B because constitution §4 says we don't add features outside the current phase.

3. **Check for existing spec.** Look in `docs/specs/` for a related spec file.

## Step 1: Write Spec (SDD)

If no existing spec found, create one at `docs/specs/<feature-slug>.md`:

```markdown
# Spec: <Feature Name>

**Requirement IDs:** R-NNN, R-NNN
**Phase:** N
**Date:** YYYY-MM-DD

## Purpose
What this feature does and why it exists.

## Inputs
What data/arguments this feature accepts.

## Outputs
What this feature produces.

## Behavior
Step-by-step description of the feature's behavior.

## Edge Cases
- Empty input
- Invalid input
- Large input
- Missing dependencies

## Acceptance Criteria
- [ ] Criterion 1
- [ ] Criterion 2
```

## Step 2: Write Failing Tests (TDD — Red)

Write tests that encode the spec's acceptance criteria.

- **Unit tests**: inline `#[cfg(test)] mod tests` in the source file
- **Integration tests**: `tests/integration/` if the feature involves CLI behavior

```bash
cargo test 2>&1
```

Confirm the new tests FAIL. If they pass, the tests aren't testing anything new — rewrite them.

## Step 3: Implement (TDD — Green)

Write the MINIMUM code to make tests pass. Follow `docs/coding-standard.md`:
- No `unwrap()`, no `panic!()` in library code
- `thiserror` for error types
- Functions under 40 lines
- Max 4 parameters

```bash
cargo test 2>&1
```

Confirm all tests pass (both new and existing).

## Step 4: Refactor (TDD — Refactor)

While tests stay green:
- Extract functions if anything exceeds 40 lines
- Check for duplication across the codebase
- Simplify where possible

```bash
cargo test 2>&1
```

Confirm tests still pass after each refactoring step.

## Step 5: Full Quality Gate

```bash
cargo fmt --check && cargo clippy -- -D warnings && cargo test
```

All three must pass. If `cargo fmt --check` fails, run `cargo fmt` to fix.

## Step 6: Completion Summary

```
+====================================================+
|           FEATURE IMPLEMENTATION SUMMARY            |
+====================================================+
| Feature     | <description>                         |
| Req IDs     | R-NNN, R-NNN                          |
| Phase       | N                                     |
+----------------------------------------------------+
| Files created  | 2 (list them)                      |
| Files modified | 3 (list them)                      |
| Tests added    | 5                                  |
| Lines added    | +142                               |
+----------------------------------------------------+
| Spec          | docs/specs/<slug>.md                  |
| fmt           | PASS                                 |
| clippy        | PASS                                 |
| tests         | PASS (47 total, 5 new)               |
+----------------------------------------------------+
| VERDICT: READY TO COMMIT                           |
+====================================================+
```

## Important Rules

1. **Tests come FIRST.** Write failing tests before any implementation. This is TDD — not optional.
2. **Spec comes before tests.** Write the spec before the tests. This is SDD — not optional.
3. **Tests must fail first.** If new tests pass immediately, they're not testing the new feature.
4. **Run tests after every change.** Green → change → green → change → green.
5. **MINIMUM code to pass.** Don't over-build. Constitution §4: simplicity is non-negotiable.
6. **Never skip the quality gate.** fmt + clippy + test must all pass.
7. **Don't modify constitution.md.** It is immutable.
