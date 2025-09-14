# 01 — Basics

```bash
cargo build --release
target/release/grieg-cli --expr 'true -> false' --pretty
target/release/grieg-cli --expr '@mem(true & true)' --mem --pretty
target/release/grieg-cli --expr '@vac(x)' --mem --pretty
```
Expected (shape):
```
Input: @mem(true & true)
Value: true
Phase: MEM
```
