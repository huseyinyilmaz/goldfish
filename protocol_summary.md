# Memcached Protocol Summary

Based on [memcached protocol.txt](https://github.com/memcached/memcached/blob/master/doc/protocol.txt)

## Legend

| Icon | Meaning |
|---|---|
| ✅ | Implemented |
| ⚠️ | Partially implemented |
| ❌ | Not implemented |

## Protocol Basics

- Two kinds of data: **text lines** (commands/responses) and **unstructured data** (values).
- Every text line ends with `\r\n`.
- Unstructured data is also terminated by `\r\n`. Always use the **length prefix** (not the terminator) to find where data ends.
- **Keys**: max 250 characters, no control characters or whitespace.
- Command names are lower-case and case-sensitive.

---

## Storage Commands

All storage commands follow the same pattern:

```
<command> <key> <flags> <exptime> <bytes> [noreply]\r\n
<data block>\r\n
```

Arguments:
- `flags` — 16/32-bit unsigned integer, opaque to the server (stored and returned on get).
- `exptime` — 0 = never expire, or Unix time / offset seconds (if > 30 days treated as Unix time).
- `bytes` — length of the data block in bytes, not counting the trailing `\r\n`.
- `noreply` — optional; if present the server sends no response.

### set

Store a key-value pair. Always succeeds (creates or overwrites).

| | |
|---|---|
| **Status** | ✅ |
| **Request** | `set mykey 0 3600 5\r\nhello\r\n` |
| **Response** | `STORED\r\n` |

### add

Store only if key does **not** already exist.

| | |
|---|---|
| **Status** | ❌ |
| **Request** | `add mykey 0 3600 5\r\nhello\r\n` |
| **Response** | `STORED\r\n` on success, `NOT_STORED\r\n` if key exists |

### replace

Store only if key **already** exists.

| | |
|---|---|
| **Status** | ❌ |
| **Request** | `replace mykey 0 3600 5\r\nhello\r\n` |
| **Response** | `STORED\r\n` on success, `NOT_STORED\r\n` if key missing |

### append

Append data after the existing value. Does not accept flags or exptime.

| | |
|---|---|
| **Status** | ❌ |
| **Request** | `append mykey 0 0 5\r\nworld\r\n` |
| **Response** | `STORED\r\n` on success, `NOT_STORED\r\n` if key missing |

### prepend

Prepend data before the existing value. Does not accept flags or exptime.

| | |
|---|---|
| **Status** | ❌ |
| **Request** | `prepend mykey 0 0 5\r\nhello\r\n` |
| **Response** | `STORED\r\n` on success, `NOT_STORED\r\n` if key missing |

### cas

Check-And-Set. Store only if the CAS unique value matches (no one else modified the item since you last fetched it).

| | |
|---|---|
| **Status** | ❌ |
| **Request** | `cas mykey 0 3600 5 47\r\nhello\r\n` |
| **Response** | `STORED\r\n` on success, `EXISTS\r\n` if CAS mismatch, `NOT_FOUND\r\n` if key gone |

---

## Retrieval Commands

### get

Fetch one or more keys. Returns each found item, then `END\r\n`.

| | |
|---|---|
| **Status** | ⚠️ Single-key only. Multi-key (`get key1 key2 key3`) not supported. |
| **Request** | `get mykey\r\n` |
| **Response** | `VALUE mykey 0 5\r\nhello\r\nEND\r\n` |
| **Miss** | `END\r\n` |

Each found item returns:

```
VALUE <key> <flags> <bytes>\r\n
<data block>\r\n
```

### gets

Like `get` but also returns a CAS unique token for each item (used with `cas`).

| | |
|---|---|
| **Status** | ❌ |
| **Request** | `gets mykey\r\n` |
| **Response** | `VALUE mykey 0 5 47\r\nhello\r\nEND\r\n` |

The extra number after `<bytes>` is the CAS unique value.

### gat

Get-And-Touch. Fetch an item and update its expiration time at the same time.

| | |
|---|---|
| **Status** | ❌ |
| **Request** | `gat 3600 mykey\r\n` |
| **Response** | Same as `get`: `VALUE mykey 0 5\r\nhello\r\nEND\r\n` |

First argument is the new expiration time. Supports multiple keys.

### gats

Get-And-Touch with CAS. Like `gat` but also returns CAS unique.

| | |
|---|---|
| **Status** | ❌ |
| **Request** | `gats 3600 mykey\r\n` |
| **Response** | `VALUE mykey 0 5 47\r\nhello\r\nEND\r\n` |

---

## Other Commands

### delete

Explicitly delete an item by key.

| | |
|---|---|
| **Status** | ❌ |
| **Request** | `delete mykey\r\n` |
| **Response** | `DELETED\r\n` on success, `NOT_FOUND\r\n` if key missing |
| **With noreply** | `delete mykey noreply\r\n` |

### incr

Atomically increment a numeric value. The stored value must be a decimal 64-bit unsigned integer.

| | |
|---|---|
| **Status** | ❌ |
| **Request** | `incr counter 5\r\n` |
| **Response** | `15\r\n` (the new value after increment) |
| **Miss** | `NOT_FOUND\r\n` |
| **Note** | Wraps around on overflow. |

### decr

Atomically decrement a numeric value. Underflow stops at 0.

| | |
|---|---|
| **Status** | ❌ |
| **Request** | `decr counter 3\r\n` |
| **Response** | `12\r\n` (the new value after decrement) |
| **Miss** | `NOT_FOUND\r\n` |

### touch

Update the expiration time of an existing item without fetching it.

| | |
|---|---|
| **Status** | ❌ |
| **Request** | `touch mykey 3600\r\n` |
| **Response** | `TOUCHED\r\n` on success, `NOT_FOUND\r\n` if key missing |

### stats

Return server statistics.

| | |
|---|---|
| **Status** | ❌ |
| **Request** | `stats\r\n` |
| **Response** | Lines of `STAT <name> <value>\r\n` terminated by `END\r\n` |
| **Subcommands** | `stats items`, `stats slabs`, `stats conns`, etc. |

### flush_all

Immediately invalidate all existing items.

| | |
|---|---|
| **Status** | ❌ |
| **Request** | `flush_all\r\n` |
| **Response** | `OK\r\n` |

### version

Return the server version string.

| | |
|---|---|
| **Status** | ✅ |
| **Request** | `version\r\n` |
| **Response** | `VERSION Goldfish 1.0\r\n` |

### quit

Close the connection gracefully.

| | |
|---|---|
| **Status** | ✅ |
| **Request** | `quit\r\n` |
| **Response** | (connection closed, no response) |

---

## Meta Commands

Meta commands use 2-character command codes and a flag-based system. They can replace most basic commands with a more flexible interface.

Common flags across meta commands:
- `q` — noreply (suppress response on success)
- `O<token>` — opaque token echoed back in the response (for pipelining)
- `k` — return the key in the response
- `c` — return CAS value
- `t` — return TTL remaining
- `T<token>` — update TTL
- `v` — return value (for `mg`)
- `b` — key is base64 encoded binary

### mg (Meta Get)

Generic key retrieval. Replaces `get`/`gets`/`gat`/`gats`/`touch`.

| | |
|---|---|
| **Status** | ❌ |
| **Request** | `mg mykey v t\r\n` |
| **Response** (hit) | `VA 5 t3600\r\nhello\r\n` |
| **Response** (no value flag) | `HD t3600\r\n` |
| **Miss** | `EN\r\n` |

Flags:
- `v` — return value (response code changes to `VA <size>`)
- `t` — return TTL remaining
- `f` — return client flags
- `c` — return CAS token
- `h` — return hit-before flag (0/1)
- `l` — return last-access time in seconds
- `s` — return item size
- `k` — return key
- `T<secs>` — update TTL on hit
- `N<secs>` — vivify on miss (create stub item with TTL)
- `R<secs>` — win recache if TTL below threshold
- `u` — don't bump LRU
- `C<cas>` — conditional get (skip value if CAS matches)
- `E<cas>` — use token as new CAS value

### ms (Meta Set)

Generic storage. Replaces `set`/`add`/`replace`/`append`/`prepend`/`cas`.

| | |
|---|---|
| **Status** | ❌ |
| **Request** | `ms mykey 5 T3600\r\nhello\r\n` |
| **Response** | `HD\r\n` on success, `NS\r\n` (not stored), `EX\r\n` (exists/CAS clash), `NF\r\n` (not found) |

Flags:
- `T<secs>` — TTL for the item
- `F<num>` — set client flags
- `C<cas>` — compare CAS (conditional store)
- `E<cas>` — override CAS value
- `c` — return new CAS value
- `M<char>` — mode switch: `S`=set, `E`=add, `R`=replace, `A`=append, `P`=prepend
- `N<secs>` — autovivify on miss (in append mode)
- `I` — invalidate/stale marking

### md (Meta Delete)

Generic deletion. Can delete or mark items as stale.

| | |
|---|---|
| **Status** | ❌ |
| **Request** | `md mykey\r\n` |
| **Response** | `HD\r\n` on success, `NF\r\n` (not found), `EX\r\n` (CAS mismatch) |

Flags:
- `C<cas>` — conditional delete (only if CAS matches)
- `I` — mark as stale instead of deleting (item kept, value preserved)
- `x` — delete value but keep item (tombstone)
- `T<secs>` — update TTL when marking stale

### ma (Meta Arithmetic)

Generic increment/decrement. Replaces `incr`/`decr`.

| | |
|---|---|
| **Status** | ❌ |
| **Request** | `ma counter D5 v\r\n` |
| **Response** | `VA 2\r\n10\r\n` (if `v` flag supplied) or `HD\r\n` |

Flags:
- `D<num>` — delta to apply (default 1)
- `M<char>` — mode: `I`=increment, `D`=decrement
- `N<secs>` — auto-create on miss with TTL
- `J<num>` — initial value if auto-created (default 0)
- `v` — return new value
- `c` — return CAS value
- `t` — return TTL
- `T<secs>` — update TTL

### mn (Meta No-Op)

Always returns `MN\r\n`. Useful as a pipeline terminator when using `q` flag.

| | |
|---|---|
| **Status** | ❌ |
| **Request** | `mn\r\n` |
| **Response** | `MN\r\n` |

### me (Meta Debug)

Dump internal metadata for an item (human readable).

| | |
|---|---|
| **Status** | ❌ |
| **Request** | `me mykey\r\n` |
| **Response** (hit) | `ME mykey exp=0 la=123 cas=47 fetch=0 cls=1 size=5\r\n` |
| **Miss** | `EN\r\n` |

Fields: `exp` (expiration), `la` (last access), `cas`, `fetch` (hit count), `cls` (slab class), `size`.

---

## Admin Commands

### slabs reassign

Manually move a page between slab classes (memory management).

| | |
|---|---|
| **Status** | ❌ |
| **Request** | `slabs reassign -1 5\r\n` |
| **Response** | `OK\r\n`, `BUSY`, `BADCLASS`, `NOSPARE`, etc. |

### slabs automove

Enable/disable the background slab rebalancer.

| | |
|---|---|
| **Status** | ❌ |
| **Request** | `slabs automove 1\r\n` |
| **Response** | `OK\r\n` |

### lru

Tune LRU algorithm parameters (hot/warm/cold percentages, modes).

| | |
|---|---|
| **Status** | ❌ |
| **Request** | `lru mode segmented\r\n` |
| **Response** | `OK\r\n` or `ERROR` |

### lru_crawler

Control the background LRU crawler (expired item reclamation).

| | |
|---|---|
| **Status** | ❌ |
| **Request** | `lru_crawler crawl all\r\n` |
| **Response** | `OK\r\n`, `BUSY`, `BADCLASS` |

### watch

Turn connection into a watcher for internal events (fetches, evictions, mutations, etc.).

| | |
|---|---|
| **Status** | ❌ |
| **Request** | `watch fetchers evictions\r\n` |
| **Response** | `OK\r\n` then log lines in `key=value` format |

---

## Error Responses

All commands may return these error strings:

| Response | Meaning |
|---|---|
| `ERROR\r\n` | Unknown command name |
| `CLIENT_ERROR <message>\r\n` | Malformed input |
| `SERVER_ERROR <message>\r\n` | Server-side failure |

---

## Expiration Times

- **0** = never expire.
- **1 to 2592000** (30 days) = offset in seconds from current time.
- **> 2592000** = Unix timestamp (absolute time).
- **Negative** = immediately expired.
- TTL of 1 may expire immediately (time is tracked on second boundaries, ±1s).

---

## Authentication

Optional username/password via a fake `set` command:

```
set <anykey> <anyflags> <anyexptime> <bytes>\r\n
<username> <password>\r\n
```

- `STORED\r\n` = authentication success.
- `CLIENT_ERROR` = authentication failure.
