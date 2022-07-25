# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from typing import Mapping
from uuid import UUID
from libparsec.types import DateTime, LocalDateTime
from struct import pack as struct_pack, unpack as struct_unpack
from msgpack import (
    packb as msgpack_packb,
    unpackb as msgpack_unpackb,
    Unpacker as msgpack_Unpacker,
    ExtType,
)
from msgpack.exceptions import ExtraData, FormatError, StackError

from parsec.serde.exceptions import SerdePackingError


MAX_BIN_LEN = 1024 * 1024  # 1 MB


def packb(data: Mapping, exc_cls=SerdePackingError) -> bytes:
    """
    Raises:
        SerdePackingError
    """

    def _default(obj):
        # TODO: we should be only testing against DateTime, but
        # asyncpg returns datetime
        if isinstance(obj, DateTime) or isinstance(obj, LocalDateTime):
            return ExtType(1, struct_pack("!d", obj.timestamp()))
        elif isinstance(obj, UUID):
            return ExtType(2, obj.bytes)

        raise TypeError("Unknown type: %r" % (obj,))

    try:
        return msgpack_packb(data, default=_default, use_bin_type=True)

    except (ExtraData, ValueError, FormatError, StackError) as exc:
        raise exc_cls(f"Invalid msgpack data: {exc}") from exc


def _unpackb_ext_hook(code, data):
    if code == 1:
        # Uses UTC as default timezone
        return DateTime.from_timestamp(struct_unpack("!d", data)[0])
    elif code == 2:
        return UUID(bytes=data)

    return ExtType(code, data)


def unpackb(raw_data: bytes, exc_cls=SerdePackingError) -> dict:
    """
    Raises:
        SerdePackingError
    """

    try:
        ret = msgpack_unpackb(
            raw_data,
            ext_hook=_unpackb_ext_hook,
            raw=False,
            max_bin_len=MAX_BIN_LEN,
            strict_map_key=False,
        )

    except (ExtraData, ValueError, FormatError, StackError) as exc:
        raise exc_cls(f"Invalid msgpack data: {exc}") from exc

    # MessagePack can return any type (int, string, list etc.) as root value
    if not isinstance(ret, dict):
        raise exc_cls(f"Invalid msgpack data: root must be a map")

    return ret


class Unpacker:
    def __init__(self, exc_cls=SerdePackingError):
        self._unpacker = msgpack_Unpacker(
            ext_hook=_unpackb_ext_hook, raw=False, max_bin_len=MAX_BIN_LEN, strict_map_key=False
        )
        self._exc_cls = exc_cls

    def feed(self, data):
        self._unpacker.feed(data)

    def __next__(self):
        try:
            return next(self._unpacker)
        except (ExtraData, ValueError, FormatError, StackError) as exc:
            raise self._exc_cls(f"Invalid msgpack data: {exc}") from exc

    def __iter__(self):
        return self
