# Spec: Generated/Minified File Detection

**Requirement IDs:** R-004
**Phase:** 1
**Date:** 2026-03-19

## Purpose

Detect files that are minified or auto-generated so they can be excluded from
user code metrics. These files inflate LOC counts without representing human effort.

## Inputs

| Input | Type |
|-------|------|
| File content | `&str` |

## Outputs

Two boolean functions:
- `is_minified(content: &str) -> bool`
- `is_generated(content: &str) -> bool`

## Behavior

### Minified Detection
A file is minified if its average line length exceeds 200 characters
(excluding blank lines).

### Generated Detection
A file is generated if any of the first 5 non-blank lines contain
a generation marker (case-insensitive):
- "generated"
- "auto-generated"
- "do not edit"
- "automatically generated"
- "@generated"

## Edge Cases

- Empty file → not minified, not generated
- File with one very long line → minified
- "generated" in a variable name deep in the file → not detected (only check header)

## Acceptance Criteria

- [ ] Detects minified files (avg line > 200 chars)
- [ ] Detects generated files by header markers
- [ ] Empty files are neither
- [ ] ScannedFile gets an `is_generated` and `is_minified` field
