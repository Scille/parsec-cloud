# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
from __future__ import annotations

from parsec.api.protocol import DeviceID, OrganizationID
from parsec.backend.backend_events import BackendEvent
from parsec.backend.ping import BasePingComponent
from parsec.backend.postgresql.handler import PGHandler, send_signal


class PGPingComponent(BasePingComponent):
    def __init__(self, dbh: PGHandler):
        self.dbh = dbh

    async def ping(self, organization_id: OrganizationID, author: DeviceID, ping: str) -> None:
        if not author:
            return
        async with self.dbh.pool.acquire() as conn:
            await send_signal(
                conn, BackendEvent.PINGED, organization_id=organization_id, author=author, ping=ping
            )
