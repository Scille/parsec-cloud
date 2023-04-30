# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
from __future__ import annotations

from typing import List, Tuple

from parsec._parsec import DateTime, DeviceID, OrganizationID, UserID, authenticated_cmds
from parsec.backend.client_context import AuthenticatedClientContext
from parsec.backend.utils import api


class MessageError(Exception):
    pass


class BaseMessageComponent:
    @api
    async def api_message_get(
        self, client_ctx: AuthenticatedClientContext, req: authenticated_cmds.latest.message_get.Req
    ) -> authenticated_cmds.latest.message_get.Rep:
        offset = req.offset
        messages = await self.get(client_ctx.organization_id, client_ctx.user_id, offset)

        return authenticated_cmds.latest.message_get.RepOk(
            messages=[
                authenticated_cmds.latest.message_get.Message(
                    count=i, body=body, timestamp=timestamp, sender=sender
                )
                for i, (sender, timestamp, body) in enumerate(messages, offset + 1)
            ],
        )

    async def send(
        self,
        organization_id: OrganizationID,
        sender: DeviceID,
        recipient: UserID,
        timestamp: DateTime,
        body: bytes,
    ) -> None:
        raise NotImplementedError()

    async def get(
        self, organization_id: OrganizationID, recipient: UserID, offset: int
    ) -> List[Tuple[DeviceID, DateTime, bytes]]:
        raise NotImplementedError()
