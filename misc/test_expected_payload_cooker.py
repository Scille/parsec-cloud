# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import binascii
import datetime
import re
import struct
import sys
import textwrap

try:
    import msgpack  # type: ignore
except ImportError:
    raise SystemExit("msgpack not installed. Run `pip install msgpack`")
try:
    import zstandard  # type: ignore
except ImportError:
    raise SystemExit("zstandard not installed. Run `pip install zstandard`")
try:
    import nacl.exceptions  # type: ignore
    import nacl.secret  # type: ignore
except ImportError:
    raise SystemExit("pynacl not installed. Run `pip install pynacl`")


TAG = "***expected:"

# Expected failure header: `FAIL [   0.004s] libparsec_types certif::tests::serde_user_certificate_redacted`
FAILING_TEST_HEADER_PATTERN = re.compile(r"\W*FAIL \[[ 0-9.]+s\] ([ \w::]+)")
TAG_PATTERN = re.compile(r"\W*\*\*\*expected: \[([ 0-9,]*)\]")


KEY_CANDIDATES = [
    binascii.unhexlify("b1b52e16c1b46ab133c8bf576e82d26c887f1e9deae1af80043a258c36fcabf3")
]


def decode_expected_raw(raw: bytes) -> dict[str, object]:
    def attempt_decrypt(raw: bytes) -> bytes | None:
        for key in KEY_CANDIDATES:
            key = nacl.secret.SecretBox(key)
            try:
                return key.decrypt(raw)
            except nacl.exceptions.CryptoError:
                continue

    def attempt_decompression(raw: bytes) -> bytes | None:
        try:
            return zstandard.decompress(raw)
        except ValueError:
            return None

    def attempt_deserialization(raw: bytes) -> dict[str, object] | None:
        if raw[0] == 0xFF:
            try:
                return msgpack.unpackb(raw[1:])
            except ValueError:
                pass

        # First byte is the version
        if raw[0] != 0x00:
            return None

        try:
            decompressed = zstandard.decompress(raw[1:])
        except ValueError:
            return None

        try:
            return msgpack.unpackb(decompressed)
        except ValueError:
            return None

    # First attempt: consider the data is not signed nor encrypted
    deserialized = attempt_deserialization(raw)
    if deserialized is not None:
        return deserialized

    # Second attempt: consider the data is only signed
    raw_without_signature = raw[64:]
    deserialized = attempt_deserialization(raw_without_signature)
    if deserialized is not None:
        return deserialized

    # Last attempt: consider the data is signed and encrypted
    decrypted = attempt_decrypt(raw)
    assert decrypted is not None
    decrypted_without_signature = decrypted[64:]
    decompressed = attempt_decompression(decrypted_without_signature)
    assert decompressed is not None
    deserialized = attempt_deserialization(decompressed)
    assert deserialized is not None
    return deserialized


def cook_msgpack_type(value):
    if isinstance(value, bytes):
        return value.hex()

    if isinstance(value, msgpack.ExtType):
        match value.code:
            # DateTime
            case 1:
                ts = struct.unpack(">q", value.data)[0]
                dt = datetime.datetime.fromtimestamp(ts / 1000000).isoformat() + "Z"
                return f"ext(1, {ts}) i.e. {dt}"
            # UUID
            case 2:
                return f"ext(2, {value.data.hex()})"

            case _:
                pass

    return value


current_test = None

lines = sys.stdin.readlines()

for line in lines:
    m = FAILING_TEST_HEADER_PATTERN.match(line)
    if m:
        current_test = m.group(1)

    m = TAG_PATTERN.match(line)
    if not m:
        continue

    expected_raw = bytes(bytearray([int(c.strip()) for c in m.group(1).split(",")]))
    expected_decoded = decode_expected_raw(expected_raw)

    txt = []
    txt.append("    // Generated from Parsec v3.0.0-b.11+dev")
    txt.append("    // Content:")

    for k, v in expected_decoded.items():
        txt.append(f"    //   {k}: {cook_msgpack_type(v)}")

    txt.append("    let data = Bytes::from_static(&hex!(")
    for part in textwrap.wrap(expected_raw.hex()):
        txt.append(f'        "{part}",')
    txt.append("    ));")

    print()
    print(
        f"===================================== {current_test} ========================================"
    )
    print()
    print("\n".join(txt))
