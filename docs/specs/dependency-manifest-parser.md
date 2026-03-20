# Spec: Dependency Manifest Parser

**Requirement IDs:** R-200
**Phase:** 3
**Date:** 2026-03-20

## Purpose

Parse dependency manifests from multiple language ecosystems to count
direct dependencies. This is the foundation for dependency analysis.

## Inputs

| Input | Type |
|-------|------|
| Manifest file path | `&Path` |
| Target directory | `&Path` (for find_manifests) |

## Outputs

- `ManifestInfo`: manifest type, file path, list of direct dependency names
- `find_manifests`: list of manifest file paths found in a directory

## Behavior

1. `find_manifests(dir)`: Walk directory, find known manifest filenames, skip heuristic-excluded dirs.
2. `parse_manifest(path)`: Detect type from filename, parse deps with simple heuristics:
   - Cargo.toml: keys under [dependencies], [dev-dependencies], [build-dependencies]
   - package.json: keys in "dependencies" and "devDependencies" objects
   - requirements.txt: each non-blank, non-comment line (strip version specifiers)
   - go.mod: lines after "require (" block
   - Gemfile: `gem "name"` lines
   - pom.xml: text inside `<artifactId>` within `<dependency>` blocks
   - build.gradle: `implementation`, `api`, `compile` lines with quoted group:artifact

## Edge Cases

- Missing or unreadable file → return None
- Empty manifest → return ManifestInfo with empty deps
- Malformed content → best-effort parse, don't error
- Manifest in excluded directory (node_modules) → skipped by find_manifests

## Acceptance Criteria

- [ ] Parses Cargo.toml dependencies
- [ ] Parses package.json dependencies
- [ ] Parses requirements.txt dependencies
- [ ] Parses go.mod dependencies
- [ ] Finds manifests recursively, skipping excluded dirs
- [ ] Returns None for unrecognized files
- [ ] Handles empty/malformed manifests gracefully
