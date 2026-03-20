# Spec: Diff Subcommand

**Requirement IDs:** Phase 9 Goal 5
**Phase:** 9
**Date:** 2026-03-20

## Purpose

Analyze only files changed in recent commits. Lighter than full scan,
useful for post-PR check ("did that refactor help?").

## Behavior

`repostat diff HEAD~5` or `repostat diff HEAD~5 ./path`:
1. Run `git diff --name-only <revspec>..HEAD` to get changed file paths
2. Filter scanner output to only those paths
3. Run normal analysis pipeline on filtered set
4. Display dashboard with filtered metrics

## Edge Cases

- Non-git directory: error "not a git repository"
- Invalid revspec: git error propagated
- No changed files: warning "no files changed in range"
- Deleted files: in diff but not on disk — silently skipped by scanner

## Acceptance Criteria

- [ ] Diff subcommand with revspec argument
- [ ] Only changed files analyzed
- [ ] Non-git dir produces clear error
- [ ] Works with HEAD~N syntax
