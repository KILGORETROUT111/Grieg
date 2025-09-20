#!/usr/bin/env bash
set -euo pipefail
SELF="$(realpath "${BASH_SOURCE[0]}")"
QA_DIR="$(dirname "$SELF")"
REPO="$(git -C "$QA_DIR" rev-parse --show-toplevel 2>/dev/null || realpath "$QA_DIR/../..")"
HOOK="$REPO/.git/hooks/pre-push"

echo "[info] repo: $REPO"
echo "[info] qa dir: $QA_DIR"

# README links
cd "$REPO"
[ -f README.md ] || echo "# Grieg" > README.md
if ! grep -q "Quality & Review Aids" README.md; then
  cat >> README.md <<'MD'

## Quality & Review Aids

- **QA Checklist:** [docs/quality/qa-checklist.md](docs/quality/qa-checklist.md)  
- **QA Positioning:** [docs/quality/qa-positioning.md](docs/quality/qa-positioning.md)
- **Runbook (Connector):** [docs/quality/runbook-telegram-connector.md](docs/quality/runbook-telegram-connector.md)
- **Runbook (HTTP Adapter):** [docs/quality/runbook-engine-http-adapter.md](docs/quality/runbook-engine-http-adapter.md)
MD
  echo "[ok] README linked to QA docs"
fi

# Install QA gate
mkdir -p "$HOME/bin"
cp -f "$QA_DIR/qa-gate.sh" "$HOME/bin/grieg-qa.sh"
chmod +x "$HOME/bin/grieg-qa.sh"
echo "[ok] installed ~/bin/grieg-qa.sh"

# Pre-push hook
mkdir -p "$(dirname "$HOOK")"
cat > "$HOOK" <<'HOOK'
#!/usr/bin/env bash
set -euo pipefail
if ! "$HOME/bin/grieg-qa.sh"; then
  echo
  echo "Pre-push blocked by QA gate. Fix issues above or bypass with --no-verify (not recommended)."
  exit 1
fi
HOOK
chmod +x "$HOOK"
echo "[ok] installed .git/hooks/pre-push"

# First run
"$HOME/bin/grieg-qa.sh" || true
echo "[DONE] QA package installed."
