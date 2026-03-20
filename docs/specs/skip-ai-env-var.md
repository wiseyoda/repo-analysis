# Spec: REPOSTAT_SKIP_AI Environment Variable

**Requirement IDs:** R-X08 (performance)
**Phase:** 9
**Date:** 2026-03-20

## Purpose

Integration tests take 222s because they invoke the full analysis pipeline
including 6 sequential Claude CLI calls. An env var to skip AI makes tests
fast without changing the binary's default behavior.

## Behavior

When `REPOSTAT_SKIP_AI` is set to `"1"` or `"true"` (case-insensitive):
- `ai::run_ai_analysis()` returns `None` immediately
- No Claude CLI detection or invocation occurs
- All other analysis proceeds normally

## Edge Cases

- Env var not set: AI runs normally
- Env var set to "0" or "false": AI runs normally
- Env var set to empty string: AI runs normally

## Acceptance Criteria

- [ ] REPOSTAT_SKIP_AI=1 causes run_ai_analysis to return None
- [ ] REPOSTAT_SKIP_AI=true causes run_ai_analysis to return None
- [ ] Without the env var, AI runs normally
- [ ] Integration tests set the env var and complete in <10s
