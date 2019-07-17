# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from datetime import datetime
from struct import pack as struct_pack
from struct import unpack as struct_unpack
from uuid import UUID

from msgpack import ExtType
from msgpack import packb as msgpack_packb
from msgpack import unpackb as msgpack_unpackb
from msgpack.exceptions import ExtraData, FormatError, StackError
from pendulum import Pendulum

from parsec.serde.exceptions import SerdePackingError


def packb(data: dict, exc_cls=SerdePackingError) -> bytes:
    """
    Raises:
        SerdePackingError
    """

    def _default(obj):
        # TODO: we should be only testing against pendulum.Pendulum, but
        # asyncpg returns datetime
        if isinstance(obj, datetime):
            return ExtType(1, struct_pack("!d", obj.timestamp()))
        elif isinstance(obj, UUID):
            return ExtType(2, obj.bytes)

        raise TypeError("Unknown type: %r" % (obj,))

    try:
        return msgpack_packb(data, default=_default, use_bin_type=True)

    except (ExtraData, ValueError, FormatError, StackError) as exc:
        raise exc_cls(f"Invalid msgpack data: {exc}") from exc


def unpackb(raw_data: bytes, exc_cls=SerdePackingError) -> dict:
    """
    Raises:
        SerdePackingError
    """

    def _ext_hook(code, data):
        if code == 1:
            return Pendulum.utcfromtimestamp(struct_unpack("!d", data)[0])
        elif code == 2:
            return UUID(bytes=data)

        return ExtType(code, data)

    try:
        return msgpack_unpackb(raw_data, ext_hook=_ext_hook, raw=False)

    except (ExtraData, ValueError, FormatError, StackError) as exc:
        raise exc_cls(f"Invalid msgpack data: {exc}") from exc
