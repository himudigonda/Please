# Examples

This folder contains runnable `pleasefile` examples across languages and stacks.

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
please --workspace . run examples_smoke --explain
```

Run one example directly:

```bash
cargo run -p please-cli -- --workspace examples/python-cli run ci --explain
```
