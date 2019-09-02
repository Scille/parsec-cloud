# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import re
from uuid import UUID, uuid4

from parsec.serde import fields

__all__ = ("EntryID", "EntryIDField", "EntryName", "EntryNameField")


class EntryID(UUID):
    __slots__ = ()

    def __init__(self, init=None):
        init = uuid4() if init is None else init
        if isinstance(init, UUID):
            super().__init__(bytes=init.bytes)
        elif isinstance(init, bytes):
            super().__init__(bytes=init)
        else:
            super().__init__(hex=init)


EntryIDField = fields.uuid_based_field_factory(EntryID)


class EntryName(str):
    __slots__ = ()
    # TODO: This regex is a bit too loose...
    regex = re.compile(r"^[^/]{1,256}$")

    def __init__(self, raw):
        if not isinstance(raw, str) or not self.regex.match(raw):
            raise ValueError("Invalid entry name")


EntryNameField = fields.str_based_field_factory(EntryName)
