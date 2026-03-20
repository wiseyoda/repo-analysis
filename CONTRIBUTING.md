# Contributing to repostat

## Quick Start

```bash
git clone https://github.com/wiseyoda/repo-analysis.git
cd repo-analysis
cargo build
cargo test
```

**Requirements:** Rust 1.85+ (edition 2024), git, cargo.

## Mandatory Reading

Before writing any code, read these in order:

1. `docs/constitution.md` — Immutable project principles
2. `docs/coding-standard.md` — Code style and rules
3. `docs/tech-stack.md` — Architecture and dependencies

## Development Workflow

### With Claude Code (recommended)

```bash
/status          # See where the project is
/go              # Pick up the next task and work on it
/ship            # Open a PR when ready
```

### Manual workflow

1. **Branch:** Create a branch from main: `git checkout -b phase-N/<feature>`
2. **Spec:** Write a spec in `docs/specs/` before coding (SDD)
3. **Test:** Write failing tests before implementation (TDD)
4. **Implement:** Minimum code to pass tests
5. **Verify:** `cargo fmt --check && cargo clippy -- -D warnings && cargo test`
6. **Commit:** Conventional Commits (`feat:`, `fix:`, `test:`, `refactor:`, `docs:`, `chore:`)
7. **PR:** Open a PR against main

## Code Rules

These are enforced by hooks and CI. They are not negotiable.

- No `unwrap()` or `expect()` outside `#[cfg(test)]`
- No `panic!()` in library code
- All `pub` items have `///` doc comments
- Functions max 40 lines
- Max 4 parameters per function
- `cargo fmt`, `cargo clippy -- -D warnings`, and `cargo test` must all pass

## Adding Dependencies

Every new dependency must satisfy `docs/constitution.md` §8:

1. Is it well-maintained?
2. Does it do something non-trivial to implement?
3. Is its transitive dependency tree reasonable?

Add a comment above the dependency in `Cargo.toml` explaining why it's needed.

## Branching Strategy

- `main` — stable, all tests pass
- `phase-N/<description>` — work branches per roadmap phase
- PRs merge to main via squash merge

## Commit Messages

```
feat(scanner): add gitignore-aware file walking
fix(metrics): handle empty files without panicking
test(loc): add edge cases for Unicode comments
refactor(config): split into submodules
docs: update architecture diagram
chore: bump version to 0.2.0
```

## Project Structure

See `docs/tech-stack.md` for the full directory layout and architecture diagram.
