import argparse
import socket
import sys
import time


PASS = "\u2705"
FAIL = "\u274c"


class CheckResult:
    def __init__(self):
        self.passed = 0
        self.failed = 0

    def ok(self, label):
        self.passed += 1
        print(f"  {PASS} {label}")

    def fail(self, label, detail=""):
        self.failed += 1
        msg = f"  {FAIL} {label}"
        if detail:
            msg += f"  ({detail})"
        print(msg)


def connect(host, port, timeout=5):
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    sock.settimeout(timeout)
    sock.connect((host, port))
    return sock


def send_raw(sock, data):
    sock.sendall(data)


def recv_line(sock):
    data = b""
    while True:
        chunk = sock.recv(1)
        if not chunk:
            break
        data += chunk
        if data.endswith(b"\r\n"):
            break
    return data


def recv_until_end(sock):
    data = b""
    while True:
        chunk = sock.recv(65536)
        if not chunk:
            break
        data += chunk
        if data.endswith(b"END\r\n"):
            break
    return data


def recv_all(sock):
    data = b""
    try:
        while True:
            chunk = sock.recv(65536)
            if not chunk:
                break
            data += chunk
    except (socket.timeout, ConnectionResetError, OSError):
        pass
    return data


def check_connection(r):
    label = "TCP connection succeeds"
    try:
        sock = connect(r.host, r.port)
        sock.close()
        r.ok(label)
    except Exception as e:
        r.fail(label, str(e))


def check_empty_line(r):
    label = "Empty line yields ERROR"
    sock = connect(r.host, r.port)
    send_raw(sock, b"\r\n")
    data = recv_line(sock)
    sock.close()
    if data == b"ERROR\r\n" or data == b"CLIENT_ERROR bad command line format\r\n":
        r.ok(label)
    else:
        r.fail(label, f"expected ERROR or CLIENT_ERROR, got {data!r}")


def check_unknown_command(r):
    label = "Unknown command yields ERROR"
    sock = connect(r.host, r.port)
    send_raw(sock, b"foobar\r\n")
    data = recv_line(sock)
    sock.close()
    if data == b"ERROR\r\n":
        r.ok(label)
    else:
        r.fail(label, f"expected ERROR, got {data!r}")


def check_case_sensitivity(r):
    label = "Commands are case-sensitive (SET -> ERROR)"
    sock = connect(r.host, r.port)
    send_raw(sock, b"SET key 0 0 5\r\nhello\r\n")
    data = b""
    data += recv_line(sock)
    data += recv_line(sock)
    sock.close()
    if data == b"ERROR\r\nERROR\r\n":
        r.ok(label)
    else:
        r.fail(label, f"expected ERROR\\r\\nERROR\\r\\n, got {data!r}")


def check_set_basic(r):
    label = "SET basic store and retrieve"
    sock = connect(r.host, r.port)
    send_raw(sock, b"set k 0 0 5\r\nhello\r\n")
    data = recv_line(sock)
    send_raw(sock, b"get k\r\n")
    get_data = recv_until_end(sock)
    sock.close()
    if data == b"STORED\r\n" and get_data == b"VALUE k 0 5\r\nhello\r\nEND\r\n":
        r.ok(label)
    else:
        r.fail(label, f"set={data!r} get={get_data!r}")


def check_set_overwrite(r):
    label = "SET overwrites existing key"
    sock = connect(r.host, r.port)
    send_raw(sock, b"set k 0 0 5\r\nhello\r\n")
    recv_line(sock)
    send_raw(sock, b"set k 0 0 5\r\nworld\r\n")
    data = recv_line(sock)
    send_raw(sock, b"get k\r\n")
    get_data = recv_until_end(sock)
    sock.close()
    if data == b"STORED\r\n" and b"world" in get_data:
        r.ok(label)
    else:
        r.fail(label, f"set={data!r} get={get_data!r}")


def check_set_flags(r):
    label = "SET preserves flags"
    sock = connect(r.host, r.port)
    send_raw(sock, b"set k 42 0 5\r\nhello\r\n")
    recv_line(sock)
    send_raw(sock, b"get k\r\n")
    data = recv_until_end(sock)
    sock.close()
    if b"VALUE k 42 5\r\n" in data:
        r.ok(label)
    else:
        r.fail(label, f"got {data!r}")


def check_set_exptime_zero(r):
    label = "SET exptime=0 never expires"
    sock = connect(r.host, r.port)
    send_raw(sock, b"set k 0 0 5\r\nhello\r\n")
    recv_line(sock)
    send_raw(sock, b"get k\r\n")
    data = recv_until_end(sock)
    sock.close()
    if b"hello" in data:
        r.ok(label)
    else:
        r.fail(label, f"key not found: {data!r}")


def check_set_noreply(r):
    label = "SET noreply yields no response"
    sock = connect(r.host, r.port)
    send_raw(sock, b"set knr 0 0 5 noreply\r\nhello\r\n")
    sock.settimeout(1)
    extra = recv_all(sock)
    sock.close()
    if extra == b"":
        r.ok(label)
    else:
        r.fail(label, f"got unexpected data {extra!r}")


def check_set_empty_value(r):
    label = "SET empty value (bytes=0)"
    sock = connect(r.host, r.port)
    send_raw(sock, b"set k 0 0 0\r\n\r\n")
    data = recv_line(sock)
    send_raw(sock, b"get k\r\n")
    get_data = recv_until_end(sock)
    sock.close()
    if data == b"STORED\r\n" and get_data == b"VALUE k 0 0\r\n\r\nEND\r\n":
        r.ok(label)
    else:
        r.fail(label, f"set={data!r} get={get_data!r}")


def check_set_binary(r):
    label = "SET binary data (including \\r\\n)"
    binary = bytes([0, 1, 2, 127, 128, 255, 13, 10])
    sock = connect(r.host, r.port)
    send_raw(sock, b"set k 0 0 8\r\n" + binary + b"\r\n")
    data = recv_line(sock)
    send_raw(sock, b"get k\r\n")
    get_data = recv_until_end(sock)
    sock.close()
    if data == b"STORED\r\n" and get_data.startswith(b"VALUE k 0 8\r\n"):
        rest = get_data[len(b"VALUE k 0 8\r\n"):-len(b"\r\nEND\r\n")]
        if rest == binary:
            r.ok(label)
        else:
            r.fail(label, f"binary mismatch: {rest!r} != {binary!r}")
    else:
        r.fail(label, f"set={data!r} get_snippet={get_data[:60]!r}")


def check_set_key_length_250(r):
    label = "SET key length 250 chars accepted"
    key = b"k" * 250
    sock = connect(r.host, r.port)
    cmd_line = b"set " + key + b" 0 0 3\r\nval\r\n"
    send_raw(sock, cmd_line)
    data = recv_line(sock)
    sock.close()
    if data == b"STORED\r\n":
        r.ok(label)
    else:
        r.fail(label, f"expected STORED, got {data!r}")


def check_set_missing_args(r):
    label = "SET missing args yields ERROR"
    sock = connect(r.host, r.port)
    send_raw(sock, b"set key\r\n")
    data = recv_line(sock)
    sock.close()
    if b"ERROR" in data:
        r.ok(label)
    else:
        r.fail(label, f"expected ERROR, got {data!r}")


def check_set_bad_flags(r):
    label = "SET bad flags yields ERROR"
    sock = connect(r.host, r.port)
    send_raw(sock, b"set key abc 0 5\r\nhello\r\n")
    data = recv_line(sock)
    sock.close()
    if b"ERROR" in data or b"CLIENT_ERROR" in data:
        r.ok(label)
    else:
        r.fail(label, f"expected ERROR or CLIENT_ERROR, got {data!r}")


def check_set_bad_bytes(r):
    label = "SET bad bytes yields ERROR"
    sock = connect(r.host, r.port)
    send_raw(sock, b"set key 0 0 abc\r\nhello\r\n")
    data = recv_line(sock)
    data += recv_line(sock)
    sock.close()
    if b"ERROR" in data:
        r.ok(label)
    else:
        r.fail(label, f"expected ERROR, got {data!r}")


def check_get_found(r):
    label = "GET existing key returns value"
    sock = connect(r.host, r.port)
    send_raw(sock, b"set k 0 0 5\r\nhello\r\n")
    recv_line(sock)
    send_raw(sock, b"get k\r\n")
    data = recv_until_end(sock)
    sock.close()
    if data == b"VALUE k 0 5\r\nhello\r\nEND\r\n":
        r.ok(label)
    else:
        r.fail(label, f"expected VALUE, got {data!r}")


def check_get_not_found(r):
    label = "GET non-existent key returns END"
    sock = connect(r.host, r.port)
    send_raw(sock, b"get nonexistent\r\n")
    data = recv_until_end(sock)
    sock.close()
    if data == b"END\r\n":
        r.ok(label)
    else:
        r.fail(label, f"expected END, got {data!r}")


def check_get_multi_key(r):
    label = "GET multiple keys"
    sock = connect(r.host, r.port)
    send_raw(sock, b"set a 0 0 1\r\nx\r\nset b 0 0 1\r\ny\r\n")
    recv_line(sock)
    recv_line(sock)
    send_raw(sock, b"get a b\r\n")
    data = recv_until_end(sock)
    sock.close()
    if b"VALUE a 0 1\r\nx\r\n" in data and b"VALUE b 0 1\r\ny\r\n" in data:
        r.ok(label)
    else:
        r.fail(label, f"got {data!r}")


def check_get_multi_some_missing(r):
    label = "GET multiple keys (some missing)"
    sock = connect(r.host, r.port)
    send_raw(sock, b"set a 0 0 1\r\nx\r\n")
    recv_line(sock)
    send_raw(sock, b"get a missingkey_b\r\n")
    data = recv_until_end(sock)
    sock.close()
    if b"VALUE a" in data and b"VALUE missingkey_b" not in data:
        r.ok(label)
    else:
        r.fail(label, f"got {data!r}")


def check_get_binary_value(r):
    label = "GET returns binary value correctly"
    binary = bytes([0x00, 0x01, 0xFF])
    sock = connect(r.host, r.port)
    send_raw(sock, b"set k 0 0 3\r\n" + binary + b"\r\n")
    recv_line(sock)
    send_raw(sock, b"get k\r\n")
    data = recv_until_end(sock)
    sock.close()
    if data.startswith(b"VALUE k 0 3\r\n") and data.endswith(b"\r\nEND\r\n"):
        inner = data[len(b"VALUE k 0 3\r\n"):-len(b"\r\nEND\r\n")]
        if inner == binary:
            r.ok(label)
        else:
            r.fail(label, f"binary mismatch: {inner!r} != {binary!r}")
    else:
        r.fail(label, f"unexpected format: {data!r}")


def check_get_returns_correct_bytes(r):
    label = "GET returns correct bytes count"
    sock = connect(r.host, r.port)
    send_raw(sock, b"set k 0 0 5\r\nhello\r\n")
    recv_line(sock)
    send_raw(sock, b"get k\r\n")
    data = recv_until_end(sock)
    sock.close()
    if b"VALUE k 0 5\r\n" in data:
        r.ok(label)
    else:
        r.fail(label, f"got {data!r}")


def check_add_stored(r):
    label = "ADD new key returns STORED"
    sock = connect(r.host, r.port)
    send_raw(sock, b"add newkey 0 0 5\r\nhello\r\n")
    data = recv_line(sock)
    sock.close()
    if data == b"STORED\r\n":
        r.ok(label)
    else:
        r.fail(label, f"expected STORED, got {data!r}")


def check_add_not_stored(r):
    label = "ADD existing key returns NOT_STORED"
    sock = connect(r.host, r.port)
    send_raw(sock, b"set k 0 0 5\r\nhello\r\n")
    recv_line(sock)
    send_raw(sock, b"add k 0 0 5\r\nworld\r\n")
    data = recv_line(sock)
    send_raw(sock, b"get k\r\n")
    get_data = recv_until_end(sock)
    sock.close()
    if data == b"NOT_STORED\r\n" and b"hello" in get_data:
        r.ok(label)
    else:
        r.fail(label, f"add={data!r} get={get_data!r}")


def check_add_noreply(r):
    label = "ADD noreply yields no response"
    sock = connect(r.host, r.port)
    send_raw(sock, b"add anr 0 0 5 noreply\r\nhello\r\n")
    sock.settimeout(1)
    extra = recv_all(sock)
    sock.close()
    if extra == b"":
        r.ok(label)
    else:
        r.fail(label, f"got unexpected data {extra!r}")


def check_replace_stored(r):
    label = "REPLACE existing key returns STORED"
    sock = connect(r.host, r.port)
    send_raw(sock, b"set k 0 0 5\r\nhello\r\n")
    recv_line(sock)
    send_raw(sock, b"replace k 0 0 5\r\nworld\r\n")
    data = recv_line(sock)
    send_raw(sock, b"get k\r\n")
    get_data = recv_until_end(sock)
    sock.close()
    if data == b"STORED\r\n" and b"world" in get_data:
        r.ok(label)
    else:
        r.fail(label, f"replace={data!r} get={get_data!r}")


def check_replace_not_stored(r):
    label = "REPLACE missing key returns NOT_STORED"
    sock = connect(r.host, r.port)
    send_raw(sock, b"replace nonexistent 0 0 5\r\nhello\r\n")
    data = recv_line(sock)
    sock.close()
    if data == b"NOT_STORED\r\n":
        r.ok(label)
    else:
        r.fail(label, f"expected NOT_STORED, got {data!r}")


def check_append_stored(r):
    label = "APPEND to existing key returns STORED"
    sock = connect(r.host, r.port)
    send_raw(sock, b"set k 0 0 5\r\nhello\r\n")
    recv_line(sock)
    send_raw(sock, b"append k 0 0 5\r\nworld\r\n")
    data = recv_line(sock)
    send_raw(sock, b"get k\r\n")
    get_data = recv_until_end(sock)
    sock.close()
    if data == b"STORED\r\n" and b"helloworld" in get_data:
        r.ok(label)
    else:
        r.fail(label, f"append={data!r} get={get_data!r}")


def check_append_not_stored(r):
    label = "APPEND missing key returns NOT_STORED"
    sock = connect(r.host, r.port)
    send_raw(sock, b"append nonexistent 0 0 5\r\nworld\r\n")
    data = recv_line(sock)
    sock.close()
    if data == b"NOT_STORED\r\n":
        r.ok(label)
    else:
        r.fail(label, f"expected NOT_STORED, got {data!r}")


def check_prepend_stored(r):
    label = "PREPEND to existing key returns STORED"
    sock = connect(r.host, r.port)
    send_raw(sock, b"set k 0 0 5\r\nhello\r\n")
    recv_line(sock)
    send_raw(sock, b"prepend k 0 0 5\r\nworld\r\n")
    data = recv_line(sock)
    send_raw(sock, b"get k\r\n")
    get_data = recv_until_end(sock)
    sock.close()
    if data == b"STORED\r\n" and b"worldhello" in get_data:
        r.ok(label)
    else:
        r.fail(label, f"prepend={data!r} get={get_data!r}")


def check_prepend_not_stored(r):
    label = "PREPEND missing key returns NOT_STORED"
    sock = connect(r.host, r.port)
    send_raw(sock, b"prepend nonexistent 0 0 5\r\nworld\r\n")
    data = recv_line(sock)
    sock.close()
    if data == b"NOT_STORED\r\n":
        r.ok(label)
    else:
        r.fail(label, f"expected NOT_STORED, got {data!r}")


def check_delete_stored(r):
    label = "DELETE existing key returns DELETED"
    sock = connect(r.host, r.port)
    send_raw(sock, b"set k 0 0 5\r\nhello\r\n")
    recv_line(sock)
    send_raw(sock, b"delete k\r\n")
    data = recv_line(sock)
    sock.close()
    if data == b"DELETED\r\n":
        r.ok(label)
    else:
        r.fail(label, f"expected DELETED, got {data!r}")


def check_delete_not_found(r):
    label = "DELETE missing key returns NOT_FOUND"
    sock = connect(r.host, r.port)
    send_raw(sock, b"delete nonexistent\r\n")
    data = recv_line(sock)
    sock.close()
    if data == b"NOT_FOUND\r\n":
        r.ok(label)
    else:
        r.fail(label, f"expected NOT_FOUND, got {data!r}")


def check_delete_noreply(r):
    label = "DELETE noreply yields no response"
    sock = connect(r.host, r.port)
    send_raw(sock, b"set k 0 0 5\r\nhello\r\n")
    recv_line(sock)
    send_raw(sock, b"delete k noreply\r\n")
    sock.settimeout(1)
    extra = recv_all(sock)
    sock.close()
    if extra == b"":
        r.ok(label)
    else:
        r.fail(label, f"got unexpected data {extra!r}")


def check_incr_basic(r):
    label = "INCR on numeric value returns new value"
    sock = connect(r.host, r.port)
    send_raw(sock, b"set counter 0 0 1\r\n5\r\n")
    recv_line(sock)
    send_raw(sock, b"incr counter 3\r\n")
    data = recv_line(sock)
    sock.close()
    if data == b"8\r\n":
        r.ok(label)
    else:
        r.fail(label, f"expected '8', got {data!r}")


def check_incr_not_found(r):
    label = "INCR missing key returns NOT_FOUND"
    sock = connect(r.host, r.port)
    send_raw(sock, b"incr nonexistent 1\r\n")
    data = recv_line(sock)
    sock.close()
    if data == b"NOT_FOUND\r\n":
        r.ok(label)
    else:
        r.fail(label, f"expected NOT_FOUND, got {data!r}")


def check_incr_non_numeric(r):
    label = "INCR on non-numeric value returns CLIENT_ERROR"
    sock = connect(r.host, r.port)
    send_raw(sock, b"set k 0 0 5\r\nhello\r\n")
    recv_line(sock)
    send_raw(sock, b"incr k 1\r\n")
    data = recv_line(sock)
    sock.close()
    if b"CLIENT_ERROR" in data:
        r.ok(label)
    else:
        r.fail(label, f"expected CLIENT_ERROR, got {data!r}")


def check_incr_noreply(r):
    label = "INCR noreply yields no response"
    sock = connect(r.host, r.port)
    send_raw(sock, b"set c 0 0 1\r\n0\r\n")
    recv_line(sock)
    send_raw(sock, b"incr c 5 noreply\r\n")
    sock.settimeout(1)
    extra = recv_all(sock)
    sock.close()
    if extra == b"":
        r.ok(label)
    else:
        r.fail(label, f"got unexpected data {extra!r}")


def check_decr_basic(r):
    label = "DECR on numeric value returns new value"
    sock = connect(r.host, r.port)
    send_raw(sock, b"set counter 0 0 1\r\n5\r\n")
    recv_line(sock)
    send_raw(sock, b"decr counter 2\r\n")
    data = recv_line(sock)
    sock.close()
    if data == b"3\r\n":
        r.ok(label)
    else:
        r.fail(label, f"expected '3', got {data!r}")


def check_decr_clamp(r):
    label = "DECR clamps to 0 (never negative)"
    sock = connect(r.host, r.port)
    send_raw(sock, b"set counter 0 0 1\r\n5\r\n")
    recv_line(sock)
    send_raw(sock, b"decr counter 100\r\n")
    data = recv_line(sock)
    sock.close()
    if data == b"0\r\n":
        r.ok(label)
    else:
        r.fail(label, f"expected '0', got {data!r}")


def check_decr_not_found(r):
    label = "DECR missing key returns NOT_FOUND"
    sock = connect(r.host, r.port)
    send_raw(sock, b"decr nonexistent 1\r\n")
    data = recv_line(sock)
    sock.close()
    if data == b"NOT_FOUND\r\n":
        r.ok(label)
    else:
        r.fail(label, f"expected NOT_FOUND, got {data!r}")


def check_cas_stored(r):
    label = "CAS with correct token returns STORED"
    sock = connect(r.host, r.port)
    send_raw(sock, b"set k 0 0 5\r\nhello\r\n")
    recv_line(sock)
    send_raw(sock, b"gets k\r\n")
    gets_data = recv_until_end(sock)
    cas_token = gets_data.split(b" ")[4].split(b"\r\n")[0].decode()
    cas_cmd = f"cas k 0 0 5 {cas_token}\r\nworld\r\n".encode()
    send_raw(sock, cas_cmd)
    data = recv_line(sock)
    send_raw(sock, b"get k\r\n")
    get_data = recv_until_end(sock)
    sock.close()
    if data == b"STORED\r\n" and b"world" in get_data:
        r.ok(label)
    else:
        r.fail(label, f"cas={data!r} get={get_data!r}")


def check_cas_exists(r):
    label = "CAS with wrong token returns EXISTS"
    sock = connect(r.host, r.port)
    send_raw(sock, b"set k 0 0 5\r\nhello\r\n")
    recv_line(sock)
    send_raw(sock, b"cas k 0 0 5 9999999999999999999\r\nworld\r\n")
    data = recv_line(sock)
    sock.close()
    if data == b"EXISTS\r\n":
        r.ok(label)
    else:
        r.fail(label, f"expected EXISTS, got {data!r}")


def check_cas_not_found(r):
    label = "CAS on missing key returns NOT_FOUND"
    sock = connect(r.host, r.port)
    send_raw(sock, b"cas nonexistent_cas_key 0 0 5 1\r\nworld\r\n")
    data = recv_line(sock)
    sock.close()
    if data == b"NOT_FOUND\r\n":
        r.ok(label)
    else:
        r.fail(label, f"expected NOT_FOUND, got {data!r}")


def check_cas_noreply(r):
    label = "CAS noreply yields no response"
    sock = connect(r.host, r.port)
    send_raw(sock, b"set k 0 0 5\r\nhello\r\n")
    recv_line(sock)
    send_raw(sock, b"gets k\r\n")
    gets_data = recv_until_end(sock)
    cas_token = gets_data.split(b" ")[4].split(b"\r\n")[0].decode()
    cas_cmd = f"cas k 0 0 5 {cas_token} noreply\r\nworld\r\n".encode()
    send_raw(sock, cas_cmd)
    sock.settimeout(1)
    extra = recv_all(sock)
    sock.close()
    if extra == b"":
        r.ok(label)
    else:
        r.fail(label, f"got unexpected data {extra!r}")


def check_cas_token_rotates(r):
    label = "CAS token changes after update"
    sock = connect(r.host, r.port)
    send_raw(sock, b"set k 0 0 5\r\nhello\r\n")
    recv_line(sock)
    send_raw(sock, b"gets k\r\n")
    data1 = recv_until_end(sock)
    tok1 = data1.split(b" ")[4].split(b"\r\n")[0]
    cas_cmd = b"cas k 0 0 5 " + tok1 + b"\r\nworld\r\n"
    send_raw(sock, cas_cmd)
    recv_line(sock)
    send_raw(sock, b"gets k\r\n")
    data2 = recv_until_end(sock)
    tok2 = data2.split(b" ")[4].split(b"\r\n")[0]
    sock.close()
    if tok1 != tok2:
        r.ok(label)
    else:
        r.fail(label, f"token unchanged: {tok1!r}")


def check_gets_basic(r):
    label = "GETS returns CAS token"
    sock = connect(r.host, r.port)
    send_raw(sock, b"set k 0 0 5\r\nhello\r\n")
    recv_line(sock)
    send_raw(sock, b"gets k\r\n")
    data = recv_until_end(sock)
    sock.close()
    parts = data.split(b" ")
    if len(parts) >= 5 and parts[0] == b"VALUE":
        cas_str = parts[4].split(b"\r\n")[0]
        try:
            int(cas_str)
            r.ok(label)
        except ValueError:
            r.fail(label, f"invalid CAS token: {cas_str!r}")
    else:
        r.fail(label, f"unexpected format: {data!r}")


def check_gets_multi(r):
    label = "GETS multiple keys"
    sock = connect(r.host, r.port)
    send_raw(sock, b"set a 0 0 1\r\nx\r\nset b 0 0 1\r\ny\r\n")
    recv_line(sock)
    recv_line(sock)
    send_raw(sock, b"gets a b\r\n")
    data = recv_until_end(sock)
    sock.close()
    if b"VALUE a" in data and b"VALUE b" in data:
        r.ok(label)
    else:
        r.fail(label, f"got {data!r}")


def check_touch_touched(r):
    label = "TOUCH existing key returns TOUCHED"
    sock = connect(r.host, r.port)
    send_raw(sock, b"set k 0 0 5\r\nhello\r\n")
    recv_line(sock)
    send_raw(sock, b"touch k 100\r\n")
    data = recv_line(sock)
    sock.close()
    if data == b"TOUCHED\r\n":
        r.ok(label)
    else:
        r.fail(label, f"expected TOUCHED, got {data!r}")


def check_touch_not_found(r):
    label = "TOUCH missing key returns NOT_FOUND"
    sock = connect(r.host, r.port)
    send_raw(sock, b"touch nonexistent 100\r\n")
    data = recv_line(sock)
    sock.close()
    if data == b"NOT_FOUND\r\n":
        r.ok(label)
    else:
        r.fail(label, f"expected NOT_FOUND, got {data!r}")


def check_touch_noreply(r):
    label = "TOUCH noreply yields no response"
    sock = connect(r.host, r.port)
    send_raw(sock, b"set k 0 0 5\r\nhello\r\n")
    recv_line(sock)
    send_raw(sock, b"touch k 100 noreply\r\n")
    sock.settimeout(1)
    extra = recv_all(sock)
    sock.close()
    if extra == b"":
        r.ok(label)
    else:
        r.fail(label, f"got unexpected data {extra!r}")


def check_touch_preserves_value(r):
    label = "TOUCH preserves flags and value"
    sock = connect(r.host, r.port)
    send_raw(sock, b"set k 42 0 5\r\nhello\r\n")
    recv_line(sock)
    send_raw(sock, b"touch k 100\r\n")
    recv_line(sock)
    send_raw(sock, b"get k\r\n")
    data = recv_until_end(sock)
    sock.close()
    if b"VALUE k 42 5\r\n" in data and b"hello" in data:
        r.ok(label)
    else:
        r.fail(label, f"got {data!r}")


def check_gat_basic(r):
    label = "GAT returns value and updates expiry"
    sock = connect(r.host, r.port)
    send_raw(sock, b"set k 0 0 5\r\nhello\r\n")
    recv_line(sock)
    send_raw(sock, b"gat 1 k\r\n")
    data = recv_until_end(sock)
    sock.close()
    if data == b"VALUE k 0 5\r\nhello\r\nEND\r\n":
        r.ok(label)
    else:
        r.fail(label, f"got {data!r}")


def check_gat_miss(r):
    label = "GAT on missing key returns END"
    sock = connect(r.host, r.port)
    send_raw(sock, b"gat 100 nonexistent\r\n")
    data = recv_until_end(sock)
    sock.close()
    if data == b"END\r\n":
        r.ok(label)
    else:
        r.fail(label, f"expected END, got {data!r}")


def check_gat_updates_expiry(r):
    label = "GAT expiry update causes key to expire"
    sock = connect(r.host, r.port)
    send_raw(sock, b"set k 0 0 5\r\nhello\r\n")
    recv_line(sock)
    send_raw(sock, b"gat 1 k\r\n")
    recv_until_end(sock)
    time.sleep(1.5)
    send_raw(sock, b"get k\r\n")
    data = recv_until_end(sock)
    sock.close()
    if data == b"END\r\n":
        r.ok(label)
    else:
        r.fail(label, f"key still present after expiry: {data!r}")


def check_gats_basic(r):
    label = "GATS returns value with CAS token"
    sock = connect(r.host, r.port)
    send_raw(sock, b"set k 0 0 5\r\nhello\r\n")
    recv_line(sock)
    send_raw(sock, b"gats 100 k\r\n")
    data = recv_until_end(sock)
    sock.close()
    parts = data.split(b" ")
    if len(parts) >= 5 and parts[0] == b"VALUE":
        cas_str = parts[4].split(b"\r\n")[0]
        try:
            int(cas_str)
            r.ok(label)
        except ValueError:
            r.fail(label, f"invalid CAS token: {cas_str!r}")
    else:
        r.fail(label, f"unexpected format: {data!r}")


def check_flush_all_ok(r):
    label = "FLUSH_ALL returns OK"
    sock = connect(r.host, r.port)
    send_raw(sock, b"flush_all\r\n")
    data = recv_line(sock)
    sock.close()
    if data == b"OK\r\n":
        r.ok(label)
    else:
        r.fail(label, f"expected OK, got {data!r}")


def check_flush_all_clears(r):
    label = "FLUSH_ALL clears all keys"
    sock = connect(r.host, r.port)
    send_raw(sock, b"set k 0 0 5\r\nhello\r\n")
    recv_line(sock)
    send_raw(sock, b"flush_all\r\n")
    recv_line(sock)
    send_raw(sock, b"get k\r\n")
    data = recv_until_end(sock)
    sock.close()
    if data == b"END\r\n":
        r.ok(label)
    else:
        r.fail(label, f"key still present: {data!r}")


def check_flush_all_delay(r):
    label = "FLUSH_ALL with delay actually delays"
    sock = connect(r.host, r.port)
    send_raw(sock, b"set k 0 0 5\r\nhello\r\n")
    recv_line(sock)
    send_raw(sock, b"flush_all 3\r\n")
    data = recv_line(sock)
    send_raw(sock, b"get k\r\n")
    get_before = recv_until_end(sock)
    time.sleep(4)
    send_raw(sock, b"get k\r\n")
    get_after = recv_until_end(sock)
    sock.close()
    if data == b"OK\r\n" and b"hello" in get_before and get_after == b"END\r\n":
        r.ok(label)
    else:
        detail = []
        if data != b"OK\r\n":
            detail.append(f"flush={data!r}")
        if b"hello" not in get_before:
            detail.append("cleared before delay (expected key to persist)")
        if get_after != b"END\r\n":
            detail.append(f"key still present after delay: {get_after!r}")
        r.fail(label, "; ".join(detail) if detail else "unexpected behavior")


def check_flush_all_noreply(r):
    label = "FLUSH_ALL noreply yields no response"
    sock = connect(r.host, r.port)
    send_raw(sock, b"flush_all noreply\r\n")
    sock.settimeout(1)
    extra = recv_all(sock)
    sock.close()
    if extra == b"":
        r.ok(label)
    else:
        r.fail(label, f"got unexpected data {extra!r}")


def check_stats_required_fields(r):
    label = "STATS returns required fields"
    sock = connect(r.host, r.port)
    send_raw(sock, b"stats\r\n")
    data = recv_until_end(sock)
    sock.close()
    required = [
        b"STAT pid ",
        b"STAT uptime ",
        b"STAT time ",
        b"STAT version ",
        b"STAT pointer_size ",
        b"STAT curr_items ",
        b"STAT total_items ",
        b"STAT bytes ",
        b"STAT cmd_get ",
        b"STAT cmd_set ",
        b"STAT get_hits ",
        b"STAT get_misses ",
        b"STAT evictions ",
        b"STAT limit_maxbytes ",
    ]
    missing = [f.decode() for f in required if f not in data]
    if not missing:
        r.ok(label)
    else:
        r.fail(label, f"missing fields: {missing}")


def check_stats_subcommand(r):
    label = "STATS subcommand returns END"
    sock = connect(r.host, r.port)
    send_raw(sock, b"stats items\r\n")
    data = recv_until_end(sock)
    sock.close()
    if data == b"END\r\n":
        r.ok(label)
    else:
        r.fail(label, f"expected END, got {data!r}")


def check_stats_counters(r):
    label = "STATS counters track commands accurately"
    sock = connect(r.host, r.port)
    send_raw(sock, b"set k 0 0 5\r\nhello\r\n")
    recv_line(sock)
    send_raw(sock, b"get k\r\n")
    recv_until_end(sock)
    send_raw(sock, b"get nonexistent\r\n")
    recv_until_end(sock)
    send_raw(sock, b"stats\r\n")
    data = recv_until_end(sock)
    sock.close()
    checks = {
        "curr_items": 1,
    }
    increases = {
        "cmd_set": 1,
        "cmd_get": 2,
        "get_hits": 1,
        "get_misses": 1,
        "total_items": 1,
    }
    failures = []
    for stat, expected in checks.items():
        marker = f"STAT {stat} ".encode()
        if marker in data:
            rest = data.split(marker)[1].split(b"\r\n")[0]
            try:
                val = int(rest)
                if val != expected:
                    failures.append(f"{stat}={val} (expected {expected})")
            except ValueError:
                failures.append(f"{stat} not parseable: {rest!r}")
        else:
            failures.append(f"{stat} missing")
    for stat, minimum_increase in increases.items():
        marker = f"STAT {stat} ".encode()
        if marker in data:
            rest = data.split(marker)[1].split(b"\r\n")[0]
            try:
                val = int(rest)
                if val < minimum_increase:
                    failures.append(f"{stat}={val} (expected >= {minimum_increase})")
            except ValueError:
                failures.append(f"{stat} not parseable: {rest!r}")
        else:
            failures.append(f"{stat} missing")
    if not failures:
        r.ok(label)
    else:
        r.fail(label, "; ".join(failures))


def check_version(r):
    label = "VERSION returns VERSION <string>"
    sock = connect(r.host, r.port)
    send_raw(sock, b"version\r\n")
    data = recv_line(sock)
    sock.close()
    if data.startswith(b"VERSION ") and data.endswith(b"\r\n"):
        r.ok(label)
    else:
        r.fail(label, f"unexpected: {data!r}")


def check_version_case_sensitive(r):
    label = "VERSION is case-sensitive (VERSION -> ERROR)"
    sock = connect(r.host, r.port)
    send_raw(sock, b"VERSION\r\n")
    data = recv_line(sock)
    sock.close()
    if data == b"ERROR\r\n":
        r.ok(label)
    else:
        r.fail(label, f"expected ERROR, got {data!r}")


def check_quit(r):
    label = "QUIT closes connection"
    sock = connect(r.host, r.port)
    send_raw(sock, b"quit\r\n")
    data = recv_all(sock)
    sock.close()
    if data == b"":
        r.ok(label)
    else:
        r.fail(label, f"got unexpected data: {data!r}")


def check_quit_pipeline(r):
    label = "QUIT in pipeline stops processing (no 2nd version)"
    sock = connect(r.host, r.port)
    send_raw(sock, b"version\r\nquit\r\nversion\r\n")
    data = recv_all(sock)
    sock.close()
    lines = data.split(b"\r\n")
    non_empty = [l for l in lines if l]
    if len(non_empty) <= 1 and (data == b"" or data.startswith(b"VERSION ")):
        r.ok(label)
    else:
        r.fail(label, f"expected at most one VERSION, got {data!r}")


def check_quit_trailing_garbage(r):
    label = "QUIT with extra args handled gracefully"
    sock = connect(r.host, r.port)
    send_raw(sock, b"quit now\r\n")
    data = recv_all(sock)
    sock.close()
    if data == b"" or data == b"ERROR\r\n" or data == b"CLIENT_ERROR bad command line format\r\n":
        r.ok(label)
    else:
        r.fail(label, f"unexpected: {data!r}")


def check_pipeline_basic(r):
    label = "Pipeline multiple commands"
    sock = connect(r.host, r.port)
    cmds = b"version\r\nset a 0 0 1\r\nx\r\nget a\r\nversion\r\n"
    send_raw(sock, cmds)
    data = b""
    data += recv_line(sock)  # VERSION
    data += recv_line(sock)  # STORED
    data += recv_line(sock)  # VALUE a 0 1
    data += recv_line(sock)  # <data>
    data += recv_line(sock)  # END
    data += recv_line(sock)  # VERSION
    sock.close()
    if b"VERSION " in data and b"STORED" in data and b"VALUE a" in data:
        r.ok(label)
    else:
        r.fail(label, f"unexpected: {data!r}")


def check_pipeline_noreply(r):
    label = "Pipeline with noreply and reply commands"
    sock = connect(r.host, r.port)
    cmds = b"set a 0 0 1 noreply\r\nx\r\nset b 0 0 1\r\ny\r\n"
    send_raw(sock, cmds)
    data = recv_line(sock)
    sock.close()
    if data == b"STORED\r\n":
        r.ok(label)
    else:
        r.fail(label, f"expected STORED, got {data!r}")


def run_all(host, port):
    r = CheckResult()
    r.host = host
    r.port = port

    label = f"{host}:{port}"
    print(f"\nGoldfish Protocol Check \u2014 {label}")
    print("\u2550" * 50)
    print()

    groups = [
        ("Connection & Basics", [
            check_connection,
            check_empty_line,
            check_unknown_command,
            check_case_sensitivity,
        ]),
        ("SET", [
            check_set_basic,
            check_set_overwrite,
            check_set_flags,
            check_set_exptime_zero,
            check_set_noreply,
            check_set_empty_value,
            check_set_binary,
            check_set_key_length_250,
            check_set_missing_args,
            check_set_bad_flags,
            check_set_bad_bytes,
        ]),
        ("GET", [
            check_get_found,
            check_get_not_found,
            check_get_multi_key,
            check_get_multi_some_missing,
            check_get_binary_value,
            check_get_returns_correct_bytes,
        ]),
        ("ADD", [
            check_add_stored,
            check_add_not_stored,
            check_add_noreply,
        ]),
        ("REPLACE", [
            check_replace_stored,
            check_replace_not_stored,
        ]),
        ("APPEND", [
            check_append_stored,
            check_append_not_stored,
        ]),
        ("PREPEND", [
            check_prepend_stored,
            check_prepend_not_stored,
        ]),
        ("DELETE", [
            check_delete_stored,
            check_delete_not_found,
            check_delete_noreply,
        ]),
        ("INCR", [
            check_incr_basic,
            check_incr_not_found,
            check_incr_non_numeric,
            check_incr_noreply,
        ]),
        ("DECR", [
            check_decr_basic,
            check_decr_clamp,
            check_decr_not_found,
        ]),
        ("CAS", [
            check_cas_stored,
            check_cas_exists,
            check_cas_not_found,
            check_cas_noreply,
            check_cas_token_rotates,
        ]),
        ("GETS", [
            check_gets_basic,
            check_gets_multi,
        ]),
        ("TOUCH", [
            check_touch_touched,
            check_touch_not_found,
            check_touch_noreply,
            check_touch_preserves_value,
        ]),
        ("GAT / GATS", [
            check_gat_basic,
            check_gat_miss,
            check_gat_updates_expiry,
            check_gats_basic,
        ]),
        ("FLUSH_ALL", [
            check_flush_all_ok,
            check_flush_all_clears,
            check_flush_all_delay,
            check_flush_all_noreply,
        ]),
        ("STATS", [
            check_stats_required_fields,
            check_stats_subcommand,
            check_stats_counters,
        ]),
        ("VERSION", [
            check_version,
            check_version_case_sensitive,
        ]),
        ("QUIT", [
            check_quit,
            check_quit_pipeline,
            check_quit_trailing_garbage,
        ]),
        ("Pipeline", [
            check_pipeline_basic,
            check_pipeline_noreply,
        ]),
    ]

    for group_name, checks in groups:
        print(f"\u2500\u2500 {group_name} \u2500\u2500")
        for check_fn in checks:
            try:
                check_fn(r)
            except Exception as e:
                r.fail(check_fn.__name__, str(e))
        print()

    print("\u2550" * 50)
    total = r.passed + r.failed
    if r.failed == 0:
        print(f"\n  {PASS} All {r.passed}/{total} checks passed!")
    else:
        print(f"\n  {PASS} {r.passed} passed, {FAIL} {r.failed} failed "
              f"({total} total)")
    print()

    return r.failed == 0


def main():
    parser = argparse.ArgumentParser(
        description="Check a memcached-compatible server for protocol compliance."
    )
    parser.add_argument(
        "--host", default="127.0.0.1",
        help="Server hostname (default: 127.0.0.1)",
    )
    parser.add_argument(
        "--port", type=int, default=11211,
        help="Server port (default: 11211)",
    )
    args = parser.parse_args()

    ok = run_all(args.host, args.port)
    sys.exit(0 if ok else 1)


if __name__ == "__main__":
    main()
