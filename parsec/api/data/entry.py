# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from unicodedata import normalize
from uuid import UUID, uuid4
from typing import Union, Type, TypeVar

from parsec.serde import fields


__all__ = ("EntryID", "EntryIDField", "EntryName", "EntryNameField")


EntryIDTypeVar = TypeVar("EntryIDTypeVar", bound="EntryID")


class EntryID(UUID):
    __slots__ = ()

    def __init__(self, raw: Union[UUID, bytes, str]):
        if isinstance(raw, UUID):
            super().__init__(bytes=raw.bytes)
        elif isinstance(raw, bytes):
            super().__init__(bytes=raw)
        else:
            super().__init__(hex=raw)

    def __repr__(self) -> str:
        return f"<EntryID {self.hex}>"

    @classmethod
    def new(cls: Type[EntryIDTypeVar]) -> EntryIDTypeVar:
        return cls(uuid4())


EntryIDField = fields.uuid_based_field_factory(EntryID)


class EntryName(str):
    __slots__ = ()

    def __new__(cls, raw: str) -> "EntryName":
        raw = normalize("NFC", raw)
        # Stick to UNIX filesystem philosophy:
        # - no `.` or `..` name
        # - no `/` or null byte in the name
        # - max 255 bytes long name
        if (
            not 0 < len(raw.encode("utf8")) < 256
            or raw == "."
            or raw == ".."
            or "/" in raw
            or "\x00" in raw
        ):
            raise ValueError("Invalid entry name")
        return super(EntryName, cls).__new__(cls, raw)


EntryNameField = fields.str_based_field_factory(EntryName)
