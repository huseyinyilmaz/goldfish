# Memcached ASCII Protocol — Goldfish Reference

Based on the [official memcached protocol specification](https://github.com/memcached/memcached/blob/master/doc/protocol.txt).

## General rules

- All text lines terminated with `\r\n`
- Command names are **lower-case** and **case-sensitive**
- Parameters on command lines are whitespace-delimited
- Data blocks are length-delimited (via `<bytes>` field), **never** scanned for `\r\n`
- Maximum key length: **250 characters**
- Keys must not contain control characters or whitespace

## Error strings (applicable to ALL commands)

| Response | Meaning |
|---|---|
| `ERROR\r\n` | Unknown / non-existent command name |
| `CLIENT_ERROR <error>\r\n` | Malformed input / protocol violation |
| `SERVER_ERROR <error>\r\n` | Server-side failure (may close connection) |

## Storage commands: `set`, `add`, `replace`, `append`, `prepend`

**Request format:**

```
<command> <key> <flags> <exptime> <bytes> [noreply]\r\n
<data block>\r\n
```

| Field | Details |
|---|---|
| `<key>` | Max 250 chars, no control characters or whitespace |
| `<flags>` | 32-bit unsigned integer. Opaque to server — stored and returned as-is. |
| `<exptime>` | `0` = never expire. If **> 30×24×60×60 (2,592,000)**, treated as **absolute Unix timestamp**. If **negative**, immediate expiry. |
| `<bytes>` | Length of data block excluding `\r\n`. May be `0`. |
| `noreply` | If present, server omits the response line. Warning: malformed requests may still elicit errors even with `noreply`. |

### Semantics

| Command | Behavior |
|---|---|
| `set` | Store unconditionally (overwrite if key exists) |
| `add` | Store **only if** key does **not** exist |
| `replace` | Store **only if** key **already** exists |
| `append` | Append data to existing key's value. Flags/exptime from original stored value are preserved. |
| `prepend` | Prepend data to existing key's value. Flags/exptime from original stored value are preserved. |

### Responses

| Response | When |
|---|---|
| `STORED\r\n` | Success |
| `NOT_STORED\r\n` | Condition not met (add on existing key, replace on missing key, append/prepend on missing key) |
| `EXISTS\r\n` | CAS conflict (CAS command only) |
| `NOT_FOUND\r\n` | CAS key doesn't exist (CAS command only) |

## Retrieval command: `get`

**Request:**

```
get <key1> [<key2> ...]\r\n
```

**Response** (zero or more items, terminated by `END`):

```
VALUE <key> <flags> <bytes>\r\n
<data block>\r\n
...
END\r\n
```

- Missing/expired/evicted keys are silently omitted from the response (no error)
- The `VALUE` line reports `<flags>` and `<bytes>` as originally stored
- Data block is exactly `<bytes>` bytes

## Statistics: `stats`

**Request:**

```
stats\r\n
```

**Response:**

```
STAT <name> <value>\r\n
...
END\r\n
```

### Returned statistics

| STAT | Description |
|---|---|
| `pid` | Server process ID |
| `uptime` | Seconds since server start |
| `time` | Current Unix timestamp |
| `version` | Server version string |
| `pointer_size` | Size of `usize` in bits (32 or 64) |
| `curr_items` | Number of items currently stored |
| `total_items` | Total items stored since server start |
| `bytes` | Total bytes of item data currently stored |
| `curr_connections` | Always 0 (not tracked) |
| `total_connections` | Always 0 (not tracked) |
| `connection_structures` | Always 0 (not tracked) |
| `cmd_get` | Total GET commands received |
| `cmd_set` | Total storage commands received (set/add/replace/append/prepend) |
| `get_hits` | GET commands that returned at least one item |
| `get_misses` | GET commands that returned no items |
| `evictions` | Always 0 (never evict) |
| `bytes_read` | Always 0 (not tracked) |
| `bytes_written` | Always 0 (not tracked) |
| `limit_maxbytes` | Always 0 (no memory limit) |

**Subcommands** (e.g. `stats items`, `stats slabs`): parsed but return `END\r\n`.

## Cache management: `flush_all`

**Request:**

```
flush_all [delay] [noreply]\r\n
```

| Field | Details |
|---|---|
| `[delay]` | Optional seconds to wait before flushing (parsed but flush is immediate in the current implementation) |

**Response:**

| Response | When |
|---|---|
| `OK\r\n` | Success |

## Retrieval command: `gets`

**Request:**

```
gets <key1> [<key2> ...]\r\n
```

**Response** (zero or more items, terminated by `END`):

```
VALUE <key> <flags> <bytes> <cas_unique>\r\n
<data block>\r\n
...
END\r\n
```

Same semantics as `get` but each VALUE line includes a unique CAS token.

## Check and Set: `cas`

**Request:**

```
cas <key> <flags> <exptime> <bytes> <cas_unique> [noreply]\r\n
<data block>\r\n
```

Same fields as `set` plus `<cas_unique>` — a 64-bit opaque CAS token (obtained from a prior `gets` response).

### Responses

| Response | When |
|---|---|
| `STORED\r\n` | Key exists and provided `cas_unique` matches |
| `EXISTS\r\n` | Key exists but `cas_unique` does not match (modified since last `gets`) |
| `NOT_FOUND\r\n` | Key does not exist |

## Deletion command: `delete`

**Request:**

```
delete <key> [noreply]\r\n
```

**Responses:**

| Response | When |
|---|---|
| `DELETED\r\n` | Success |
| `NOT_FOUND\r\n` | Key does not exist |

## Arithmetic commands: `incr`, `decr`

**Request:**

```
incr <key> <delta> [noreply]\r\n
decr <key> <delta> [noreply]\r\n
```

| Field | Details |
|---|---|
| `<key>` | Existing key whose value is a decimal number |
| `<delta>` | 64-bit unsigned integer to add or subtract |

**Responses:**

| Response | When |
|---|---|
| `<new_value>\r\n` | Success (new value as decimal string) |
| `NOT_FOUND\r\n` | Key does not exist |
| `CLIENT_ERROR cannot increment or decrement non-numeric value\r\n` | Stored value is not a valid unsigned integer |

- `decr` clamps to 0 (never goes below)
- Original flags and exptime are preserved
- The stored value is replaced with the new decimal string

## Meta commands

### `version`

```
version\r\n
```

Response: `VERSION <string>\r\n`

### `quit`

```
quit\r\n
```

Response: none (server closes connection).

## Command reference (current status)

| Command | Implemented | Notes |
|---|---|---|
| `set` | Yes | |
| `get` | Yes | Multi-key supported |
| `add` | Yes | |
| `replace` | Yes | |
| `append` | Yes | |
| `prepend` | Yes | |
| `delete` | Yes | |
| `incr` | Yes | |
| `decr` | Yes | |
| `version` | Yes | |
| `quit` | Yes | |
| `flush_all` | Yes | Immediate flush (delay is parsed but not applied) |
| `stats` | Yes | General stats with counter tracking |
| `cas` | Yes | Check-and-set with CAS token — returns `STORED`/`EXISTS`/`NOT_FOUND` |
| `gets` | Yes | Like `get` but includes `cas_unique` in VALUE line |
| `gets` | No | |
