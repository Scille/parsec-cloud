from parsec.backend.message import BaseMessageComponent
from collections import defaultdict


class MemoryMessageComponent(BaseMessageComponent):
    def __init__(self, *args):
        super().__init__(*args)
        self._messages = defaultdict(list)

    async def perform_message_new(self, sender_device_id, recipient_user_id, body):
        self._messages[recipient_user_id].append((sender_device_id, body))
        self._signal_message_arrived.send(recipient_user_id)

    async def perform_message_get(self, recipient_user_id, offset):
        return self._messages[recipient_user_id][offset:]
