# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
r"""
This script helps to fix old/legacy payloads in libparsec serialization tests.

Serialization tests are usually written in the following structure:
- a hard-coded raw value (the payload)
- a hard-coded expected value corresponding to the payload
- an assert comparing the value loaded from raw to the expected value

Any change impacting the serialization format or the expected value will make
the payload obsolete. This scripts helps regenerate it for the failing test.

1. Add a println with following syntax "***expected: <dumped expected value>"

Example:
```rust
    let raw = hex!(
        //...
    );
    let expected =
        // ...
    ;
    println!("***expected: {:?}", expected.dump().unwrap());
    // ...
    p_assert_eq!(data, expected);

2. Run the failing test(s) and check that the print is displayed

3. Run the failing test(s) redirecting stderr to stdout and pipe its output
   int this script

Example:
```shell
$ cargo nextest run -p libparsec_protocol 2>&1 | python ./misc/test_expected_payload_cooker.py

================== libparsec_protocol::serialization anonymous_cmds::latest::organization_bootstrap_rep_timestamp_out_of_ballpark ==================

    // Generated from Parsec v3.0.0-b.11+dev
    "85a6737461747573b974696d657374616d705f6f75745f6f665f62616c6c7061726bbc"
    "62616c6c7061726b5f636c69656e745f6561726c795f6f6666736574cb4072c0000000"
    "0000bb62616c6c7061726b5f636c69656e745f6c6174655f6f6666736574cb40740000"
    "00000000b0636c69656e745f74696d657374616d70d70100035d162fa2e400b0736572"
    "7665725f74696d657374616d70d70100035d162fa2e400"
```

So for each failing tests, the script will print:
- the name of the test
- a comment line with the libparsec version used to generate the payload
- a comment line with the content of the payload (if possible, see TODO comment below)
- the corresponding raw value (aka payload)

You should then:
1. Replace the test's payload with the one printed by this script.
2. Replace the test's comment line containing the libparsec version
3. Replace the test's comment line containing the expected value (if printed by this script)
4. Remove the println
5. Re-run test(s) which should not fail anymore \o/
"""

import binascii
import datetime
import io
import os
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
    import nacl.public  # type: ignore
    import nacl.secret  # type: ignore
except ImportError:
    raise SystemExit("pynacl not installed. Run `pip install pynacl`")

# File containing the current version of libparsec
LIBPARSEC_VERSION_FILE = os.path.join(os.path.dirname(__file__), "..", "libparsec", "version")

# Expected failure header: `FAIL [   0.004s] libparsec_types certif::tests::serde_user_certificate_redacted`
FAILING_TEST_HEADER_PATTERN = re.compile(r"\W*FAIL \[[ 0-9.]+s\] ([ \w::]+)")

# Expected print message: `***expected: <dump>`
##     println!("***expected: {:?}", expected.dump().unwrap());
TAG_PATTERN = re.compile(r"\W*\*\*\*expected: \[([ 0-9,]*)\]")

SECRET_KEY_CANDIDATES = [
    binascii.unhexlify("b1b52e16c1b46ab133c8bf576e82d26c887f1e9deae1af80043a258c36fcabf3"),
]
PRIVATE_KEY_CANDIDATES = [
    # Alice fixture privkey
    binascii.unhexlify("74e860967fd90d063ebd64fb1ba6824c4c010099dd37508b7f2875a5db2ef8c9"),
    # Bob fixture privkey
    binascii.unhexlify("16767ec446f2611f971c36f19c2dc11614d853475ac395d6c1d70ba46d07dd49"),
]


def decode_expected_raw(raw: bytes) -> dict[str, object]:
    def attempt_decrypt(raw: bytes) -> bytes | None:
        for key in SECRET_KEY_CANDIDATES:
            key = nacl.secret.SecretBox(key)
            try:
                return key.decrypt(raw)
            except nacl.exceptions.CryptoError:
                continue

        for key in PRIVATE_KEY_CANDIDATES:
            box = nacl.public.SealedBox(nacl.public.PrivateKey(key))
            try:
                return box.decrypt(raw)
            except nacl.exceptions.CryptoError:
                continue

    def attempt_msgpack_deserialization(raw: bytes) -> dict[str, object] | None:
        try:
            # `strict_map_key` is needed because shamir_recovery_brief_certificate
            # uses `DeviceID` (i.e. ExtType) as dict key.
            return msgpack.unpackb(raw, strict_map_key=False)
        except ValueError:
            return None

    def attempt_format_0x00_deserialization(raw: bytes) -> dict[str, object] | None:
        # First byte is the version
        if raw[0] != 0x00:
            return None

        try:
            io_input = io.BytesIO(raw[1:])
            io_output = io.BytesIO()
            decompressor = zstandard.ZstdDecompressor()
            decompressor.copy_stream(io_input, io_output)
            decompressed = io_output.getvalue()
        except zstandard.ZstdError:
            return None

        return attempt_msgpack_deserialization(decompressed)

    # 1st attempt: consider the data is only msgpack-encoded (what is used for the protocols)
    deserialized = attempt_msgpack_deserialization(raw)
    if deserialized is not None:
        return deserialized

    # 2nd attempt: consider the data is not signed nor encrypted
    deserialized = attempt_format_0x00_deserialization(raw)
    if deserialized is not None:
        return deserialized

    # 3rd attempt: consider the data is only signed
    raw_without_signature = raw[64:]
    deserialized = attempt_format_0x00_deserialization(raw_without_signature)
    if deserialized is not None:
        return deserialized

    # Last attempt: consider the data is encrypted...
    decrypted = attempt_decrypt(raw)
    assert decrypted is not None
    # ...and signed ?
    decrypted_without_signature = decrypted[64:]
    deserialized = attempt_format_0x00_deserialization(decrypted_without_signature)
    if deserialized is None:
        # ...or not signed ?
        deserialized = attempt_format_0x00_deserialization(decrypted)

    if deserialized is None:
        # For historical reason, `LocalDevice` & `LocalDeviceFile` don't use
        # format 0x00 encoding, but instead serialize as msgpack without headers.
        deserialized = attempt_msgpack_deserialization(decrypted)
        assert deserialized is None or deserialized.get("type") == (
            "local_device",
            "keyring",
            "password",
            "recovery",
            "smartcard",
        ), "Only `LocalDevice`&`LocalDeviceFile` are allowed not to use format 0x00"

    assert deserialized is not None, "Cannot deserialize this payload..."
    return deserialized


def multilines_indent(lines: list[str], newline_indent: str) -> str:
    sub_items = lines[1:-1]
    if sub_items:
        return f"\n{newline_indent}  ".join(lines[:-1]) + f"\n{newline_indent}{lines[-1]}"
    else:
        return f"{lines[0]}\n{newline_indent}{lines[-1]}"


def cook_msgpack_type(value, newline_indent: str) -> str:
    if isinstance(value, bytes):
        return f"0x{value.hex()}"

    if isinstance(value, msgpack.ExtType):
        match value.code:
            # DateTime
            case 1:
                ts = struct.unpack(">q", value.data)[0]
                dt = datetime.datetime.fromtimestamp(ts / 1000000).isoformat() + "Z"
                return f"ext(1, {ts}) i.e. {dt}"
            # UUID
            case 2:
                return f"ext(2, 0x{value.data.hex()})"

            case _:
                pass

    if isinstance(value, dict):
        lines = ["{"]
        indent = f"{newline_indent}  "
        for k, v in value.items():
            if not isinstance(k, str):
                k = cook_msgpack_type(k, newline_indent=indent)
            lines.append(f"{k}: {cook_msgpack_type(v, newline_indent=indent)},")
        lines.append("}")
        mono_line = " ".join(lines)
        if len(mono_line) <= 80:
            return mono_line
        else:
            return multilines_indent(lines, newline_indent=newline_indent)

    if isinstance(value, list):
        lines = ["["]
        indent = f"{newline_indent}  "
        for v in value:
            lines.append(f"{cook_msgpack_type(v, newline_indent=indent)},")
        lines.append("]")
        mono_line = " ".join(lines)
        if len(mono_line) <= 80:
            return mono_line
        else:
            return multilines_indent(lines, newline_indent=newline_indent)

    return repr(value)


def parse_lines(lines):
    current_test = None

    for line in lines:
        match_failing_test = FAILING_TEST_HEADER_PATTERN.match(line)
        if match_failing_test:
            current_test = match_failing_test.group(1)

        match_print_expected = TAG_PATTERN.match(line)
        if not match_print_expected:
            continue

        expected_raw = bytes(
            bytearray([int(byte.strip()) for byte in match_print_expected.group(1).split(",")])
        )

        expected_decoded = decode_expected_raw(expected_raw)

        output_lines = []

        with open(LIBPARSEC_VERSION_FILE) as f:
            libparsec_version = f.readline()

        output_lines.append(f"    // Generated from Parsec {libparsec_version}")

        output_lines.append("    // Content:")
        for k, v in expected_decoded.items():
            indent = "    //   "
            output_lines.append(f"{indent}{k}: {cook_msgpack_type(v, newline_indent=indent)}")

        output_lines.append("    let raw: &[u8] = hex!(")

        indent = " " * 8
        # The raw value (payload) to be used in the test
        output_lines.extend(f'{indent}"{part}"' for part in textwrap.wrap(expected_raw.hex()))

        output_lines.append("    ).as_ref();")

        print()
        print(f"================== {current_test} ==================")
        print()
        print("\n".join(output_lines))


if __name__ == "__main__":
    parse_lines(sys.stdin.readlines())
