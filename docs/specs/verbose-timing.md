# Spec: Verbose Phase Timing

**Requirement IDs:** R-X05 (actionable output)
**Phase:** 9
**Date:** 2026-03-20

## Purpose

Users can't tell which analysis phase is slow. A `--verbose` flag shows
per-phase timing to stderr, helping users diagnose performance issues.

## Behavior

When `--verbose` / `-v` is passed:
- Time each phase using `std::time::Instant`
- Print timing summary to stderr after analysis completes
- Format: right-aligned labels, seconds with 1 decimal

Output goes to stderr so it doesn't interfere with --json or --markdown.

## Edge Cases

- Without --verbose: no timing output
- Combined with --json: timing on stderr, JSON on stdout
- Combined with --markdown: timing on stderr, markdown on stdout

## Acceptance Criteria

- [ ] `--verbose` flag accepted by clap
- [ ] Phase timing printed to stderr when --verbose
- [ ] No timing output without --verbose
- [ ] Timing doesn't interfere with --json or --markdown stdout
