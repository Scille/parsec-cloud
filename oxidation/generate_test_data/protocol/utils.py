# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

import struct
import math
import zlib
import msgpack
from binascii import unhexlify

from parsec._version import __version__
from parsec.crypto import SigningKey, PrivateKey, SecretKey, VerifyKey
from parsec.api.data import EntryID
from parsec.api.protocol import HumanHandle, DeviceID, DeviceLabel, UserProfile
from parsec.core.types import LocalDevice, BackendOrganizationAddr


def generate_ALICE_local_device():
    return LocalDevice(
        organization_addr=BackendOrganizationAddr.from_url(
            "parsec://alice_dev1.example.com:9999/CoolOrg?no_ssl=true&rvk=XYUXM4ZM5SGKSTXNZ4FK7VATZUKZGY7A7LOJ42CXFR32DYL5TO6Qssss"
        ),
        device_id=DeviceID("alice@dev1"),
        device_label=DeviceLabel("My dev1 machine"),
        human_handle=HumanHandle("alice@example.com", "Alicey McAliceFace"),
        signing_key=SigningKey(
            unhexlify("d544f66ece9c85d5b80275db9124b5f04bb038081622bed139c1e789c5217400")
        ),
        private_key=PrivateKey(
            unhexlify("74e860967fd90d063ebd64fb1ba6824c4c010099dd37508b7f2875a5db2ef8c9")
        ),
        profile=UserProfile.ADMIN,
        user_manifest_id=EntryID.from_str("a4031e8bcdd84df8ae12bd3d05e6e20f"),
        user_manifest_key=SecretKey(
            unhexlify("26bf35a98c1e54e90215e154af92a1af2d1142cdd0dba25b990426b0b30b0f9a")
        ),
        local_symkey=SecretKey(
            unhexlify("125a78618995e2e0f9a19bc8617083c809c03deb5457d5b82df5bcaec9966cd4")
        ),
    )


def generate_BOB_local_device():
    return LocalDevice(
        organization_addr=BackendOrganizationAddr.from_url(
            "parsec://bob_dev1.example.com:9999/CoolOrg?no_ssl=true&rvk=XYUXM4ZM5SGKSTXNZ4FK7VATZUKZGY7A7LOJ42CXFR32DYL5TO6Qssss"
        ),
        device_id=DeviceID("bob@dev1"),
        device_label=DeviceLabel("My dev1 machine"),
        human_handle=HumanHandle("bob@example.com", "Boby McBobFace"),
        signing_key=SigningKey(
            unhexlify("85f47472a2c0f30f01b769617db248f3ec8d96a490602a9262f95e9e43432b30")
        ),
        private_key=PrivateKey(
            unhexlify("16767ec446f2611f971c36f19c2dc11614d853475ac395d6c1d70ba46d07dd49")
        ),
        profile=UserProfile.STANDARD,
        user_manifest_id=EntryID.from_str("71568d41afcb4e2380b3d164ace4fb85"),
        user_manifest_key=SecretKey(
            unhexlify("65de53d2c6cd965aa53a1ba5cc7e54b331419e6103466121996fa99a97197a48")
        ),
        local_symkey=SecretKey(
            unhexlify("93f25b18491016f20b10dcf4eb7986716d914653d6ab4e778701c13435e6bdf0")
        ),
    )


ALICE = generate_ALICE_local_device()
BOB = generate_BOB_local_device()
KEY = SecretKey(unhexlify("b1b52e16c1b46ab133c8bf576e82d26c887f1e9deae1af80043a258c36fcabf3"))


KEYS_PRIORITY = (
    "type",
    "author",
    "timestamp",
    "name",
    "id",
    "version",
    "created",
    "updated",
)


def custom_order_iter(d):
    for key in KEYS_PRIORITY:
        try:
            value = d[key]
        except KeyError:
            continue
        yield key, value
    for key in sorted(list(d.keys() - set(KEYS_PRIORITY))):
        yield key, d[key]


def _format_msgpack(content, max_width):
    output = ""
    for key, value in custom_order_iter(content):
        repr_value = _format_item(value, max_width=max_width)
        output += f"{key}: {repr_value}\n"

    return output


INDENT_SIZE = 2


def _format_item(value, max_width):
    if isinstance(value, list):
        short_repr = "["
        short_repr += ", ".join(_format_item(item, max_width=math.inf) for item in value)
        short_repr += "]"
        if len(short_repr) <= max_width:
            return short_repr

        output = "[\n"
        for item in value:
            output += _indent(_format_item(item, max_width=max_width - INDENT_SIZE))
            output += "\n"
        output += "]"
        return output

    elif isinstance(value, dict):
        short_repr = "{"
        short_repr += ", ".join(
            f"{k}:" + _format_item(v, max_width=math.inf) for k, v in custom_order_iter(value)
        )
        short_repr += "}"
        if len(short_repr) <= max_width:
            return short_repr

        output = "{\n"
        for item_key, item_value in custom_order_iter(value):
            item_repr = " " * INDENT_SIZE
            item_repr += f"{item_key}: "
            item_repr += _indent(
                _format_item(item_value, max_width=max_width - INDENT_SIZE),
                indent_first_line=False,
            )
            output += item_repr
            output += "\n"
        output += "}"
        return output

    else:
        return _format_scalar(value, max_width=max_width)


def _format_scalar(scalar, max_width):
    if isinstance(scalar, msgpack.ExtType):
        if scalar.code == 1:
            # datetime serialized as float
            data_as_float = struct.unpack("!d", scalar.data)[0]
            return f"ext(1, {data_as_float})"
        else:
            assert scalar.code == 2  # uuid serialized as bytes
            return f'ext(2, hex!("{scalar.data.hex()}"))'
    elif isinstance(scalar, bytes):
        raw_repr = f'hex!("{scalar.hex()}")'
        if len(raw_repr) > max_width:
            raw_repr = "hex!(\n"
            raw_repr += _indent(_format_text(scalar.hex(), max_width=max_width - INDENT_SIZE))
            raw_repr += "\n)"
        return raw_repr
    elif isinstance(scalar, str):
        # Don't try to do _format_text here given string are most likely short
        # enough to fit on one line anyway, and in the case of URL splitting
        # the long line is not very readable
        return f'"{scalar}"'
    elif isinstance(scalar, bool):
        return "true" if scalar else "false"
    elif scalar is None:
        return "None"
    else:
        assert isinstance(scalar, (int, float)), f"bad type {repr(scalar)}"
        return str(scalar)


def _indent(txt, indent_first_line=True):
    lines = txt.split("\n")
    output = []
    if not indent_first_line:
        output.append(lines[0])
        lines = lines[1:]
    output += [" " * INDENT_SIZE + line for line in lines]
    return "\n".join(output)


def _format_text(text, max_width):
    text = "".join(text.splitlines())
    if max_width == math.inf:
        return text
    line_template = '"{line}"'
    content_length_per_line = max_width - 2

    output = []
    line = text[:content_length_per_line]
    text = text[content_length_per_line:]
    while line:
        output.append(line_template.format(line=line))
        line = text[:content_length_per_line]
        text = text[content_length_per_line:]

    return "\n".join(output)


def display(name, raw, get_content_pipeline, max_width=89):
    content = raw
    for item in get_content_pipeline:
        if isinstance(item, SecretKey):
            content = item.decrypt(content)
        elif isinstance(item, PrivateKey):
            content = item.decrypt_from_self(content)
        elif isinstance(item, VerifyKey):
            content = item.verify(content)
        elif item == "zip":
            content = zlib.decompress(content)
        else:
            raise RuntimeError(f"Unknown pipeline element {item!r}")
    content = msgpack.unpackb(content, strict_map_key=False)

    output = ""
    output += "\n"
    output += (
        f"================================ {name} ==============================================\n"
    )
    output += "\n"
    output += f"// Generated from Rust implementation (Parsec {__version__})\n"
    output += "// Content:\n"
    for line in _indent(_format_msgpack(content, max_width=max_width - 3 - INDENT_SIZE)).split(
        "\n"
    ):
        output += f"// {line}\n"
    output += _format_scalar(raw, max_width=max_width - INDENT_SIZE - 9)
    output += "\n"

    print(output)

    return content
