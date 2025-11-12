# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    OrganizationID,
    anonymous_cmds,
    anonymous_server_cmds,
    authenticated_account_cmds,
    authenticated_cmds,
    invited_cmds,
)
from parsec.api import api
from parsec.client_context import (
    AnonymousClientContext,
    AnonymousServerClientContext,
    AuthenticatedAccountClientContext,
    AuthenticatedClientContext,
    InvitedClientContext,
)


class BasePingComponent:
    async def ping(self, organization_id: OrganizationID, ping: str) -> None:
        raise NotImplementedError

    @api
    async def authenticated_api_ping(
        self, client_ctx: AuthenticatedClientContext, req: authenticated_cmds.latest.ping.Req
    ) -> authenticated_cmds.latest.ping.Rep:
        await self.ping(client_ctx.organization_id, req.ping)
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

    @api
    async def anonymous_server_api_ping(
        self, client_ctx: AnonymousServerClientContext, req: anonymous_server_cmds.latest.ping.Req
    ) -> anonymous_server_cmds.latest.ping.Rep:
        return anonymous_server_cmds.latest.ping.RepOk(pong=req.ping)

    @api
    async def authenticated_account_api_ping(
        self,
        client_ctx: AuthenticatedAccountClientContext,
        req: authenticated_account_cmds.latest.ping.Req,
    ) -> authenticated_account_cmds.latest.ping.Rep:
        return authenticated_account_cmds.latest.ping.RepOk(pong=req.ping)
