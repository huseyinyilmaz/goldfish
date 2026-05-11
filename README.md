
Concurrent memcached implementation in Rust.

- Sample interactions: `notebooks/basic_commands.ipynb`
- Protocol reference: `PROTOCOL.md` and `protocol_summary.md`
- [Memcached protocol specification](https://github.com/memcached/memcached/blob/master/doc/protocol.txt)

## Protocol Support

### Storage Commands

| Command | Status | Notes |
|---|---|---|
| `set` | ✅ | Full support, including `noreply` |
| `add` | ✅ | Full support, including `noreply` |
| `replace` | ✅ | Full support, including `noreply` |
| `append` | ✅ | Preserves original flags and exptime |
| `prepend` | ✅ | Preserves original flags and exptime |
| `cas` | ✅ | Check-and-set with CAS token — STORED/EXISTS/NOT_FOUND |

### Retrieval Commands

| Command | Status | Notes |
|---|---|---|
| `get` | ✅ | Full support, including multi-key |
| `gets` | ✅ | Like get but includes cas_unique in VALUE line |
| `gat` | ✅ | Get and touch — returns value and updates expiration |
| `gats` | ✅ | Get and touch with CAS — returns value+cas and updates expiration |

### Other Commands

| Command | Status | Notes |
|---|---|---|
| `delete` | ✅ | Full support, including `noreply` |
| `incr` / `decr` | ✅ | Full support, including `noreply` |
| `touch` | ✅ | Update key expiration — TOUCHED/NOT_FOUND |
| `stats` | ✅ | General stats with counter tracking |
| `flush_all` | ✅ | Immediate flush (delay parsed but not applied) |
| `version` | ✅ | |
| `quit` | ✅ | |

### Meta Commands

| Command | Status |
|---|---|
| `mg`, `ms`, `md`, `ma`, `mn`, `me` | ❌ |

## Benchmarks

10,000 pipelined commands (4 KB values), Goldfish vs memcached 1.6.41.

| Benchmark | Goldfish | Memcached | Ratio |
|---|---|---|---|---|
| single-threaded, 95% write | 8,373 ops/s | 7,715 ops/s | **1.09x** |
| single-threaded, 95% read | 6,705 ops/s | 7,258 ops/s | 0.92x |
| single-threaded, 50/50 | 8,360 ops/s | 8,043 ops/s | **1.04x** |
| 100 clients, 95% write | 10,596 ops/s | 9,771 ops/s | **1.08x** |
| 100 clients, 95% read | 7,786 ops/s | 9,266 ops/s | 0.84x |
| 100 clients, 50/50 | 9,140 ops/s | 9,719 ops/s | 0.94x |

Goldfish leads on write-heavy workloads (1.08–1.09x) and trails on reads (6–16%).

