# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from typing import Type
from parsec.serde import fields
from parsec._parsec import EntryName, EntryID

__all__ = ("EntryID", "EntryIDField", "EntryName", "EntryNameField")


class EntryNameTooLongError(ValueError):
    pass


EntryIDField: Type[fields.Field] = fields.uuid_based_field_factory(EntryID)
EntryNameField: Type[fields.Field] = fields.str_based_field_factory(EntryName)
