---
name: review
description: Code review against project standards. Use before committing.
disable-model-invocation: true
user-invocable: true
allowed-tools: Read, Grep, Glob, Bash(cargo *), Bash(git diff*)
---

Perform a thorough code review of the current changes against project standards.

## Scope

If $ARGUMENTS is provided, review only those files/modules. Otherwise, review all
uncommitted changes (`git diff` and `git diff --cached`).

## Review Checklist

Work through each item. Report violations with file:line references.

### Constitution Compliance (docs/constitution.md)
- [ ] No `unwrap()` or `expect()` in non-test code
- [ ] No `panic!()` in library code
- [ ] All new features have tests that were written FIRST (TDD)
- [ ] Errors are propagated with `Result<T, E>`, never swallowed
- [ ] No unnecessary abstractions (Rule of Three)
- [ ] New dependencies justified per §8 (Dependency Discipline)

### Coding Standard (docs/coding-standard.md)
- [ ] Functions under 40 lines
- [ ] Max 4 parameters per function
- [ ] Early returns, no deep nesting
- [ ] Imports grouped: std → external → internal
- [ ] All `pub` items have `///` doc comments
- [ ] Error types use `thiserror`, app boundary uses `anyhow`
- [ ] Naming conventions followed (PascalCase types, snake_case functions)

### Architecture (docs/tech-stack.md)
- [ ] Code is in the correct module per the directory structure
- [ ] Module `mod.rs` files only re-export, no logic
- [ ] Public API surface is minimal (`pub(crate)` by default)

### CLI Standards
- [ ] Errors go to stderr, data to stdout
- [ ] Error messages are actionable (what happened + what to do)
- [ ] `NO_COLOR` respected for terminal output

## Output Format

For each finding:
```
[SEVERITY] file:line — Description
  Suggestion: How to fix
```

Severities: CRITICAL (blocks merge), WARNING (should fix), NOTE (consider fixing).

End with a summary: total findings by severity, overall verdict (APPROVE / REQUEST CHANGES).
