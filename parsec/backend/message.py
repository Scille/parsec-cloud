# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from typing import List, Tuple
from pendulum import Pendulum, now as pendulum_now

from parsec.types import DeviceID, UserID, OrganizationID
from parsec.api.protocole import message_send_serializer, message_get_serializer
from parsec.backend.utils import catch_protocole_errors
from parsec.crypto import timestamps_in_the_ballpark


class MessageError(Exception):
    pass


class BaseMessageComponent:
    @catch_protocole_errors
    async def api_message_send(self, client_ctx, msg):
        msg = message_send_serializer.req_load(msg)

        now = pendulum_now()
        if not timestamps_in_the_ballpark(msg["timestamp"], now):
            return {"status": "bad_timestamp", "reason": f"Timestamp is out of date."}

        await self.send(
            client_ctx.organization_id,
            client_ctx.device_id,
            msg["recipient"],
            msg["timestamp"],
            msg["body"],
        )

        return message_send_serializer.rep_dump({"status": "ok"})

    @catch_protocole_errors
    async def api_message_get(self, client_ctx, msg):
        msg = message_get_serializer.req_load(msg)

        offset = msg["offset"]
        messages = await self.get(client_ctx.organization_id, client_ctx.user_id, offset)

        return message_get_serializer.rep_dump(
            {
                "status": "ok",
                "messages": [
                    {"count": i, "body": body, "timestamp": timestamp, "sender": sender}
                    for i, (sender, timestamp, body) in enumerate(messages, offset + 1)
                ],
            }
        )

    async def send(
        self,
        organization_id: OrganizationID,
        sender: DeviceID,
        recipient: UserID,
        timestamp: Pendulum,
        body: bytes,
    ) -> None:
        raise NotImplementedError()

    async def get(
        self, organization_id: OrganizationID, recipient: UserID, offset: int
    ) -> List[Tuple[DeviceID, Pendulum, bytes]]:
        raise NotImplementedError()
