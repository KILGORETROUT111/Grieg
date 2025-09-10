#!/usr/bin/env bash
set -euo pipefail
part=${1:-patch} # major|minor|patch
toml=Cargo.toml

current=$(grep '^version = "' "$toml" | head -1 | sed -E 's/.*"([^"]+)".*/\1/')
IFS=. read -r MA MI PA <<<"$current"
case "$part" in
  major) MA=$((MA+1)); MI=0; PA=0 ;;
  minor) MI=$((MI+1)); PA=0 ;;
  patch) PA=$((PA+1)) ;;
  *) echo "usage: $0 {major|minor|patch}"; exit 1 ;;
esac
next="${MA}.${MI}.${PA}"

# workspace root
sed -i -E '0,/^version = "/s/^version = ".*"/version = "'$next'"/' Cargo.toml

# member crates
for f in grieg-*/Cargo.toml; do
  sed -i -E '0,/^version = "/s/^version = ".*"/version = "'$next'"/' "$f"
  # keep path deps without versions; if versions present, update
  sed -i -E "s/(grieg-[a-z-]\".*version = \")([0-9]+\.[0-9]+\.[0-9]+)(\")/\\1$next\\3/g" "$f" || true
done

echo "Bumped version: $current -> $next"
git add Cargo.toml grieg-*/Cargo.toml
git commit -s -m "release: bump version to $next"
echo "Next: 'just tag' to create a release tag."
