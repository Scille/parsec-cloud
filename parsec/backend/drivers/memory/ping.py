from parsec.types import DeviceID, OrganizationID
from parsec.event_bus import EventBus
from parsec.backend.ping import BasePingComponent


class MemoryPingComponent(BasePingComponent):
    def __init__(self, event_bus: EventBus):
        self.event_bus = event_bus

    async def ping(self, organization_id: OrganizationID, author: DeviceID, ping: str) -> None:
        if author:
            self.event_bus.send("pinged", organization_id=organization_id, author=author, ping=ping)
