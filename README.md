
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
|---|---|---|---|
| single-threaded, 95% write | 10,135 ops/s | 8,064 ops/s | **1.26x** |
| single-threaded, 95% read | 6,894 ops/s | 7,431 ops/s | 0.93x |
| single-threaded, 50/50 | 7,624 ops/s | 8,089 ops/s | 0.94x |
| 100 clients, 95% write | 10,411 ops/s | 9,817 ops/s | **1.06x** |
| 100 clients, 95% read | 8,991 ops/s | 9,261 ops/s | 0.97x |
| 100 clients, 50/50 | 9,416 ops/s | 10,113 ops/s | 0.93x |

Goldfish leads on write-heavy workloads (1.06–1.26x) and trails slightly on reads (3–7%).

