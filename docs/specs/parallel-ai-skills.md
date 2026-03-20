# Spec: Parallel AI Skills

**Requirement IDs:** Phase 9 Goal 2
**Phase:** 9
**Date:** 2026-03-20

## Purpose

AI analysis takes ~60s because 6 Claude CLI calls run sequentially.
Running them concurrently via rayon::scope reduces this to ~15s.

## Behavior

- All 6 skill invocations run concurrently in rayon::scope
- Results collected into AiAnalysisResult via Mutex
- Individual skill failures still logged and skipped
- Output identical to sequential version

## Acceptance Criteria

- [ ] Skills run via rayon::scope instead of sequential loop
- [ ] All existing AI tests still pass
- [ ] Error handling unchanged (warnings per failed skill)
