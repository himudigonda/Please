# Please

`Please` is a deterministic task runner for polyglot projects.

## Alpha status
`Please` is currently in **alpha**. The first public alpha release is `v0.1.0-alpha.1`.
Use it for dogfooding and feedback, not as a guaranteed drop-in replacement for mature build systems yet.

## v0.1 capabilities
- TOML `pleasefile` parsing with semantic validation.
- DAG scheduling with deterministic topological layers.
- Content-hash fingerprints (BLAKE3) for task invalidation.
- Local CAS + SQLite cache metadata.
- Staged execution with transactional output promotion.
- Isolation policy support:
  - Linux: strict isolation via `bwrap`.
  - macOS: best-effort isolation (strict mode is not supported).

## Supported alpha binaries
- `x86_64-unknown-linux-gnu`
- `aarch64-apple-darwin`

Release assets are published as:
- `please-<tag>-x86_64-unknown-linux-gnu.tar.gz`
- `please-<tag>-aarch64-apple-darwin.tar.gz`
- `SHA256SUMS.txt`

## Install (release binary)
Install the latest published release:

```bash
curl -fsSL https://raw.githubusercontent.com/himudigonda/Please/main/install.sh | bash
```

Install a specific release tag:

```bash
PLEASE_VERSION=v0.1.0-alpha.1 curl -fsSL https://raw.githubusercontent.com/himudigonda/Please/main/install.sh | bash
```

By default, the binary installs to `~/.local/bin` (`INSTALL_DIR` can override this).

## Developer quick start
```bash
just setup
just ci
just run -- list
```

## Dogfooding mode (staged)
During alpha, both task runners are kept:
- Preferred: `please run ci`
- Fallback: `just ci`

The root `pleasefile` mirrors core quality gates (`fmt`, `lint`, `test`, `cov`, `ci`).

## Examples
- Minimal runnable demo: [`examples/basic/pleasefile`](examples/basic/pleasefile)
- Polyglot template: [`examples/polyglot/pleasefile`](examples/polyglot/pleasefile)

## Coverage gate
- `just ci` enforces coverage through `cargo llvm-cov`.
- Override threshold with `PLEASE_COVERAGE_MIN` (default `45` during bootstrap).

## Parser mode
- Default parser: TOML (`PLEASE_PARSER_MODE=toml`).
- Experimental parser path: `PLEASE_PARSER_MODE=winnow`.
