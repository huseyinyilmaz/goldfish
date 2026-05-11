# Goldfish

Concurrent memcached implementation in Rust. Drop-in replacement for memcached that matches its protocol behavior and outperforms it on write-heavy workloads.

## Features

- **Full memcached ASCII protocol** — supports all standard commands including `get`/`set`, `add`, `replace`, `append`, `prepend`, `cas`, `gets`, `gat`/`gats`, `touch`, `delete`, `incr`/`decr`, `stats`, `flush_all`, `version`, `quit`
- **Drop-in compatible** — listens on the default memcached port (`11211`), speaks the same wire protocol, verified by a 69-check protocol compliance suite
- **Concurrent** — single shared `HashMap` behind `Arc<RwLock<>` allows parallel reads
- **Pipeline friendly** — processes pipelined commands in a single batch
- **Async** — built on `tokio` for efficient connection handling

## Quick start

```sh
git clone <repo>
cd goldfish
cargo build --release --locked
./target/release/goldfish
```

Connect with any memcached client:

```sh
echo -e "set mykey 0 0 5\r\nhello\r\n" | nc localhost 11211
echo -e "get mykey\r\n" | nc localhost 11211
```

## Usage

A convenience `run` script is provided:

```sh
./run build              # cargo build
./run run_release        # build release binary and run it
./run check              # cargo check
./run test               # cargo test
./run fmt                # cargo fmt --check
./run clippy             # cargo clippy -D warnings
```

### Benchmarks

```sh
./run bench                           # run criterion benchmarks
./run bench baseline create [name]    # create a saved baseline
./run bench baseline list             # list saved baselines
./run benchmark                       # Python benchmark against a server
```

### Protocol compliance

```sh
./run check_protocol                  # run protocol compliance check
./run check_protocol --host HOST --port PORT
```

## Configuration

Goldfish discovers configuration automatically (first match wins):

| Method | Example |
|--------|---------|
| Config file | `goldfish.toml`, `goldfish.json`, `goldfish.yaml` |
| Environment | `GOLDFISH_PORT=11211` `GOLDFISH_IP_ADDRESS=0.0.0.0` |

Default address: `0.0.0.0:11211` (standard memcached port).

Set log level with the `GOLDFISH_LOG_LEVEL` env var (default: `info`).

## Protocol support

Every command has been tested against both goldfish and memcached 1.6.41 via the protocol compliance checker.

| Command | Notes |
|---------|-------|
| `set`, `add`, `replace`, `append`, `prepend` | Full support, including `noreply` |
| `cas` | Check-and-set with CAS token — `STORED`/`EXISTS`/`NOT_FOUND` |
| `get`, `gets` | Multi-key support; `gets` includes `cas_unique` token |
| `get`, `gets`, `gat`, `gats` | Multi-key support; `gat`/`gats` update expiration |
| `delete` | Full support, including `noreply` |
| `incr`, `decr` | Full support; `decr` clamps to 0; includes `noreply` |
| `touch` | Update key expiration — `TOUCHED`/`NOT_FOUND` |
| `stats` | General stats with counter tracking |
| `flush_all` | Honors delay argument |
| `version`, `quit` | Standard meta commands |

Meta commands (`mg`, `ms`, `md`, `ma`, `mn`, `me`) are not implemented.

Full protocol reference: `PROTOCOL.md` and
[memcached protocol specification](https://github.com/memcached/memcached/blob/master/doc/protocol.txt).

## Architecture

```
main.rs → lib.rs::run() → run_server()
                              ├── settings.rs (config loading)
                              ├── state.rs (HashMap<Vec<u8>, Data>)
                              ├── parser/*.rs (nom-based command parsers)
                              └── handler/*.rs (command handlers)
```

- **State** is a single `HashMap<Vec<u8>, Data>` behind `Arc<RwLock<>>` shared across all connections. GETs acquire a read lock (parallel reads), writes acquire a write lock.
- **Parsers** use `nom` combinators with a catch-all fallback — every input produces a valid parse result.
- **Integration tests** exercise the parser and handler layers directly via `goldfish::process_input()` without TCP.

## Benchmarks

10,000 pipelined commands (4 KB values), Goldfish vs memcached 1.6.41.

| Benchmark | Goldfish | Memcached | Ratio |
|---|---|---|---|
| single-threaded, 95% write | 8,373 ops/s | 7,715 ops/s | **1.09x** |
| single-threaded, 95% read | 6,705 ops/s | 7,258 ops/s | 0.92x |
| single-threaded, 50/50 | 8,360 ops/s | 8,043 ops/s | **1.04x** |
| 100 clients, 95% write | 10,596 ops/s | 9,771 ops/s | **1.08x** |
| 100 clients, 95% read | 7,786 ops/s | 9,266 ops/s | 0.84x |
| 100 clients, 50/50 | 9,140 ops/s | 9,719 ops/s | 0.94x |

Goldfish leads on write-heavy workloads (1.08–1.09x) and trails on reads (6–16%).

## Development

```sh
cargo build
cargo test                    # all tests
cargo test -- test_suite_name
cargo fmt -- --check
cargo clippy -- -D warnings
cargo check
cargo build --release --locked
```

CI order: `check` → `build --release --locked` → `test` → `fmt` → `clippy`.

### Dependencies

- Rust edition 2021, stable toolchain
- `tokio` (full) — async runtime
- `nom` 8.0 — parser combinators
- `config` + `serde` — configuration
- `env_logger` + `log` — logging
- `criterion` — benchmarks (dev dependency)

### Testing

Integration tests live in `tests/`. They test at the parser level via `goldfish::process_input()` — no TCP or external services needed.

Protocol compliance against a running server:

```sh
./run check_protocol          # tests :11211
./run check_protocol --port 11212
```

Criterion benchmarks in `benches/benchmarks.rs`:

```sh
cargo criterion               # run with HTML report
open target/criterion/report/index.html
```
