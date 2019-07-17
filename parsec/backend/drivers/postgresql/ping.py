# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from parsec.backend.drivers.postgresql.handler import PGHandler, send_signal
from parsec.backend.ping import BasePingComponent
from parsec.types import DeviceID, OrganizationID


class PGPingComponent(BasePingComponent):
    def __init__(self, dbh: PGHandler):
        self.dbh = dbh

    async def ping(self, organization_id: OrganizationID, author: DeviceID, ping: str) -> None:
        if not author:
            return
        async with self.dbh.pool.acquire() as conn:
            await send_signal(
                conn, "pinged", organization_id=organization_id, author=author, ping=ping
            )
