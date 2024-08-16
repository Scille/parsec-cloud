from __future__ import annotations

from typing import Any, override

from parsec._parsec import OrganizationID
from parsec.components.chat_message import BaseMessageComponent
from parsec.components.events import EventBus
from parsec.events import EventChatReceived


class MemoryMessageComponent(BaseMessageComponent):
    def __init__(self, event_bus: EventBus) -> None:
        self._event_bus = event_bus

    def register_components(self, **other_components: Any) -> None:
        pass

    @override
    async def message(self, organization_id: OrganizationID, messageContent: str) -> None:
        await self._event_bus.send(EventChatReceived(organization_id=organization_id, messageContent=messageContent))
