---
paths:
  - "Cargo.toml"
  - "Cargo.lock"
---

# Cargo Configuration Rules

When modifying Cargo.toml:

- Every new dependency must be justified against constitution.md §8 (Dependency Discipline)
- Check: is it well-maintained? Does it do something non-trivial? Is its dep tree reasonable?
- Pin major versions: use `"1"` not `"*"` or `">=1"`
- Dev dependencies go in `[dev-dependencies]`, not `[dependencies]`
- Features are explicit — no default features unless needed: `default-features = false`
- Add a comment above non-obvious dependencies explaining why they're needed
- Never manually edit `Cargo.lock` — let cargo manage it
- After adding a dep, run `cargo tree -d` to check for duplicate versions
