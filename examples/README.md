# Examples

This folder contains runnable `broskifile` examples across languages and stacks.

## Included examples
- `minimal`: smallest possible task contract.
- `polyglot`: generated-data step + Rust backend build/package.
- `python-cli`: Python module tested with `unittest`.
- `go-http`: Go HTTP service with `go test` and build.
- `node-web`: Node HTTP service with `node:test` and build.
- `showcase`: React + Rust + Docker demonstration.

## Run examples
From repository root:

```bash
broski --workspace . run examples_smoke --explain
```

Run one example directly:

```bash
cargo run -p broski-cli -- --workspace examples/python-cli run ci --explain
```
