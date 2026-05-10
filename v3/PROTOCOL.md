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
| `version` | Yes | |
| `quit` | Yes | |
| `cas` | No | |
| `gets` | No | |
