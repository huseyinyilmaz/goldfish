import argparse
import socket
import time
import threading
from pymemcache.client.base import Client

TOTAL_OPS = 10000
VALUE_SIZE = 4096
VALUE_DATA = b"x" * VALUE_SIZE
CLIENTS = 100
COMMANDS_PER_CLIENT = TOTAL_OPS // CLIENTS


def pre_populate(host, port, n):
    client = Client((host, port), connect_timeout=5, timeout=10)
    for i in range(n):
        client.set(f"key{i}", VALUE_DATA, expire=0, noreply=False)
    client.quit()


def benchmark_single(host, port, name, write_pct):
    client = Client((host, port), connect_timeout=5, timeout=30)

    start = time.perf_counter()
    for i in range(TOTAL_OPS):
        if i % 100 < write_pct:
            client.set(f"stb_key{i}", VALUE_DATA, expire=0, noreply=False)
        else:
            client.get(f"stb_key{i}")
    elapsed = time.perf_counter() - start

    client.quit()
    throughput = TOTAL_OPS / elapsed if elapsed > 0 else 0
    return {"name": name, "ops": TOTAL_OPS, "elapsed": elapsed, "throughput": throughput}


def benchmark_multi(host, port, name, write_pct):
    pre_populate(host, port, TOTAL_OPS)
    results = [None] * CLIENTS

    def run_client(idx):
        c = Client((host, port), connect_timeout=5, timeout=30)
        for j in range(COMMANDS_PER_CLIENT):
            key = f"mt_key{idx}_{j}"
            if j % 100 < write_pct:
                c.set(key, VALUE_DATA, expire=0, noreply=False)
            else:
                c.get(key)
        c.quit()
        results[idx] = True

    start = time.perf_counter()
    threads = []
    for i in range(CLIENTS):
        t = threading.Thread(target=run_client, args=(i,))
        threads.append(t)
        t.start()
    for t in threads:
        t.join()
    elapsed = time.perf_counter() - start
    throughput = TOTAL_OPS / elapsed if elapsed > 0 else 0
    return {"name": name, "ops": TOTAL_OPS, "elapsed": elapsed, "throughput": throughput}


def pipeline_bug_check(host, port):
    NUM = 32
    SPLIT = 65536

    batch = b"".join(
        f"set key{i} 0 10 {VALUE_SIZE}\r\n".encode() + VALUE_DATA + b"\r\n"
        for i in range(NUM)
    )

    chunk1, chunk2 = batch[:SPLIT], batch[SPLIT:]

    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    sock.connect((host, port))
    sock.settimeout(10)

    sock.sendall(chunk1)
    time.sleep(0.05)
    sock.sendall(chunk2)

    response = b""
    try:
        while True:
            data = sock.recv(65536)
            if not data:
                break
            response += data
    except socket.timeout:
        pass

    sock.close()
    stored = response.count(b"STORED")
    ok = stored == NUM
    status = "PASS" if ok else "FAIL"
    print(f"  pipeline_chunking_{NUM}p:  {status:>4s}  ({stored}/{NUM} STORED)")
    return ok


def run_all_benchmarks(host, port, label):
    print(f"\n{'='*60}")
    print(f"  {label} ({host}:{port})")
    print(f"{'='*60}")
    results = []

    r = benchmark_single(host, port, "st_write_heavy_5p_read_95p_write", 95)
    results.append(r)
    print(f"  {r['name']:40s}  {r['throughput']:>10.0f} ops/sec  ({r['elapsed']:.2f}s)")

    r = benchmark_single(host, port, "st_read_heavy_95p_read_5p_write", 5)
    results.append(r)
    print(f"  {r['name']:40s}  {r['throughput']:>10.0f} ops/sec  ({r['elapsed']:.2f}s)")

    r = benchmark_single(host, port, "st_balanced_50p_read_50p_write", 50)
    results.append(r)
    print(f"  {r['name']:40s}  {r['throughput']:>10.0f} ops/sec  ({r['elapsed']:.2f}s)")

    r = benchmark_multi(host, port, "mt_write_heavy_5p_read_95p_write", 95)
    results.append(r)
    print(f"  {r['name']:40s}  {r['throughput']:>10.0f} ops/sec  ({r['elapsed']:.2f}s)")

    r = benchmark_multi(host, port, "mt_read_heavy_95p_read_5p_write", 5)
    results.append(r)
    print(f"  {r['name']:40s}  {r['throughput']:>10.0f} ops/sec  ({r['elapsed']:.2f}s)")

    r = benchmark_multi(host, port, "mt_balanced_50p_read_50p_write", 50)
    results.append(r)
    print(f"  {r['name']:40s}  {r['throughput']:>10.0f} ops/sec  ({r['elapsed']:.2f}s)")

    pipeline_ok = pipeline_bug_check(host, port)
    return results, pipeline_ok


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Benchmark a memcached-compatible server."
    )
    parser.add_argument(
        "--host",
        default="127.0.0.1",
        help="Server hostname or IP address (default: 127.0.0.1)",
    )
    parser.add_argument(
        "--port",
        type=int,
        default=11211,
        help="Server port (default: 11211)",
    )
    args = parser.parse_args()

    label = f"{args.host}:{args.port}"
    results, pipeline_ok = run_all_benchmarks(args.host, args.port, label)

    print(f"\n{'='*60}")
    print("  SUMMARY")
    print(f"{'='*60}")
    print(f"  {'Benchmark':40s} {'Throughput':>10s}")
    print(f"  {'-'*50}")
    for r in results:
        print(f"  {r['name']:40s} {r['throughput']:>8.0f} ops/sec")
