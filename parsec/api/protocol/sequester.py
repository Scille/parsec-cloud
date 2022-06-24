# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS


from enum import Enum
from parsec.api.protocol.base import fields
from parsec.serde.schema import BaseSchema


class SequesterServiceType(Enum):
    SEQUESTRE = "SEQUESTRE"
    WEBHOOK = "WEBHOOK"


SequesterServiceTypeField = fields.enum_field_factory(SequesterServiceType)


class SequesterServiceSchema(BaseSchema):
    service_type = SequesterServiceTypeField(required=True)
    service_id = fields.UUID(required=True)
    sequester_encryption_key = fields.Bytes(required=True)
    sequester_encryption_key_signature = fields.Bytes(required=True)
