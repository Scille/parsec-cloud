from parsec.backend.message import BaseMessageComponent
from collections import defaultdict


class MemoryMessageComponent(BaseMessageComponent):
    def __init__(self, *args):
        super().__init__(*args)
        self._messages = defaultdict(list)

    async def perform_message_new(self, device_id, recipient, body):
        self._messages[recipient].append((device_id, body))
        self._signal_message_arrived.send(recipient)

    async def perform_message_get(self, id, offset):
        return self._messages[id][offset:]
