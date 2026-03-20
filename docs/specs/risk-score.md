# Spec: Churn + Complexity Risk Score

**Requirement IDs:** Phase 9 Goal 1
**Phase:** 9
**Date:** 2026-03-20

## Purpose

Files that are both frequently modified AND complex are the most likely
to cause production incidents. Combine churn and complexity into a single
risk score per file.

## Inputs

- Per-file churn counts: `BTreeMap<PathBuf, usize>` from `collect_file_churn()`
- Per-file complexity: extracted from hotspots (file → max cyclomatic)

## Outputs

- `Vec<RiskEntry>` sorted by risk score descending
- Each entry: file path, churn_count, max_complexity, risk_score

## Behavior

1. For each file in churn map, look up its max cyclomatic complexity
2. risk_score = churn_count * max_complexity
3. Files with churn but no complexity data get risk = churn * 1
4. Files with complexity but no churn get risk = 0 (no recent changes)
5. Sort by risk_score descending, return top entries

## Edge Cases

- No git repo (no churn data): return empty vec
- No complexity data: use 1 as default complexity
- File in churn but not in complexity: include with complexity=1
- All scores are 0: return empty vec

## Acceptance Criteria

- [ ] RiskEntry struct with file, churn_count, max_complexity, risk_score
- [ ] compute_risk_scores function produces sorted results
- [ ] Risk section rendered in dashboard (top 10)
- [ ] Graceful when no churn data available
