#!/usr/bin/env bash
# init.sh — post-install setup for ascii-chord skill
# Run once after installing the skill into your workspace.

set -e

SKILL_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
echo "Initializing ascii-chord skill at: $SKILL_DIR"

# 1. Create .gitignore to exclude cargo build artifacts
GITIGNORE="$SKILL_DIR/.gitignore"
if [ ! -f "$GITIGNORE" ]; then
  echo "/target" > "$GITIGNORE"
  echo "✔ Created .gitignore (excludes /target)"
else
  if ! grep -q "^/target" "$GITIGNORE"; then
    echo "/target" >> "$GITIGNORE"
    echo "✔ Added /target to existing .gitignore"
  else
    echo "✔ .gitignore already configured"
  fi
fi

# 2. Pre-warm cargo build cache
echo "Building ascii-chord (first build may take 30-60 seconds)..."
cd "$SKILL_DIR"
cargo build --release 2>/dev/null && echo "✔ Build complete — subsequent runs will be fast"

echo ""
echo "ascii-chord is ready! Try: cargo run -- get Am"
