# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
from __future__ import annotations

from typing import Union

from parsec._parsec import (
    AuthenticatedPingReq,
    AuthenticatedPingRep,
    AuthenticatedPingRepOk,
    ClientType,
    InvitedPingReq,
    InvitedPingRep,
    InvitedPingRepOk,
)
from parsec.api.protocol import DeviceID, OrganizationID
from parsec.backend.client_context import AuthenticatedClientContext, BaseClientContext
from parsec.backend.utils import catch_protocol_errors, api, api_typed_msg_adapter


class BasePingComponent:
    @api(
        "ping",
        client_types=[
            ClientType.AUTHENTICATED,
            ClientType.INVITED,
            ClientType.APIV1_ANONYMOUS,
        ],
    )
    @catch_protocol_errors
    @api_typed_msg_adapter(
        (AuthenticatedPingReq, InvitedPingReq),
        Union[AuthenticatedPingRep, InvitedPingRep],
    )
    async def api_ping(
        self, client_ctx: BaseClientContext, req: AuthenticatedPingReq | InvitedPingReq
    ) -> AuthenticatedPingRep | InvitedPingRep:
        if client_ctx.TYPE == ClientType.AUTHENTICATED:
            assert isinstance(client_ctx, AuthenticatedClientContext)

            await self.ping(client_ctx.organization_id, client_ctx.device_id, req.ping)
            return AuthenticatedPingRepOk(pong=req.ping)
        else:
            return InvitedPingRepOk(pong=req.ping)

    async def ping(self, organization_id: OrganizationID, author: DeviceID, ping: str) -> None:
        raise NotImplementedError()
