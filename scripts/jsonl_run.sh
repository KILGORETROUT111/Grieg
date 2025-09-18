#!/usr/bin/env bash
set -euo pipefail
FILE="${1:?usage: scripts/jsonl_run.sh <jsonl-file> [global_mem:true|false]}"
GLOBAL_MEM="${2:-false}"

while IFS= read -r line || [ -n "$line" ]; do
  # skip empties
  [ -z "$line" ] && continue

  expr=$(echo "$line" | jq -r '.expr')
  mem=$(echo "$line" | jq -r '.mem // empty')
  expect=$(echo "$line" | jq -r '.expect_phase // empty')
  note=$(echo "$line" | jq -r '.note // empty')

  # build args
  args=(--expr "$expr" --ast)
  if [ "$mem" = "true" ] || { [ -z "$mem" ] && [ "$GLOBAL_MEM" = "true" ]; }; then
    args+=(--mem)
  fi

  # run CLI silently; capture its JSON
  out=$(cargo run -q -p grieg-cli -- "${args[@]}" || true)

  # extract phase from CLI JSON
  phase=$(echo "$out" | jq -r '.phase // empty')

  # ok flag if expect provided
  ok=null
  if [ -n "$expect" ]; then
    if [ "$phase" = "$expect" ]; then ok=true; else ok=false; fi
  fi

  # print one JSON object
  jq -n \
    --arg input "$expr" \
    --arg phase "$phase" \
    --arg exp "$expect" \
    --arg note "$note" \
    --argjson ok "$ok" \
    '{
       input:$input,
       phase:$phase,
       ok:$ok,
       expect_phase: (if $exp=="" then null else $exp end),
       note: (if $note=="" then null else $note end)
     }'
done < "$FILE"
