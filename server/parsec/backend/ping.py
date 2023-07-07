# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

from parsec._parsec import (
    DeviceID,
    OrganizationID,
    anonymous_cmds,
    authenticated_cmds,
    invited_cmds,
)
from parsec.backend.client_context import (
    AnonymousClientContext,
    AuthenticatedClientContext,
    InvitedClientContext,
)
from parsec.backend.utils import api


class BasePingComponent:
    @api
    async def authenticated_api_ping(
        self, client_ctx: AuthenticatedClientContext, req: authenticated_cmds.latest.ping.Req
    ) -> authenticated_cmds.latest.ping.Rep:
        await self.ping(client_ctx.organization_id, client_ctx.device_id, req.ping)
        return authenticated_cmds.latest.ping.RepOk(pong=req.ping)

    @api
    async def invited_api_ping(
        self, client_ctx: InvitedClientContext, req: invited_cmds.latest.ping.Req
    ) -> invited_cmds.latest.ping.Rep:
        return invited_cmds.latest.ping.RepOk(pong=req.ping)

    @api
    async def anonymous_api_ping(
        self, client_ctx: AnonymousClientContext, req: anonymous_cmds.latest.ping.Req
    ) -> anonymous_cmds.latest.ping.Rep:
        return anonymous_cmds.latest.ping.RepOk(pong=req.ping)

    async def ping(self, organization_id: OrganizationID, author: DeviceID, ping: str) -> None:
        raise NotImplementedError()
