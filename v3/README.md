[WIP]
Concurrent memcached implementation.


* memcached protocol
  https://github.com/memcached/memcached/blob/master/doc/protocol.txt



## Minimum Viable Protocol

For a functional memcached server that real clients can rely on, the following commands constitute the "base functionality" set:

| Command | Status | Why it's needed |
|---|---|---|
| `set` | ✅ | Core write — create or update any key |
| `get` | ✅ | Core read — retrieve values (multi-key supported) |
| `delete` | ❌ | Core delete — without it keys accumulate forever |
| `add` | ❌ | Conditional create — "store only if key doesn't exist" |
| `replace` | ❌ | Conditional update — "store only if key exists" |
| `incr` / `decr` | ❌ | Atomic counters — widely used for rate limits, sessions |
| `stats` | ❌ | Monitoring — required for health checks in production |
| `flush_all` | ❌ | Bulk invalidation — needed for cache lifecycle management |
| `quit` | ✅ | Clean connection teardown |
| `version` | ✅ | Server identification |

All other commands (append, prepend, cas, gets, gat, gats, touch, meta commands, admin commands) are enhancements beyond the base set.

## Protocol Support

### Storage Commands

| Command | Status | Notes |
|---|---|---|
| `set` | ✅ | Full support, including `noreply` |
| `add` | ❌ | |
| `replace` | ❌ | |
| `append` | ❌ | |
| `prepend` | ❌ | |
| `cas` | ❌ | |

### Retrieval Commands

| Command | Status | Notes |
|---|---|---|
| `get` | ✅ | Full support, including multi-key |
| `gets` | ❌ | Requires CAS tracking |
| `gat` | ❌ | |
| `gats` | ❌ | |

### Other Commands

| Command | Status | Notes |
|---|---|---|
| `delete` | ❌ | |
| `incr` / `decr` | ❌ | |
| `touch` | ❌ | |
| `stats` | ❌ | |
| `flush_all` | ❌ | |
| `version` | ✅ | |
| `quit` | ✅ | |

### Meta Commands

| Command | Status | Notes |
|---|---|---|
| `mg` | ❌ | |
| `ms` | ❌ | |
| `md` | ❌ | |
| `ma` | ❌ | |
| `mn` | ❌ | |
| `me` | ❌ | |

### Admin Commands

| Command | Status | Notes |
|---|---|---|
| `slabs reassign` | ❌ | |
| `slabs automove` | ❌ | |
| `lru` / `lru_crawler` | ❌ | |
| `watch` | ❌ | |
