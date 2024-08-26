# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from typing import override

from parsec._parsec import OrganizationID
from parsec.components.events import EventBus
from parsec.components.ping import BasePingComponent
from parsec.events import EventPinged


class MemoryPingComponent(BasePingComponent):
    def __init__(self, event_bus: EventBus) -> None:
        self._event_bus = event_bus

    @override
    async def ping(self, organization_id: OrganizationID, ping: str) -> None:
        await self._event_bus.send(EventPinged(organization_id=organization_id, ping=ping))
