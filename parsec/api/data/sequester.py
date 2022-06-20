# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from typing import Dict, Any
import attr
from parsec.api.data.base import BaseAPIData
from parsec.serde import fields, post_load
from parsec.serde.schema import BaseSchema
from parsec.sequester_crypto import DerPublicKey
from pendulum import DateTime


@attr.s(slots=True, frozen=True, auto_attribs=True, kw_only=True, eq=False)
class SequesterDerServiceEncryptionKey(BaseAPIData):
    class SCHEMA_CLS(BaseSchema):

        type = fields.CheckedConstant("sequester_service_der_encryption_key", required=True)
        encryption_key = fields.DerPublicKey(required=True)
        timestamp = fields.DateTime(required=True)

        @post_load
        def make_obj(self, data: Dict[str, Any]) -> "SequesterDerServiceEncryptionKey":
            data.pop("type")
            return SequesterDerServiceEncryptionKey(**data)

    encryption_key: DerPublicKey
    timestamp: DateTime
