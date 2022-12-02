# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import EntryID, EntryName
from parsec.serde import fields

__all__ = ("EntryID", "EntryIDField", "EntryName", "EntryNameField")


EntryIDField = fields.uuid_based_field_factory(EntryID)
EntryNameField = fields.str_based_field_factory(EntryName)
