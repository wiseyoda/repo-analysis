#!/bin/bash
# Pre-commit hook: run fmt + clippy + test before git commit
# Enforces: coding-standard.md commit rules and constitution.md §2

INPUT=$(cat)
COMMAND=$(echo "$INPUT" | jq -r '.tool_input.command // empty')

# Only intercept git commit commands
if ! echo "$COMMAND" | grep -qE '^git commit'; then
  exit 0
fi

PROJECT_DIR="${CLAUDE_PROJECT_DIR:-.}"

# Check if Cargo.toml exists (project is initialized)
if [ ! -f "$PROJECT_DIR/Cargo.toml" ]; then
  exit 0
fi

# Run cargo fmt check
FMT_OUTPUT=$(cargo fmt --check --manifest-path "$PROJECT_DIR/Cargo.toml" 2>&1)
if [ $? -ne 0 ]; then
  echo "BLOCKED: cargo fmt check failed. Run 'cargo fmt' first." >&2
  echo "$FMT_OUTPUT" >&2
  exit 2
fi

# Run clippy
CLIPPY_OUTPUT=$(cargo clippy --manifest-path "$PROJECT_DIR/Cargo.toml" -- -D warnings 2>&1)
if [ $? -ne 0 ]; then
  echo "BLOCKED: cargo clippy has warnings. Fix all clippy warnings before committing." >&2
  echo "$CLIPPY_OUTPUT" | tail -20 >&2
  exit 2
fi

# Run tests
TEST_OUTPUT=$(cargo test --manifest-path "$PROJECT_DIR/Cargo.toml" 2>&1)
if [ $? -ne 0 ]; then
  echo "BLOCKED: cargo test failed. All tests must pass before committing." >&2
  echo "$TEST_OUTPUT" | tail -20 >&2
  exit 2
fi

exit 0
