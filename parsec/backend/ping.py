# Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

from parsec.api.protocol import DeviceID, OrganizationID, ping_serializer
from parsec.backend.utils import catch_protocol_errors, api, ClientType


class BasePingComponent:
    @api(
        "ping",
        client_types=[ClientType.AUTHENTICATED, ClientType.INVITED, ClientType.APIV1_ANONYMOUS],
    )
    @catch_protocol_errors
    async def api_ping(self, client_ctx, msg):
        msg = ping_serializer.req_load(msg)
        if client_ctx.TYPE == ClientType.AUTHENTICATED:
            await self.ping(client_ctx.organization_id, client_ctx.device_id, msg["ping"])
        return ping_serializer.rep_dump({"status": "ok", "pong": msg["ping"]})

    async def ping(self, organization_id: OrganizationID, author: DeviceID, ping: str) -> None:
        raise NotImplementedError()
