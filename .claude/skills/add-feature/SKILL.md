---
name: add-feature
description: Add a new feature using TDD and SDD workflow
disable-model-invocation: true
user-invocable: true
allowed-tools: Read, Write, Edit, Bash(cargo *), Bash(mkdir *), Glob, Grep
---

Add a new feature to repostat following the full TDD + SDD workflow.

**Feature**: $ARGUMENTS

## Workflow (follow this order exactly)

### Step 1: Spec (SDD)
- Read `docs/requirements.md` to find the relevant requirement IDs.
- Read `ROADMAP.md` to confirm this feature is in the current phase.
- Create a spec in `docs/specs/` describing WHAT the feature does, not HOW.
- The spec must include: purpose, inputs, outputs, edge cases, acceptance criteria.

### Step 2: Test First (TDD — Red)
- Write failing tests that encode the spec's acceptance criteria.
- Run `cargo test` to confirm they fail.
- Tests must be in the correct location:
  - Unit tests: inline `#[cfg(test)] mod tests` in the source file
  - Integration tests: `tests/integration/`

### Step 3: Implement (TDD — Green)
- Write the MINIMUM code to make tests pass.
- Follow `docs/coding-standard.md` strictly.
- No `unwrap()`, no `panic!()` in library code.
- Use `thiserror` for error types.

### Step 4: Refactor (TDD — Refactor)
- Clean up while keeping tests green.
- Extract functions if anything exceeds 40 lines.
- Check for duplication across the codebase.

### Step 5: Verify
- Run `cargo fmt --check`
- Run `cargo clippy -- -D warnings`
- Run `cargo test`
- All three must pass.

### Step 6: Report
- Summarize what was built and which requirement IDs it satisfies.
- List all new files and modified files.
- Show test results.
