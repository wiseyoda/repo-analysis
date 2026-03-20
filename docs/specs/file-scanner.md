# Spec: File Scanner

**Requirement IDs:** R-002, R-003
**Phase:** 1
**Date:** 2026-03-19

## Purpose

Recursively walk a target directory and return a list of files to analyze,
applying a three-layer exclusion system (ADR-005): gitignore rules, built-in
heuristic directory exclusions, and user config include/exclude patterns.

## Inputs

| Input | Type | Source |
|-------|------|--------|
| Target directory | `&Path` | From CLI validation |
| Config | `&Config` | From config loading |

## Outputs

`Result<Vec<PathBuf>, ScanError>` — sorted list of regular file paths that
survived all three exclusion layers.

## Behavior

1. Build an `ignore::WalkBuilder` rooted at the target directory.
   - Gitignore support is enabled by default in `ignore` crate (Layer 1).
   - Add built-in heuristic directory exclusions as custom ignore rules (Layer 2).
2. Walk the directory tree, collecting only regular files (skip directories, symlinks).
3. For each file, apply config-based exclusion (Layer 3):
   - If the file matches any `exclude_patterns` glob, exclude it.
   - If the file also matches any `include_patterns` glob, re-include it (override).
4. Sort the result by path for deterministic output (Constitution §10).
5. Return the collected file paths.

## Three-Layer Exclusion (ADR-005)

| Layer | Source | Mechanism |
|-------|--------|-----------|
| 1 | `.gitignore` | `ignore` crate handles this natively |
| 2 | Built-in heuristics | Custom ignore rules for: `node_modules`, `vendor`, `build`, `dist`, `.next`, `Pods`, `target`, `.git`, `__pycache__`, `.venv`, `venv` |
| 3 | `.repostat.toml` | Config `exclude_patterns` / `include_patterns` globs |

**Precedence:** Include overrides exclude at Layer 3 only. Layers 1 and 2 cannot
be overridden by include patterns (by design — if it's gitignored, it's not source).

## Error Conditions

| Condition | Error |
|-----------|-------|
| Walk error (permissions, broken symlink) | `ScanError::WalkError` — skip file, don't abort |

Non-fatal walk errors (permission denied on a single file) should be collected
or logged to stderr but should NOT abort the entire scan.

## Edge Cases

- Empty directory — returns empty Vec.
- Directory with only gitignored files — returns empty Vec.
- Directory with a committed `vendor/` — excluded by Layer 2 heuristics.
- Config include overrides config exclude — file is kept.
- Symlinks to files — included (the `ignore` crate follows them by default).
- Hidden files (`.foo`) — included unless gitignored.

## Acceptance Criteria

- [ ] Walks a directory recursively and returns file paths
- [ ] Respects .gitignore rules (files in .gitignore are excluded)
- [ ] Excludes built-in heuristic directories (node_modules, vendor, etc.)
- [ ] Applies config exclude patterns
- [ ] Config include patterns override config exclude patterns
- [ ] Returns only regular files (not directories)
- [ ] Returns sorted paths for deterministic output
- [ ] Handles empty directories gracefully
- [ ] Does not abort on single-file permission errors
