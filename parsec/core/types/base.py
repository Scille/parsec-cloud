# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import attr


from parsec.serde import BaseSchema, MsgpackSerializer
from parsec.api.data import BaseData

__all__ = ("BaseLocalData",)


@attr.s(slots=True, frozen=True, auto_attribs=True, kw_only=True, eq=False)
class BaseLocalData(BaseData):
    """Unsigned and uncompressed base class for local data"""

    SCHEMA_CLS = BaseSchema
    SERIALIZER_CLS = MsgpackSerializer
