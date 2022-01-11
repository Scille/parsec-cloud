# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from unicodedata import normalize

from parsec.types import UUID4
from parsec.serde import fields


__all__ = ("EntryID", "EntryIDField", "EntryName", "EntryNameField")


class EntryNameInvalidError(ValueError):
    pass


class EntryNameTooLongError(EntryNameInvalidError):
    pass


class EntryID(UUID4):
    __slots__ = ()


EntryIDField = fields.uuid_based_field_factory(EntryID)


class EntryName(str):
    __slots__ = ()

    def __new__(cls, raw: str) -> "EntryName":
        raw = normalize("NFC", raw)
        # Stick to UNIX filesystem philosophy:
        # - no `.` or `..` name
        # - no `/` or null byte in the name
        # - max 255 bytes long name
        if len(raw.encode("utf8")) >= 256:
            raise EntryNameTooLongError(raw)
        if raw == "" or raw == "." or raw == ".." or "/" in raw or "\x00" in raw:
            raise EntryNameInvalidError(raw)
        return super(EntryName, cls).__new__(cls, raw)


EntryNameField = fields.str_based_field_factory(EntryName)
