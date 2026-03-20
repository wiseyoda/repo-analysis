---
name: status
version: 1.0.0
description: |
  Project dashboard. Read-only view of where we are: phase, progress, health,
  tests, LOC, recent commits. Does no work — just orients you. Use when asked
  "where are we", "status", "what's next", or at the start of a session.
disable-model-invocation: false
user-invocable: true
allowed-tools:
  - Bash
  - Read
  - Glob
  - Grep
---

## Gather Data

Run all of these to collect project state:

```bash
# Git state
git branch --show-current 2>/dev/null || echo "NO_BRANCH"
git log --oneline -5 2>/dev/null || echo "NO_COMMITS"
git status --porcelain 2>/dev/null | wc -l | tr -d ' '
```

```bash
# Code metrics (if src/ exists)
find src -name "*.rs" -type f 2>/dev/null | wc -l | tr -d ' '
cat src/**/*.rs 2>/dev/null | wc -l | tr -d ' '
find tests -name "*.rs" -type f 2>/dev/null | wc -l | tr -d ' '
```

```bash
# Test count (if Cargo.toml exists)
[ -f Cargo.toml ] && cargo test 2>&1 | grep "test result" || echo "NO_TESTS"
```

```bash
# Dependency count
[ -f Cargo.toml ] && grep -c '^\w' Cargo.toml 2>/dev/null | head -1 || echo "0"
[ -f Cargo.lock ] && echo "LOCK_EXISTS" || echo "NO_LOCK"
```

Read `.repostat/state.md` for current task and progress.
Read `ROADMAP.md` and count checked vs unchecked items per phase.

## Output

```
+====================================================+
|              repostat — PROJECT STATUS              |
+====================================================+
| Branch       | phase-1                              |
| Last commit  | abc1234 feat: add line counter        |
| Dirty files  | 0                                    |
+----------------------------------------------------+
|                                                     |
| ROADMAP PROGRESS                                    |
| Phase 1: Foundation     ████████░░░░░░  5/15  33%  |
| Phase 2: Complexity     ░░░░░░░░░░░░░░  0/8    0%  |
| Phase 3: Dependencies   ░░░░░░░░░░░░░░  0/7    0%  |
| Phase 4: Documentation  ░░░░░░░░░░░░░░  0/6    0%  |
| Phase 5: AI Analysis    ░░░░░░░░░░░░░░  0/13   0%  |
| Phase 6: History        ░░░░░░░░░░░░░░  0/7    0%  |
| Phase 7: Polish         ░░░░░░░░░░░░░░  0/8    0%  |
|                                                     |
+----------------------------------------------------+
| CODEBASE                                            |
| Source files | 12                                   |
| Source lines | 1,450                                |
| Test files   | 4                                    |
| Tests        | 23 passing                           |
| Dependencies | 8 direct                             |
+----------------------------------------------------+
| CURRENT TASK                                        |
| Phase 1: Line counting engine                       |
| Status: idle                                        |
+----------------------------------------------------+
| RECENT COMMITS                                      |
| abc1234 feat(scanner): add file walker              |
| def5678 test(scanner): add walker tests             |
| 789abcd chore: project scaffold                     |
+----------------------------------------------------+
| Run /go to continue working.                        |
+====================================================+
```

If there are learnings in `.repostat/state.md`, show the 3 most recent:
```
| RECENT LEARNINGS                                    |
| - tree-sitter TS grammar needs "typescript" feature |
| - ignore crate handles nested .gitignore files      |
```

## Important Rules

1. **Read-only.** Do not modify any files. Do not run tests that change state.
2. **Always show progress bars.** Visual progress per phase is the most important output.
3. **Show the next task.** The user should know exactly what `/go` will do next.
4. **Fast.** This should complete in seconds. Don't run expensive commands.
5. **Graceful when empty.** If there's no src/ or no Cargo.toml, show what exists.
