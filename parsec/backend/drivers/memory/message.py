from typing import List, Tuple
from collections import defaultdict

from parsec.types import UserID, DeviceID
from parsec.backend.message import BaseMessageComponent


class MemoryMessageComponent(BaseMessageComponent):
    def __init__(self, event_bus):
        self.event_bus = event_bus
        self._messages = defaultdict(list)

    async def send(self, sender: DeviceID, recipient: UserID, body: bytes) -> None:
        self._messages[recipient].append((sender, body))
        index = len(self._messages[recipient])
        self.event_bus.send("message.received", author=sender, recipient=recipient, index=index)

    async def get(self, recipient: UserID, offset: int) -> List[Tuple[UserID, bytes]]:
        return self._messages[recipient][offset:]
