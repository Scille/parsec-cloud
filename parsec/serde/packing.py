# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from typing import Any, Mapping
from uuid import UUID
from parsec._parsec import DateTime, LocalDateTime
from struct import pack as struct_pack, unpack as struct_unpack
from msgpack import (  # type: ignore[import]
    packb as msgpack_packb,
    unpackb as msgpack_unpackb,
    Unpacker as msgpack_Unpacker,
    ExtType,
)
from msgpack.exceptions import ExtraData, FormatError, StackError  # type: ignore[import]

from parsec.serde.exceptions import SerdePackingError


MAX_BIN_LEN = 1024 * 1024  # 1 MB


def packb(data: Mapping[str, Any], exc_cls: Any = SerdePackingError) -> bytes:
    """
    Raises:
        SerdePackingError
    """

    def _default(obj: Any) -> Any:
        if isinstance(obj, (DateTime, LocalDateTime)):
            return ExtType(1, struct_pack("!d", obj.timestamp()))
        elif isinstance(obj, UUID):
            return ExtType(2, obj.bytes)

        raise TypeError("Unknown type: %r" % (obj,))

    try:
        return msgpack_packb(data, default=_default, use_bin_type=True)

    except (ExtraData, ValueError, FormatError, StackError) as exc:
        raise exc_cls(f"Invalid msgpack data: {exc}") from exc


def _unpackb_ext_hook(code: int, data: bytes) -> Any:
    if code == 1:
        # Uses UTC as default timezone
        return DateTime.from_timestamp(struct_unpack("!d", data)[0])
    elif code == 2:
        return UUID(bytes=data)

    return ExtType(code, data)


def unpackb(raw_data: bytes, exc_cls: Any = SerdePackingError) -> dict[str, Any]:
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
    def __init__(self, exc_cls: Any = SerdePackingError) -> None:
        self._unpacker = msgpack_Unpacker(
            ext_hook=_unpackb_ext_hook, raw=False, max_bin_len=MAX_BIN_LEN, strict_map_key=False
        )
        self._exc_cls = exc_cls

    def feed(self, data: Any) -> Any:
        self._unpacker.feed(data)

    def __next__(self) -> Any:
        try:
            return next(self._unpacker)
        except (ExtraData, ValueError, FormatError, StackError) as exc:
            raise self._exc_cls(f"Invalid msgpack data: {exc}") from exc

    def __iter__(self) -> Unpacker:
        return self
