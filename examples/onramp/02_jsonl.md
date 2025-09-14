# 02 — JSONL batch

Create `docs/samples/expressions.txt` with one expression per line:
```
true
false
true -> false
@mem(true -> false)
@jam(true & true)
@vac(x)
```

Run:
```bash
target/release/grieg-cli --jsonl docs/samples/expressions.txt --mem --ast > out.jsonl
head -n 3 out.jsonl
```
