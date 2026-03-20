# Constitution

> **This document is IMMUTABLE.** It must never be modified, overridden, or weakened.
> All other project documents, decisions, and code must conform to these principles.
> If a conflict arises between this constitution and any other document, this constitution wins.

## Core Principles

### 1. Correctness Over Cleverness

Code must be correct first. A simple, readable solution that works is always preferred over
a clever one that might fail. Every behavior must be provable through tests.

### 2. Tests Are Not Optional

- No feature ships without tests that preceded its implementation (TDD).
- No refactor lands without the existing test suite passing.
- Test coverage is a floor, not a ceiling. Cover edge cases, error paths, and invariants.
- If a bug is found, a failing test is written BEFORE the fix.

### 3. Spec Before Code (SDD)

- Every feature begins as a specification describing WHAT, not HOW.
- The spec is reviewed and agreed upon before implementation starts.
- Code implements the spec. If the spec is wrong, update the spec first, then the code.

### 4. Simplicity Is Non-Negotiable

- Every line of code must justify its existence.
- Abstractions are introduced only when duplication is proven (Rule of Three).
- Prefer deletion over deprecation. Dead code is removed, not commented out.
- Configuration is added only when two or more concrete use cases demand it.

### 5. Explicit Over Implicit

- No hidden side effects. Functions do what their name says.
- Errors are propagated, never swallowed. Use `Result<T, E>` — not panics, not silent defaults.
- Dependencies are declared, not assumed. If it's not in `Cargo.toml`, it doesn't exist.

### 6. The User Comes First

- CLI ergonomics matter. Flags, help text, error messages — all user-facing text is crafted.
- Fast by default. The user should never wait for something the machine can do instantly.
- Failure is communicated clearly. The user always knows what happened and what to do next.

### 7. Refactor Relentlessly

- When you touch code, leave it better than you found it.
- Refactoring is not a separate task — it is part of every task.
- Technical debt is paid immediately when the cost is small. It is tracked when the cost is large.

### 8. Dependency Discipline

- Every dependency must earn its place. Prefer std-lib solutions.
- A dependency is justified only when it is: (a) well-maintained, (b) does something non-trivial
  to implement, and (c) does not pull in a transitive dependency tree larger than the value
  it provides.

### 9. Immutable Data by Default

- Data structures are immutable unless mutation is required by the algorithm.
- Shared mutable state is the root of most bugs. Avoid it. When unavoidable, contain it.

### 10. Reproducibility

- Given the same input, the tool produces the same output. Always.
- Snapshots are deterministic. AI analysis results are stored, not regenerated.
- Builds are reproducible. `cargo build` today and `cargo build` next month produce
  equivalent binaries given the same `Cargo.lock`.

## Enforcement

- CI must enforce these principles through linting, testing, and formatting checks.
- Code review must verify adherence. "It works" is necessary but not sufficient.
- Any agent (human or AI) writing code for this project must read and follow this constitution.
- Violations are not merged. There are no exceptions.
