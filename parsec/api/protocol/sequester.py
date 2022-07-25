# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from parsec.types import UUID4
from parsec.serde import fields


class SequesterServiceID(UUID4):
    __slots__ = ()


SequesterServiceIDField = fields.uuid_based_field_factory(SequesterServiceID)
