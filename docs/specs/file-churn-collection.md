# Spec: Per-File Churn Collection

**Requirement IDs:** Phase 9 (risk scoring prerequisite)
**Phase:** 9
**Date:** 2026-03-20

## Purpose

Collect per-file commit counts to feed into risk score computation.
Files changed frequently are higher risk, especially when also complex.

## Inputs

- Target directory path (must be a git repo)

## Outputs

- `BTreeMap<PathBuf, usize>` — file path → commit count in last 6 months
- `None` if not a git repo or git unavailable

## Behavior

1. Run `git log --format="" --name-only --since="6 months ago"`
2. Parse output: each non-empty line is a file path
3. Count occurrences per file path
4. Return the map

## Edge Cases

- Non-git directory: return None
- Empty git history (new repo): return Some(empty map)
- Deleted files appear in history: include them (they still represent churn)
- Binary files in history: include them
- Blank lines between commits: skip

## Acceptance Criteria

- [ ] Returns BTreeMap<PathBuf, usize> with commit counts per file
- [ ] Single git command (not per-file queries)
- [ ] Returns None for non-git directories
- [ ] Returns Some(empty) for repos with no recent commits
- [ ] Skips blank lines in git output
