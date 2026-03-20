---
name: analyze-arch
version: 1.0.0
description: |
  Analyze project architecture against docs/tech-stack.md. Produces a health
  score, dependency map, conformance report, and identifies structural drift.
  Use when asked to "analyze architecture", "check structure", or "audit deps".
disable-model-invocation: false
user-invocable: true
context: fork
allowed-tools:
  - Read
  - Glob
  - Grep
  - Bash(cargo tree*)
  - Bash(cargo metadata*)
  - Bash(wc *)
  - Bash(find *)
  - AskUserQuestion
---

## Arguments

- `/analyze-arch` — full architecture analysis
- `/analyze-arch deps` — dependency analysis only
- `/analyze-arch coupling` — internal coupling analysis only
- `/analyze-arch drift` — structure drift check only

## Preconditions

```bash
[ -f Cargo.toml ] && echo "CARGO_OK" || echo "NO_CARGO"
[ -f docs/tech-stack.md ] && echo "TECHSTACK_OK" || echo "NO_TECHSTACK"
```

**If NO_CARGO:** "No Cargo.toml found." — STOP.
**If NO_TECHSTACK:** "No docs/tech-stack.md found. Cannot compare against intended architecture." — continue with warnings.

Read `docs/tech-stack.md` to load the intended architecture.

## Step 1: Module Structure Audit

Map the actual `src/` directory structure:

```bash
find src -name "*.rs" -type f | sort
```

Compare against the intended structure in `docs/tech-stack.md`.

**Grade:**
- A: Exact match to intended structure
- B: Minor deviations (extra utility files, missing empty modules)
- C: Structural drift (files in wrong modules, unexpected directories)
- D: Significant divergence from intended architecture
- F: Unrecognizable — needs restructuring

Flag: missing modules, unexpected files, files in wrong locations.

## Step 2: Dependency Audit

```bash
cargo tree --depth 1
cargo tree -d
```

Check each dependency against the approved list in `docs/tech-stack.md`.

**Grade:**
- A: All deps approved, no duplicates, minimal transitive count
- B: All deps approved, minor duplication
- C: 1-2 unapproved deps or significant duplication
- D: Multiple unapproved deps
- F: Dependency chaos

Flag: unapproved deps, duplicate versions, heavy transitive trees.

## Step 3: Internal Coupling

```bash
grep -rn "use crate::" src/ | sort
```

Build a module dependency map. For each module, calculate:
- **Fan-out**: How many other modules does it depend on?
- **Fan-in**: How many other modules depend on it?

**Grade:**
- A: Clear layered architecture, no circular deps, fan-out ≤ 3
- B: Mostly layered, minor coupling concerns
- C: Some circular dependencies or high fan-out (4-6)
- D: Tangled dependencies, fan-out > 6
- F: Everything depends on everything

Produce an ASCII dependency map:

```
  cli ──→ config
   │         │
   ▼         ▼
  scanner ──→ metrics
   │            │
   ▼            ▼
  snapshot ──→ report
   │
   ▼
   ai
```

## Step 4: Code Size Analysis

```bash
wc -l src/**/*.rs 2>/dev/null | sort -rn
```

Flag:
- Files exceeding 500 lines
- Functions exceeding 40 lines (grep for `fn ` and count lines to next `fn ` or `}`)
- Modules with more than 10 public exports

**Grade:**
- A: All files under 300 lines, all functions under 40
- B: A few files 300-500, functions under 40
- C: Files over 500 or functions over 40
- D: Multiple large files and functions
- F: Monolithic files

## Step 5: Public API Surface

```bash
grep -rn "^pub " src/ | grep -v "pub(crate)" | grep -v "#\[cfg(test)\]"
```

Flag items that are `pub` but could be `pub(crate)`. Score:
- A: Minimal public API, everything else is `pub(crate)`
- B: A few unnecessary `pub` items
- C: Significant API surface leakage
- D: Most items unnecessarily `pub`

## Step 6: Health Score

| Category | Weight | Grade |
|----------|--------|-------|
| Module structure | 25% | |
| Dependencies | 20% | |
| Internal coupling | 20% | |
| Code size | 20% | |
| API surface | 15% | |

Convert: A=100, B=80, C=60, D=40, F=20. Weighted average = Health Score.

## Step 7: Completion Summary

```
+====================================================+
|           ARCHITECTURE HEALTH REPORT                |
+====================================================+
| Module structure    | B | Minor drift in scanner/   |
| Dependencies        | A | All approved, 12 direct   |
| Internal coupling   | B | scanner→metrics tight     |
| Code size           | C | config.rs at 520 lines    |
| API surface         | A | Minimal pub exposure      |
+----------------------------------------------------+
| HEALTH SCORE: 82/100 (B)                           |
+====================================================+
|                                                     |
| DEPENDENCY MAP:                                     |
|   cli ──→ config                                    |
|    │         │                                      |
|    ▼         ▼                                      |
|   scanner ──→ metrics                               |
|    │            │                                   |
|    ▼            ▼                                   |
|   snapshot ──→ report ──→ ai                        |
|                                                     |
+----------------------------------------------------+
| FINDINGS:                                           |
|  1. [WARNING] config.rs exceeds 500 lines           |
|  2. [NOTE] scanner depends on 4 modules (fan-out)   |
|  3. [NOTE] 3 pub items could be pub(crate)          |
+----------------------------------------------------+
| TOP 3 RECOMMENDATIONS:                              |
|  1. Split config.rs into config/mod.rs + submodules |
|  2. Consider facade pattern for scanner deps        |
|  3. Tighten API surface in metrics module           |
+====================================================+
```

## Important Rules

1. **Read tech-stack.md first.** Every finding is relative to the intended architecture.
2. **Never modify code.** This is analysis only.
3. **Grade, don't just list.** Every category gets a letter grade.
4. **Produce the dependency map.** ASCII art is mandatory.
5. **Recommendations are specific.** "Split X into Y" not "consider refactoring."
6. **Cross-reference ROADMAP.md.** Note if structural issues block upcoming phases.
