#!/bin/bash
# Session start: inject project context for Claude
# Provides current phase, recent activity, and reminders

PROJECT_DIR="${CLAUDE_PROJECT_DIR:-.}"

echo "=== repostat session ==="

# Show current git state if available
if command -v git &>/dev/null && [ -d "$PROJECT_DIR/.git" ]; then
  BRANCH=$(git -C "$PROJECT_DIR" branch --show-current 2>/dev/null)
  LAST_COMMIT=$(git -C "$PROJECT_DIR" log --oneline -1 2>/dev/null)
  DIRTY=$(git -C "$PROJECT_DIR" status --porcelain 2>/dev/null | head -5)

  echo "Branch: $BRANCH"
  echo "Last commit: $LAST_COMMIT"
  if [ -n "$DIRTY" ]; then
    echo "Uncommitted changes:"
    echo "$DIRTY"
  fi
fi

# Show active roadmap phase
if [ -f "$PROJECT_DIR/ROADMAP.md" ]; then
  PHASE=$(grep -m1 '^\- \[ \]' "$PROJECT_DIR/ROADMAP.md" 2>/dev/null | head -1)
  if [ -n "$PHASE" ]; then
    echo "Next task: $PHASE"
  fi
fi

# Remind about mandatory reading
echo ""
echo "Reminders: Read docs/constitution.md before ANY code changes. TDD required. No unwrap() in non-test code."
