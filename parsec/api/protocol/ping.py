# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from parsec.serde import fields
from parsec._parsec import (
    AuthenticatedPingReq,
    AuthenticatedPingRep,
    InvitedPingReq,
    InvitedPingRep,
)
from parsec.api.protocol.base import (
    BaseReqSchema,
    BaseRepSchema,
    CmdSerializer,
    ApiCommandSerializer,
)


__all__ = ("ping_serializer",)


class PingReqSchema(BaseReqSchema):
    ping = fields.String(required=True)


class PingRepSchema(BaseRepSchema):
    pong = fields.String(required=True)


ping_serializer = CmdSerializer(PingReqSchema, PingRepSchema)
auth_ping_serializer = ApiCommandSerializer(AuthenticatedPingReq, AuthenticatedPingRep)
invited_ping_serializer = ApiCommandSerializer(InvitedPingReq, InvitedPingRep)
