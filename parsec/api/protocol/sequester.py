# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS


from enum import Enum
from parsec.api.protocol.base import BaseRepSchema, BaseReqSchema, CmdSerializer, fields


class SequesterServiceType(Enum):
    SEQUESTRE = "SEQUESTRE"
    WEBHOOK = "WEBHOOK"


SequesterServiceTypeField = fields.enum_field_factory(SequesterServiceType)


class SequesterRegisterServiceReqSchema(BaseReqSchema):
    service_type = SequesterServiceTypeField(required=True)
    service_id = fields.UUID(required=True)
    sequester_der_payload = fields.Bytes(required=True)
    sequester_der_payload_signature = fields.Bytes(required=True)


class SequesterRegisterServiceRepSchema(BaseRepSchema):
    pass


sequester_register_service_serializer = CmdSerializer(
    SequesterRegisterServiceReqSchema, SequesterRegisterServiceRepSchema
)
