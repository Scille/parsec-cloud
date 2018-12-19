from typing import List, Tuple

from parsec.types import DeviceID, UserID
from parsec.api.protocole import message_send_serializer, message_get_serializer
from parsec.backend.utils import catch_protocole_errors


class MessageError(Exception):
    pass


class BaseMessageComponent:
    @catch_protocole_errors
    async def api_message_send(self, client_ctx, msg):
        msg = message_send_serializer.req_load(msg)

        await self.send(client_ctx.device_id, msg["recipient"], msg["body"])

        return message_send_serializer.rep_dump({"status": "ok"})

    @catch_protocole_errors
    async def api_message_get(self, client_ctx, msg):
        msg = message_get_serializer.req_load(msg)

        offset = msg["offset"]
        messages = await self.get(client_ctx.user_id, offset)

        return message_get_serializer.rep_dump(
            {
                "status": "ok",
                "messages": [
                    {"count": i, "body": body, "sender": sender}
                    for i, (sender, body) in enumerate(messages, offset + 1)
                ],
            }
        )

    async def send(self, sender: DeviceID, recipient: UserID, body: bytes) -> None:
        raise NotImplementedError()

    async def get(self, recipient: UserID, offset: int) -> List[Tuple[DeviceID, bytes]]:
        raise NotImplementedError()
