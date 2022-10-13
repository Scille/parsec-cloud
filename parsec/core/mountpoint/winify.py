# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import re

from parsec.api.data import EntryName

# https://docs.microsoft.com/en-us/windows/win32/fileio/naming-a-file
# tl;dr: https://twitter.com/foone/status/1058676834940776450
_WIN32_RES_CHARS = tuple(chr(x) for x in range(1, 32)) + (
    "<",
    ">",
    ":",
    '"',
    "\\",
    "|",
    "?",
    "*",
)  # Ignore `\`
_WIN32_RES_NAMES = (
    "CON",
    "PRN",
    "AUX",
    "NUL",
    "COM1",
    "COM2",
    "COM3",
    "COM4",
    "COM5",
    "COM6",
    "COM7",
    "COM8",
    "COM9",
    "LPT1",
    "LPT2",
    "LPT3",
    "LPT4",
    "LPT5",
    "LPT6",
    "LPT7",
    "LPT8",
    "LPT9",
)


def winify_entry_name(name: EntryName) -> str:
    name = name.str
    prefix, *suffixes = name.split(".", 1)
    if prefix in _WIN32_RES_NAMES:
        full_suffix = f".{'.'.join(suffixes)}" if suffixes else ""
        name = f"{prefix[:-1]}~{ord(prefix[-1]):02x}{full_suffix}"

    else:
        for reserved in _WIN32_RES_CHARS:
            name = name.replace(reserved, f"~{ord(reserved):02x}")

    if name[-1] in (".", " "):
        name = f"{name[:-1]}~{ord(name[-1]):02x}"

    return name


def unwinify_entry_name(name: str) -> EntryName:
    # Given / is not allowed, no need to check if path already contains it
    if "~" not in name:
        return EntryName(name)

    else:
        *to_convert_parts, last_part = re.split(r"(~[0-9A-Fa-f]{2})", name)
        converted_parts = []
        is_escape = False
        for part in to_convert_parts:
            if is_escape:
                converted_chr = chr(int(part[1:], 16))
                if converted_chr in ("/", "\x00"):
                    raise ValueError("Invalid escaped value")
                converted_parts.append(converted_chr)

            else:
                converted_parts.append(part)

            is_escape = not is_escape

        converted_parts.append(last_part)
        return EntryName("".join(converted_parts))
