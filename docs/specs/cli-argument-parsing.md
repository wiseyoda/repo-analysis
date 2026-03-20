# Spec: CLI Argument Parsing

**Requirement IDs:** R-001, R-016
**Phase:** 1
**Date:** 2026-03-19

## Purpose

Accept a path argument from the user, validate it points to an existing directory,
and provide clear help text. This is the entry point for all repostat operations.

## Inputs

| Input | Type | Required | Default |
|-------|------|----------|---------|
| `<PATH>` | positional argument | No | current working directory (`.`) |

## Outputs

- On success: validated `PathBuf` pointing to an existing directory, passed to analysis pipeline.
- On user error (exit 1): actionable error message to stderr.
- `--help`: usage text with examples to stdout.
- `--version`: version string to stdout.

## Behavior

1. Parse CLI arguments using clap derive.
2. If no path argument provided, default to the current working directory.
3. Canonicalize the path (resolve symlinks, relative paths).
4. Validate the path exists. If not, print error to stderr, exit 1.
5. Validate the path is a directory. If not, print error to stderr, exit 1.
6. Return the validated path for use by the analysis pipeline.

## Error Messages

| Condition | Message | Exit Code |
|-----------|---------|-----------|
| Path does not exist | `error: path does not exist: <path>` | 1 |
| Path is not a directory | `error: path is not a directory: <path>` | 1 |

## Edge Cases

- Path is `.` (current directory) — valid, use cwd.
- Path is a symlink to a directory — valid after canonicalization.
- Path is a file, not a directory — error.
- Path contains Unicode — must handle correctly.
- Path has trailing slash — must handle correctly.

## Acceptance Criteria

- [ ] Accepts a positional path argument
- [ ] Defaults to current directory when no path given
- [ ] Validates path exists, returns exit code 1 with stderr message if not
- [ ] Validates path is a directory, returns exit code 1 with stderr message if not
- [ ] `--help` prints usage with examples and exits 0
- [ ] `--version` prints version and exits 0
- [ ] Existing integration tests remain green
