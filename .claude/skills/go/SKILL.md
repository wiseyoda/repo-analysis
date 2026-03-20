---
name: go
version: 1.0.0
description: |
  Autonomous orchestrator. Reads project state, determines what needs to happen
  next, and does it. Can be run at any moment — picks up where it left off.
  Follows TDD+SDD, updates ROADMAP.md checkboxes, and persists learnings.
  Use when asked to "go", "continue", "work on the project", or "what's next".
user-invocable: true
allowed-tools:
  - Bash
  - Read
  - Write
  - Edit
  - Glob
  - Grep
  - AskUserQuestion
  - Skill
  - Agent
---

## How This Skill Works

You are the project's autonomous driver. When invoked, you:

1. **Assess** — Read state, figure out where we are
2. **Plan** — Pick the next task to work on
3. **Execute** — Do the work (TDD+SDD, following all project standards)
4. **Verify** — Run quality gate
5. **Record** — Update state, ROADMAP.md, and learnings
6. **Continue or stop** — If context budget allows, pick the next task. Otherwise, save state and stop gracefully.

## Step 1: Load State

Read these files to understand current project state:

```bash
cat .repostat/state.md 2>/dev/null || echo "NO_STATE"
cat ROADMAP.md
ls src/ 2>/dev/null || echo "NO_SRC"
[ -f Cargo.toml ] && echo "CARGO_EXISTS" || echo "NO_CARGO"
git log --oneline -5 2>/dev/null || echo "NO_COMMITS"
```

**If NO_STATE:** This is a fresh start. Create `.repostat/state.md` with the initial state template (see State File Format below). The first task is always the first unchecked item in Phase 1 of ROADMAP.md.

**If state exists:** Read it. The `## Current Task` section tells you exactly what to resume.

## Step 2: Determine Next Action

Parse ROADMAP.md. Find the active phase (first phase with unchecked `- [ ]` items). Find the first unchecked item in that phase. That's your task.

Cross-reference with `.repostat/state.md`:
- If `status: in-progress` — resume that exact task
- If `status: blocked` — read the blocker, try to resolve it, or skip to next task
- If `status: idle` — pick the next unchecked ROADMAP item

**Task ordering within a phase matters.** Earlier items are foundations for later items. Don't skip ahead.

## Step 3: Execute the Task

Every task follows the same workflow. Adapt based on task type:

### For new code (features, modules, engines):

**3a. Spec (if none exists)**
Check `docs/specs/` for an existing spec. If missing, write one:
- `docs/specs/<task-slug>.md` with: Purpose, Inputs, Outputs, Behavior, Edge Cases, Acceptance Criteria
- Reference requirement IDs from `docs/requirements.md`

**3b. Write failing tests (TDD Red)**
- Unit tests: inline `#[cfg(test)] mod tests` in the implementation file
- Integration tests: `tests/integration/` for CLI-level behavior
- Run `cargo test` — confirm the new tests FAIL

**3c. Implement (TDD Green)**
- Write minimum code to make tests pass
- Follow `docs/coding-standard.md` strictly
- No `unwrap()`, no `panic!()` in library code
- Use `thiserror` for error types
- Run `cargo test` — confirm ALL tests pass

**3d. Refactor (TDD Refactor)**
- Clean up while tests stay green
- Extract functions over 40 lines
- Check for duplication

### For scaffolding tasks (Cargo.toml, module structure, CI):

Skip the spec. Just build it following `docs/tech-stack.md` for the intended structure.
Still write tests where applicable (e.g., config parsing tests).

### For integration tasks (wiring modules together, CLI flags):

Write integration tests in `tests/integration/` using `assert_cmd`.

## Step 4: Quality Gate

After completing work on a task, run the full gate:

```bash
cargo fmt --check 2>&1 || cargo fmt
cargo clippy -- -D warnings 2>&1
cargo test 2>&1
```

**If clippy fails:** Fix the warnings. Re-run.
**If tests fail:** Fix the failures. Re-run.
**All must pass before marking a task complete.**

## Step 5: Update State

### 5a. Mark ROADMAP.md checkbox

Change `- [ ]` to `- [x]` for the completed task. Use the Edit tool with the exact checkbox text.

### 5b. Update `.repostat/state.md`

Update the Current Task section to either:
- The next unchecked task (if continuing)
- `status: idle` (if stopping)

Add any learnings to the `## Learnings` section.

### 5c. Commit the work

```bash
git add -A
git commit -m "<type>(<scope>): <description>"
```

Use Conventional Commits. The type depends on the task:
- `feat:` for new features
- `chore:` for scaffolding
- `test:` for test-only changes
- `refactor:` for refactoring

## Step 6: Continue or Stop

**Continue if:**
- There are more unchecked tasks in the current phase
- The conversation context is not getting large
- No blockers encountered

**Stop gracefully if:**
- The current phase is complete (all items checked) — celebrate, then note the phase exit criteria need verification
- A blocker is hit that requires user input
- The task is ambiguous and needs clarification
- You've completed 3+ substantial tasks in this session (to keep commits reviewable)

When stopping, always:
1. Ensure `.repostat/state.md` is current
2. Ensure ROADMAP.md checkboxes reflect actual state
3. Commit all work
4. Print a status summary

## State File Format

`.repostat/state.md`:

```markdown
# repostat — Project State

> Auto-maintained by /go. Persists across sessions.

## Current Task

- **Phase:** 1
- **Task:** <exact text from ROADMAP.md>
- **Status:** idle | in-progress | blocked
- **Blocker:** <if blocked, describe why>
- **Started:** YYYY-MM-DD

## Progress

### Phase 1: Foundation & Core Metrics
- [x] Task 1 — completed YYYY-MM-DD
- [x] Task 2 — completed YYYY-MM-DD
- [ ] Task 3 — next

### Phase 2: Complexity Analysis
- [ ] Not started

## Learnings

> Things discovered during implementation that future sessions need to know.
> These are NOT in the docs — they're runtime discoveries.

- YYYY-MM-DD: <learning>

## Session Log

| Date | Tasks Completed | Notes |
|------|----------------|-------|
| YYYY-MM-DD | Task 1, Task 2 | Initial scaffold |
```

## Status Summary Format

Print this when stopping:

```
+====================================================+
|              /go — SESSION SUMMARY                  |
+====================================================+
| Phase        | 1: Foundation & Core Metrics         |
| Tasks done   | 3 this session                       |
| Total done   | 5 / 15 in phase                      |
| Next task    | Line counting engine                 |
| Blockers     | None                                 |
+----------------------------------------------------+
| Quality gate | PASS (fmt ✓ clippy ✓ test ✓)         |
| Commits      | 3 new commits                        |
+----------------------------------------------------+
| Run /go again to continue.                         |
+====================================================+
```

## Phase Completion

When all items in a phase are checked off:

1. Read the phase's **Exit Criteria** from ROADMAP.md
2. Verify each criterion is actually met (run the tool, check output, etc.)
3. If criteria pass: print "Phase N complete!" and move to Phase N+1
4. If criteria don't pass: note what's missing, create a task to fix it
5. Use AskUserQuestion to confirm with the user before starting the next phase:
   - "Phase N is complete. All exit criteria met. Ready to start Phase N+1?"
   - A) Start Phase N+1
   - B) Review Phase N first — run /review or /analyze-arch
   - C) Stop here

## Handling Ambiguity

If a ROADMAP task is vague or you're unsure how to implement it:

1. Check `docs/requirements.md` for the detailed requirement
2. Check `docs/specs/` for an existing spec
3. Check `docs/decisions.md` for relevant ADRs
4. Check `.repostat/state.md` learnings for prior context

If still unclear: Use AskUserQuestion. Don't guess on architecture decisions.

## Important Rules

1. **Always read state first.** Never assume you know where the project is.
2. **TDD is mandatory.** Tests before implementation. No exceptions.
3. **One task at a time.** Complete fully (spec → test → implement → verify) before moving on.
4. **Commit after each task.** Small, atomic commits. Don't batch.
5. **Update ROADMAP.md checkboxes.** They are the source of truth for progress.
6. **Record learnings.** If you discover something non-obvious, write it down.
7. **Don't skip ahead.** Tasks within a phase are ordered by dependency.
8. **Stop gracefully.** Always leave state clean so the next /go picks up seamlessly.
9. **Never modify constitution.md.** It is immutable.
10. **Follow coding-standard.md.** Every line of code must conform.
