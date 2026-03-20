#!/bin/bash
# Pre-edit hook: block unsafe Rust patterns in non-test code
# Enforces constitution.md §2 (tests) and §5 (explicit errors)

INPUT=$(cat)
FILE=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')

# Only check Rust source files
if [[ ! "$FILE" =~ \.rs$ ]]; then
  exit 0
fi

# Get the new content being written
# For Edit tool: check new_string field
# For Write tool: check content field
NEW_CONTENT=$(echo "$INPUT" | jq -r '.tool_input.new_string // .tool_input.content // empty')

if [ -z "$NEW_CONTENT" ]; then
  exit 0
fi

# Allow unwrap/expect in test code
if echo "$NEW_CONTENT" | grep -q '#\[cfg(test)\]\|#\[test\]'; then
  exit 0
fi

# Check for unwrap() — but not in test modules or test functions
# We check if the file path contains "test" as an additional heuristic
if [[ "$FILE" =~ /tests/ ]] || [[ "$FILE" =~ _test\.rs$ ]]; then
  exit 0
fi

# Block unwrap() and expect() in non-test code
if echo "$NEW_CONTENT" | grep -qE '\.unwrap\(\)'; then
  echo "BLOCKED: .unwrap() detected in non-test code ($FILE). Use proper error handling with ? or map_err(). See docs/constitution.md §5 and docs/coding-standard.md." >&2
  exit 2
fi

if echo "$NEW_CONTENT" | grep -qE '\.expect\(' ; then
  echo "BLOCKED: .expect() detected in non-test code ($FILE). Use proper error handling with ? or map_err(). See docs/constitution.md §5 and docs/coding-standard.md." >&2
  exit 2
fi

# Block panic!() in library code (allow in main.rs and test code)
if [[ ! "$FILE" =~ main\.rs$ ]] && echo "$NEW_CONTENT" | grep -qE 'panic!\('; then
  echo "BLOCKED: panic!() detected in library code ($FILE). Return Result<T, E> instead. See docs/coding-standard.md." >&2
  exit 2
fi

exit 0
