# Migration Guide: Make/Just to Please

## Make -> Please
- Replace implicit target timestamps with explicit `inputs` and `outputs`.
- Move shell body from Make recipes into `run` entries.
- Add `deps` to capture prerequisite targets directly.

Example:
```make
build: src/main.rs
	cargo build --release
```

```toml
[task.build]
inputs = ["src/main.rs", "Cargo.toml"]
outputs = ["target/release/app"]
run = "cargo build --release"
```

## Just -> Please
- Keep commands mostly unchanged in `run`.
- Add deterministic contract (`inputs`, `outputs`) so cache and invalidation work.

Example:
```just
build:
  cargo build --release
```

```toml
[task.build]
inputs = ["src/**/*.rs", "Cargo.toml"]
outputs = ["target/release/app"]
run = "cargo build --release"
```

## Notes
- `please` treats undeclared output mutations as non-portable behavior.
- Prefer relative paths and explicit output declarations for reliable cache hits.
