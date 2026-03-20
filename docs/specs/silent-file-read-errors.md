# Spec: Fix Silent File Read Errors

**Requirement IDs:** R-X05 (actionable error messages), R-X06 (proper error handling)
**Phase:** 9
**Date:** 2026-03-20

## Purpose

Files that can't be read (permissions, encoding, symlink issues) are silently
dropped from analysis via `read_to_string().ok()?`. This violates Constitution §5
("Errors are propagated, never swallowed") and can produce misleading metrics.

## Inputs

- Scanner produces a list of `ScannedFile` entries
- Some files may fail `std::fs::read_to_string()` due to IO errors

## Outputs

- Warning messages on stderr for each unreadable file
- A count of skipped files displayed in the dashboard summary
- The count available in JSON and markdown output

## Behavior

1. When `read_to_string()` fails for a file, log to stderr:
   `warning: skipped <path>: <error>`
2. Count the number of skipped files across the analysis
3. Display "N files skipped (unreadable)" in the dashboard if N > 0
4. Include `skipped_files: N` in snapshot JSON

## Edge Cases

- Zero files skipped: no warning, no dashboard line
- All files skipped: warn + show 0 metrics (separate issue handles zero-file warning)
- Binary files: `read_to_string` fails with invalid UTF-8 — should be counted as skipped

## Acceptance Criteria

- [ ] Unreadable files produce a warning on stderr with path and error
- [ ] Skipped file count is tracked and available after analysis
- [ ] Dashboard shows skipped count when > 0
- [ ] JSON output includes skipped_files field
- [ ] Existing tests continue to pass
