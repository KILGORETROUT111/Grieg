# grieg (Python binding)

```bash
pipx install maturin   # or: pip install maturin
maturin develop        # from this directory
python -c "import grieg; print(grieg.eval('@mem(true -> false)', mem=True, ast=True))"
```
