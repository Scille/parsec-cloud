# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from parsec.serde import fields
from parsec._parsec import EntryName, EntryID

__all__ = ("EntryID", "EntryIDField", "EntryName", "EntryNameField")


class EntryNameTooLongError(ValueError):
    pass


EntryIDField = fields.uuid_based_field_factory(EntryID)
EntryNameField = fields.str_based_field_factory(EntryName)
