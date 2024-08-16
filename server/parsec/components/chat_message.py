from __future__ import annotations

from uuid import UUID

from parsec._parsec import (
    DateTime,
    DeviceID,
    OrganizationID,
    UserID,
    authenticated_cmds,
)
from parsec.api import api
from parsec.client_context import (
    AuthenticatedClientContext,
)


class BaseMessageComponent:
    async def message(self, organization_id: OrganizationID, message_encrypted: bytes) -> None:
        raise NotImplementedError

    async def create(
            self,
            id: int,
            author: DeviceID,
            timestamp: DateTime,
            recipient: UserID,
            messageEncrypted: bytes,
    ) -> authenticated_cmds.latest.chat_create.RepOk:
        return authenticated_cmds.latest.chat_create.RepOk(
            id=id,
            author=author,
            timestamp=timestamp,
            recipient=recipient,
            messageEncrypted=messageEncrypted,
        ) # TODO error handling

    @api
    async def authenticated_api_chat_post(
        self, client_ctx: AuthenticatedClientContext, req: authenticated_cmds.latest.chat_post.Req
    ) -> authenticated_cmds.latest.chat_post.Rep:
        await self.message(client_ctx.organization_id, req.messageEncrypted)
        return authenticated_cmds.latest.chat_post.RepOk()

    @api
    async def api_chat_create(
        self, client_ctx: AuthenticatedClientContext, req: authenticated_cmds.latest.chat_create.Req
    ) -> authenticated_cmds.latest.chat_create.Rep:
        msg = await self.create(
            req.id,
            req.author,
            req.timestamp,
            req.recipient,
            req.messageEncrypted,
        )
        return msg
