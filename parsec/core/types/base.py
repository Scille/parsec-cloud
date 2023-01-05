# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import attr

from parsec.api.data import BaseData
from parsec.serde import BaseSchema, MsgpackSerializer

__all__ = ("BaseLocalData",)


@attr.s(slots=True, frozen=True, auto_attribs=True, kw_only=True, eq=False)
class BaseLocalData(BaseData):
    """Unsigned and uncompressed base class for local data"""

    SCHEMA_CLS = BaseSchema
    SERIALIZER_CLS = MsgpackSerializer
