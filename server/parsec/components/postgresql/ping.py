# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from typing import override

import asyncpg

from parsec._parsec import OrganizationID
from parsec.components.ping import BasePingComponent
from parsec.components.postgresql.handler import send_signal
from parsec.components.postgresql.utils import transaction
from parsec.events import EventPinged


class PGPingComponent(BasePingComponent):
    def __init__(self, pool: asyncpg.Pool):
        self.pool = pool

    @override
    @transaction
    async def ping(
        self, conn: asyncpg.Connection, organization_id: OrganizationID, ping: str
    ) -> None:
        await send_signal(
            conn,
            EventPinged(organization_id=organization_id, ping=ping),
        )
