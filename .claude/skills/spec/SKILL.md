---
name: spec
version: 2.0.0
description: |
  Architect / tech writer. Write a feature spec (SDD) without implementing.
  Creates a docs/specs/ file with purpose, behavior, edge cases, and acceptance
  criteria. Use when asked to "spec", "design", "plan a feature", or "write a spec".
disable-model-invocation: false
user-invocable: true
allowed-tools:
  - Read
  - Write
  - Grep
  - Glob
  - AskUserQuestion
---

## Arguments

- `/spec <feature description>` — write a spec for the described feature
- `/spec R-NNN` — write a spec for a specific requirement ID
- `/spec` with no args — STOP with "Usage: `/spec <feature or requirement ID>`"

## Preconditions

Check if a spec already exists:

```bash
ls docs/specs/ 2>/dev/null || echo "NO_SPECS_DIR"
```

If `docs/specs/` doesn't exist, create it.

## Step 1: Research

Read project context to inform the spec:

1. `docs/requirements.md` — find the matching requirement IDs and their details
2. `docs/tech-stack.md` — understand where this feature fits architecturally
3. `docs/decisions.md` — check for relevant ADRs
4. `docs/constitution.md` — note constraints that apply
5. Existing code in `src/` — understand what already exists that this builds on

If $ARGUMENTS is a requirement ID (R-NNN), look it up directly.
Otherwise, search requirements.md for matching text.

## Step 2: Design

Think through the feature completely before writing. Consider:

- **Data flow:** What comes in? What goes out? What transforms happen?
- **Error cases:** What can go wrong at each step?
- **Edge cases:** Empty input, huge input, malformed input, concurrent access
- **Dependencies:** What existing modules does this need? Any new deps required?
- **Testing strategy:** How will we verify each behavior?

If the design has genuine alternatives with meaningful tradeoffs, use AskUserQuestion:
- Describe the decision point in plain English
- Present 2-3 options with effort and tradeoffs
- Recommend one

## Step 3: Write the Spec

Create `docs/specs/<feature-slug>.md`:

```markdown
# Spec: <Feature Name>

**Requirement IDs:** R-NNN, R-NNN
**Phase:** N
**Status:** Draft
**Date:** YYYY-MM-DD

## Purpose

Why this feature exists. What problem it solves. One paragraph.

## Architecture

Where this fits in the codebase. Which modules are involved.

```
  [input] → [module A] → [module B] → [output]
```

## Inputs

| Input | Type | Source | Validation |
|-------|------|--------|------------|
| path | PathBuf | CLI argument | Must exist, must be directory |

## Outputs

| Output | Type | Destination |
|--------|------|------------|
| metrics | FileMetrics | Aggregated into report |

## Behavior

Step-by-step description of what happens:

1. First, ...
2. Then, ...
3. Finally, ...

## Error Handling

| Error Condition | Response | User Sees |
|----------------|----------|-----------|
| Path not found | Return ScanError::PathNotFound | "path does not exist: /foo" |

## Edge Cases

- **Empty directory:** Returns zero metrics, no error
- **Permission denied:** Skip file, log warning, continue
- **Symlink loop:** Detect and skip (ignore crate handles this)

## Acceptance Criteria

- [ ] Criterion 1: <specific, testable statement>
- [ ] Criterion 2: <specific, testable statement>
- [ ] Criterion 3: <specific, testable statement>

## Testing Strategy

- **Unit tests:** <what to unit test and where>
- **Integration tests:** <what to integration test>
- **Edge case tests:** <specific edge cases to cover>

## Dependencies

| Crate | Why | New? |
|-------|-----|------|
| ignore | gitignore-aware file walking | No, already in tech-stack |

## Open Questions

- <anything unresolved that needs discussion>
```

## Step 4: Completion

When the spec is written and saved, the skill is done. Do NOT print a summary box.

## Important Rules

1. **Design only.** Do not write any Rust code. Do not create source files.
2. **Be specific.** Acceptance criteria must be testable. "It works" is not a criterion.
3. **Show the data flow.** ASCII diagram is mandatory for non-trivial features.
4. **Name the errors.** Don't say "handle errors." List each error type and the response.
5. **Edge cases are required.** At minimum: empty input, invalid input, large input.
6. **Reference requirement IDs.** Every spec maps to R-NNN IDs from requirements.md.
7. **Ask about real tradeoffs.** If there's a genuine design decision, use AskUserQuestion. Don't ask about things with obvious answers.
