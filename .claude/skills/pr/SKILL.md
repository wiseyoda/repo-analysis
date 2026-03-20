---
name: pr
version: 2.0.0
description: |
  Release engineer. Merges main into branch, runs full quality gate, reviews
  diff, bumps VERSION, updates CHANGELOG, commits, pushes, and opens a PR.
  Use when asked to "open a PR", "push", or "create PR". Note: /ship is
  the gstack global skill; /pr is the project-specific one.
disable-model-invocation: false
user-invocable: true
allowed-tools:
  - Bash
  - Read
  - Write
  - Edit
  - Grep
  - Glob
  - Skill
  - AskUserQuestion
---

## Arguments

- `/pr` — ship current branch (auto-detect base)
- `/pr <message>` — ship with a custom PR title

## Preconditions

```bash
git branch --show-current 2>/dev/null || echo "NO_GIT"
[ -f Cargo.toml ] && echo "CARGO_OK" || echo "NO_CARGO"
git status --porcelain 2>/dev/null | head -5
```

**If on `main`:** "You're on main. Ship from a feature/phase branch." — STOP.
**If NO_CARGO:** "No Cargo.toml found." — STOP.

---

## Step 1: Pre-flight

```bash
BRANCH=$(git branch --show-current)
echo "BRANCH: $BRANCH"
git fetch origin main --quiet 2>/dev/null
git diff origin/main --stat 2>/dev/null | tail -5
git log origin/main..HEAD --oneline 2>/dev/null
```

If uncommitted changes exist, commit them first:
```bash
git add -A && git commit -m "chore: stage uncommitted work before ship"
```

---

## Step 2: Merge main

```bash
git merge origin/main --no-edit
```

**If merge conflicts:** Try auto-resolve for VERSION and CHANGELOG (take ours + append).
For complex conflicts: STOP, show them, ask the user.

---

## Step 3: Full Quality Gate

Invoke **Skill: `/test`** — runs fmt + clippy + full test suite.

**If it fails:** STOP. Show failures. Do not proceed.

---

## Step 4: Pre-Landing Review

Invoke **Skill: `/review --staged`** against the full diff from main.

This does the fix-first flow: auto-fixes mechanical issues, asks about judgment calls.

**If review applies fixes:** re-run `/test` to verify.

---

## Step 5: Version Bump

```bash
cat VERSION 2>/dev/null || echo "NO_VERSION"
git diff origin/main...HEAD --stat | tail -1
```

**Auto-decide:**
- < 50 lines changed → PATCH bump
- 50+ lines → MINOR bump
- Major architecture changes → ask user

**For MINOR or MAJOR:** Use AskUserQuestion to confirm.

Bump the VERSION file. Reset lower digits to 0.

---

## Step 6: CHANGELOG

Read `CHANGELOG.md` header to know the format. Auto-generate entry from:

```bash
git log origin/main..HEAD --oneline
git diff origin/main...HEAD --stat
```

Categorize into `### Added`, `### Changed`, `### Fixed`, `### Removed`.
Write concise, user-forward bullets ("You can now..." not "Refactored the...").
Insert after the file header, dated today: `## [X.Y.Z] - YYYY-MM-DD`.

---

## Step 7: Commit

```bash
git add VERSION CHANGELOG.md
git commit -m "$(cat <<'INNEREOF'
chore: bump version and changelog (vX.Y.Z)

Co-Authored-By: Claude Opus 4.6 (1M context) <noreply@anthropic.com>
INNEREOF
)"
```

---

## Step 8: Push + PR

```bash
git push -u origin $BRANCH
```

Create PR:
```bash
gh pr create --base main --title "<type>: <summary>" --body "$(cat <<'INNEREOF'
## Summary
<bullets from CHANGELOG>

## Quality Gate
- fmt: PASS
- clippy: PASS
- test: PASS (N tests)

## Review
<summary from /review output>

## Test Plan
- [x] All tests pass
- [x] Pre-landing review clean

Generated with [Claude Code](https://claude.com/claude-code)
INNEREOF
)"
```

**Output the PR URL** — this is the final output the user sees.

---

## Important Rules

1. **Never ship from main.** Ship from feature/phase branches.
2. **Never skip tests.** If tests fail, stop.
3. **Never force push.** Regular `git push` only.
4. **Never skip review.** `/review` runs on every ship.
5. **Auto-decide PATCH.** Only ask for MINOR/MAJOR.
6. **CHANGELOG is user-facing.** Write what users can DO, not implementation details.
7. **One commit for version bump.** Everything else should already be committed.
