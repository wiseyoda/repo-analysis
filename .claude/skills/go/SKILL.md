---
name: go
version: 3.0.0
description: |
  Autonomous project driver. Reads state, picks the next task, delegates to
  the right skill, updates progress, and continues. Uses branch-per-phase
  strategy. Run at any moment — picks up where it left off.
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

`/go` is a thin orchestrator. It does NOT implement features itself — it figures out
what needs to happen and invokes the right skill to do it. Think of it as the project
manager that delegates to specialists:

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

## Step 1: Assess — Where Are We?

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

## Step 1b: Branch Management

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

## Step 2: Plan — What's Next?

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

## Step 3: Execute — Delegate to the Right Skill

### For scaffolding (Cargo.toml, module structure):

This is the one case where `/go` works directly, because it's wiring up the project
skeleton. Create `Cargo.toml`, set up the directory structure from `docs/tech-stack.md`,
and use `/add-module` for each module that needs creating:

1. Create `Cargo.toml` with dependencies from `docs/tech-stack.md`
2. Create `src/main.rs` with minimal entry point
3. For each module in the planned structure, invoke: **Skill: `/add-module <name>`**
4. Invoke: **Skill: `/check`** to verify everything compiles

### For features / enhancements:

Compose the task description from ROADMAP.md + the matching requirements in `docs/requirements.md`.

For complex features (touching 3+ files or with non-obvious design), write the spec first:

**Skill: `/spec <composed description>`**

Then implement it:

**Skill: `/add-feature <composed description>`**

`/add-feature` will find and use the spec that `/spec` created. It handles the full
TDD+SDD cycle (failing test → implement → refactor → verify).

For simple features (single file, obvious design), skip `/spec` and go straight to `/add-feature`.

### For infra tasks (CI, completions):

Handle directly — create the files, test them, verify.

---

## Step 4: Verify — Run Quality Checks

After the delegated skill completes, run a verification pass:

**Skill: `/check`**

This gives a quick PASS/FAIL. If it fails, fix issues before proceeding.

If this is the 3rd+ task completed in the session, also run:

**Skill: `/review`**

This catches anything that accumulated across multiple tasks (coupling, missed docs, etc).

---

## Step 5: Record — Update Everything

### 5a. Mark ROADMAP.md checkbox
Use Edit to change `- [ ] <task text>` to `- [x] <task text>` for the completed task.

### 5b. Update `.repostat/state.md`
- Move the completed task to the Progress section with today's date
- Set Current Task to the next unchecked item (or `status: idle` if stopping)
- Add any learnings to the Learnings section

### 5c. Commit
Stage and commit all changes with a Conventional Commit message:
```bash
git add <specific files>
git commit -m "<type>(<scope>): <description>"
```

---

## Step 6: Continue or Stop?

### Handling Failures

If `/check` or `/test` fails after a task:
1. Read the error output carefully
2. Invoke **Skill: `/fix <error description>`** to diagnose and fix
3. Re-run `/check` to verify the fix
4. If the fix works, continue. If not, mark the task as blocked in state.md.

**Continue if:**
- More unchecked tasks remain in the current phase
- No blockers encountered
- The session is productive (no repeated failures)

**Stop gracefully if:**
- Phase complete → verify exit criteria, then ask about next phase
- Blocker hit → record it, ask user
- 3+ substantial tasks done → keep commits reviewable
- Ambiguity → don't guess, ask

### Phase Completion

When all items in a phase are checked:

1. Run **Skill: `/analyze-arch`** to verify structural health
2. Run **Skill: `/test`** for full test suite verification
3. Run **Skill: `/refactor`** on any files that grew large during the phase
4. Check exit criteria from ROADMAP.md
5. Use AskUserQuestion:
   - "Phase N complete. Exit criteria: [list]. All met. Ready to ship?"
   - A) Run `/ship` to open a PR and merge this phase to main
   - B) Run `/review` first for a thorough check before shipping
   - C) Keep working — there's more to polish
   - D) Stop here
   - RECOMMENDATION: Choose A — the quality gate already passed, let's ship it.

6. If shipping: invoke **Skill: `/ship`** to push the phase branch and open a PR.
7. After merge, create the next phase branch:
   ```bash
   git checkout main && git pull && git checkout -b phase-N+1/<slug>
   ```

### Periodic Maintenance

Every 5 completed tasks (check the session log), suggest:
- **Skill: `/refactor`** on the largest/most-changed files
- **Skill: `/analyze-arch`** to check for structural drift

---

## Status Summary

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
| Skills used  | /add-module x2, /add-feature x1,     |
|              | /check x3, /review x1                |
+----------------------------------------------------+
| Quality gate | PASS (fmt, clippy, test)             |
| Commits      | 3 new                                |
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

1. **Delegate, don't duplicate.** Use the existing skills. Don't reimplement their logic.
2. **Always read state first.** Never assume where the project is.
3. **Branch per phase.** Work on `phase-N/<slug>`, ship to main via PR.
4. **One task at a time.** Fully complete (via skill) before moving on.
5. **Commit after each task.** Small, atomic commits.
6. **Update ROADMAP.md checkboxes.** Source of truth for progress.
7. **Record learnings.** Non-obvious discoveries go in state.md.
8. **Don't skip tasks.** Phase ordering matters — foundations first.
9. **Stop gracefully.** Always leave state clean for next `/go`.
10. **Invoke skills by name.** Use the Skill tool to call them.
11. **Verify after every task.** Always run `/check` after delegated work completes.
12. **Ship at phase boundaries.** Use `/ship` to PR the phase branch to main.
