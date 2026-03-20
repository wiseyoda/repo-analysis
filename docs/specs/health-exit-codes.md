# Spec: Health Score Exit Codes

**Requirement IDs:** Phase 9 Goal 3
**Phase:** 9
**Date:** 2026-03-20

## Purpose

Enable zero-cost CI integration via exit codes that indicate code health.
Distinct from tool error codes (exit 1/2).

## Behavior

After analysis, evaluate metrics against thresholds:
- Exit 0: healthy (all metrics within thresholds)
- Exit 10: warning (any metric exceeds warning threshold)
- Exit 20: critical (any metric exceeds critical threshold)
- Exit 1: tool error (unchanged — path not found, config error, etc.)

Highest severity wins (critical > warning > healthy).

## Config

```toml
[health]
warn_complexity = 25
crit_complexity = 50
warn_function_lines = 60
crit_function_lines = 100
```

Defaults apply when no [health] section exists.

## Acceptance Criteria

- [ ] HealthThresholds struct with defaults
- [ ] Config parses [health] section
- [ ] evaluate_health() returns 0/10/20
- [ ] main.rs exits with health code after analysis
- [ ] Exit 1 still used for tool errors
