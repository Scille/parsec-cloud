# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import SequesterServiceID
from parsec.serde import fields

SequesterServiceIDField = fields.uuid_based_field_factory(SequesterServiceID)
