---
paths:
  - "docs/**/*.md"
  - "ROADMAP.md"
  - "BACKLOG.md"
---

# Documentation Rules

When editing project documentation:

- `docs/constitution.md` is IMMUTABLE — never modify it
- Requirements in `docs/requirements.md` use the ID format `R-NNN`
- New requirements get the next available ID in their phase range
- Decision records in `docs/decisions.md` follow the ADR template
- Accepted decisions are never deleted, only superseded
- Roadmap checkboxes are marked `[x]` only when the item is fully complete and tested
- Backlog items include: description, origin, and promotion criteria
- All dates are absolute (YYYY-MM-DD), never relative
