---
name: go
version: 4.0.0
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

## What This Skill Does

`/go` is an autonomous loop that drives the project forward. It reads state,
picks the next task, delegates to the right skill, commits, and **loops back**
to pick the next task — until the phase is complete or a blocker is hit.

It does NOT implement features itself — it delegates to specialists:

| Situation | Delegates to |
|-----------|-------------|
| Need to understand where we are | **`/status`** |
| Need a feature designed first | **`/spec <feature>`** |
| Need a new module created | **`/add-module <name>`** |
| Need a feature implemented | **`/add-feature <description>`** |
| Need to verify code quality | **`/check`** or **`/test`** |
| Need pre-commit review | **`/review`** |
| Need to clean up code | **`/refactor <target>`** |
| Need structural health check | **`/analyze-arch`** |
| Something is broken | **`/fix <description>`** |
| Phase is complete, time to PR | **`/ship`** |
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
│  Verify (/check), fix if needed         │────▶│  Blocked? → STOP │
└──────────────┬──────────────────────────┘     └──────────────────┘
               ▼
┌─────────────────────────────────────────┐
│  Record: ROADMAP ✓, state, commit       │
└──────────────┬──────────────────────────┘
               ▼
┌─────────────────────────────────────────┐     ┌──────────────────┐
│  More tasks in phase?                   │─No─▶│  Phase done →    │
└──────────────┬──────────────────────────┘     │  finalize & STOP │
               │ Yes                            └──────────────────┘
               │
               │  (periodic: /review every 3rd,
               │   /refactor every 5th task)
               │
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

**If on `main` and starting a new phase:** Create the branch:
```bash
git checkout -b phase-N/<slug>
```

**If on the correct phase branch:** Continue working.

**If on a different phase branch:** Something is wrong. Check state.md, ask the user.

---

## LOOP START — Repeat Steps 2–6 for each task

### Step 2: Plan — What's Next?

Find the next task: the first unchecked `- [ ]` item in the active ROADMAP phase.

Classify it to pick the right skill:

| Task pattern | Classification | Skill |
|-------------|---------------|-------|
| "module structure", "scaffold" | **Scaffolding** | Direct + `/add-module` |
| "CLI argument parsing", "config", "flag" | **Feature** | `/add-feature` |
| "File scanner", "Line counting", "detection" | **Feature** | `/add-feature` |
| "Terminal dashboard", "output" | **Feature** | `/add-feature` |
| "Snapshot storage", "diffing" | **Feature** | `/add-feature` |
| "Tree-sitter", "complexity" | **Feature** | `/add-feature` |
| "Parallel", "color", "performance" | **Enhancement** | `/add-feature` |
| "CI pipeline" | **Infra** | Direct execution |

Update `.repostat/state.md` — set the task to `status: in-progress`.

---

### Step 3: Execute — Delegate to the Right Skill

#### For scaffolding (Cargo.toml, module structure):

This is the one case where `/go` works directly, because it's wiring up the project
skeleton. Create `Cargo.toml`, set up the directory structure from `docs/tech-stack.md`,
and use `/add-module` for each module that needs creating:

1. Create `Cargo.toml` with dependencies from `docs/tech-stack.md`
2. Create `src/main.rs` with minimal entry point
3. For each module in the planned structure, invoke: **Skill: `/add-module <name>`**
4. Invoke: **Skill: `/check`** to verify everything compiles

#### For features / enhancements:

Compose the task description from ROADMAP.md + the matching requirements in `docs/requirements.md`.

For complex features (touching 3+ files or with non-obvious design), write the spec first:

**Skill: `/spec <composed description>`**

Then implement it:

**Skill: `/add-feature <composed description>`**

`/add-feature` will find and use the spec that `/spec` created. It handles the full
TDD+SDD cycle (failing test → implement → refactor → verify).

For simple features (single file, obvious design), skip `/spec` and go straight to `/add-feature`.

#### For infra tasks (CI, completions):

Handle directly — create the files, test them, verify.

---

### Step 4: Verify — Run Quality Checks

After the delegated skill completes, run a verification pass:

**Skill: `/check`**

This gives a quick PASS/FAIL. If it fails, invoke **Skill: `/fix <error>`** and retry.
If the fix fails twice, mark the task as blocked and STOP.

**Periodic maintenance** (counted across the entire session, not reset per loop):
- Every 3rd completed task: also run **Skill: `/review`**
- Every 5th completed task: also run **Skill: `/refactor`** on the largest changed files
  and **Skill: `/analyze-arch`** to check for structural drift

---

### Step 5: Record — Update Everything

#### 5a. Mark ROADMAP.md checkbox
Use Edit to change `- [ ] <task text>` to `- [x] <task text>` for the completed task.

#### 5b. Update `.repostat/state.md`
- Move the completed task to the Progress section with today's date
- Set Current Task to the next unchecked item (or `status: idle` if phase is done)
- Add any learnings to the Learnings section
- Update session log

#### 5c. Commit
Stage and commit all changes with a Conventional Commit message:
```bash
git add <specific files>
git commit -m "<type>(<scope>): <description>"
```

The commit message must end with:
```
Co-Authored-By: Claude Opus 4.6 (1M context) <noreply@anthropic.com>
```

---

### Step 6: Continue or Stop?

**CONTINUE (loop back to Step 2) if:**
- More unchecked tasks remain in the current phase
- No blockers encountered
- No repeated failures (2+ consecutive failures = stop)

**STOP if:**
- Phase complete → go to Phase Completion below
- Blocker hit → record it in state.md, print summary, ask user
- 2+ consecutive task failures → record, print summary, ask user
- Ambiguity on a task → don't guess, ask user

**DO NOT stop just because N tasks are done.** The goal is to complete the phase.

---

## LOOP END

---

## Phase Completion

When all items in a phase are checked:

1. Run **Skill: `/test`** for full test suite verification
2. Run **Skill: `/analyze-arch`** to verify structural health
3. Run **Skill: `/review`** for a final review pass
4. Run **Skill: `/refactor`** on any files that grew large during the phase
5. Check exit criteria from ROADMAP.md
6. Use AskUserQuestion:
   - "Phase N complete. Exit criteria: [list]. All met. Ready to ship?"
   - A) Run `/ship` to open a PR and merge this phase to main
   - B) Keep working — there's more to polish
   - C) Stop here
   - RECOMMENDATION: Choose A — the quality gate already passed, let's ship it.

7. If shipping: invoke **Skill: `/ship`** to push the phase branch and open a PR.
8. After merge, create the next phase branch:
   ```bash
   git checkout main && git pull && git checkout -b phase-N+1/<slug>
   ```

---

## Status Summary

Print this when stopping (whether at phase completion, blocker, or error):

```
+====================================================+
|              /go — SESSION SUMMARY                  |
+====================================================+
| Phase        | N: <phase name>                      |
| Tasks done   | X this session                       |
| Total done   | Y / Z in phase                       |
| Next task    | <next task or "Phase complete">       |
| Blockers     | <None or description>                |
+----------------------------------------------------+
| Skills used  | /add-feature xN, /check xN, ...      |
+----------------------------------------------------+
| Quality gate | PASS/FAIL (fmt, clippy, test)        |
| Commits      | N new                                |
+----------------------------------------------------+
| Run /go again to continue.                         |
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
- [x] Task 2 — completed YYYY-MM-DD
- [ ] Task 3 — next

## Learnings

> Runtime discoveries that future sessions need to know.

- YYYY-MM-DD: <learning>

## Session Log

| Date | Tasks Completed | Skills Used | Notes |
|------|----------------|-------------|-------|
| YYYY-MM-DD | Task 1, Task 2 | /add-module x2, /check x2 | Initial scaffold |
```

---

## Handling Ambiguity

If a task is unclear:
1. Check `docs/requirements.md` for the detailed requirement (R-NNN IDs)
2. Check `docs/specs/` for an existing spec
3. Check `docs/decisions.md` for relevant ADRs
4. Check `.repostat/state.md` Learnings section

If still unclear: AskUserQuestion. Don't guess on architecture.

---

## Skill Quick Reference

| Skill | When /go uses it |
|-------|-----------------|
| `/status` | First thing on session start if state.md is stale |
| `/spec` | Before complex features (3+ files, non-obvious design) |
| `/add-module` | When ROADMAP task involves creating a new module |
| `/add-feature` | For implementing features (the workhorse) |
| `/check` | After every completed task (quick gate) |
| `/test` | Full suite at phase completion |
| `/review` | Every 3rd task, and before shipping |
| `/refactor` | Every 5th task, and at phase completion |
| `/analyze-arch` | At phase completion |
| `/fix` | When tests fail unexpectedly |
| `/ship` | When a phase is complete and ready to merge |

## Important Rules

1. **Loop until done.** Don't stop after one task. Complete the phase.
2. **Delegate, don't duplicate.** Use the existing skills. Don't reimplement their logic.
3. **Always read state first.** Never assume where the project is.
4. **Branch per phase.** Work on `phase-N/<slug>`, ship to main via PR.
5. **One task at a time.** Fully complete (via skill) before moving on.
6. **Commit after each task.** Small, atomic commits.
7. **Update ROADMAP.md checkboxes.** Source of truth for progress.
8. **Record learnings.** Non-obvious discoveries go in state.md.
9. **Don't skip tasks.** Phase ordering matters — foundations first.
10. **Stop gracefully on errors.** Always leave state clean for next `/go`.
11. **Invoke skills by name.** Use the Skill tool to call them.
12. **Verify after every task.** Always run `/check` after delegated work completes.
13. **Ship at phase boundaries.** Use `/ship` to PR the phase branch to main.
