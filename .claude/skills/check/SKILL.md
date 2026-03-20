---
name: check
description: Quick quality check — fmt, clippy, test in one command
disable-model-invocation: true
user-invocable: true
allowed-tools: Bash(cargo *)
---

Run the full quality gate:

```bash
cargo fmt --check && cargo clippy -- -D warnings && cargo test
```

Report results as:

```
fmt:    PASS/FAIL
clippy: PASS/FAIL (N warnings)
test:   PASS/FAIL (N passed, M failed)
```

If anything fails, show the specific errors and suggest fixes.
Do not auto-fix — just report. Use `/test` to fix issues.
