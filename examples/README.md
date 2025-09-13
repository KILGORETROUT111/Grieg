# Examples

Small, copy-pasteable snippets that exercise **Grieg** via the CLI.  
These examples are designed to be readable on mobile and runnable on a laptop without extra setup.


### Examples
See examples/ for copy-pasteable CLI snippets

- ✅ No extra crates needed  
- ✅ Works with `grieg-cli`  
- ✅ Shows **phase** output (ALIVE / JAM / MEM / VAC)  

If you're new here, start with examples/ counterfactuals.md

## How to run

From repo root:

```bash
# Build once
cargo build

# Run a single expression with pretty output and MEM enabled
cargo run -p grieg-cli -- --expr '@mem(true -> false)' --mem --pretty


