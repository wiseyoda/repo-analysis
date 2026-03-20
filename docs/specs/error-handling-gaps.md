# Spec: Fix Error Handling Gaps

**Requirement IDs:** R-X05 (actionable error messages), R-X06 (proper error handling)
**Phase:** 9
**Date:** 2026-03-20

## Purpose

Two error paths silently swallow failures, violating Constitution §5:
1. Corrupt snapshot JSON produces a cryptic serde error
2. Cross-repo index write failures are silently ignored

## Behavior

### Corrupt Snapshot
When `load_latest()` finds a snapshot file but fails to parse it:
- Log: `warning: corrupt snapshot <filename>, skipping diff: <error>`
- Return `Ok(None)` — analysis continues without diff display

### Index Write Failure
When `register_repo()` fails to write `~/.repostat/repos.json`:
- Log: `warning: failed to update repo index: <error>`
- Continue normally — index is non-critical

## Acceptance Criteria

- [ ] Corrupt snapshot file produces a friendly warning on stderr
- [ ] Corrupt snapshot does not crash the tool — returns Ok(None)
- [ ] Index write failure produces a warning on stderr
- [ ] Index write failure does not crash the tool
