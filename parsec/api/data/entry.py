# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from typing import TYPE_CHECKING

from parsec.types import UUID4
from parsec.serde import fields
from parsec.api.protocol import StrBased


__all__ = ("EntryID", "EntryIDField", "EntryName", "EntryNameField")


class EntryNameTooLongError(ValueError):
    pass


class EntryID(UUID4):
    __slots__ = ()


_PyEntryID = EntryID
if not TYPE_CHECKING:
    try:
        from libparsec.types import EntryID as _RsEntryID
    except:
        pass
    else:
        EntryID = _RsEntryID


EntryIDField = fields.uuid_based_field_factory(EntryID)


class EntryName(StrBased):
    # Ignore the REGEX
    REGEX = None
    MAX_BYTE_SIZE = 255

    def __init__(self, raw: str):
        try:
            super().__init__(raw)
        except ValueError:
            raise EntryNameTooLongError("Invalid data")
        if raw == "" or raw == "." or raw == ".." or "/" in raw or "\x00" in raw:
            raise ValueError("Invalid data")


_PyEntryName = EntryName
if not TYPE_CHECKING:
    try:
        from libparsec.types import EntryName as _RsEntryName
    except:
        pass
    else:
        EntryName = _RsEntryName


EntryNameField = fields.str_based_field_factory(EntryName)
