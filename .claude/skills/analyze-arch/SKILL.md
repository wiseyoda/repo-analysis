---
name: analyze-arch
description: Analyze project architecture against docs/tech-stack.md
disable-model-invocation: true
user-invocable: true
context: fork
allowed-tools: Read, Glob, Grep, Bash(cargo tree*), Bash(cargo metadata*)
---

Analyze the current architecture of repostat against the design in `docs/tech-stack.md`.

## Analysis Steps

### 1. Module Structure
- Map the actual `src/` directory structure.
- Compare against the intended structure in `docs/tech-stack.md`.
- Flag missing modules, unexpected files, or structural drift.

### 2. Dependency Graph
- Run `cargo tree` to visualize the dependency tree.
- Check each dependency against `docs/tech-stack.md` approved list.
- Flag any dependency not in the approved list.
- Report total dependency count (direct + transitive).

### 3. Internal Coupling
- Grep for `use crate::` to map internal imports.
- Identify which modules depend on which.
- Flag circular dependencies or high fan-out modules.

### 4. Public API Surface
- Grep for `pub fn`, `pub struct`, `pub enum`, `pub trait`.
- Flag items that are `pub` but could be `pub(crate)`.

### 5. Code Metrics
- Count lines per module.
- Identify the largest files and functions.
- Flag anything exceeding thresholds (500 lines/file, 40 lines/function).

## Output

Produce a structured report:
- Architecture conformance: matches design? Y/N with details
- Module dependency map (text-based)
- Findings sorted by severity
- Recommendations for structural improvements
