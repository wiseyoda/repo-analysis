---
name: add-module
version: 2.0.0
description: |
  Scaffold a new module following project structure conventions from tech-stack.md.
  Creates files, wires up mod.rs, adds test scaffolding. Use when asked to "create
  a module", "scaffold", or "add a new module".
disable-model-invocation: false
user-invocable: true
allowed-tools:
  - Read
  - Write
  - Edit
  - Bash(cargo *)
  - Bash(mkdir *)
  - Glob
  - Grep
  - AskUserQuestion
---

## Arguments

- `/add-module <name>` — create a module with the given name
- `/add-module <parent>/<name>` — create a submodule under the specified parent
- If no argument: STOP with "Usage: `/add-module <name>` or `/add-module <parent>/<name>`"

## Preconditions

```bash
[ -f Cargo.toml ] && echo "CARGO_OK" || echo "NO_CARGO"
[ -f docs/tech-stack.md ] && echo "TECHSTACK_OK" || echo "NO_TECHSTACK"
```

**If NO_CARGO:** "No Cargo.toml found." — STOP.

## Step 0: Validate Against Architecture

Read `docs/tech-stack.md` to check:

1. **Is this module in the planned structure?** If yes, proceed.
2. **If not planned:** Use AskUserQuestion:
   - "Module `$ARGUMENTS` is not in the planned architecture (docs/tech-stack.md)."
   - A) Create it anyway and update tech-stack.md
   - B) Abort — pick a module from the planned structure
   - RECOMMENDATION: Choose B — unplanned modules cause structural drift.

Read `docs/requirements.md` to find relevant requirement IDs.

## Step 1: Scaffold

Determine if this needs a directory module or single-file module:

**Directory module** (if it will have multiple files or submodules):

```
src/<parent>/<name>/
├── mod.rs        # Re-exports only
└── <impl>.rs     # Implementation
```

**Single-file module** (if it's self-contained):

```
src/<parent>/<name>.rs
```

### mod.rs template:

```rust
//! Brief description of the module.

mod implementation;

pub use implementation::PublicType;
pub(crate) use implementation::InternalHelper;
```

### Implementation file template:

```rust
//! Detailed description of this file's responsibility.

use std::path::Path;

use serde::Serialize;

use crate::errors::RepostatError;

// Implementation here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn placeholder_compiles() {
        // Replace with real tests before implementing features
    }
}
```

## Step 2: Wire Up

1. Add `mod <name>;` to the parent module's `mod.rs` or `lib.rs`/`main.rs`.
2. Add any required `pub use` re-exports.
3. If new dependencies are needed in `Cargo.toml`, add them with a justification comment:
   ```toml
   # Required for <module> — <justification per constitution §8>
   new_dep = "1"
   ```

## Step 3: Verify

```bash
cargo check 2>&1
cargo test 2>&1
```

Both must pass. The placeholder test confirms the module compiles.

## Step 4: Completion

When the module compiles and tests pass, the skill is done. Do NOT print a summary box.

## Important Rules

1. **Check tech-stack.md first.** Unplanned modules are architectural drift.
2. **mod.rs is re-exports only.** Never put logic in mod.rs.
3. **Imports follow the standard.** std → external → internal, separated by blank lines.
4. **Include a test scaffold.** At minimum, a placeholder that confirms compilation.
5. **Justify dependencies.** Every new crate addition needs a comment citing constitution §8.
6. **Prefer `pub(crate)` over `pub`.** Only export what consumers need.

When finished, do not end the session, continue on to the next skill controlled by /go skill.
