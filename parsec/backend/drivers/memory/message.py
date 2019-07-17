# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from collections import defaultdict
from typing import List, Tuple

from pendulum import Pendulum

from parsec.backend.message import BaseMessageComponent
from parsec.event_bus import EventBus
from parsec.types import DeviceID, OrganizationID, UserID


class MemoryMessageComponent(BaseMessageComponent):
    def __init__(self, event_bus: EventBus):
        self.event_bus = event_bus
        self._organizations = defaultdict(lambda: defaultdict(list))

    async def send(
        self,
        organization_id: OrganizationID,
        sender: DeviceID,
        recipient: UserID,
        timestamp: Pendulum,
        body: bytes,
    ) -> None:
        messages = self._organizations[organization_id]
        messages[recipient].append((sender, timestamp, body))
        index = len(messages[recipient])
        self.event_bus.send(
            "message.received",
            organization_id=organization_id,
            author=sender,
            recipient=recipient,
            index=index,
        )

    async def get(
        self, organization_id: OrganizationID, recipient: UserID, offset: int
    ) -> List[Tuple[DeviceID, bytes]]:
        messages = self._organizations[organization_id]
        return messages[recipient][offset:]
