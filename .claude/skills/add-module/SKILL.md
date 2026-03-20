---
name: add-module
description: Scaffold a new module following project structure conventions
disable-model-invocation: true
user-invocable: true
allowed-tools: Read, Write, Edit, Bash(cargo *), Bash(mkdir *), Glob
---

Create a new module: $ARGUMENTS

## Process

### 1. Validate
- Check `docs/tech-stack.md` to confirm where this module belongs.
- Check `docs/requirements.md` to find relevant requirement IDs.
- If the module isn't in the planned structure, stop and ask before proceeding.

### 2. Scaffold
Create the module with this structure:

```
src/<parent>/<module_name>/
├── mod.rs        # Re-exports only, no logic
└── <files>.rs    # Implementation files
```

Or for simple modules:
```
src/<parent>/<module_name>.rs
```

### 3. Module file template

`mod.rs` should only contain:
```rust
//! Brief description of the module.

mod implementation_file;

pub use implementation_file::PublicType;
pub(crate) use implementation_file::InternalType;
```

Implementation files should start with:
```rust
//! Detailed description of this file's responsibility.

use std::...;

use external_crate::...;

use crate::...;
```

### 4. Wire up
- Add `mod <module_name>;` to the parent module.
- Add any new dependencies to `Cargo.toml` (with justification comment).

### 5. Test scaffold
- Add a `#[cfg(test)] mod tests` block with at least one placeholder test.
- Run `cargo test` to verify compilation.

### 6. Report
- List all created files.
- Show the module's position in the architecture.
