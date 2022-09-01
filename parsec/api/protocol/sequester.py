# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from typing import Type
from parsec.serde import fields
from parsec._parsec import SequesterServiceID


SequesterServiceIDField: Type[fields.Field] = fields.uuid_based_field_factory(SequesterServiceID)
