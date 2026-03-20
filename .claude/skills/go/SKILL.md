---
name: go
version: 5.0.0
description: |
  Autonomous project driver. Reads state, picks the next task, delegates to
  the right skill, updates progress, and loops until the phase is complete.
  Uses branch-per-phase strategy. Run at any moment — picks up where it left off.
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

## Critical Behavior: NEVER STOP BETWEEN TASKS

**When a delegated skill (like /add-feature) completes, DO NOT:**
- Print a summary to the user
- Wait for user input
- Output any text explaining what happened

**Instead, IMMEDIATELY:**
1. Record the result (ROADMAP checkbox, state.md, commit)
2. Loop back to pick the next task
3. Only stop when the phase is complete or a blocker is hit

This is the #1 most important rule. The whole point of /go is autonomous
execution. Every pause between tasks requires a new `/go` invocation from
the user, which defeats the purpose.

---

## What This Skill Does

`/go` is an autonomous loop that drives the project forward. It reads state,
picks the next task, delegates to the right skill, commits, and **loops back**
to pick the next task — until the phase is complete or a blocker is hit.

It does NOT implement features itself — it delegates to specialists:

| Situation | Delegates to |
|-----------|-------------|
| Need a feature implemented | **`/add-feature <description>`** |
| Need a feature designed first | **`/spec <feature>`** |
| Need a new module created | **`/add-module <name>`** |
| Need to verify code quality | **`/check`** or **`/test`** |
| Need pre-commit review | **`/review`** |
| Need to clean up code | **`/refactor <target>`** |
| Need structural health check | **`/analyze-arch`** |
| Something is broken | **`/fix <description>`** |
| Phase is complete, time to PR | **`/pr`** |
| Scaffolding (Cargo.toml, CI) | Direct execution (no skill needed) |

---

## The Loop

```
┌─────────────────────────────────────────┐
│  START: Assess (read state, branch)     │
└──────────────┬──────────────────────────┘
               ▼
┌─────────────────────────────────────────┐
│  Pick next unchecked task from ROADMAP  │◄────────────────┐
└──────────────┬──────────────────────────┘                 │
               ▼                                            │
┌─────────────────────────────────────────┐                 │
│  Delegate to skill (/add-feature, etc)  │                 │
└──────────────┬──────────────────────────┘                 │
               ▼                                            │
┌─────────────────────────────────────────┐     ┌───────────┴──────┐
│  Quality gate passes?                   │─No─▶│  Fix or STOP     │
└──────────────┬──────────────────────────┘     └──────────────────┘
               │ Yes
               ▼
┌─────────────────────────────────────────┐
│  Record: ROADMAP ✓, state, commit       │
│  (NO output to user, NO summary)        │
└──────────────┬──────────────────────────┘
               ▼
┌─────────────────────────────────────────┐     ┌──────────────────┐
│  More tasks in phase?                   │─No─▶│  Phase done →    │
└──────────────┬──────────────────────────┘     │  finalize & STOP │
               │ Yes                            └──────────────────┘
               └────────────────────────────────────────────┘
```

---

## Step 1: Assess — Where Are We? (run once at start)

Read state and project context:

```bash
cat .repostat/state.md 2>/dev/null || echo "NO_STATE"
```

```bash
[ -f Cargo.toml ] && echo "CARGO_EXISTS" || echo "NO_CARGO"
ls src/ 2>/dev/null || echo "NO_SRC"
git log --oneline -5 2>/dev/null || echo "NO_GIT_HISTORY"
```

Read `ROADMAP.md` to find the active phase (first phase with unchecked `- [ ]` items).

**If NO_STATE:** Create `.repostat/state.md` using the State File Format below.

**If state exists:** Read it. The `## Current Task` section tells you what to resume.
- `status: in-progress` → resume that exact task
- `status: blocked` → read the blocker, try to unblock or skip
- `status: idle` → pick the next unchecked ROADMAP item

---

## Step 1b: Branch Management (run once at start)

This project uses **branch-per-phase** strategy:

```bash
git branch --show-current
```

**Expected branch:** `phase-N/<slug>` (e.g., `phase-1/foundation`, `phase-2/complexity`)

**If on `main` and starting a new phase:** Create the branch.
**If on the correct phase branch:** Continue working.
**If on a different phase branch:** Something is wrong. Ask the user.

---

## LOOP START — Repeat for each task until phase complete

### Step 2: Plan — What's Next?

Find the next task: the first unchecked `- [ ]` item in the active ROADMAP phase.

Classify it to pick the right skill:

| Task pattern | Skill |
|-------------|-------|
| "scaffold", "module structure" | Direct + `/add-module` |
| Any feature or enhancement | `/add-feature` |
| "CI pipeline" | Direct execution |

Compose the task description from ROADMAP.md + matching requirements in `docs/requirements.md`.

---

### Step 3: Execute — Delegate to the Right Skill

Invoke the skill. When the skill completes:

**DO NOT print anything. DO NOT summarize. Go straight to Step 4.**

---

### Step 4: Verify — Quality Gate

The delegated skill already runs the quality gate internally. But verify:

```bash
cargo fmt --check && cargo clippy -- -D warnings && cargo test
```

If it fails, fix the issue directly (don't delegate to /fix for simple clippy/fmt issues).
If it fails twice consecutively, mark as blocked and STOP.

**Periodic maintenance** (counted across the entire session):
- Every 3rd completed task: also run `/review`
- Every 5th completed task: also run `/refactor` and `/analyze-arch`

---

### Step 5: Record — Update Everything (silently)

Do all of these without printing anything to the user:

**5a.** Mark ROADMAP.md checkbox: `- [ ]` → `- [x]`
**5b.** Update `.repostat/state.md` — completed task, next task, learnings
**5c.** Commit with Conventional Commit message ending with:
```
Co-Authored-By: Claude Opus 4.6 (1M context) <noreply@anthropic.com>
```

---

### Step 6: Loop Decision

**IMMEDIATELY loop back to Step 2 if:**
- More unchecked tasks remain in the current phase
- No blockers

**STOP only if:**
- Phase complete → go to Phase Completion below
- Blocker hit → record in state.md, print summary, ask user
- 2+ consecutive failures → record, print summary, ask user
- Ambiguity on a task → ask user

---

## LOOP END

---

## Phase Completion

When all items in a phase are checked:

1. Run `/test` for full test suite verification
2. Run `/analyze-arch` to verify structural health
3. Run `/review` for a final review pass
4. Check exit criteria from ROADMAP.md
5. Use AskUserQuestion:
   - "Phase N complete. Exit criteria: [list]. Ready to ship?"
   - A) Run `/pr` to open PR
   - B) Keep polishing
   - C) Stop here

---

## Status Summary

Print this ONLY when stopping (phase complete, blocker, or error):

```
+====================================================+
|              /go — SESSION SUMMARY                  |
+====================================================+
| Phase        | N: <phase name>                      |
| Tasks done   | X this session                       |
| Total done   | Y / Z in phase                       |
| Next task    | <next or "Phase complete">            |
| Blockers     | <None or description>                |
+----------------------------------------------------+
| Quality gate | PASS/FAIL                            |
| Commits      | N new                                |
+====================================================+
```

---

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
- [ ] Task 2 — next

## Learnings

- YYYY-MM-DD: <learning>

## Session Log

| Date | Tasks Completed | Notes |
|------|----------------|-------|
| YYYY-MM-DD | Task 1, Task 2 | Brief note |
```

---

## Important Rules

1. **NEVER STOP BETWEEN TASKS.** When a skill finishes, immediately record and loop.
2. **NEVER print summaries mid-loop.** Only print the session summary when truly stopping.
3. **Delegate, don't duplicate.** Use skills. Don't reimplement their logic.
4. **Always read state first.** Never assume where the project is.
5. **One task at a time.** Complete via skill before moving on.
6. **Commit after each task.** Small, atomic commits.
7. **Update ROADMAP.md checkboxes.** Source of truth for progress.
8. **Don't skip tasks.** Phase ordering matters.
9. **Stop gracefully on errors.** Always leave state clean.
10. **Ship at phase boundaries.** Use `/pr` to PR the phase branch to main.
