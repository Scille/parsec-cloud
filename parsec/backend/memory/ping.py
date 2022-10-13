# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
from __future__ import annotations

from parsec.api.protocol import DeviceID, OrganizationID
from parsec.backend.ping import BasePingComponent
from parsec.backend.backend_events import BackendEvent


class MemoryPingComponent(BasePingComponent):
    def __init__(self, send_event):
        self._send_event = send_event

    def register_components(self, **other_components):
        pass

    async def ping(self, organization_id: OrganizationID, author: DeviceID, ping: str) -> None:
        if author:
            await self._send_event(
                BackendEvent.PINGED, organization_id=organization_id, author=author, ping=ping
            )
