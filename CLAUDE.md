# CLAUDE.md — repostat

> Instructions for AI agents working on this project.

## Project

`repostat` is a Rust CLI tool that analyzes repository complexity, tracks coding progress
over time, and produces AI-augmented reports. See `ROADMAP.md` for current phase.

## Mandatory Reading

Before writing ANY code, read these documents in order:

1. **`docs/constitution.md`** — Immutable principles. Never violate these.
2. **`docs/coding-standard.md`** — Code style, error handling, testing rules.
3. **`docs/tech-stack.md`** — Architecture, dependencies, directory structure.
4. **`docs/requirements.md`** — What we're building, in what order.
5. **`docs/decisions.md`** — Why we made the choices we made.

## Enforcement

This project uses automated enforcement via `.claude/` configuration:

- **Hooks** (`.claude/hooks/`) — Automated checks that run during development:
  - `rust-safety.sh` — Blocks `unwrap()`, `expect()`, `panic!()` in non-test Rust code
  - `protect-constitution.sh` — Blocks any edits to `docs/constitution.md`
  - `pre-commit-check.sh` — Runs fmt + clippy + test before any `git commit`
  - `post-edit-fmt.sh` — Auto-formats Rust files after edits
  - `session-start.sh` — Injects project context at session start

- **Rules** (`.claude/rules/`) — Path-specific instructions loaded contextually:
  - `rust-source.md` — Loaded when editing `src/**/*.rs`
  - `test-files.md` — Loaded when editing `tests/**/*.rs`
  - `documentation.md` — Loaded when editing `docs/**/*.md`
  - `cargo-config.md` — Loaded when editing `Cargo.toml`

- **Skills** (`.claude/skills/`) — Project-specific slash commands:
  - **`/go`** — Autonomous orchestrator. Picks up where it left off, does what's next.
  - **`/status`** — Read-only project dashboard. Where are we, what's next.
  - `/spec` — Write a feature spec (SDD) without implementing
  - `/add-module` — Scaffold a new module following conventions
  - `/add-feature` — Add a feature following TDD + SDD workflow
  - `/fix` — Debug workflow: reproduce, test, fix, verify
  - `/test` — Run full quality gate (fmt + clippy + test)
  - `/check` — Quick pass/fail quality gate report
  - `/review` — Pre-commit code review against project standards
  - `/refactor` — Refactor with continuous test verification
  - `/analyze-arch` — Architecture analysis against tech-stack.md
  - **`/ship`** — Push branch, open PR, bump version, update changelog

- **State** (`.repostat/state.md`) — Persistent project state across sessions.
  Updated by `/go`. Tracks current task, progress, learnings, and session log.

## Workflow

### Before Writing Code

1. Check `ROADMAP.md` to confirm which phase is active.
2. Read the relevant spec in `docs/specs/` if one exists.
3. If no spec exists, write one first (SDD — Spec Driven Development).

### Writing Code

1. **Write the test first** (TDD). The test must fail before you write implementation.
2. Write the minimum code to make the test pass.
3. Refactor while tests remain green.
4. Run the full check suite before considering the work done:
   ```bash
   cargo fmt --check && cargo clippy -- -D warnings && cargo test
   ```

### Code Rules (Non-Negotiable)

- No `unwrap()` or `expect()` outside `#[cfg(test)]` blocks.
- No `panic!()` in library code.
- All `pub` items have `///` doc comments.
- Functions max 40 lines. Extract if longer.
- Max 4 parameters per function. Use a struct beyond that.
- Early returns over nested conditionals.
- Errors go to stderr. Data goes to stdout.

### Committing

- Conventional Commits: `feat:`, `fix:`, `refactor:`, `test:`, `docs:`, `chore:`, `perf:`
- The pre-commit hook will automatically run fmt, clippy, and tests.
- If the hook blocks the commit, fix the issues before retrying.

## Architecture Quick Reference

```
src/
  main.rs          → Entry point, CLI setup (clap)
  cli.rs           → Argument definitions
  config.rs        → .repostat.toml parsing
  errors.rs        → Error types (thiserror)
  scanner/         → File walking, language detection, exclusions
  metrics/         → LOC counting, complexity, dependencies
  ai/              → Claude CLI invocation, skill files, response parsing
  snapshot/        → JSON snapshot read/write/diff
  report/          → Dashboard, markdown, trends
```

## Key Decisions

- **Rust** for speed and single-binary distribution (ADR-001)
- **Tree-sitter** for multi-language complexity analysis (ADR-002)
- **Claude CLI** (`claude -p`) for AI analysis, not direct API (ADR-003)
- **JSON files** for snapshot storage, not SQLite (ADR-004)
- **Three-layer exclusions**: gitignore → heuristics → config (ADR-005)
- **Skill files** in `~/.repostat/skills/` for AI prompts (ADR-006)
- **Fast model always** for AI analysis speed (ADR-007)

## What NOT To Do

- Don't add dependencies without checking `docs/constitution.md` §8.
- Don't skip tests. Ever. For any reason.
- Don't use `String` where `&str` suffices in function parameters.
- Don't create abstractions for single-use code.
- Don't add features not in the current roadmap phase.
- Don't modify `docs/constitution.md`. It is immutable.
