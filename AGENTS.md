# Goldfish — concurrent memcached implementation in Rust

## Active version

**v3** is the only active version. CI only runs against `v3/`. v2 is archived.

All commands must use `--manifest-path=v3/Cargo.toml` from the repo root, or run inside `v3/`.

## Developer commands (run from repo root or `v3/`)

```sh
cargo build --manifest-path=v3/Cargo.toml
cargo test --manifest-path=v3/Cargo.toml          # all tests
cargo test --manifest-path=v3/Cargo.toml -- test_suite_name
cargo fmt --manifest-path=v3/Cargo.toml -- --check
cargo clippy --manifest-path=v3/Cargo.toml -- -D warnings
cargo check --manifest-path=v3/Cargo.toml
cargo build --release --locked --manifest-path=v3/Cargo.toml
```

Run order from CI: `check` → `build --release --locked` → `test` → `fmt -- --check` → `clippy -- -D warnings`.

## Benchmarks

Six criterion benchmarks in `v3/benches/benchmarks.rs` (10,000 pipelined commands each, `BatchSize::SmallInput`):

| Benchmark | Type | Mix |
|---|---|---|
| `st_write_heavy_5p_read_95p_write` | single-threaded | 95% SET, 5% GET |
| `st_read_heavy_95p_read_5p_write` | single-threaded | 95% GET, 5% SET (pre-pop 10k keys) |
| `st_balanced_50p_read_50p_write` | single-threaded | 50% SET, 50% GET |
| `mt_write_heavy_5p_read_95p_write` | 100 clients | 95% SET, 5% GET |
| `mt_read_heavy_95p_read_5p_write` | 100 clients | 95% GET, 5% SET (pre-pop 10k keys) |
| `mt_balanced_50p_read_50p_write` | 100 clients | 50% SET, 50% GET |

```sh
cargo criterion --manifest-path=v3/Cargo.toml        # run + HTML report

# To use named baselines with cargo-criterion, freeze results manually:
#   cargo criterion
#   cp -r target/criterion target/criterion.baseline-v1
#   ... make changes ...
#   rm -rf target/criterion && cp -r target/criterion.baseline-v1 target/criterion
#   cargo criterion    # compares against v1
```

HTML report: `v3/target/criterion/report/index.html` (open in browser).

## Tests

Integration tests live in `v3/tests/`. They test at the parser level via `goldfish::process_input()` (no TCP). No external services needed.

Implemented commands covered: `set`, `get`, `version`. Tests use `noreply` variants and multi-command inputs.

## Config & env

- Config file: `goldfish.*` (optional, auto-discovered) or env vars with `GOLDFISH_*` prefix.
- Log level: `GOLDFISH_LOG_LEVEL` env var (default: `info`).
- Default address: `0.0.0.0:11211` (standard memcached port).

## Architecture

```
main.rs → lib.rs::run() → run_server()
                              ├── settings.rs (config loading)
                              ├── state.rs (HashMap<Vec<u8>, Data>)
                              ├── parser/*.rs (nom-based command parsers)
                              └── handler/*.rs (command handlers)
```

Public API for testing: `goldfish::process_input(state: &Arc<Mutex<State>>, input: &[u8]) -> Option<Vec<u8>>`.

State is an `Arc<Mutex<State>>` — a single `HashMap<Vec<u8>, Data>` shared across all connections.

Parsers use `nom::branch::alt` with a catch-all `CannotParse` fallback — always succeeds.

## Implemented memcached commands

`set`, `get`, `quit`, `version`. The `Command` enum and `CommandResponse` enum in `v3/src/parser/` define the protocol.

## Rust toolchain

edition 2021, stable toolchain. Dependencies: tokio (full), nom 8.0, config, env_logger, log, serde.

## Not implemented

`add`, `replace`, `append`, `prepend`, `cas`, `gets`, `gat`, `gats`, `delete`, `incr`/`decr`, `touch`, `stats`, `flush_all`, meta commands.
