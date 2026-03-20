# Spec: HTML Dashboard Output

**Requirement IDs:** Phase 9 Goal 6
**Phase:** 9
**Date:** 2026-03-20

## Purpose

Self-contained HTML report with SVG charts for richer visualization
than terminal/markdown output. No JavaScript dependencies.

## Behavior

`repostat --html ./path` writes `repostat-report.html` to the target dir.
Contains: summary, language breakdown SVG bar chart, hotspots table,
risk table, dependency summary. Pure inline CSS + SVG.

## Acceptance Criteria

- [ ] --html flag generates repostat-report.html
- [ ] File is self-contained (no external deps)
- [ ] Contains SVG bar chart for language breakdown
- [ ] Contains summary metrics and hotspots table
