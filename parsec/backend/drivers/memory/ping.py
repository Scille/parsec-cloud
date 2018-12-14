from parsec.types import DeviceID
from parsec.event_bus import EventBus
from parsec.backend.ping import BasePingComponent


class MemoryPingComponent(BasePingComponent):
    def __init__(self, event_bus: EventBus):
        self.event_bus = event_bus

    async def ping(self, author: DeviceID, ping: str) -> None:
        if author:
            self.event_bus.send("pinged", author=author, ping=ping)
