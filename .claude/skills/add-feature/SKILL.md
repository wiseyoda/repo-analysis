---
name: add-feature
version: 2.0.0
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
**If tests fail:** "Existing tests are failing. Fix them first." — STOP.

## Step 0: Scope Check

Read `docs/requirements.md` and `ROADMAP.md`.

1. **Find matching requirement IDs** for $ARGUMENTS.
2. **Check roadmap phase.** If not in active phase, ask user.
3. **Check for existing spec** in `docs/specs/`.

## Step 1: Write Spec (SDD)

If no existing spec found, create one at `docs/specs/<feature-slug>.md`:

```markdown
# Spec: <Feature Name>

**Requirement IDs:** R-NNN, R-NNN
**Phase:** N
**Date:** YYYY-MM-DD

## Purpose

## Inputs

## Outputs

## Behavior

## Edge Cases

## Acceptance Criteria

- [ ] Criterion 1
```

## Step 2: Write Failing Tests (TDD — Red)

Write tests that encode the spec's acceptance criteria.

- **Unit tests**: inline `#[cfg(test)] mod tests` in the source file
- **Integration tests**: `tests/` if the feature involves CLI behavior

```bash
cargo test 2>&1
```

Confirm the new tests FAIL.

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
- Simplify where possible

## Step 5: Full Quality Gate

```bash
cargo fmt --check && cargo clippy -- -D warnings && cargo test
```

All three must pass. If `cargo fmt --check` fails, run `cargo fmt`.
If clippy fails, fix the issues. Do NOT leave clippy warnings.

**When the quality gate passes, the feature is done. Stop here.**

Do NOT print a summary box. The caller (/go) handles recording and continuation.

## Important Rules

1. **Tests come FIRST.** Write failing tests before any implementation.
2. **Spec comes before tests.** Write the spec before the tests.
3. **Tests must fail first.** If new tests pass immediately, they're not testing anything new.
4. **Run tests after every change.** Green → change → green → change → green.
5. **MINIMUM code to pass.** Don't over-build.
6. **Never skip the quality gate.** fmt + clippy + test must all pass.
7. **Don't modify constitution.md.** It is immutable.
8. **No summary box.** Just finish when the quality gate passes.

When finished, do not end the session, continue on to the next skill controlled by /go skill.
