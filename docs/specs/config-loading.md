# Spec: Config File Loading

**Requirement IDs:** R-005
**Phase:** 1
**Date:** 2026-03-19

## Purpose

Load and validate a `.repostat.toml` configuration file from the target directory,
providing user-customizable include/exclude patterns for file filtering. The config
is optional — a missing file is not an error.

## Inputs

| Input | Type | Source |
|-------|------|--------|
| Target directory path | `&Path` | From CLI validation |

The config file format (`.repostat.toml`):

```toml
[exclude]
patterns = ["*.generated.*", "vendor/**"]

[include]
patterns = ["vendor/important/**"]
```

## Outputs

A `Config` struct containing:
- `exclude_patterns`: `Vec<String>` — glob patterns to exclude from analysis
- `include_patterns`: `Vec<String>` — glob patterns to force-include (overrides exclude)

## Behavior

1. Construct the config file path: `<target_dir>/.repostat.toml`.
2. If the file does not exist, return a default `Config` (empty pattern lists).
3. If the file exists, read and parse it as TOML.
4. Validate that all patterns are non-empty strings.
5. Return the parsed `Config`.

## Error Conditions

| Condition | Error |
|-----------|-------|
| TOML parse error | `ConfigError::ParseFailed { path, source }` |
| File read error (permissions, etc.) | `ConfigError::ReadFailed { path, source }` |

## Edge Cases

- Config file does not exist — return defaults (not an error).
- Config file is empty — return defaults.
- Config file has only `[exclude]` — `include_patterns` defaults to empty.
- Config file has only `[include]` — `exclude_patterns` defaults to empty.
- Config file has unknown keys — ignore them (forward compatibility).

## Acceptance Criteria

- [ ] Returns default config when no `.repostat.toml` exists
- [ ] Parses exclude patterns from `[exclude]` table
- [ ] Parses include patterns from `[include]` table
- [ ] Returns actionable error for malformed TOML
- [ ] Returns actionable error for unreadable file
- [ ] Handles empty config file gracefully
- [ ] Handles partial config (only exclude or only include)
