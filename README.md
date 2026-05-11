
Concurrent memcached implementation in Rust.

- Sample interactions: https://github.com/huseyinyilmaz/goldfish/blob/main/notebooks/basic_commands.ipynb
- Memcached protocol reference: https://github.com/memcached/memcached/blob/master/doc/protocol.txt

Note: multiple versions exist as I reimplement learning more about Rust and the ecosystem. See `v3/` for the latest.

## Benchmarks

10,000 pipelined commands (4 KB values), Goldfish v3 vs memcached 1.6.41.

| Benchmark | Goldfish | Memcached | Ratio |
|---|---|---|---|
| single-threaded, 95% write | 10,135 ops/s | 8,064 ops/s | **1.26x** |
| single-threaded, 95% read | 6,894 ops/s | 7,431 ops/s | 0.93x |
| single-threaded, 50/50 | 7,624 ops/s | 8,089 ops/s | 0.94x |
| 100 clients, 95% write | 10,411 ops/s | 9,817 ops/s | **1.06x** |
| 100 clients, 95% read | 8,991 ops/s | 9,261 ops/s | 0.97x |
| 100 clients, 50/50 | 9,416 ops/s | 10,113 ops/s | 0.93x |

Goldfish leads on write-heavy workloads (1.06–1.26x) and trails slightly on reads (3–7%).

