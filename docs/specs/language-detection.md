# Spec: Language Detection

**Requirement IDs:** R-007
**Phase:** 1
**Date:** 2026-03-19

## Purpose

Detect the programming language of a source file based on its file extension.
This feeds into per-language line counting and aggregation.

## Inputs

| Input | Type | Source |
|-------|------|--------|
| File path | `&Path` | From scanner output |

## Outputs

`Option<Language>` — `None` for unrecognized extensions.

The `Language` enum variant knows its display name (e.g., `Language::Cpp` → `"C++"`).

## Behavior

1. Extract the file extension from the path (lowercase).
2. For special filenames without extensions (e.g., `Makefile`, `Dockerfile`), match the filename.
3. Look up the extension in a static map.
4. Return `Some(Language)` if found, `None` otherwise.

## Extension Map (50+ languages)

Multiple extensions can map to the same language (e.g., `.ts` and `.tsx` → TypeScript).

## Scanner Integration

Change `scan()` return type from `Vec<PathBuf>` to `Vec<ScannedFile>` where:

```rust
pub(crate) struct ScannedFile {
    pub path: PathBuf,
    pub language: Option<Language>,
}
```

## Edge Cases

- No extension — try filename match, then return None.
- Unknown extension — return None (file still scanned, just uncategorized).
- Case insensitivity — `.RS` and `.rs` both → Rust.
- Double extensions — use the last one (`.d.ts` → TypeScript via `.ts`).

## Acceptance Criteria

- [ ] Detects 50+ languages by file extension
- [ ] Handles special filenames (Makefile, Dockerfile)
- [ ] Case-insensitive extension matching
- [ ] Returns None for unknown extensions
- [ ] Scanner returns ScannedFile with language populated
- [ ] Display name works for all variants
