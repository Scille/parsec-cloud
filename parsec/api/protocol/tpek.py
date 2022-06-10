# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS


from enum import Enum
from parsec.api.protocol.base import BaseRepSchema, BaseReqSchema, CmdSerializer, fields


class TpekServiceType(Enum):
    SEQUESTRE = "SEQUESTRE"
    WEBHOOK = "WEBHOOK"


TpekServiceTypeField = fields.enum_field_factory(TpekServiceType)


class TpekRegisterServiceReqSchema(BaseReqSchema):
    service_type = TpekServiceTypeField(required=True)
    service_id = fields.UUID(required=True)
    service_certificate = fields.Bytes(required=True)


class TpekRegisterServiceRepSchema(BaseRepSchema):
    pass


tpek_register_service_serializer = CmdSerializer(
    TpekRegisterServiceReqSchema, TpekRegisterServiceRepSchema
)
