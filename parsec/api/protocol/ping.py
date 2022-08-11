# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from parsec.serde import fields
from parsec.api.protocol.base import BaseReqSchema, BaseRepSchema, CmdSerializer


__all__ = ("ping_serializer",)


class PingReqSchema(BaseReqSchema):
    ping = fields.String(required=True)


class PingRepSchema(BaseRepSchema):
    pong = fields.String(required=True)


ping_serializer = CmdSerializer(PingReqSchema, PingRepSchema)
