from parsec.backend.message import BaseMessageComponent
from collections import defaultdict


class MemoryMessageComponent(BaseMessageComponent):
    def __init__(self, event_bus):
        self.event_bus = event_bus
        self._messages = defaultdict(list)

    async def perform_message_new(self, sender_device_id, recipient_user_id, body):
        self._messages[recipient_user_id].append((sender_device_id, body))
        index = len(self._messages[recipient_user_id])
        self.event_bus.send(
            "message.received", author=sender_device_id, recipient=recipient_user_id, index=index
        )

    async def perform_message_get(self, recipient_user_id, offset):
        return self._messages[recipient_user_id][offset:]
