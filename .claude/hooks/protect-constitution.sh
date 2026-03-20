#!/bin/bash
# Protect immutable files from modification
# Enforces: constitution.md is IMMUTABLE

INPUT=$(cat)
FILE=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')

# Normalize path
BASENAME=$(basename "$FILE" 2>/dev/null)
RELPATH=$(echo "$FILE" | sed "s|.*/docs/|docs/|" 2>/dev/null)

if [ "$RELPATH" = "docs/constitution.md" ] || [ "$BASENAME" = "constitution.md" ]; then
  echo "BLOCKED: docs/constitution.md is IMMUTABLE and must never be modified. This is a core project principle. See the header of constitution.md." >&2
  exit 2
fi

exit 0
