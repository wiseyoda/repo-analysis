# Spec: Zero Files Warning

**Requirement IDs:** R-X05 (actionable error messages)
**Phase:** 9
**Date:** 2026-03-20

## Purpose

When a valid directory contains no source files after the 3-layer exclusion
filter, the dashboard shows all zeros with no explanation. Users think the
tool is broken. A warning makes the situation clear.

## Behavior

After the scanner returns files and the minified/generated filter runs,
if zero files remain for analysis, print to stderr:
`warning: no source files found after filtering. Check your .repostat.toml exclude patterns.`

Analysis continues (snapshot with zeros is still written).

## Edge Cases

- Empty directory: warning printed
- Directory with only excluded files: warning printed
- Directory with 1+ analyzable files: no warning

## Acceptance Criteria

- [ ] Warning printed to stderr when 0 files pass filtering
- [ ] No warning when 1+ files pass filtering
- [ ] Analysis still completes (snapshot written, dashboard rendered)
