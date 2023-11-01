# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

from contextlib import asynccontextmanager
from typing import Any, AsyncGenerator

import anyio
import asyncpg

from parsec.components.blockstore import blockstore_factory
from parsec.components.events import BaseEventsComponent, EventBus
from parsec.components.postgresql.block import PGBlockComponent
from parsec.components.postgresql.handler import PGHandler, send_signal
from parsec.components.postgresql.invite import PGInviteComponent
from parsec.components.postgresql.message import PGMessageComponent
from parsec.components.postgresql.organization import PGOrganizationComponent
from parsec.components.postgresql.ping import PGPingComponent
from parsec.components.postgresql.pki import PGPkiEnrollmentComponent
from parsec.components.postgresql.realm import PGRealmComponent
from parsec.components.postgresql.sequester import PGPSequesterComponent
from parsec.components.postgresql.user import PGUserComponent
from parsec.components.postgresql.vlob import PGVlobComponent
from parsec.config import BackendConfig
from parsec.events import Event
from parsec.webhooks import WebhooksComponent


@asynccontextmanager
async def components_factory(  # type: ignore[misc]
    config: BackendConfig, event_bus: EventBus
) -> AsyncGenerator[dict[str, Any], None]:
    dbh = PGHandler(config.db_url, config.db_min_connections, config.db_max_connections, event_bus)

    async def _send_event(
        event: Event,
        conn: asyncpg.Connection | None = None,
    ) -> None:
        if conn is None:
            async with dbh.pool.acquire() as conn:
                await send_signal(conn, event)
        else:
            await send_signal(conn, event)

    webhooks = WebhooksComponent(config)
    organization = PGOrganizationComponent(dbh=dbh, webhooks=webhooks, config=config)
    user = PGUserComponent(dbh=dbh, event_bus=event_bus)
    invite = PGInviteComponent(dbh=dbh, event_bus=event_bus, config=config)
    message = PGMessageComponent(dbh)
    realm = PGRealmComponent(dbh)
    vlob = PGVlobComponent(dbh)
    ping = PGPingComponent(dbh)
    blockstore = blockstore_factory(config=config.blockstore_config, postgresql_dbh=dbh)
    block = PGBlockComponent(dbh=dbh, blockstore_component=blockstore)
    pki = PGPkiEnrollmentComponent(dbh)
    sequester = PGPSequesterComponent(dbh)
    events = BaseEventsComponent(realm_component=realm, send_event=_send_event)

    components = {
        "events": events,
        "webhooks": webhooks,
        "organization": organization,
        "user": user,
        "invite": invite,
        "message": message,
        "realm": realm,
        "vlob": vlob,
        "ping": ping,
        "block": block,
        "blockstore": blockstore,
        "pki": pki,
        "sequester": sequester,
    }
    for component in components.values():
        method = getattr(component, "register_components", None)
        if method is not None:
            method(**components)

    async with anyio.create_task_group() as task_group:
        await dbh.init(task_group=task_group, events_component=events)
        try:
            yield components

        finally:
            await dbh.teardown()
