from msgpack import packb as msgpack_packb, unpackb as msgpack_unpackb
from msgpack.exceptions import ExtraData, FormatError, StackError

from parsec.serde.exceptions import SerdePackingError


def packb(data: dict, exc_cls=SerdePackingError) -> bytes:
    """
    Raises:
        SerdePackingError
    """
    try:
        return msgpack_packb(data, use_bin_type=True)

    except (ExtraData, ValueError, FormatError, StackError) as exc:
        raise exc_cls(f"Invalid msgpack data: {exc}") from exc


def unpackb(raw_data: bytes, exc_cls=SerdePackingError) -> dict:
    """
    Raises:
        SerdePackingError
    """
    try:
        return msgpack_unpackb(raw_data, raw=False)

    except (ExtraData, ValueError, FormatError, StackError) as exc:
        raise exc_cls(f"Invalid msgpack data: {exc}") from exc
