# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
from __future__ import annotations

from typing import List, Tuple

from parsec._parsec import (
    DateTime,
    MessageGetReq,
    MessageGetRep,
    MessageGetRepOk,
    Message,
)
from parsec.api.protocol import DeviceID, UserID, OrganizationID
from parsec.backend.client_context import AuthenticatedClientContext
from parsec.backend.utils import catch_protocol_errors, api, api_typed_msg_adapter


class MessageError(Exception):
    pass


class BaseMessageComponent:
    @api("message_get")
    @catch_protocol_errors
    @api_typed_msg_adapter(MessageGetReq, MessageGetRep)
    async def api_message_get(
        self, client_ctx: AuthenticatedClientContext, req: MessageGetReq
    ) -> MessageGetRep:
        offset = req.offset
        messages = await self.get(client_ctx.organization_id, client_ctx.user_id, offset)

        return MessageGetRepOk(
            messages=[
                Message(count=i, body=body, timestamp=timestamp, sender=sender)
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
