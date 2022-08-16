# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from typing import Type
from parsec.types import UUID4
from parsec.serde import fields


class SequesterServiceID(UUID4):
    __slots__ = ()


SequesterServiceIDField: Type[fields.Field] = fields.uuid_based_field_factory(SequesterServiceID)
