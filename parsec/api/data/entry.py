# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from typing import TYPE_CHECKING

from parsec.serde import fields
from parsec.api.protocol import StrBased
from parsec._parsec import EntryID

__all__ = ("EntryID", "EntryIDField", "EntryName", "EntryNameField")


class EntryNameTooLongError(ValueError):
    pass


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
