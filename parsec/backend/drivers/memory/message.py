from parsec.backend.message import BaseMessageComponent
from collections import defaultdict


class MemoryMessageComponent(BaseMessageComponent):
    def __init__(self, signal_ns):
        self._signal_message_received = signal_ns.signal("message.received")
        self._messages = defaultdict(list)

    async def perform_message_new(
        self, sender_device_id, recipient_user_id, body, author="anonymous"
    ):
        self._messages[recipient_user_id].append((sender_device_id, body))
        index = len(self._messages[recipient_user_id])
        self._signal_message_received.send(
            None, author=sender_device_id, recipient=recipient_user_id, index=index
        )

    async def perform_message_get(self, recipient_user_id, offset):
        return self._messages[recipient_user_id][offset:]
