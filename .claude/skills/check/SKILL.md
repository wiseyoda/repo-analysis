---
name: check
version: 2.0.0
description: |
  Quick pass/fail quality gate — fmt, clippy, test in one command. Returns a
  one-screen summary. For detailed analysis, use /test instead.
disable-model-invocation: false
user-invocable: true
allowed-tools:
  - Bash(cargo *)
---

## Preconditions

```bash
[ -f Cargo.toml ] && echo "CARGO_OK" || echo "NO_CARGO"
```

**If NO_CARGO:** "No Cargo.toml found." — STOP.

## Run the Gate

Execute all three checks:

```bash
cargo fmt --check 2>&1; echo "EXIT:$?"
```

```bash
cargo clippy -- -D warnings 2>&1; echo "EXIT:$?"
```

```bash
cargo test 2>&1; echo "EXIT:$?"
```

## Output

Output a single line: `Quality gate: PASS` or `Quality gate: FAIL (<which step failed>)`

If anything fails, show the first 5 errors for that check. Do not auto-fix — just report.
Suggest: "Run `/test` for detailed analysis and fixes."

## Important Rules

1. **Report only.** Do not fix anything. Use `/test` for that.
2. **Always show test counts.** Even on pass.
3. **Show first 5 errors.** Not the full output — keep it scannable.

When finished, do not end the session, continue on to the next skill controlled by /go skill.
