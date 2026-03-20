---
name: refactor
description: Refactor code while preserving all behavior. Tests must stay green.
disable-model-invocation: true
user-invocable: true
allowed-tools: Read, Write, Edit, Bash(cargo *), Glob, Grep
---

Refactor: $ARGUMENTS

## Rules (non-negotiable)

1. **Tests pass before you start.** Run `cargo test` first. If tests fail, stop and report.
2. **Tests pass after every change.** Run `cargo test` after each refactoring step.
3. **No behavior changes.** Refactoring changes structure, not behavior.
4. **Small steps.** Each change should be independently verifiable.

## Refactoring Checklist

Look for and fix these issues in the target code:

- [ ] Functions over 40 lines → extract sub-functions
- [ ] More than 4 parameters → introduce options struct
- [ ] Deep nesting (3+ levels) → early returns
- [ ] Duplicated logic → extract shared function (only if 3+ occurrences)
- [ ] Raw types where newtypes would add clarity
- [ ] `String` parameters that should be `&str`
- [ ] Missing error context in `?` propagation → add `.map_err()` or `.context()`
- [ ] Dead code → delete it (don't comment it out)
- [ ] `pub` that should be `pub(crate)`
- [ ] Missing doc comments on `pub` items

## Process

1. Run `cargo test` — confirm green
2. Identify specific refactoring targets
3. Make ONE refactoring change
4. Run `cargo test` — confirm still green
5. Repeat 3-4 until done
6. Run full suite: `cargo fmt --check && cargo clippy -- -D warnings && cargo test`
7. Report: what changed, why, before/after metrics
