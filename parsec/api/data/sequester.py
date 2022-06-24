# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from enum import Enum
from typing import Dict, Any
import attr
from parsec.api.data.base import BaseAPIData
from parsec.serde import fields, post_load
from parsec.serde.schema import BaseSchema
from pendulum import DateTime


class EncryptionKeyFormat(Enum):
    RSA = "RSA"


EncryptionKeyFormatFields = fields.enum_field_factory(EncryptionKeyFormat)


@attr.s(slots=True, frozen=True, auto_attribs=True, kw_only=True, eq=False)
class SequesterServiceEncryptionKey(BaseAPIData):
    class SCHEMA_CLS(BaseSchema):

        type = fields.CheckedConstant("sequester_service_encryption_key", required=True)
        encryption_key = fields.Bytes(required=True)
        encryption_key_format = EncryptionKeyFormatFields(
            required=True
        )  # TODO: does it has to be signed
        timestamp = fields.DateTime(required=True)
        service_name = fields.String(required=True)

        @post_load
        def make_obj(self, data: Dict[str, Any]) -> "SequesterServiceEncryptionKey":
            data.pop("type")
            return SequesterServiceEncryptionKey(**data)

    encryption_key: bytes
    encryption_key_format: EncryptionKeyFormat
    timestamp: DateTime
    service_name: str
