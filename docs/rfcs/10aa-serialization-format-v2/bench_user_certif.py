# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import sys
import time
import uuid
import zlib

import msgpack

# Regarding compression level in deflate, we use -1 which is the best.
# Compared to 1 (the fastest) there is no changes in practice for such a small payload.


def bench(obj, serialized, with_compression, iterations=10000):
    items = []
    a = time.monotonic_ns()
    for _ in range(iterations):
        out = msgpack.packb(obj)
        if with_compression:
            out = zlib.compress(out)
        items.append(out)
    b = time.monotonic_ns()
    items.clear()

    c = time.monotonic_ns()
    for _ in range(iterations):
        if with_compression:
            out = zlib.decompress(serialized)
        else:
            out = serialized
        out = msgpack.unpackb(out, strict_map_key=False)
        items.append(out)
    d = time.monotonic_ns()

    return int((b - a) / iterations), int((d - c) / iterations)


default_perfs = None
perfs = []


def pack(name, with_compression, obj):
    global default_perfs
    out = msgpack.packb(obj)
    if with_compression:
        out = zlib.compress(out)
    perf_size = len(out)
    perf_ser_ns, perf_deser_ns = bench(obj, out, with_compression)

    if not default_perfs:
        default_perfs = (perf_size, perf_ser_ns, perf_deser_ns)
        gain_size = ""
        gain_ser = ""
        gain_deser = ""
    else:
        (default_perf_size, default_perf_ser_ns, default_perf_deser_ns) = default_perfs
        gain_size = f" {perf_size / default_perf_size * 100:.2f}%"
        gain_ser = f" {perf_ser_ns / default_perf_ser_ns * 100:.2f}%"
        gain_deser = f" {perf_deser_ns / default_perf_deser_ns * 100:.2f}%"

    msg = f"""{name}{' (deflate)' if with_compression else ' (no deflate)'}:
    size: {perf_size}o{gain_size}
    serialize: {perf_ser_ns/1000:.2f}us{gain_ser}
    deserialize: {perf_deser_ns/1000:.2f}us{gain_deser}
"""
    perfs.append((perf_size, perf_ser_ns, perf_deser_ns, msg))


# default
for with_compression in [True, False]:
    pack(
        "default",
        with_compression,
        {
            "type": "user_certificate",
            "author": "5f37a31b86a64c43964d12e9d6099be8@5f37a31b86a64c43964d12e9d6099be8",
            "timestamp": msgpack.ExtType(code=1, data=b"A\xcc6\xa1\xc0\x00\x00\x00"),
            "user_id": "f76e65bb55a1451da321398e50d05ef1",
            "human_handle": ["userX@example.com", "UserX"],
            "public_key": b"\xe0{\x17\x93\xab5\x88r(\xb2T\x1f\xf4A\xcb<\x88\xfdjS,\x94\x95\xce\xe2\xb1\x9d\xd8\x8c\x94\x15=",
            "is_admin": False,
            "profile": "STANDARD",
        },
    )

    # Indexed keys: 150o
    pack(
        "indexed_keys",
        with_compression,
        {
            1: "user_certificate",
            2: "5f37a31b86a64c43964d12e9d6099be8@5f37a31b86a64c43964d12e9d6099be8",
            3: msgpack.ExtType(code=1, data=b"A\xcc6\xa1\xc0\x00\x00\x00"),
            4: "f76e65bb55a1451da321398e50d05ef1",
            5: ["userX@example.com", "UserX"],
            6: b"\xe0{\x17\x93\xab5\x88r(\xb2T\x1f\xf4A\xcb<\x88\xfdjS,\x94\x95\xce\xe2\xb1\x9d\xd8\x8c\x94\x15=",
            7: False,
            8: "STANDARD",
        },
    )

    # Indexed discriminant
    pack(
        "indexed_discriminant",
        with_compression,
        {
            "type": 1,
            "author": "5f37a31b86a64c43964d12e9d6099be8@5f37a31b86a64c43964d12e9d6099be8",
            "timestamp": msgpack.ExtType(code=1, data=b"A\xcc6\xa1\xc0\x00\x00\x00"),
            "user_id": "f76e65bb55a1451da321398e50d05ef1",
            "human_handle": ["userX@example.com", "UserX"],
            "public_key": b"\xe0{\x17\x93\xab5\x88r(\xb2T\x1f\xf4A\xcb<\x88\xfdjS,\x94\x95\xce\xe2\xb1\x9d\xd8\x8c\x94\x15=",
            "is_admin": False,
            "profile": "STANDARD",
        },
    )

    # Optimized enums
    pack(
        "indexed_enums",
        with_compression,
        {
            "type": "user_certificate",
            "author": "5f37a31b86a64c43964d12e9d6099be8@5f37a31b86a64c43964d12e9d6099be8",
            "timestamp": msgpack.ExtType(code=1, data=b"A\xcc6\xa1\xc0\x00\x00\x00"),
            "user_id": uuid.uuid4().bytes,
            "human_handle": ["userX@example.com", "UserX"],
            "public_key": b"\xe0{\x17\x93\xab5\x88r(\xb2T\x1f\xf4A\xcb<\x88\xfdjS,\x94\x95\xce\xe2\xb1\x9d\xd8\x8c\x94\x15=",
            "is_admin": False,
            "profile": 1,
        },
    )

    # Optimized UserID&Device when UUID
    pack(
        "uuid_when_possible",
        with_compression,
        {
            "type": "user_certificate",
            "author": [uuid.uuid4().bytes, uuid.uuid4().bytes],
            "timestamp": msgpack.ExtType(code=1, data=b"A\xcc6\xa1\xc0\x00\x00\x00"),
            "user_id": uuid.uuid4().bytes,
            "human_handle": ["userX@example.com", "UserX"],
            "public_key": b"\xe0{\x17\x93\xab5\x88r(\xb2T\x1f\xf4A\xcb<\x88\xfdjS,\x94\x95\xce\xe2\xb1\x9d\xd8\x8c\x94\x15=",
            "is_admin": False,
            "profile": 1,
        },
    )

    # Timestamp as int of us
    pack(
        "timestamp_as_int_of_us",
        with_compression,
        {
            "type": "user_certificate",
            "author": "5f37a31b86a64c43964d12e9d6099be8@5f37a31b86a64c43964d12e9d6099be8",
            "timestamp": 1695630038225064,
            "user_id": "f76e65bb55a1451da321398e50d05ef1",
            "human_handle": ["userX@example.com", "UserX"],
            "public_key": b"\xe0{\x17\x93\xab5\x88r(\xb2T\x1f\xf4A\xcb<\x88\xfdjS,\x94\x95\xce\xe2\xb1\x9d\xd8\x8c\x94\x15=",
            "is_admin": False,
            "profile": "STANDARD",
        },
    )

    # Full patate
    pack(
        "full_patate",
        with_compression,
        {
            1: 1,
            2: [uuid.uuid4().bytes, uuid.uuid4().bytes],
            3: 1695630038225064,
            4: uuid.uuid4().bytes,
            5: ["userX@example.com", "UserX"],
            6: b"\xe0{\x17\x93\xab5\x88r(\xb2T\x1f\xf4A\xcb<\x88\xfdjS,\x94\x95\xce\xe2\xb1\x9d\xd8\x8c\x94\x15=",
            7: False,
            8: 1,
        },
    )

try:
    order_by = sys.argv[1]
except IndexError:
    order_by = "size"

if order_by == "size":
    perfs = sorted([(perf_size, msg) for (perf_size, _, _, msg) in perfs])
elif order_by == "ser":
    perfs = sorted([(perf_ser, msg) for (_, perf_ser, _, msg) in perfs])
elif order_by == "deser":
    perfs = sorted([(perf_deser, msg) for (_, _, perf_deser, msg) in perfs])
else:
    raise SystemExit("Bad filter, allowed are: size/ser/deser")

for _, msg in sorted(perfs):
    print(msg)
