#!/bin/bash
# Post-edit hook: auto-format Rust files after edits
# Runs async so it doesn't block the workflow

INPUT=$(cat)
FILE=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')

# Only format Rust files
if [[ ! "$FILE" =~ \.rs$ ]]; then
  exit 0
fi

PROJECT_DIR="${CLAUDE_PROJECT_DIR:-.}"

# Check if Cargo.toml exists
if [ ! -f "$PROJECT_DIR/Cargo.toml" ]; then
  exit 0
fi

# Run rustfmt on the specific file
if command -v rustfmt &>/dev/null; then
  rustfmt "$FILE" 2>/dev/null
fi

exit 0
