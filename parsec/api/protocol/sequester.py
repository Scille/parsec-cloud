# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS


from enum import Enum
from parsec.api.protocol.base import fields
from parsec.serde.schema import BaseSchema
from parsec.serde.serializer import JSONSerializer


class SequesterServiceType(Enum):
    SEQUESTRE = "SEQUESTRE"
    WEBHOOK = "WEBHOOK"


SequesterServiceTypeField = fields.enum_field_factory(SequesterServiceType)


class SequesterRegisterServiceSchema(BaseSchema):
    service_type = SequesterServiceTypeField(required=True)
    service_id = fields.UUID(required=True)
    sequester_der_payload = fields.Bytes(required=True)
    sequester_der_payload_signature = fields.Bytes(required=True)


sequester_register_service_serializer = JSONSerializer(SequesterRegisterServiceSchema)
