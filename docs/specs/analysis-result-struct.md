# Spec: AnalysisResult Builder Struct

**Requirement IDs:** R-X06 (proper error handling patterns), coding-standard (max 4 params)
**Phase:** 9
**Date:** 2026-03-20

## Purpose

Replace the 7-parameter `Snapshot::from_aggregate()` function with a single
`AnalysisResult` struct that collects all analysis outputs. This makes the
function extensible for Phase 9 additions (risk scores, health status) without
growing the parameter list further.

## Inputs

The `AnalysisResult` struct collects:
- `AggregateMetrics` (line counts, language breakdown)
- `git_sha: Option<String>`
- Hotspots: `Vec<(String, FunctionInfo)>`
- `DependencySummary`
- `DocumentationMetrics` (optional)
- `AiAnalysisResult` (optional)
- `skipped_files: usize`

## Outputs

- `Snapshot::from_analysis(result: &AnalysisResult) -> Self`

## Behavior

1. `AnalysisResult` is constructed in `main.rs` as analysis proceeds
2. Passed to `Snapshot::from_analysis()` as a single parameter
3. Dashboard and other consumers use `AnalysisResult` fields directly

## Edge Cases

- All optional fields are None (minimal analysis)
- Backward compatible: existing snapshot deserialization unchanged

## Acceptance Criteria

- [ ] `AnalysisResult` struct exists with all analysis fields
- [ ] `Snapshot::from_analysis()` accepts a single `&AnalysisResult` parameter
- [ ] Old `from_aggregate()` is removed
- [ ] `main.rs` constructs `AnalysisResult` and passes it
- [ ] All existing tests pass
